"use client"

import React, { useEffect, useState } from 'react';
import ChainList from '../Common/ChainList';
import { useRouter } from 'next/navigation';
import { useStore } from '@/states/state';
import axios from 'axios';
import { tokenIdMap } from '../Common/tokenMap';

const Predict = () => {
  const router = useRouter();
  
  const tokenSelected = useStore((token : any) => token.tokenSelected)
  console.log("selected token", tokenSelected)
  const pricesArray = useStore((state : any) => state.prices)
  const timestampsArray = useStore((state : any) => state.timestamps)
  const updatePricesArray = useStore((state : any) => state.changePrices)
  const updateTimestampsArray = useStore((state : any) => state.changeTimestamps)

  const tokenId = tokenIdMap[tokenSelected as keyof typeof tokenIdMap]

  console.log("token id", tokenId)

  const handlePredict = async () => {
          console.log("token id", tokenId)
          const response = await axios.get(`http://localhost:5000/`, {
            params: {
              token: tokenId
            }
          });
          const timestamps = response.data.map((item : any)=> item.timestamp);
          const prices = response.data.map((item : any) => item.price);
          console.log("prices", prices)
          console.log("timestamps", timestamps)
          updatePricesArray(prices);
          updateTimestampsArray(timestamps);
          console.log("prices array", pricesArray)
          console.log("timestamps array", timestampsArray)
      router.push('/CreatePool');

        };
        
      

    
  

  
  

  return (
    <div className="max-w-2xl mx-auto my-10">
      <div className="bg-white rounded-lg shadow-lg border border-gray-200">
        {/* Header */}
        <div className="p-6 border-b border-gray-200">
          <h1 className="text-2xl font-bold text-gray-900">Make a Prediction</h1>
        </div>
        
        {/* Content */}
        <div className="p-6 space-y-6">
          <div>
            <h2 className="text-lg font-semibold mb-2 text-gray-800">Select Primary Token</h2>
            <ChainList />
          </div>
          
         
        </div>

        {/* Footer */}
        <div className="p-6 border-t border-gray-200">
          <button
            onClick={handlePredict}
            className="w-full px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors duration-200"
          >
            Predict
          </button>
        </div>
      </div>
    </div>
  );
};

export default Predict;