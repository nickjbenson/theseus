use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
  // Open a connection to the mini-redis server.
  let mut client = client::connect("127.0.0.1:1337").await?;

  // Set the key "hello" to "world".
  client.set("hello", "world".into()).await?;

  // Get key "hello".
  let result = client.get("hello").await?;

  println!("Got back key's value from the server: {:?}", result);

  Ok(())
}
