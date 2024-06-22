#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

struct AlwaysEqual;

pub fn basic_examples() {
    {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("some@example.com"),
            sign_in_count: 1,
        };

        user1.email = String::from("another@example.com");

        let user2 = User {
            active: user1.active, 
            username: user1.username, 
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };

        let user3 = User {
            email: String::from("another@example.com"),
            ..user2
        };
        // the String attributes of isntance user1 are not valid after we have assigned them to user2, 
        // because they got moved to user2, so hey are not valid anynmore, the bool and u64 on the other hand
        // got copied, so they are still valid and can be used in user2 and user3

        println!("user3: {:#?}", user3);
    }
    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        // each struct is it's own data type
        println!("black: {}", black.0);
    }
    {
        let subject = AlwaysEqual;
        // AlwaysEqual is a unit-like struct because it has no fields
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true, 
        username,
        email,
        sign_in_count: 1,
    }
}