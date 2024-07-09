use futures::executor::block_on;

async fn meow() {
    println!("meow")
}
async fn hello_during_sleep() {
    meow().await;
    sleep().await;
}
async fn sleep() {
    println!("sleeping");
}
async fn meow_hello() {
    futures::join!(hello_during_sleep(), hello());
}
async fn hello() {
    println!("hello")
}

fn main() {
    block_on(meow_hello());
}
