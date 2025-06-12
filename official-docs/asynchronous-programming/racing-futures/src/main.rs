use std::time::Duration;

fn main() {
    trpl::run(async {
        let slow = async {
            println!("'slow' started");
            trpl::sleep(Duration::from_secs(2)).await;
            println!("'slow' finished");
        };

        let fast = async {
            println!("'fast' started");
            trpl::sleep(Duration::from_secs(1)).await;
            println!("'fast' finished");
        };

        // The implementation of this particular race function is not fair. 
        // It always runs the futures passed in as arguments in the order 
        // in which they’re passed. Other implementations are fair and 
        // will randomly choose which future to poll first. 
        trpl::race(slow, fast).await;
        // trpl::race(fast, slow).await;
    }); 
}