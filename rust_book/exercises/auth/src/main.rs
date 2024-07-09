mod authorization;
use authorization::{login, registration};

fn main() {
    registration::registration();

    login::login();
}
