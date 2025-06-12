use std::time::Duration;

fn main() {
    trpl::run(async {
        let future_1 = async {
            for i in 1..=10 {
                println!("Hi number {i} from spawned thread");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let future_2 = async {
            for i in 1..=5 {
                println!("Hi number {i} from main thread");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(future_1, future_2).await;
    });
}
