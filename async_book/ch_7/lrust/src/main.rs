use futures;

mod question_mark_async {
    use std::io;

    async fn _foo() -> Result<(), io::Error> {
        Ok(())
    }

    pub async fn _nono() {
        let _fut = async {
            // type annotations needed; uncomment code under to see error:

            // foo().await?;
            //
            // Ok(())
        };
    }
    pub async fn _yesyes() {
        let _fut = async {
            _foo().await?;

            // explicit type conversion
            Ok::<(), io::Error>(())
        };
    }
}

use send_approximation::{expected_send, good_use_notsend};
mod send_approximation {
    use std::rc::Rc;

    #[derive(Default)]
    struct NotSend(Rc<()>);

    async fn foo() {}

    pub async fn _not_use_notsend() {
        NotSend::default();

        foo().await;
    }

    // try paste this func in argument of function `expected_send()`
    // to see error:
    pub async fn _bad_use_notsend() {
        let _ns = NotSend { 0: Rc::new(()) };

        foo().await;
    }

    pub async fn good_use_notsend() {
        {
            let _ns = NotSend { 0: Rc::new(()) };
        }

        foo().await;
    }
    pub fn expected_send(_: impl Send) {}
}
fn main() {
    // Modules in programm:
    //
    // 1. mod question_mark_async
    // 2. mod send_approximation
    //
    expected_send(good_use_notsend());
}
