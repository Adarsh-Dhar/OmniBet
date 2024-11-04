"use client"
import { useEffect, useState } from "react";
import { useTransaction } from "../../interaction/useTransaction";
import { useRouter } from 'next/navigation';
import { useBetStore } from '@/states/state';

const GetAllPool = () => {
  const [pools, setPools] = useState<any[]>([]);
  const { getAllPools } = useTransaction();
  const router = useRouter();
  const changeToken = useBetStore((token : any) => token.changeToken)

  useEffect(() => {
    const fetchPools = async () => {
      const result = await getAllPools();
      console.log("result", result.pools[0])
      if (result) {
        setPools(result.pools || []);
      }
    };

    fetchPools();
  }, []);

  const handlePoolClick = (pool: any) => {
    console.log("pool", pool)
    console.log("token", pool.token)
    changeToken(pool.token)
    router.push(`/Predict`);
  };

  return (
    <div className="container mx-auto p-4">
      <h2 className="text-2xl font-bold mb-4">All Betting Pools</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {pools.map((pool, index) => (
          <div 
            key={index} 
            className="bg-gray-800 p-4 rounded-lg shadow cursor-pointer transform transition-transform hover:scale-105"
            onClick={() => handlePoolClick(pool)}
          >
            <div className="text-white">
              <p className="font-semibold">Token: {pool.token}</p>
              <p>End Date: {new Date(pool.end_date * 1000).toLocaleDateString()}</p>
              <p>Deadline: {new Date(pool.deadline * 1000).toLocaleDateString()}</p>

              <p>Amount: {pool.total_amount}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default GetAllPool;
