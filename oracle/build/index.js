"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const express = require("express");
const cors = require("cors");
const CoinpaprikaAPI = require("@coinpaprika/api-nodejs-client");
const client = new CoinpaprikaAPI();
const app = express();
app.use(cors());
app.use(express.json());
const getHistoricalTickers = (token) => __awaiter(void 0, void 0, void 0, function* () {
    const start = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000);
    try {
        const historicalTickers = yield client.getAllTickers({
            coinId: token,
            historical: {
                start: start.toISOString().slice(0, 10),
                interval: "1d",
            },
        });
        if (historicalTickers.error)
            throw new Error(historicalTickers.error);
        return historicalTickers.map((ticker) => ({
            timestamp: ticker.timestamp,
            price: ticker.price,
            marketcap: ticker.market_cap,
            volume24h: ticker.volume_24h,
        }));
        console.log(historicalTickers);
    }
    catch (error) {
        console.error(error);
        throw error;
    }
});
app.get("/", (req, res) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        const token = req.query.token;
        const data = yield getHistoricalTickers(token);
        res.json(data);
    }
    catch (error) {
        //@ts-ignore
        res.status(500).json({ error: error.message });
    }
}));
app.listen(5000, () => {
    console.log("Server running on http://localhost:5000");
});
