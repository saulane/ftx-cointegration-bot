import ccxt
import os

from statsmodels.tsa.stattools import adfuller
from statsmodels.tsa.stattools import coint

import pandas as pd
from statsmodels.tsa.stattools import coint


try:
    os.remove("CointPairs.txt")
except Exception as e:
    print(e)

def future_list(exchange, blacklist, resolution, minvolume):
    markets = exchange.fetch_markets()
    futures = dict()
    for i in markets:
        if "id" in i and float(i['info']['volumeUsd24h'])>= minvolume:
            if i["id"] not in blacklist and i["id"].endswith("PERP"):
                    futures[i["symbol"]] = pd.DataFrame(exchange.fetchOHLCV(i["symbol"], timeframe=resolution, limit=1000), columns=['Timestamp', 'Open', 'High', 'Low', 'Close', 'Volume'])["Close"]

    print("Futures Fetched!")
    return futures


def create_pair(futures: dict):
    pairs = []
    for i in futures.keys():
        for j in futures.keys():
            if i != j:
                if [i, j] not in pairs and [j,i] not in pairs:
                    pairs.append([i,j])
    print("Pairs Created!")
    return pairs


def cointegrated_pair(pairs: list, prices: dict):
    coint_pairs = []
    j=0
    for i in pairs:
        j+=1
        x = prices[i[0]]
        y = prices[i[1]]

        if len(x) == len(y):
            c1 = coint(x,y)[1]
            if c1 <= 0.01:
                c2 = coint(y,x)[1]
                if min(c1,c2) == c1:
                    coint_pairs.append(i)
                    print("Found one cointegrated pair:", i, "Total Found:", len(coint_pairs), "Total tested:", j)
                    print(i[0]+","+i[1], file=open("CointPairs.txt", "a"))
                elif min(c1,c2) == c2:
                    coint_pairs.append([i[1],i[0]])
                    print("Found one cointegrated pair:", [i[1],i[0]], "Total Found:", len(coint_pairs), "Total tested:", j)
                    print(i[1]+","+i[0], file=open("CointPairs.txt", "a"))                    
    return coint_pairs


blacklist = ["DOGE-PERP", "VET-PERP", "BTC-PERP", "EOS-PERP"]
exchange = ccxt.ftx({})


futures_list = future_list(exchange, blacklist, "15m", 10000000)
print("Nombres de futures répondant aux critères", len(futures_list))
pairs_list = create_pair(futures_list)
print("Nombre de paires", len(pairs_list))
coint_pairs = cointegrated_pair(pairs_list, futures_list)
print(coint_pairs)
