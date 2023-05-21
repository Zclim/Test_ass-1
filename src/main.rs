use websocket::OwnedMessage;
use websocket::ClientBuilder;

struct Client {
    websocket_client: websocket::client::sync::Client<std::net::TcpStream>,
}

impl Client {
    fn connect() -> Result<Client, Box<dyn std::error::Error>> {
        // Create a new WebSocket client builder
        let websocket_client = ClientBuilder::new("wss://ws.gate.io/v4/")?
            .connect_insecure()?;

        Ok(Client { websocket_client })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect()?;

    // Subscribe to the ticker channel for the BTC_USDT
    let ticker_subscription = r#"{
        "time": 123456789,
        "channel": "spot.tickers",
        "event": "subscribe",
        "payload": {
            "contract": "BTC_USDT"
        }
    }"#;
    let ticker_subscription_message = OwnedMessage::Text(ticker_subscription.to_string());
    client.websocket_client.send_message(&ticker_subscription_message)?;

    // Subscribe to the depth channel for the BTC_USDT
    let depth_subscription = r#"{
        "time": 123456789,
        "channel": "spot.order_book_update",
        "event": "subscribe",
        "payload": {
            "contract": "BTC_USDT",
            "interval": "100ms",
            "depth": 10
        }
    }"#;
    let depth_subscription_message = OwnedMessage::Text(depth_subscription.to_string());
    client.websocket_client.send_message(&depth_subscription_message)?;

    // Subscribe to the trade channel for the BTC_USDT
    let trade_subscription = r#"{
        "time": 123456789,
        "channel": "spot.trades",
        "event": "subscribe",
        "payload": {
            "contract": "BTC_USDT"
        }
    }"#;
    let trade_subscription_message = OwnedMessage::Text(trade_subscription.to_string());
    client.websocket_client.send_message(&trade_subscription_message)?;

    // Read messages from the server
    loop {
        match client.websocket_client.recv_message() {
            Ok(message) => {
                println!("Received message: {:?}", message);

                // Handle received message
                if let OwnedMessage::Text(message_text) = message {
                    // Deserialize message text into a JSON object
                    let json_object: serde_json::Value = serde_json::from_str(&message_text)?;

                    // Extract the fields from the JSON object based on the channel
                    let channel = json_object["channel"].as_str().unwrap();
                    match channel {
                        "spot.tickers" => {
                            // Process ticker data
                            let ticker_data = json_object["payload"]["data"].as_array().unwrap();
                            println!("Ticker Data: {:?}", ticker_data);
                        }
                        "spot.order_book_update" => {
                            // Process order book data
                            let contract = json_object["payload"]["contract"].as_str().unwrap();
                            let bids = json_object["payload"]["bids"].as_array().unwrap();
                            let asks = json_object["payload"]["asks"].as_array().unwrap();
                            println!("Contract: {}", contract);
                            println!("Bids: {:?}", bids);
                            println!("Asks: {:?}", asks);
                        }
                        "spot.trades" => {
                            // Process trade data
                            let trade_data = json_object["payload"]["data"].as_array().unwrap();
                            println!("Trade Data: {:?}", trade_data);
                        }
                        _ => {
                            // Handle other channels if necessary
                        }
                    }
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
                break;
            }
        }
    }

    Ok(())
}

