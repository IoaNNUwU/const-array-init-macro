use const_array_init::const_arr;

const ARR1: [i32; 3] = const_arr!([i32; 3], div_2);

const fn div_2(n: usize) -> i32 {
    n as i32 / 2
}

#[test]
fn function_test() {
    assert_eq!(ARR1, [0, 0, 1]);
}

const ARR2: [i32; 3] = const_arr!([i32; 3], |i| i as i32 / 2);

#[test]
fn closure_test() {
    assert_eq!(ARR2, [0, 0, 1]);
}

const ARR3: [i32; 3] = const_arr!([i32; 3], |_| 4 / 2);

#[test]
fn easy_closure_test() {
    assert_eq!(ARR3, [2, 2, 2]);
}

const ARR4: [i32; 3] = const_arr!([i32; 3], |i| {
    let n = 2;
    i as i32 / n
});

#[test]
fn advanced_closure_test() {
    assert_eq!(ARR4, [0, 0, 1]);
}

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u32,
}

const ARR5: [User; 3] = const_arr!([User; 3], |i| User { id: i as u32 });

#[test]
fn super_advanced_closure_test() {
    assert_eq!(ARR5, [User { id: 0 }, User { id: 1 }, User { id: 2 },]);
}

const ARR6: [User; 3] = const_arr!([User; 3], |_| User { id: 1 as u32 });

#[test]
fn super_advanced_closure_test_empty_closure() {
    assert_eq!(ARR6, [User { id: 1 }, User { id: 1 }, User { id: 1 },]);
}


const fn create_user(n: usize) -> User {
    User { id: n as u32 }
}

const ARR7: [User; 3] = const_arr!([User; 3], create_user);

#[test]
fn super_advanced_func_test() {
    assert_eq!(ARR7, [User { id: 0 }, User { id: 1 }, User { id: 2 },]);
}
