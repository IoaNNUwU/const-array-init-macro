#![no_std] 

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
/// Examples:
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
/// This is good `quick-fix` opportunity from your language server.
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
    ([$TYPE:ty; $SIZE:literal], $func_name:ident ) => {
        {
            let mut arr: [$TYPE; $SIZE] = [0; $SIZE];

            let mut i = 0;
            while i < $SIZE {
                arr[i] = $func_name(i);
                i += 1;
            }
            arr
        }
    };
    ([$TYPE:ty; $SIZE:literal], |$name:ident| $func:expr ) => {
        {
            let mut arr: [$TYPE; $SIZE] = [0; $SIZE];

            let mut $name = 0;
            while $name < $SIZE {
                arr[$name] = $func;
                $name += 1;
            }
            arr
        }
    };
}

/// ### Wrapper around [`const_arr`] macro. Allows to specify the type of an array `single` time.
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
    ($NAME:ident, [$TYPE:ty; $SIZE:literal], |$name:ident| $func:expr ) => {
        const $NAME: [$TYPE; $SIZE] = {
            let mut arr: [$TYPE; $SIZE] = [0; $SIZE];

            let mut $name = 0;
            while $name < $SIZE {
                arr[$name] = $func;
                $name += 1;
            }
            arr
        };
    };
    ($NAME:ident, [$TYPE:ty; $SIZE:literal], $func_name:ident ) => {
        const $NAME: [$TYPE; $SIZE] = {
            let mut arr: [$TYPE; $SIZE] = [0; $SIZE];

            let mut i = 0;
            while i < $SIZE {
                arr[i] = $func_name(i);
                i += 1;
            }
            arr
        };
    }
}
