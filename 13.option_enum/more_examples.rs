fn get_user_id(username: &str) -> Option<u32> {
    if username == "admin" {
        Some(1)
    } else {
        None
    }
}

fn main() {
    // let user name is admin 
    let username = "admin";
    match get_user_id(username) {
        Some(id) => println!("User {} has ID: {}", username, id),
        None => println!("User {} not found", username),
    }
}
