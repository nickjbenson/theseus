use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
pub async fn main() {
  // Bind the TCP listener to the address.
  let mut listener = TcpListener::bind("127.0.0.1:1337").await.unwrap();
  println!("Listening.");

  let db = Arc::new(Mutex::new(HashMap::new()));

  loop {
    // The second item contains the ip and port of the new connection.
    let (socket, _) = listener.accept().await.unwrap();

    // Clone the DB handle.
    let db = db.clone();

    println!("Accepted");
    // Instead of dedicating this whole thread to processing this socket, we spawn a Tokio task to handle this socket. The socket is moved to the new task (its new owner) and processed there.
    // (Tokio may process multiple tasks concurrently on a single thread.)
    // process(socket).await;
    tokio::spawn(async move {
      process(socket, db).await;
    });
  }
}

async fn process(socket: TcpStream, db: Db) {
  use mini_redis::Command::{self, Get, Set};
  use std::collections::HashMap;
  
  // The 'Connection' lets us read/write redis **frames** instead of byte streams. The `Connection` type is defined by mini-redis.
  let mut connection = Connection::new(socket);

  while let Some(frame) = connection.read_frame().await.unwrap() {
    println!("GOT: {:?}", frame);

    let response = match Command::from_frame(frame).unwrap() {
      Set(cmd) => {
          let mut db = db.lock().unwrap();
        db.insert(cmd.key().to_string(), cmd.value().clone());
        Frame::Simple("OK".to_string())
      }
      Get(cmd) => {
        let db = db.lock().unwrap();
        if let Some(value) = db.get(cmd.key()) {
          Frame::Bulk(value.clone())
        } else {
          Frame::Null
        }
      }
      cmd => panic!("unimplemented: {:?}", cmd)
    };

    // connection.write_frame(&response).await.unwrap();
    println!("Responding with: {:?}", response);

    // Respond with an error
    // let response = Frame::Error("unimplemented".to_string());
    connection.write_frame(&response).await.unwrap();
  }
}
