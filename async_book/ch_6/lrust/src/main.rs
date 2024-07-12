use futures::{self, executor::block_on, join, try_join};

use join_macroc::{pure_join::get_music_and_book, try_join::try_get_music_and_book};
mod join_macroc {
    use super::*;

    pub mod pure_join {
        async fn get_music() {
            println!("get music");
        }
        async fn get_book() {
            println!("get book");
        }
        pub async fn get_music_and_book() {
            // will async execute
            futures::join!(get_book(), get_music());
        }
    }
    pub mod try_join {
        use super::*;

        async fn try_get_music() -> Result<(), String> {
            println!("try get music");

            Ok(())
        }
        async fn try_get_book() -> Result<(), String> {
            println!("try get book");

            Err(String::from("error from try_get_book"))
        }
        pub async fn try_get_music_and_book() {
            let get_m = try_get_music();
            let get_b = try_get_book();

            // will async execute
            try_join!(get_m, get_b).unwrap();
        }
    }
}
fn main() {
    // Modules in programm:
    //
    // 1. mod join_macro

    block_on(get_music_and_book());
    block_on(try_get_music_and_book());
}
