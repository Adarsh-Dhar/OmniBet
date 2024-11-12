const express = require("express");
const cors = require("cors");


const CoinpaprikaAPI = require("@coinpaprika/api-nodejs-client");
const client = new CoinpaprikaAPI();
import { Router, Request, Response } from "express";

const app = express();


app.use(cors());
app.use(express.json());

const getHistoricalTickers = async (token: any) => {
  const start = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000);
  console.log("start", start)
  console.log("start string", start.toISOString().slice(0, 10))
  try {
    const historicalTickers = await client.getAllTickers({
      coinId: token,
      historical: {
        start: start.toISOString().slice(0, 10),
        interval: "1d",
      },
    });

    if (historicalTickers.error) throw new Error(historicalTickers.error);

    return historicalTickers.map((ticker : any) => ({
      timestamp: ticker.timestamp.slice(0, 10),
      price: ticker.price,
    }));
    console.log(historicalTickers)
  } catch (error) {
    console.error(error);
    throw error;
  }
};

const getPriceByDate = async (token: any, date: any) => {
  try {
    const historicalTickers = await client.getAllTickers({
      coinId: token,
      historical: {
        start: date,
        interval: "1d",
      },
    });

    if (historicalTickers.error) throw new Error(historicalTickers.error);

    // Get the first (and should be only) ticker for that date
    const ticker = historicalTickers[0];
    if (!ticker) {
      throw new Error('No price data available for the specified date');
    }

    return {
      timestamp: ticker.timestamp.slice(0, 10),
      price: ticker.price
    };
  } catch (error) {
    console.error(error);
    throw error;
  }
};

app.get("/", async (req : Request, res : Response) => {
  try {
    const token  = req.query.token;
    const data = await getHistoricalTickers(token);
    res.json(data);
  } catch (error) {
    //@ts-ignore
    res.status(500).json({ error: error.message });
  }
});

app.get("/byDate", async (req : Request, res : Response) => {
  try {
    const token  = req.query.token;
    const date = req.query.date;
    const data = await getPriceByDate(token,date);
    res.json(data);
  } catch (error) {
    //@ts-ignore
    res.status(500).json({ error: error.message });
  }
});

app.listen(5000, () => {
  console.log("Server running on http://localhost:5000");
});
