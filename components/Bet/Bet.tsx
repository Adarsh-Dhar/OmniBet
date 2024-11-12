"use client"
import { useState, useEffect } from 'react';

import { tokenIdMap } from '../Common/tokenMap';
import { useBetStore, useStore } from '@/states/state';
import { useTransaction } from '../../interaction/useTransaction';
import axios from 'axios';


const Bet = () => {
  const [amount, setAmount] = useState('');
  const [prediction, setPrediction] = useState('');
  const tokenSelected = useBetStore((token : any) => token.tokenSelected)
  console.log("token selected in Bet", tokenSelected)
  const tokenId = tokenIdMap[tokenSelected as keyof typeof tokenIdMap]
  console.log("token id", tokenId)
  const { enterBet } = useTransaction();
  const userAddress = useStore((user: any) => user.address)
  console.log("user address", userAddress)
  const poolId = useBetStore((poolId : any) => poolId.poolId)

  const handleBet = () => {
    // Handle betting logic here
    const currentDate = Math.floor(Date.now() / 1000).toString();
    console.log('Betting amount:', amount);
    console.log('Prediction:', prediction);
    console.log("pool id", poolId)
    enterBet(poolId, amount.toString(), prediction.toString(), currentDate,userAddress);
    
}

  

  return (
    <div className="container mx-auto p-4">
      <div className="max-w-md mx-auto">

        <div className="mb-4">
          <label htmlFor="predictionDate" className="block text-sm font-medium text-gray-700">
            Amount
          </label>
          <textarea
            id="amount"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="mt-1 p-2 border border-gray-300 rounded-md w-full"
            placeholder="Enter amount..."
          />
        </div>

        <div className="mb-4">
        <label htmlFor="predictionDate" className="block text-sm font-medium text-gray-700">
            Your Prediction
          </label>
          <textarea
            id="prediction"
            value={prediction}
            onChange={(e) => setPrediction(e.target.value)}
            className="mt-1 p-2 border border-gray-300 rounded-md w-full"
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
