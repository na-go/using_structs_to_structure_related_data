struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn defining_and_instantiating_structs () {


    let mut user1 = User {
        username: String::from("na-go"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true 
    };

    println!("user1's username:{}", user1.username);
    println!("user1's email:{}", user1.email);
    println!("user1's sigm in count:{}", user1.sign_in_count);
    println!("user1's active:{}", user1.active);
    
    user1.sign_in_count = 3;
    println!("user1's sigm in count:{}", user1.sign_in_count);

    let user2 = build_user(String::from("examlpleeeee@example.com"), String::from("whoami"));
    println!("user2's username:{}", user2.username);
}

fn main() {
    defining_and_instantiating_structs();
    
}
