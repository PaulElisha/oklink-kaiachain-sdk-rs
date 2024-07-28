mod oklink;

use oklink::Oklink;
use tokio;
use types::ProtocolType;

#[tokio::main]
async fn main() {
    let api_key = "YOUR_API_KEY";
    let oklink = Oklink::new(api_key);

    // usage
    match oklink.address_token_balance("0xYourAddress", ProtocolType::Token20, None, None, None).await {
        Ok(info) => println!("{:?}", info),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}