// /pages/api/getHistoricalTickers.js

const CoinpaprikaAPI = require("@coinpaprika/api-nodejs-client");
const {usePriceStore} = require("../states/state")

 const prices = usePriceStore((state) => state.prices);
 const dates = usePriceStore((state) => state.dates);
 const addPrice = usePriceStore((state ) => state.addPrice);
 const addDates = usePriceStore((state ) => state.addDates);
const client = new CoinpaprikaAPI();

async function handler() {
  try {
    const start = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000);

    const historicalTickers = await client.getAllTickers({
      coinId: "btc-bitcoin",
      historical: {
        start: start.toISOString().slice(0, 10),
        interval: "1d",
      },
    });

    if (historicalTickers.error) {
      throw new Error(historicalTickers.error);
    }

    const formattedData = historicalTickers.map((ticker ) => ({
      timestamp: ticker.timestamp,
      price: ticker.price,
      marketcap: ticker.market_cap,
      volume24h: ticker.volume_24h,
    }));

    // Push each price to PRICE_LIST
    addPrice(formattedData.map((data ) => data.price));
    addDates(formattedData.map((data ) => data.timestamp));
    console.log("Prices:", prices);
    console.log("Dates:", dates);

    console.log("Formatted Data:", formattedData);
    

    return formattedData;
  } catch (error) {
    console.error("Error fetching historical tickers:", error);
  }
}

handler()


