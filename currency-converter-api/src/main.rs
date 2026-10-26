use std::error::Error;
use reqwest::Client;
use dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        3 => get_conversion_difference(args[1].to_string(), args[2].to_string()).await.unwrap(),
        4 =>
            get_conversion_amount(
                args[1].to_string(),
                args[2].to_string(),
                args[3].to_string()
            ).await.unwrap(),
        _ => get_listed_currencies().await.unwrap(),
    }

    Ok(())
}

async fn get_conversion_difference(
    base_currency: String,
    conversion_currency: String
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://v6.exchangerate-api.com/v6/{}/pair/{}/{}",
        dotenv::var("KEY").unwrap(),
        base_currency,
        conversion_currency
    );
    let client = Client::new();

    let json_res = client.get(url).send().await?.json::<serde_json::Value>().await?;
    println!(
        "1 {} to {} you receive -> ${:.02?}",
        base_currency,
        conversion_currency,
        json_res["conversion_rate"].as_f64().unwrap()
    );
    Ok(())
}

async fn get_conversion_amount(
    base_currency: String,
    conversion_currency: String,
    base_amount: String
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://v6.exchangerate-api.com/v6/{}/pair/{}/{}/{}",
        dotenv::var("KEY").unwrap(),
        base_currency,
        conversion_currency,
        base_amount
    );
    let client = Client::new();

    let json_res = client.get(url).send().await?.json::<serde_json::Value>().await?;
    println!(
        "{} {} to {} you receive -> ${:.02?}",
        base_amount,
        base_currency,
        conversion_currency,
        json_res["conversion_result"].as_f64().unwrap()
    );
    Ok(())
}

async fn get_listed_currencies() -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://v6.exchangerate-api.com/v6/{}/latest/USD",
        dotenv::var("KEY").unwrap()
    );

    let client = Client::new();
    let json_res = client.get(url).send().await?.json::<serde_json::Value>().await?;

    println!("Currency available");
    if let Some(currencies) = json_res["conversion_rates"].as_object() {
        for key in currencies.keys() {
            println!("{}", key);
        }
    }

    Ok(())
}
