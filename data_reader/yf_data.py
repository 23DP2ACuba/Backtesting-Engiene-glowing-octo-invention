import yfinance as yf
import os

def check_connection(check = "check"):
    return f"connnected, msg>> {check}"

def get_data(ticker, start_date, end_date, period):
    # data = yf.Ticker(ticker).history(start = start_date, end = end_date, period = period)
    data = yf.Ticker("MSFT").history(start="2024-05-11", end="2025-05-11")
    volume = data.Volume
    data.drop(columns=["Stock Splits", "Dividends", "Volume"], inplace=True)
    data["Adj Close"], data["Volume"] = data.Close, volume
    fn = ticker + ".csv"
    data.to_csv(fn)
    return fn

def delete_data(fn):
    os.remove(fn)
    print(f"removed file{fn}")

get_data(ticker = "MSFT", period="1d", start_date="2020-01-01", end_date="2026-01-01")