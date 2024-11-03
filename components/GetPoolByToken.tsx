"use client"
import { useEffect, useState } from "react";
import { useTransaction } from "../interaction/useTransaction";

const GetPoolByToken = () => {
  const [pools, setPools] = useState<any[]>([]);
  const [token, setToken] = useState("");
  const { getPoolByToken } = useTransaction();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const result = await getPoolByToken(token);
    if (result) {
      setPools(result.pools || []);
    }
  };

  return (
    <div className="container mx-auto p-4">
      <h2 className="text-2xl font-bold mb-4">Search Betting Pools by Token</h2>
      
      <form onSubmit={handleSubmit} className="mb-6">
        <div className="flex gap-4">
          <input
            type="text"
            value={token}
            onChange={(e) => setToken(e.target.value)}
            placeholder="Enter token denom"
            className="p-2 border rounded bg-gray-700 text-white"
            required
          />
          <button 
            type="submit"
            className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
          >
            Search
          </button>
        </div>
      </form>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {pools.map((pool, index) => (
          <div key={index} className="bg-gray-800 p-4 rounded-lg shadow">
            <div className="text-white">
              <p className="font-semibold">Token: {pool.token}</p>
              <p>Date: {pool.date}</p>
              <p>Amount: {pool.amount}</p>
              <p>Owner: {pool.owner}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default GetPoolByToken;
