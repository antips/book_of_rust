fn main() {
    instanciate_user();
    update_attribute();
    duplicate_user_but_change_one_attribute();
    tuple_structs();
    unit_like_structs();
}

struct User {
    _active: bool,
    _username: String,
    email: String,
    _sign_in_count: u64,
}

fn instanciate_user() {
    let _user1 = User {
        _active: true,
        _username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        _sign_in_count: 1,
    };
}

fn update_attribute() {
    let mut user1 = User {
        _active: true,
        _username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        _sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        _active: true,
        _username: username,
        email,
        _sign_in_count: 1,
    }
}

fn duplicate_user_but_change_one_attribute() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn unit_like_structs() {
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}
