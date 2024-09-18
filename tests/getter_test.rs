use rombok::Getter;

#[derive(Getter)]
struct User {
    username: String,
    email: String,
    password: String,
    #[skip_getter]
    age: u8,
}

#[derive(Getter)]
struct GenericUser<T, Y> {
    username: T,
    age: Y,
}

#[derive(Getter)]
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
fn test_getter_for_struct() {
    let user = sample_user();
    assert_eq!(user.username(), "user");
    assert_eq!(user.email(), "user@example.com");
    assert_eq!(user.password(), "password");
    // not compiled, `User` doesn't have `age` getter because of `#[skip_getter]`
    // assert_eq!(user.age(), 10);
}

#[test]
fn test_getter_for_generic_struct() {
    let user = sample_generic_user();
    assert_eq!(user.username(), "user");
}

#[test]
fn test_getter_for_lifetime_struct() {
    let username = "abc".to_string();
    let user = sample_lifetime_user(&username, &10u8);
    let un = user.username();
    println!("{:?}", un);
    // let age = user.age();
    // assert_eq!(*age, &10u8);
}
