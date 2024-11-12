"use client"
import { useEffect, useState } from "react";
import { useTransaction } from "../../interaction/useTransaction";
import ChainList from "../Common/ChainList";
import { useStore } from "../../states/state";
import { useRouter } from 'next/navigation';
import { useBetStore } from '@/states/state';
import { 
  Card, 
  CardHeader, 
  CardTitle, 
  CardContent 
} from "@/components/ui/card";

const GetPoolByToken = () => {
  const [pools, setPools] = useState<any[]>([]);
  const { getPoolByToken } = useTransaction();
  const tokenSelected = useStore((state: any) => state.tokenSelected);
  const router = useRouter();
  const changeToken = useBetStore((token: any) => token.changeToken);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (tokenSelected) {
      const result = await getPoolByToken(tokenSelected);
      if (result) {
        setPools(result.pools || []);
      }
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
      <h2 className="text-2xl font-bold mb-4">Search Betting Pools by Token</h2>
      
      <form onSubmit={handleSubmit} className="mb-6">
        <div className="flex gap-4">
          <ChainList />
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
          <Card 
            key={index}
            className="cursor-pointer transform transition-transform hover:scale-105 bg-gray-800"
            onClick={() => handlePoolClick(pool)}
          >
            <CardHeader>
              <CardTitle className="text-white">{pool.token}</CardTitle>
            </CardHeader>
            <CardContent className="text-gray-300">
              <div className="space-y-2">
                <p>Date: {new Date(pool.end_date * 1000).toLocaleDateString()}</p>
                <p>Amount: {pool.total_amount}</p>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
};

export default GetPoolByToken;
