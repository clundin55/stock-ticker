use clap::Parser;
use serde::Deserialize;
use serde_json;
use std::collections::HashSet;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// A comma-separated list of stock ticker symbols
    #[arg(short, long)]
    tickers: String,
}

#[derive(Deserialize, Debug, Clone)]
struct StockQuote<'a> {
    symbol: &'a str,
    price: f64,
    change: f64,
}

fn parse_prices<'a>(
    response: &'a str,
    tickers: &HashSet<&'a str>,
) -> Result<Vec<StockQuote<'a>>, Box<dyn std::error::Error>> {
    let quotes: Vec<StockQuote<'a>> = serde_json::from_str(response)?;
    Ok(quotes
        .into_iter()
        .filter(|quote| tickers.contains(quote.symbol))
        .collect())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let tickers: HashSet<&str> = args.tickers.split(',').collect();

    let api_key = env::var("PMP_KEY").map_err(|_| "PMP_KEY environment variable not set")?;
    let url = format!(
        "https://financialmodelingprep.com/stable/quote/?symbol={}&apikey={}",
        args.tickers, api_key
    );

    let resp = reqwest::get(&url).await?.text().await?;
    let quotes = parse_prices(&resp, &tickers)?;

    let mut output = String::new();
    for quote in quotes {
        output.push_str(&format!(
            "{} ${} ({}) ",
            quote.symbol, quote.price, quote.change
        ));
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
        let response = r#"[{"symbol":"GOOG","price":2830.42,"change":10.5},{"symbol":"AAPL","price":142.42,"change":-1.2}]"#;
        let tickers = HashSet::from(["AAPL", "GOOG"]);
        let quotes = parse_prices(response, &tickers).unwrap();
        assert_eq!(quotes.len(), 2);

        let aapl_quote = quotes.iter().find(|q| q.symbol == "AAPL").unwrap();
        assert_eq!(aapl_quote.price, 142.42);
        assert_eq!(aapl_quote.change, -1.2);

        let goog_quote = quotes.iter().find(|q| q.symbol == "GOOG").unwrap();
        assert_eq!(goog_quote.price, 2830.42);
        assert_eq!(goog_quote.change, 10.5);
    }

    #[test]
    fn test_parse_prices_not_found() {
        let response = r#"[{"symbol":"GOOG","price":2830.42}]"#;
        let tickers = HashSet::from(["AAPL"]);
        assert!(parse_prices(response, &tickers).is_err());
    }
}
