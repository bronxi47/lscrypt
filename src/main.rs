use clap::Parser;

#[tokio::main]
async fn crypto_info(name: String) -> Result<String, Box<dyn std::error::Error>> {

    let resp = reqwest::get(format!("https://data.messari.io/api/v1/assets/{}/metrics", name))
        .await?
        .json::<serde_json::Value>()
        .await?;
        let output = format!("USD {}", resp["data"]["market_data"]["price_usd"]);

        Ok(output)
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Crypto to check price (btc,eth,sol,ada)
    crypto: String,

}

fn main() {
    let args = Args::parse();
        println!("Checking {}...!", args.crypto);
        let crypto = args.crypto;
        println!("{:?}", crypto_info(crypto));
}
