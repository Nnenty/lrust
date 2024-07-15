use futures;

mod question_mark_async {
    use std::io;

    // type annotations needed
    pub async fn nono() {
        let fut = async {
            // foo().await?;

            // Ok(())
        };
    }
    async fn foo() -> Result<(), io::Error> {
        Ok(())
    }

    pub async fn yesyes() {
        let fut = async {
            foo().await?;

            // explicit type conversion
            Ok::<(), io::Error>(())
        };
    }
}
mod send_approximation {}
fn main() {}
