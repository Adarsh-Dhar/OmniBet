"use client"

import React, { useEffect, useState } from 'react';
import ChainList from './ChainList';
import handler from '@/scripts/oracle';
import { useRouter } from 'next/navigation';
import { useStore } from '@/states/state';
import axios from 'axios';

const Predict = () => {
  const [primaryToken, setPrimaryToken] = useState(null);
  const [referenceToken, setReferenceToken] = useState(null);
  const router = useRouter();
  const tokenIdMap = {
    "archway": "arch-archway",
    "nibiru": "nibi-nibiru",
    "coreum": "cor-coreum",
    "injective-protocol": "inj-injective-protocol",
    "neutron": "ntrn-neutron",
    "stargaze": "stars-stargaze",
    // Add more token-tokenId mappings here
  };
  const tokenSelected = useStore((token : any) => token.tokenSelected)
  console.log("selected token", tokenSelected)

  const tokenId = tokenIdMap[tokenSelected as keyof typeof tokenIdMap]

  console.log("token id", tokenId)

  const handlePredict = async () => {
    
      console.log(`Predicting ${primaryToken} against ${referenceToken}`);

      
          const response = await axios.get(`http://localhost:5000/`, {
            params: {
              token: tokenId
            }
          });
          console.log(response.data);
      router.push('/Prediction');

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