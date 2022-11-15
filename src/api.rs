extern crate kucoin_rs;

use kucoin_rs::futures::TryStreamExt;
use kucoin_rs::tokio;
use kucoin_rs::failure;
use kucoin_rs::kucoin::client::{Kucoin, Credentials, KucoinEnv};
use kucoin_rs::kucoin::model::websocket::{Subscribe, KucoinWebsocketMsg, WSType, WSTopic, WSResp};


async fn api_helpers() -> Result<(), failure::Error>  {
    // If credentials are needed, generate a new Credentials struct w/ the necessary keys
    let credentials = Credentials::new(
        "xxxxxxxxxxxxxXXXXXXxxx",
        "XXxxxxx-xxxxxx-xXxxxx-xxxx",
        "xxxxxx"
    );

    // Initialize the Kucoin API struct
    let api = Kucoin::new(KucoinEnv::Live, Some(credentials))?;
     
    // Generate the dynamic Public or Private websocket url and endpoint from Kucoin
    // which includes a token required for connecting
    let url = api.get_socket_endpoint(WSType::Public).await?;
     
    // Initialize the websocket
    let mut ws = api.websocket();

    // Generate a Vec<WSTopic> of desired subs. Note they need to be public or private
    // depending on the url
    let subs = vec![WSTopic::Ticker(vec!["BTC-USDT".to_string()])];
     
    // Initalize your subscription and use await to unwrap the future   
    ws.subscribe(url, subs).await?;
     
    // Handle incoming responses matching messages. Note, the message matching is
    // not required for a single subscription but may be desired
    // for more complex event handling for multi-subscribed sockets add the additional
    // KucoinWebSocketMsg matches.
    while let Some(msg) = ws.try_next().await? {
        match msg {
            KucoinWebsocketMsg::TickerMsg(msg) => println!("{:#?}", msg),
            KucoinWebsocketMsg::PongMsg(msg) => println!("{:#?}", msg),     // Optional
            KucoinWebsocketMsg::WelcomeMsg(msg) => println!("{:#?}", msg),  // Optional
            _ => (),
        }
    }
    Ok(())
}
