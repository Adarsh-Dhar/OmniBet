"use client"
import { useState, useEffect } from 'react';

import { tokenIdMap } from '../Common/tokenMap';
import { useBetStore } from '@/states/state';
import axios from 'axios';
const Bet = () => {
  const [amount, setAmount] = useState('');
  const [prediction, setPrediction] = useState('');
  const tokenSelected = useBetStore((token : any) => token.tokenSelected)
  console.log("token selected in Bet", tokenSelected)
  const tokenId = tokenIdMap[tokenSelected as keyof typeof tokenIdMap]
  console.log("token id", tokenId)

  const handleBet = () => {
    // Handle betting logic here
    console.log('Betting amount:', amount);
    console.log('Prediction:', prediction);

    useEffect(() => {
        const fetchData = async () => {
            const response = await axios.get(`http://localhost:5000/`, {
                params: {
                    token: tokenId
                }
      }
        );
        console.log("response", response)

    }
    fetchData();
  },[])
}

  

  return (
    <div className="container mx-auto p-4">
      <div className="max-w-md mx-auto">
        <div className="mb-4">
          <label htmlFor="amount" className="block text-white mb-2">Amount</label>
          <textarea
            id="amount"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="w-full p-2 bg-gray-700 text-white rounded"
            placeholder="Enter amount..."
          />
        </div>

        <div className="mb-4">
          <label htmlFor="prediction" className="block text-white mb-2">Your Prediction</label>
          <textarea
            id="prediction"
            value={prediction}
            onChange={(e) => setPrediction(e.target.value)}
            className="w-full p-2 bg-gray-700 text-white rounded"
            placeholder="Enter your prediction..."
          />
        </div>

        <button
          onClick={handleBet}
          className="w-full bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
        >
          Bet
        </button>
      </div>
    </div>
  );
};


export default Bet;
