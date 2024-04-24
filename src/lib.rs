#![no_std]

//! # Macros used to initialize an array in const context using `closure` syntax or `const fn`.
//! 
//! All macros in this crate implemented using `macro_rules!` which is very **IDE-friendly** way.
//!
//! # Examples
//!
//! Using `const_arr!` macro:
//! ```
//! use const_array_init::const_arr;
//! 
//! const ARR1: [i32; 5] = const_arr!([i32; 5], |i| i as i32 + 1);
//! assert_eq!(ARR1, [1, 2, 3, 4, 5]);
//! 
//! const fn to_i32_plus_one(n: usize) -> i32 {
//!     n as i32 + 1
//! }
//! const ARR2: [i32; 5] = const_arr!([i32; 5], to_i32_plus_one);
//! assert_eq!(ARR2, [1, 2, 3, 4, 5]);
//! ```
//! 
//! Using `make_const_arr!` macro:
//! ```
//! use const_array_init::make_const_arr;
//! 
//! make_const_arr!(ARR1, [i32; 5], |i| i as i32 + 1);
//! assert_eq!(ARR1, [1, 2, 3, 4, 5]);
//! 
//! const fn to_i32_plus_one(n: usize) -> i32 {
//!     n as i32 + 1
//! }
//! 
//! make_const_arr!(ARR2, [i32; 5], to_i32_plus_one);
//! assert_eq!(ARR2, [1, 2, 3, 4, 5]);
//! ```
//! Advanced usage:
//! - Note that `User` isn't `Copy`, yet still you can use it in `const` context with this macro.
//! 
//! ```
//! use const_array_init::const_arr;
//! 
//! #[derive(Debug, PartialEq, Eq)]
//! struct User { id: u32 }
//! 
//! const fn create_user_from_i(i: usize) -> User {
//!     User { id: i as u32 }
//! }
//! 
//! const USERS: [User; 1024] = const_arr!([User; 1024], create_user_from_i);
//! 
//! const USERS2: [User; 1024] = const_arr!([User; 1024], |i| User { id: i as u32 });
//! ```

