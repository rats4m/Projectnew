use tokio::net::TcpListener;
use std::io;
use crate::backend::data_serialization::{deserialize_event, Event};

pub async fn ingest_realtime_data() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening for real-time data on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Received real-time connection: {:?}", socket);

        // Placeholder to read and process real-time data from the socket
        let dummy_data = r#"{"timestamp": 1630456800, "source": "192.168.1.1", "destination": "192.168.1.2", "event_type": "login"}"#;
        let event: Event = deserialize_event(dummy_data).unwrap();
        println!("Deserialized Event: {:?}", event);
    }
}
