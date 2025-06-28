use clap::Parser;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::env;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// A comma-separated list of stock ticker symbols
    #[arg(short, long)]
    tickers: String,
}

#[derive(Deserialize, Debug, Clone)]
struct StockQuote {
    symbol: String,
    price: f64,
}

fn parse_prices(response: &str, tickers: &[&str]) -> Result<Vec<StockQuote>, Box<dyn std::error::Error>> {
    let quotes: Vec<StockQuote> = serde_json::from_str(response)?;
    let quote_map: HashMap<String, StockQuote> = quotes
        .into_iter()
        .map(|q| (q.symbol.clone(), q))
        .collect();

    let mut found_quotes = Vec::new();
    for ticker in tickers {
        let quote = quote_map
            .get(*ticker)
            .ok_or_else(|| format!("Ticker {} not found in response", ticker))?;
        found_quotes.push(quote.clone());
    }
    Ok(found_quotes)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let tickers: Vec<&str> = args.tickers.split(',').collect();

    let api_key = env::var("PMP_KEY")
        .map_err(|_| "PMP_KEY environment variable not set")?;
    let url = format!(
        "https://financialmodelingprep.com/api/v3/quote/{}?apikey={}",
        args.tickers, api_key
    );

    let resp = reqwest::get(&url).await?.text().await?;
    let quotes = parse_prices(&resp, &tickers)?;

    let mut output = String::new();
    for quote in quotes {
        output.push_str(&format!("{} ${} ", quote.symbol, quote.price));
    }
    println!("{}", output.trim_end());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // API Schema:
    // [
    //   {
    //     "symbol": "AAPL",
    //     "name": "Apple Inc.",
    //     "price": 201.08,
    //     "changesPercentage": 0.039801,
    //     "change": 0.08,
    //     "dayLow": 200.62,
    //     "dayHigh": 203.22,
    //     "yearHigh": 260.1,
    //     "yearLow": 169.21,
    //     "marketCap": 3003290664000,
    //     "priceAvg50": 202.6222,
    //     "priceAvg200": 223.33345,
    //     "exchange": "NASDAQ",
    //     "volume": 70534466,
    //     "avgVolume": 62515124,
    //     "open": 201.895,
    //     "previousClose": 201,
    //     "eps": 7.09,
    //     "pe": 28.36,
    //     "earningsAnnouncement": "2025-07-30T10:59:00.000+0000",
    //     "sharesOutstanding": 14935800000,
    //     "timestamp": 1751054402
    //   }
    // ]
    #[test]
    fn test_parse_prices() {
        let response = r#"[{"symbol":"GOOG","price":2830.42},{"symbol":"AAPL","price":142.42}]"#;
        let tickers = ["AAPL", "GOOG"];
        let quotes = parse_prices(response, &tickers).unwrap();
        assert_eq!(quotes.len(), 2);

        let aapl_quote = quotes.iter().find(|q| q.symbol == "AAPL").unwrap();
        assert_eq!(aapl_quote.price, 142.42);

        let goog_quote = quotes.iter().find(|q| q.symbol == "GOOG").unwrap();
        assert_eq!(goog_quote.price, 2830.42);
    }

    #[test]
    fn test_parse_prices_not_found() {
        let response = r#"[{"symbol":"GOOG","price":2830.42}]"#;
        let tickers = ["AAPL"];
        assert!(parse_prices(response, &tickers).is_err());
    }
}
