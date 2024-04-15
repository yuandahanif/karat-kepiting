struct User {
    active: bool,
    name: String,
    sign_in_count: u32,
    email: String,
}

// tupple struct
struct Color(i32, u32, u32);
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

fn main() {
    let mut user_1 = User {
        active: true,
        name: String::from("tom"),
        sign_in_count: 2042,
        email: String::from("wrong_format@"),
    };
    user_1.email = String::from("a@at.dot");

    let user_2 = user_builder(String::from("abe"), String::from("email@mail.domain"));

    // copy update struct
    let user3 = User {
        email: String::from("same_email@mail.domain"),
        ..user_2
    }; // user_2.name no longer valid

    const BLACK: Color = Color(0, 0, 0);

    let subject = AlwaysEqual;

    println!("{} {} {}", user_1.email, user_2.email, BLACK.1);
}

fn user_builder(name: String, email: String) -> User {
    User {
        name,
        email,
        active: true,
        sign_in_count: 1,
    }
}
