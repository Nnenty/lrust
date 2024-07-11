pub use futures::{self, executor::block_on};
pub use std::future::Future;

mod different_async_uses {
    use super::*;

    // this two functions returns equal types
    pub fn async_block() -> impl Future<Output = u8> {
        // async block
        async { 5 }
    }
    pub async fn async_func() -> u8 {
        5
    }
}

mod async_lifetimes {
    use super::*;

    // this function:
    pub async fn foo(x: &u8) -> u8 {
        *x
    }

    // equial to this function with arranged lifetimes:
    pub fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
        async move { *x }
    }

    pub async fn borrow_x(x: &u8) -> u8 {
        *x
    }

    // lifetime of x will extend to returns lifetime:
    pub fn good_lifetime() -> impl Future<Output = u8> {
        async {
            let x: u8 = 5;

            borrow_x(&x).await
        }
    }
    // if you uncomment code under you can see error
    // x is not live enough:
    //
    // fn bad_lifetime() -> impl Future<Output = u8> {
    //     let x: u8 = 5;

    //     borrow_x(&x)
    // }
}

use async_move::{blocks, move_blocks};
mod async_move {
    use super::*;

    // multiple different 'async' blocks can access the same local variable
    pub async fn blocks() {
        let hello = String::from("hello");

        let fut1 = async {
            println!("{hello} async Rust");
        };
        let fut2 = async {
            println!("second {hello}, async Rust");
        };

        futures::join!(fut1, fut2);
    }

    // only 'async move' block can asess the same captured variable
    pub async fn move_blocks() {
        let hello = String::from("hello");

        let fut1 = async move {
            println!("{hello} async Rust");
        };

        // if you uncomment code under, you can see error
        // used of moved value because fut1 captured 'hello':
        //
        // let fut2 = async move {
        //     println!("second {hello}, async Rust");
        // };
    }
}
fn main() {
    // Modules in this program:
    //
    // - rules-demonstration modules:
    // 1. mod different_async_uses
    // 2. mod async_lifetimes
    //
    // - modules with funcs to use:
    // 3. mod async_move

    block_on(blocks());
    block_on(move_blocks());
}
