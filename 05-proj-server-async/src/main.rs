use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
pub async fn main() {
  let mut socket_listener = TcpListener::bind("127.0.0.1:4589").await.unwrap();

  loop {
    let (socket, _conn_info) = socket_listener.accept().await.unwrap();

    tokio::spawn(async move {
      process(socket).await;
    });
  }
}

async fn process(_socket: TcpStream) {
  println!("hi");
}
