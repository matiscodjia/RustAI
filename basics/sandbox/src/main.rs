#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = User {
        email: String::from("machin@gamil.com"),
        username: String::from("machin"),
        active: true,
        sign_in_count: 1,
    };
    dbg!(&user1);
    let _name: String = user1.username;
    user1.username = String::from("wallace");
}
