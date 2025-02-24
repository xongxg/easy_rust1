use std::cell::RefCell;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
}

#[test]
fn test_refcell() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    println!("user_1 active: {:?}", user_1.active);

    user_1.active.replace(false);
    println!("user_1 active: {:?}", user_1.active);

    // 🚧
    let date = 2020;
    user_1
        .active
        .replace_with(|_| if date < 2000 { true } else { false });
    println!("user_1 active: {:?}", user_1.active);

    let borrow_one = user_1.active.borrow_mut(); // first mutable borrow - okay
    let borrow_two = user_1.active.borrow_mut(); // second mutable borrow - not okay
}
