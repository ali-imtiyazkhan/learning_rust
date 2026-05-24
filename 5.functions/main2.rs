struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn create_user(name: String, email: String) -> User {
    User {
        name,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("---- User Details ----");
    println!("Name        : {}", user.name);
    println!("Email       : {}", user.email);
    println!("Active      : {}", user.active);
    println!("Sign-in Count: {}", user.sign_in_count);
    println!("----------------------");
}

fn sign_in(user: &mut User) {
    user.sign_in_count += 1;
    println!("{} signed in! Total sign-ins: {}", user.name, user.sign_in_count);
}

fn deactivate_user(user: &mut User) {
    user.active = false;
    println!("{}'s account has been deactivated.", user.name);
}

fn main() {
    let mut user1 = create_user(
        String::from("Rahul Sharma"),
        String::from("rahul@example.com"),
    );
    print_user(&user1);

    sign_in(&mut user1);
    sign_in(&mut user1);

    print_user(&user1);

    deactivate_user(&mut user1);

    print_user(&user1);
}