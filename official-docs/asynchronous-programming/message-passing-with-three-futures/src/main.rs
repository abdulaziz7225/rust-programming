use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        let tx_future_1 = async move {
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
        };

        let rx_future = async {
            while let Some(value) = rx.recv().await {
                println!("Received message: {value}");
            }
        };

        let tx_future = async move {
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
        };

        trpl::join3(tx_future_1, tx_future, rx_future).await;
    });
}
