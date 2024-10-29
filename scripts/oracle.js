// /pages/api/getHistoricalTickers.js
const CoinpaprikaAPI = require("@coinpaprika/api-nodejs-client");


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
      timestamp: ticker.timestamp.slice(0, 10),
      price: ticker.price,
      marketcap: ticker.market_cap,
      volume24h: ticker.volume_24h,
    }));

    console.log(formattedData)
    

    // res.status(200).json(formattedData);
  } catch (error) {
    console.error("Error fetching historical tickers:", error);
    
  }
}

handler()