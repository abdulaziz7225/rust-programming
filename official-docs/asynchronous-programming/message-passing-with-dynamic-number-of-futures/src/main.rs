use std::time::Duration;
use std::pin::{Pin, pin};

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        let tx_future_1 = pin!(async move {
            let values = vec![
                String::from("Good"),
                String::from("morning,"),
                String::from("from"),
                String::from("async"),
                String::from("future"),
            ];

            for value in values {
                tx1.send(value).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
        });

        let rx_future = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("Received message: {value}");
            }
        });

        let tx_future = pin!(async move {
            let values = vec![
                String::from("Warm"),
                String::from("greeting,"),
                String::from("to"),
                String::from("all"),
                String::from("Rustaceans"),
            ];

            for value in values {
                tx.send(value).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
        });

        // This macro form works with a dynamic number of futures as long as they all have the same type
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_future_1, tx_future, rx_future];
        trpl::join_all(futures).await;
    });
}
