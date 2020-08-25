use bytes::Bytes;
use tokio::sync::oneshot;

// Possible Commands we'll send over the channel between working task and the client connect manager task.
#[derive(Debug)]
enum Command {
  Get {
    key: String,
    resp: Responder<Option<Bytes>>
  },
  Set {
    key: String,
    val: Bytes,
    resp: Responder<()>
  }
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

use tokio::sync::mpsc;
use mini_redis::client;

#[tokio::main]
async fn main() {
  // Create a new channel with a capacity of at most 32.
  let (mut tx, mut rx) = mpsc::channel(32);
  // There are two tasks that want to send messages, so we need two transmitters; each task owns one of the transmitters.
  let mut tx2 = tx.clone();

  // tokio::spawn offers back JoinHandles when spawning tasks. Hold onto these so we can await thread completion before the process exits.
  let t1 = tokio::spawn(async move {
    // Within the worker task, we set up the response channel that the client manager task will use to report back our result. We pass that channel *in the command* we send to the client manager.
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::Get {
      key: "hello".to_string(),
      resp: resp_tx
    };

    // Send the GET request.
    println!("[t1] Sending GET 'hello'.");
    tx.send(cmd).await.unwrap();

    // Await the response.
    let res = resp_rx.await;
    println!("[t1] GOT = {:?}", res);
  });

  let t2 = tokio::spawn(async move {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::Set {
      key: "foo".to_string(),
      val: "bar".into(),
      resp: resp_tx
    };

    // Send the SET request.
    println!("[t2] Sending SET 'foo' 'bar'.");
    tx2.send(cmd).await.unwrap();

    // Await the response.
    let res = resp_rx.await;
    println!("[t2] GOT = {:?}", res);
  });

  let manager = tokio::spawn(async move {
    // First, connect to the server.
    let mut client = client::connect("127.0.0.1:1337").await.unwrap();

    // Start receiving messages from other client threads to communicate with the server.
    while let Some(cmd) = rx.recv().await {
      use Command::*;

      match cmd {
        Get { key, resp } => {
          let res = client.get(&key).await;

          // Forward the response through the response channel. This might fail if the receiving end has been dropped, but we don't care if that happens.
          let _ = resp.send(res);
        }
        Set { key, val, resp } => {
          let res = client.set(&key, val).await;

          // Forward the response through the response channel.
          let _ = resp.send(res);
        }
      }
    }
  });

  // Await the join handles from the two worker tasks and the client manager task.
  t1.await.unwrap();
  t2.await.unwrap();
  manager.await.unwrap();
}
