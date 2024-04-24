# `const_array_init` Rust crate

### Macros used to initialize an array in const context using `closure` syntax or `const fn`.

All macros in this crate implemented using `macro_rules!` which is very **IDE-friendly** way.

### Examples
Using `const_arr!` macro:
```rust
use const_array_init::const_arr;

const ARR1: [i32; 5] = const_arr!([i32; 5], |i| i as i32 + 1);
assert_eq!(ARR1, [1, 2, 3, 4, 5]);

const fn to_i32_plus_one(n: usize) -> i32 {
    n as i32 + 1
}
const ARR2: [i32; 5] = const_arr!([i32; 5], to_i32_plus_one);
assert_eq!(ARR2, [1, 2, 3, 4, 5]);
```
 
Using `make_const_arr!` macro:
```rust
use const_array_init::make_const_arr;

make_const_arr!(ARR1, [i32; 5], |i| i as i32 + 1);
assert_eq!(ARR1, [1, 2, 3, 4, 5]);

const fn to_i32_plus_one(n: usize) -> i32 {
    n as i32 + 1
}

make_const_arr!(ARR2, [i32; 5], to_i32_plus_one);
assert_eq!(ARR2, [1, 2, 3, 4, 5]);
```
Advanced usage:
- Note that `User` isn't `Copy`, yet still you can use it in `const` context with this macro.

```rust
use const_array_init::const_arr;

#[derive(Debug, PartialEq, Eq)]
struct User { id: u32 }

const fn create_user_from_i(i: usize) -> User {
    User { id: i as u32 }
}

const USERS: [User; 1024] = const_arr!([User; 1024], create_user_from_i);

const USERS2: [User; 1024] = const_arr!([User; 1024], |i| User { id: i as u32 });
```