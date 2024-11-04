"use client"
import React, { useState, useRef, useEffect } from 'react';
import { ChevronDown, Check } from 'lucide-react';
import {useStore} from '../states/state';


const ChainList = () => {
  const [open, setOpen] = useState(false);
  const [value, setValue] = useState("");
  const [searchTerm, setSearchTerm] = useState("");
  const dropdownRef = useRef(null);

  
  

  const tokens = [
    {
      value: "ethereum",
      label: "ETH",
      icon: "⟠"
    },
    {
      value: "bitcoin",
      label: "BTC",
      icon: "₿"
    },
    {
      value: "solana", 
      label: "SOL",
      icon: "◎"
    },
    {
      value: "binance",
      label: "BNB",
      icon: "🟡"
    },
    {
      value: "cardano",
      label: "ADA",
      icon: "₳"
    },
    {
      value: "polkadot",
      label: "DOT",
      icon: "●"
    },
    {
      value: "ripple",
      label: "XRP",
      icon: "✕"
    },
    {
      value: "dogecoin",
      label: "DOGE",
      icon: "Ð"
    },
    {
      value: "avalanche",
      label: "AVAX",
      icon: "🔺"
    },
    {
      value: "polygon",
      label: "MATIC",
      icon: "⬡"
    },
    {
      value: "archway",
      label: "ARCH",
      icon: "🏛️"
    },
    {
      value: "coreum",
      label: "CORE",
      icon: "💫"
    },
    {
      value: "injective-protocol",
      label: "INJ",
      icon: "📊"
    },
    {
      value: "nibiru",
      label: "NIBI",
      icon: "🌌"
    }
  ];

  // Format euclid_balance to be more readable
  const formateuclid_balance = (euclid_balance : any) => {
    const num = parseInt(euclid_balance);
    if (num >= 1000000) {
      return `${(num / 1000000).toFixed(2)}M`;
    } else if (num >= 1000) {
      return `${(num / 1000).toFixed(2)}K`;
    }
    return num.toString();
  };

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event : any) => {
        //@ts-ignore
      if (dropdownRef.current && !dropdownRef.current.contains(event.target)) {
        setOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const filteredTokens = tokens.filter(token =>
    token.label.toLowerCase().includes(searchTerm.toLowerCase()) ||
    token.value.toLowerCase().includes(searchTerm.toLowerCase()) 

  );

  const updateToken = useStore((token : any) => token.changeToken)
  const tokenSelected = useStore((token : any) => token.tokenSelected)
  


  return (

             <div className="relative w-64" ref={dropdownRef}>
      <button
        onClick={() => {
          setOpen(!open)
          console.log("slected token from list", tokenSelected)
        }}
        className="w-full flex items-center justify-between px-4 py-2 text-sm bg-white border rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        {value ? (
          <span className="flex items-center">
            <span className="mr-2">
              {tokens.find(token => token.value === value)?.icon}
            </span>
            <span className="font-medium">{tokens.find(token => token.value === value)?.label}</span>

            
          </span>
        ) : (
          "Select chain..."
        )}
        <ChevronDown className="w-4 h-4" />
      </button>

      {open && (
        <div className="absolute w-full mt-1 bg-white border rounded-lg shadow-lg z-10">
          <input
            type="text"
            placeholder="Search chains..."
            className="w-full px-4 py-2 text-sm border-b focus:outline-none"
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
          <div className="max-h-64 overflow-auto">
            {filteredTokens.length === 0 ? (
              <div className="px-4 py-2 text-sm text-gray-500">
                No chains found
              </div>
            ) : (
              filteredTokens.map((token) => (
                <button
                  key={token.value}
                  className="w-full flex items-center justify-between px-4 py-2 text-sm hover:bg-gray-50"
                  onClick={() => {
                    setValue(token.value === value ? "" : token.value);
                    setOpen(false);
                    setSearchTerm("");
                    updateToken(token.value);
                    
                    

                    // console.log(tokenSelected)
                  }}
                >
                  <div className="flex items-center">
                    <span className="w-4 h-4 mr-2">
                      {value === token.value && <Check className="w-4 h-4 text-blue-500" />}
                    </span>
                    <span className="mr-2">{token.icon}</span>
                    <span className="font-medium">{token.label}</span>
                  </div>
                  <div className="flex flex-col items-end">
                    
                  </div>
                </button>
              ))
            )}
          </div>
        </div>
      )}
    </div>

   
  );
};

export default ChainList;