use clap::Parser;

#[tokio::main]
async fn crypto_info(coin: String) -> Result<String, Box<dyn std::error::Error>> {

    let resp = reqwest::get(format!("https://data.messari.io/api/v1/assets/{}/metrics", coin))
        .await?
        .json::<serde_json::Value>()
        .await?;
    let output = format!("{}", resp["data"]["market_data"]["price_usd"]);

    Ok(output)
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Crypto to check price (btc,eth,sol,ada)
    crypt: String,
}

fn main() {
    let args = Args::parse();
    let crypto = args.crypt;
    println!("Checking {}...!", crypto);
    
    match crypto_info(crypto) {
        Ok(output) => println!("USD {}", output),
        Err(e) => eprintln!("Sorry, something is wrong :( \n  {}", e),
    }
}
