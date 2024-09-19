# `lombok_next`

`lombok_next` is a Rust library that mimics the functionality of `Java's Lombok`. It aims to provide various `Lombok` features through Rust proc-macros. Here are some of the features implemented in `lombok_next`:

# Usage

Below is description of features that `lombok_next` currently supports.

## Getter Derive Macro

```rust
use lombok_next::Getter;

#[derive(Getter)]
struct User {
    username: String,
    email: String,
    password: String,
    #[skip_getter]
    age: u8,
}
```

`Getter` derive macro is used to generate getter methods for the fields of a struct.

so the code above will be expanded to:

```rust
struct User {
    username: String,
    email: String,
    password: String,
    age: u8,
}

// automatically generated by `#[derive(Getter)]`
impl User {
    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn email(&self) -> &String {
        &self.email
    }  

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn age(&self) -> &u8 {
        &self.age
    }
}
```

`#[skip_getter]` is used to skip the generation of a getter method for a specific field.

```rust
#[derive(Getter)]
struct User {
    username: String,
    email: String,
    password: String,
    #[skip_getter]
    age: u8,
}
```

This time, the `age` field will be skipped from getter method generation.

so the code above will be expanded to:

```rust
struct User {
    username: String,
    email: String,
    password: String,
    age: u8,
}

// automatically generated by `#[derive(Getter)]`
impl User {
    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn email(&self) -> &String {
        &self.email
    }  

    pub fn password(&self) -> &String {
        &self.password
    }
}
```

like the above, getter method for `age` field is not generated.

## Setter Derive Macro

```rust
use lombok_next::Setter;

#[derive(Setter)]
struct User {
    username: String,
    email: String,
    password: String,
    age: u8,
}
```

`Setter` derive macro is used to generate setter methods for the fields of a struct.

The setter methods are named with a `set_` prefix followed by the field name.

so the code above will be expanded to:

```rust
struct User {
    username: String,
    email: String,
    password: String,
    age: u8,
}

// automatically generated by `#[derive(Setter)]`
impl User {
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}
```

`#[skip_setter]` is used to skip the generation of a setter method for a specific field.

It's the same as `#[skip_getter]`.

```rust
#[derive(Setter)]
struct User {
    username: String,
    email: String,
    password: String,
    #[skip_setter]
    age: u8,
}

// automatically generated by `#[derive(Setter)]`
struct User {
    username: String,
    email: String,
    password: String,
    age: u8,
}

impl User {
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}
```

like the above, setter method for `age` field is not generated.

---

Rombok is just a hobby project.

It's never gonna be as good as Java's Lombok and no one wants to use it.

So don't use it in production.

