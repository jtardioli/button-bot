use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
    sol_types::SolEvent,
};
use eyre::Result;
use futures_util::stream::StreamExt;
use tokio::time::{sleep, Duration};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IButton,
    "src/abi/IButton.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting the program");

    let rpc_url = "https://base-mainnet.g.alchemy.com/v2/rQx-6cqyqQMt6JKdFcRGqYR4psGgsPQH".parse()?;
    let ws_rpc_url = "wss://base-mainnet.g.alchemy.com/v2/rQx-6cqyqQMt6JKdFcRGqYR4psGgsPQH";
    let ws = WsConnect::new(ws_rpc_url);
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let ws_provider = ProviderBuilder::new().on_ws(ws).await?;
    let button_address = address!("5F7577811f2069f6181C7B90b4aEC0A34780B48f");
    let button_contract = IButton::new(button_address, provider);

    let filters = Filter::new().address(button_address).from_block(BlockNumberOrTag::Latest);
    let sub = ws_provider.subscribe_logs(&filters).await?;
    let mut stream = sub.into_stream();

    let mut timer: i32 = 60;

    loop {
        tokio::select! {
            // Handle incoming logs from the WebSocket stream
            Some(log) = stream.next() => {
                let deadline = button_contract.deadline().call().await?._0;
                println!("New deadline: {}", deadline);

                // Reset the timer to 60
                timer = 57;
            }

            // Timer countdown logic
            _ = sleep(Duration::from_secs(1)) => {
                if timer > 0 {
                    timer -= 1;
                    println!("Timer: {}", timer);

                    if timer < 5 {
                        println!("uh oh");
                        button_contract.press().send().await?
                    }
                }
            }
        }
    }
}
