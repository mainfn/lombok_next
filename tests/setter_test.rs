#![allow(dead_code)]

use lombok_next::{Getter, Setter};

#[derive(Setter, Getter)]
struct User {
    username: String,
    email: String,
    password: String,
    #[skip_setter]
    #[skip_getter]
    age: u8,
}

#[derive(Setter, Getter)]
struct GenericUser<T, Y> {
    username: T,
    age: Y,
}

#[derive(Setter, Getter)]
struct LifetimeUser<'a, 'b, T, Y> {
    username: &'a T,
    age: &'b Y,
}

fn sample_user() -> User {
    User {
        username: "user".to_string(),
        email: "user@example.com".to_string(),
        password: "password".to_string(),
        age: 10,
    }
}

fn sample_generic_user() -> GenericUser<String, u8> {
    GenericUser {
        username: "user".to_string(),
        age: 10,
    }
}

fn sample_lifetime_user<'a, 'b>(
    username: &'a String,
    age: &'b u8,
) -> LifetimeUser<'a, 'b, String, u8> {
    LifetimeUser { username, age }
}

#[test]
fn test_setter_for_struct() {
    let mut user = sample_user();
    assert_eq!(user.username(), "user");
    assert_eq!(user.email(), "user@example.com");
    assert_eq!(user.password(), "password");

    user.set_username("updated".to_string());
    user.set_email("updated@example.com".to_string());
    user.set_password("1234".to_string());

    assert_eq!(user.username(), "updated");
    assert_eq!(user.email(), "updated@example.com");
    assert_eq!(user.password(), "1234");

    // not compiled, `User` doesn't have `set_age`, `age` because of `#[skip_setter]`, `#[skip_getter]`
    // user.set_age(11);
    // assert_eq!(user.age(), 10);
}

#[test]
fn test_setter_for_generic_struct() {
    let mut user = sample_generic_user();

    assert_eq!(user.username(), "user");

    user.set_username("updated".to_string());
    assert_eq!(user.username(), "updated");
}

#[test]
fn test_setter_for_lifetime_struct() {
    let username = "abc".to_string();
    let user = sample_lifetime_user(&username, &10u8);
    let un = user.username();
    println!("{:?}", un);
}
