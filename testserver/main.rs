extern crate websocket;
extern crate serde;
extern crate serde_json;

use websocket::OwnedMessage;
use websocket::sync::Client;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new web socket client and connect to the gate.io server
    let mut client = Client::connect("wss://ws.gate.io/v3/").unwrap();

    // Subscribe to the depth channel for the BTC_USDT market with a depth of 10 levels and a precision of 0.1
    let subscription_request = r#"{
        "id": 123,
        "method": "depth.subscribe",
        "params": ["BTC_USDT", 10, "0.1"]
    }"#;
    let subscription_message = OwnedMessage::Text(subscription_request.to_string());
    client.send_message(&subscription_message).unwrap();

    // Read messages from the server
    loop {
        match client.recv_message() {
            Ok(message) => {
                println!("Received message: {:?}", message);

                // Handle the received message
                if let OwnedMessage::Text(message_text) = message {
                    // Deserialize the message text into a JSON object
                    let json_object: serde_json::Value = serde_json::from_str(&message_text).unwrap();

                    // Extract the fields from the JSON object and do something with them
                    let channel_name = json_object["params"]["channel"].as_str().unwrap();
                    let bids = json_object["params"]["bids"].as_array().unwrap();
                    let asks = json_object["params"]["asks"].as_array().unwrap();
                    println!("Channel: {}", channel_name);
                    println!("Bids: {:?}", bids);
                    println!("Asks: {:?}", asks);
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

