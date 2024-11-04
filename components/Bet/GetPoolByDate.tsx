"use client"
import { useEffect, useState } from "react";
import { useTransaction } from "../../interaction/useTransaction";
import { useRouter } from 'next/navigation';
import { useBetStore } from '@/states/state';

const GetPoolByDate = () => {
  const [pools, setPools] = useState<any[]>([]);
  const [date, setDate] = useState("");
  const { getPoolByDate } = useTransaction();
  const router = useRouter();
  const changeToken = useBetStore((token: any) => token.changeToken);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    // Convert date to Unix timestamp (seconds) at 00:00:00
    const unixTimestamp = Math.floor(new Date(`${date}T00:00:00Z`).getTime() / 1000);
    console.log("date", date);
    console.log("deadline in query", unixTimestamp.toString());
    const result = await getPoolByDate(unixTimestamp.toString());
    if (result) {
      setPools(result.pools || []);
    }
  };

  const handlePoolClick = (pool: any) => {
    console.log("pool", pool);
    console.log("token", pool.token);
    changeToken(pool.token);
    router.push(`/Predict`);
  };

  return (
    <div className="container mx-auto p-4">
      <h2 className="text-2xl font-bold mb-4">Search Betting Pools by Date</h2>
      
      <form onSubmit={handleSubmit} className="mb-6">
        <div className="flex gap-4">
          <input
            type="date"
            value={date}
            onChange={(e) => setDate(e.target.value)}
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
          <div 
            key={index} 
            className="bg-gray-800 p-4 rounded-lg shadow cursor-pointer transform transition-transform hover:scale-105"
            onClick={() => handlePoolClick(pool)}
          >
            <div className="text-white">
              <p className="font-semibold">Token: {pool.token}</p>
              <p>Date: {new Date(pool.end_date * 1000).toLocaleDateString()}</p>
              <p>Amount: {pool.total_amount}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default GetPoolByDate;
