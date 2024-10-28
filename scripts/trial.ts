// /pages/api/getHistoricalTickers.js
const CoinpaprikaAPI = require("@coinpaprika/api-nodejs-client");
const { usePriceStore } = require("../states/state");

const client = new CoinpaprikaAPI();

export default async function handler() {
  try {
    
    const addPrice = usePriceStore((state : any) => state.addPrice);
    const addDates = usePriceStore((state : any) => state.addDates);

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

    const formattedData = historicalTickers.map((ticker : any) => ({
      timestamp: ticker.timestamp,
      price: ticker.price,
      marketcap: ticker.market_cap,
      volume24h: ticker.volume_24h,
    }));

    // Push each price to PRICE_LIST
    addPrice(formattedData.map((data : any) => data.price));
    addDates(formattedData.map((data : any) => data.timestamp));
    

    // res.status(200).json(formattedData);
  } catch (error) {
    console.error("Error fetching historical tickers:", error);
    
  }
}