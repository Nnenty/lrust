use futures::executor::block_on;

use join_macro::{pure_join::get_music_and_book, try_join::try_get_music_and_book};
mod join_macro {
    use futures;

    pub mod pure_join {
        use super::*;

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
            futures::try_join!(get_m, get_b).unwrap();
        }
    }
}

use select_macro::{
    default_and_complete_branches::sum_with_select, fuse_test::run_loop,
    simple_use_example::select_task,
};
mod select_macro {
    pub mod simple_use_example {
        use futures::{future::FutureExt, pin_mut, select};

        async fn first_task() {
            println!("first task")
        }
        async fn second_task() {
            println!("second task")
        }

        pub async fn select_task() {
            let t1 = first_task().fuse();
            let t2 = second_task().fuse();

            pin_mut!(t1, t2);

            select! {
                () = t2 => {
                    println!("second task ends first");
                },
                () = t1 => {
                    println!("first task ends first");
                }
            }
        }
    }

    pub mod default_and_complete_branches {
        use futures::{future, select};

        pub async fn sum_with_select() {
            let mut fut1 = future::ready(2);
            let mut fut2 = future::ready(2);

            let mut sum = 0;

            loop {
                select! {
                    a = fut1 => {
                        sum += a;
                    },
                    b = fut2 => {
                        sum += b;
                    },
                    complete => break,
                    default => unreachable!(),
                };
            }

            assert_eq!(sum, 4);

            println!("sum = {sum}");
        }
    }

    pub mod fuse_test {
        use futures::{
            future::{Fuse, FusedFuture, FutureExt},
            pin_mut, select,
            stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
        };

        async fn get_num() -> u8 {
            5
        }
        async fn run_new_num(_: u8) -> u8 {
            5
        }
        pub async fn run_loop(
            mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
            starting_num: u8,
        ) {
            let mut run_new_num_fut = FuturesUnordered::new();
            run_new_num_fut.push(run_new_num(starting_num));

            let get_num_fut = Fuse::terminated();
            pin_mut!(get_num_fut);

            loop {
                select! {
                    () = interval_timer.select_next_some() => {
                        if get_num_fut.is_terminated(){
                            get_num_fut.set(get_num().fuse());
                        }
                    },
                    new_num = get_num_fut => {
                        run_new_num_fut.push(run_new_num(new_num));
                    },
                    res = run_new_num_fut.select_next_some() =>{
                        println!("run_new_num_fut returned {res:?}");
                    }
                    complete => panic!("`interval_timer` completed unexpectedly"),
                };
            }
        }
    }
}
fn main() {
    // Modules in programm:
    //
    // 1. mod join_macro
    // 2. mod select_macro

    // functions from join_macro module:
    //
    // block_on(get_music_and_book());
    // block_on(try_get_music_and_book());

    // functions from select_macro module:
    //
    // block_on(select_task());
    // block_on(sum_with_select());
}
