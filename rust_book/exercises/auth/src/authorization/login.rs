use crate::authorization;

pub fn login() {
    println!("Login:");

    authorization::enter_username();
    authorization::enter_password();
}
