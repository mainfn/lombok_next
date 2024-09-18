# Rombok

Rombok is a Rust library that mimics the functionality of `Java's Lombok`. It aims to provide various `Lombok` features through Rust proc-macros. Here are some of the features implemented in `Rombok`:

1. `#[derive(Getter)]`: Automatically generates getter methods for the fields of a struct. The generated methods have the same visibility as the field and are named after the field.
2. `#[derive(Setter)]`: Automatically generates setter methods for the fields of a struct. The generated methods have the same visibility as the field and are named with a `set_` prefix followed by the field name.
3. `#[skip_getter]`: Skips the generation of a getter method for a specific field.
4. `#[skip_setter]`: Skips the generation of a setter method for a specific field.
5. `#[derive(ToString)]`: Automatically generates a `ToString` implementation for the struct. This feature is already provided by Rust's `std::fmt::Display` trait, so it does not need to be implemented.
6. `#[derive(Equals)]`: Automatically generates `PartialEq` and `Eq` implementations for the struct. This feature is already provided by Rust's `#[derive(PartialEq, Eq)]`, so it does not need to be implemented.
7. `#[derive(Hash)]`: Automatically generates a `Hash` implementation for the struct. This feature is already provided by Rust's `#[derive(Hash)]`, so it does not need to be implemented.
8. `#[derive(Clone)]`: Automatically generates a `Clone` implementation for the struct. This feature is already provided by Rust's `#[derive(Clone)]`, so it does not need to be implemented.
9. `#[derive(Debug)]`: Automatically generates a `Debug` implementation for the struct. This feature is already provided by Rust's `#[derive(Debug)]`, so it does not need to be implemented.

Please note that Rombok is a work in progress and more features will be added in the future. I don't think so but maybe.

---

Rombok is just a hobby project.

It's never gonna be as good as Java's Lombok and no one wants to use it.

So don't use it in production.

---

# ROMBOK

Rombok은 `Java의 Lombok` 기능을 모방한 Rust 라이브러리입니다. 이 라이브러리는 Rust proc-macros를 통해 다양한 `Lombok` 기능을 제공합니다. 다음은 `Rombok`에서 구현된 기능들입니다:

1. `#[derive(Getter)]`: 구조체의 필드에 대한 getter 메서드를 자동으로 생성합니다. 생성된 메서드는 필드와 동일한 가시성을 가지며, 필드 이름을 따릅니다.
2. `#[derive(Setter)]`: 구조체의 필드에 대한 setter 메서드를 자동으로 생성합니다. 생성된 메서드는 필드와 동일한 가시성을 가지며, `set_` 접두사를 붙인 필드 이름을 따릅니다.
3. `#[skip_getter]`: 특정 필드에 대한 getter 메서드 생성을 건너뜁니다.
4. `#[skip_setter]`: 특정 필드에 대한 setter 메서드 생성을 건너뜁니다.
5. `#[derive(ToString)]`: 구조체에 대한 `ToString` 구현을 자동으로 생성합니다. 이 기능은 Rust의 `std::fmt::Display` 트레이트에 의해 이미 제공되므로, 구현할 필요가 없습니다.
6. `#[derive(Equals)]`: 구조체에 대한 `PartialEq` 및 `Eq` 구현을 자동으로 생성합니다. 이 기능은 Rust의 `#[derive(PartialEq, Eq)]`에 의해 이미 제공되므로, 구현할 필요가 없습니다.
7. `#[derive(Hash)]`: 구조체에 대한 `Hash` 구현을 자동으로 생성합니다. 이 기능은 Rust의 `#[derive(Hash)]`에 의해 이미 제공되므로, 구현할 필요가 없습니다.
8. `#[derive(Clone)]`: 구조체에 대한 `Clone` 구현을 자동으로 생성합니다. 이 기능은 Rust의 `#[derive(Clone)]`에 의해 이미 제공되므로, 구현할 필요가 없습니다.
9. `#[derive(Debug)]`: 구조체에 대한 `Debug` 구현을 자동으로 생성합니다. 이 기능은 Rust의 `#[derive(Debug)]`에 의해 이미 제공되므로, 구현할 필요가 없습니다.

Rombok은 현재 진행 중인 프로젝트이며, 앞으로 더 많은 기능이 추가될 예정일지도 모릅니다.

---

Rombok은 걍 취미 프로젝트입니다.

Java의 Lombok보다도 구리며, 아무도 사용하지 않을 것이라 생각하지만 만에 하나라도 프로덕션에서 쓰지 마세요.

좆됩니다.