# Stock Ticker CLI

A simple command-line tool to fetch and display the current price of one or more stock tickers.

## Usage

To use this tool, you will need a free API key from [Financial Modeling Prep](https://site.financialmodelingprep.com/developer/docs).

1.  **Set the API Key:**

    Export your API key as an environment variable named `PMP_KEY`.
    ```bash
    export PMP_KEY="YOUR_API_KEY"
    ```

2.  **Run the CLI:**

    You can run the program using `cargo run`, passing the desired stock symbols as a comma-separated list to the `--tickers` flag.

    ```bash
    cargo run -- --tickers AAPL,GOOG
    ```

## Example

### Input
```bash
PMP_KEY="<EXAMPLE>" cargo run -- --tickers AAPL,GOOG
```

### Output
```
AAPL $201.08 GOOG $178.27 
```
