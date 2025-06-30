use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // Writes some prefix of the bytes string, but not necessarily all of it
    let n = file.write(b"some bytes").await?;

    println!("Wrote the first {n} bytes of 'some bytes'");
    Ok(())
}