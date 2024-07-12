mod stream_learn {
    use futures::stream::Stream;
    use futures::stream::StreamExt;
    use futures::stream::TryStreamExt;
    use std::pin::Pin;

    pub async fn jump_around(
        mut stream: Pin<&mut dyn Stream<Item = Result<u8, std::io::Error>>>,
    ) -> Result<(), std::io::Error> {
        use futures::stream::TryStreamExt;
        const MAX_CONCURRENT_JUMPERS: usize = 100;

        stream
            .try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
                jump_n_times(num).await;
                Ok(())
            })
            .await?;

        Ok(())
    }
    pub async fn jump_n_times(num: u8) {
        println!("jump {num}");
    }

    pub async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
        let mut sum = 0;

        while let Some(item) = stream.next().await {
            sum += item;
        }

        sum
    }

    pub async fn sum_with_try_next(
        mut stream: Pin<&mut dyn Stream<Item = Result<i32, std::io::Error>>>,
    ) -> Result<i32, std::io::Error> {
        let mut sum = 0;

        while let Some(item) = stream.try_next().await? {
            sum += item;
        }

        Ok(sum)
    }
}

fn main() {
    // Modules in programm:
    //
    // 1. mod stream
}