/// ### Macro used to initialize arrays in constant context
/// #### Supports both `closure` syntax and `const fn` initialization.
/// 
/// Usage:
/// ```ignore
/// const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], CONST_INIT_FN);
/// ```
/// 
/// - `CONST_INIT_FN` is const function or const-like closure from 
/// `array index`(`usize`) to `TYPE`
/// 
/// ### Examples:
/// ```
/// use const_array_init::const_arr;
/// 
/// const ARR1: [i32; 5] = const_arr!([i32; 5], |i| i as i32 + 1);
/// assert_eq!(ARR1, [1, 2, 3, 4, 5]);
/// 
/// const fn to_i32_plus_one(n: usize) -> i32 {
///     n as i32 + 1
/// }
/// 
/// const ARR2: [i32; 5] = const_arr!([i32; 5], to_i32_plus_one);
/// assert_eq!(ARR2, [1, 2, 3, 4, 5]);
/// ```
/// You have to specify array type in const context, even if compiler can infer it.
/// 
/// This is good `quick-fix` opportunity for your language server.
/// 
/// ```ignore
/// const ARR = const_arr!([i32; 5], |i| i as i32);
/// //    ^^^ help: provide a type for the constant: `: [i32; 5]`
/// const ARR: [i32; 5] = const_arr!([i32; 5], |i| i as i32);
/// ```
/// 
/// But if you don't want to specify type twice you can use 
/// - `make_const_arr!(NAME, [TYPE; SIZE], INIT_FN)` macro.
/// 
/// ```
/// use const_array_init::make_const_arr;
/// 
/// make_const_arr!(ARR, [i32; 5], |i| i as i32 + 1);
/// 
/// assert_eq!(ARR, [1, 2, 3, 4, 5]);
/// ```
/// 
/// - See [`make_const_arr`]
#[macro_export]
#[rustfmt::skip]
macro_rules! const_arr {
    ([$TYPE:ty; $SIZE:literal], $func_name:ident) => {
        {
            // Create array of proper SIZE and initialize it with garbage data 
            // using $func_name(0) call as if every value had index 0.
            // 
            // There is no way to create array without initializing it and
            // we cannot initialize it with 0-s because it isn't always valid (e.g. references)
            // and MaybeUninit is unsafe and unstable in const context.
            const TEMP_ITEM: $TYPE = $func_name(0);
            let mut arr: [$TYPE; $SIZE] = [TEMP_ITEM; $SIZE];

            // Initialize array with proper data using $func_name(ind) call
            let mut ind = 0;
            while ind < $SIZE {
                arr[ind] = $func_name(ind);
                ind += 1;
            }
            arr
        }
    };
    ([$TYPE:ty; $SIZE:literal], |$name:ident| $body:expr) => {
        {
            // Create array of proper SIZE and initialize it with garbage data 
            // using $body with $name predefined to 0 as if every value had index 0.
            // 
            // There is no way to create array without initializing it and
            // we cannot initialize it with 0-s because it isn't always valid (e.g. references)
            // and MaybeUninit is is unsafe and unstable in const context.
            #[allow(non_upper_case_globals)]
            let mut arr: [$TYPE; $SIZE] = {
                const $name: usize = 0;
                const TEMP_ITEM: $TYPE = $body;
                [TEMP_ITEM; $SIZE]
            };

            // Initialize array with proper data from closure's body
            let mut $name = 0;
            while $name < $SIZE {
                arr[$name] = $body;
                $name += 1;
            }
            arr
        }
    };
    ([$TYPE:ty; $SIZE:literal], |_| $body:expr ) => {
        {
            const TEMP_ITEM: $TYPE = $body;
            [TEMP_ITEM; $SIZE]
        }
    };
    () => {compile_error!("Please specify array type TYPE: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
    ([$type:ty; $size:literal]) => {compile_error!("Please specify init function INIT_FN: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
    ([$type:ty; $size:literal], ) => {compile_error!("Please specify init function INIT_FN: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
    ([$type:ty; $size:literal], ||) => {compile_error!("Init function has wrong format. It should be |i| i: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
    ([$type:ty; $size:literal], || $_wha:tt) => {compile_error!("Init function has wrong format. It should be |i| i: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
    ([$type:ty; $size:literal], $num:literal) => {compile_error!("Please add |_| to last argument to turn it to closure: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
    ($type:ty) => {compile_error!("Array type has wrong format. It should be [TYPE; SIZE]: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
    ($type:ty, ) => {compile_error!("Array type has wrong format. It should be [TYPE; SIZE]: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
    ($type:ty,$size:literal) => {compile_error!("Array type has wrong format. It should be [TYPE; SIZE]: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
    ($type:ty,$size:literal, $_:tt) => {compile_error!("Array type has wrong format. It should be [TYPE; SIZE]: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
    ($type:ty,$size:literal, |$_:tt| $_n2:tt) => {compile_error!("Array type has wrong format. It should be [TYPE; SIZE]: \n      const ARR: [TYPE; SIZE] = const_arr!([TYPE; SIZE], INIT_FN);\n e.g. const ARR: [i32;  10  ] = const_arr!([i32;  10  ], |i| i as i32);"); };
}

/// ### Wrapper around [`const_arr`] macro. Allows to specify the type of an array `single` time.
/// #### Supports both `closure` syntax and `const fn` initialization.
/// 
/// Usage:
/// ```ignore
/// make_const_arr!(ARR_NAME, [TYPE; SIZE], CONST_INIT_FN);
/// ```
/// 
/// Desugars to:
/// ```ignore
/// const ARR_NAME: [TYPE; SIZE] = const_arr!([TYPE; SIZE], CONST_INIT_FN);
/// ```
/// 
/// - `CONST_INIT_FN` is const function or const-like closure from 
/// `array index`(`usize`) to `TYPE`
/// 
/// Examples:
/// ```
/// use const_array_init::make_const_arr;
/// 
/// make_const_arr!(ARR1, [i32; 5], |i| i as i32 + 1);
/// assert_eq!(ARR1, [1, 2, 3, 4, 5]);
/// 
/// const fn to_i32_plus_one(n: usize) -> i32 {
///     n as i32 + 1
/// }
/// 
/// make_const_arr!(ARR2, [i32; 5], to_i32_plus_one);
/// assert_eq!(ARR2, [1, 2, 3, 4, 5]);
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! make_const_arr {
    ($NAME:ident, [$TYPE:ty; $SIZE:literal], $func_name:ident ) => {
        const $NAME: [$TYPE; $SIZE] = {
            // Create array of proper SIZE and initialize it with garbage data 
            // using $func_name(0) call as if every value had index 0.
            // 
            // There is no way to create array without initializing it and
            // we cannot initialize it with 0-s because it isn't always valid (e.g. references)
            // and MaybeUninit is unsafe and unstable in const context.
            const TEMP_ITEM: $TYPE = $func_name(0);
            let mut arr: [$TYPE; $SIZE] = [TEMP_ITEM; $SIZE];

            // Initialize array with proper data using $func_name(ind) call
            let mut ind = 0;
            while ind < $SIZE {
                arr[ind] = $func_name(ind);
                ind += 1;
            }
            arr
        }
    ;
    };
    ($NAME:ident, [$TYPE:ty; $SIZE:literal], |$name:ident| $body:expr ) => {
        const $NAME: [$TYPE; $SIZE] = {
            // Create array of proper SIZE and initialize it with garbage data 
            // using $body with $name predefined to 0 as if every value had index 0.
            // 
            // There is no way to create array without initializing it and
            // we cannot initialize it with 0-s because it isn't always valid (e.g. references)
            // and MaybeUninit is is unsafe and unstable in const context.
            #[allow(non_upper_case_globals)]
            let mut arr: [$TYPE; $SIZE] = {
                const $name: usize = 0;
                const TEMP_ITEM: $TYPE = $body;
                [TEMP_ITEM; $SIZE]
            };

            // Initialize array with proper data from closure's body
            let mut $name = 0;
            while $name < $SIZE {
                arr[$name] = $body;
                $name += 1;
            }
            arr
        };
    };
    ($NAME:ident, [$TYPE:ty; $SIZE:literal], |_| $body:expr ) => {
        const $NAME: [$TYPE; $SIZE] = {
            const TEMP_ITEM: $TYPE = $body;
            [TEMP_ITEM; $SIZE]
        };
    };
    () => { compile_error!("Please specify array name ARR_NAME: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
    ($_:literal) => { compile_error!("Please specify array name ARR_NAME: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
    ($NAME:ident) => { compile_error!("Please specify array type TYPE: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
    ($NAME:ident, ) => { make_const_arr!($NAME); };
    ($NAME:ident, [$type:ty]) => { compile_error!("Please add SIZE to array type: It should be [TYPE; SIZE]: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
    ($NAME:ident, [$type:ty;]) => { compile_error!("Please add SIZE to array type: It should be [TYPE; SIZE]: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
    ($NAME:ident, [$type:ty;$size:literal]) => { compile_error!("Please specify init function INIT_FN: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
    ($NAME:ident, [$type:ty;$size:literal], $num:literal) => { compile_error!("Please add |_| to last argument to turn it to closure: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
    ($NAME:ident, $_:tt) => { compile_error!("Array type has wrong format. It should be [TYPE; SIZE]: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
    ($NAME:ident, $_n1:tt, $_n2:tt) => { compile_error!("Array type has wrong format. It should be [TYPE; SIZE]: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };($NAME:ident, $_n1:tt, $_n2:tt, $_n3:tt) => { compile_error!("Array type has wrong format. It should be [TYPE; SIZE]: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
    ($NAME:ident, $_n1:tt, $_n2:tt, $_fn_name:ident) => { compile_error!("Array type has wrong format. It should be [TYPE; SIZE]: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
    ($NAME:ident, $_n1:tt, $_n2:tt, |$_cl:tt| $_b:tt) => { compile_error!("Array type has wrong format. It should be [TYPE; SIZE]: \n      make_const_arr!(ARR_NAME, [TYPE; SIZE], INIT_FN);\n e.g. make_const_arr!(MY_ARR  , [i32;  1024], |i| i as i32);"); };
}