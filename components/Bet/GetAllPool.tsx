"use client"
import { useEffect, useState } from "react";
import { useTransaction } from "../../interaction/useTransaction";
import { useRouter } from 'next/navigation';
import { useBetStore, useStore } from '@/states/state';
import { Card, CardHeader, CardTitle, CardContent } from "../ui/card";
import { Button } from "../ui/button";
import axios from "axios";
import { tokenIdMap } from "../Common/tokenMap";


const GetAllPool = () => {
  const [pools, setPools] = useState<any[]>([]);
  const { getAllPools } = useTransaction();
  const router = useRouter();
  const changeToken = useBetStore((token : any) => token.changeToken)
  const changePoolId = useBetStore((poolId : any) => poolId.changePoolId)
  const claimBet = useTransaction().claimBet;
  const userAddress = useStore((state : any) => state.address)

  useEffect(() => {
    const fetchPools = async () => {
      const result = await getAllPools();
      console.log("result", result)
      if (result) {
        setPools(result.pools || []);
      }
    };

    fetchPools();
  }, []);

  const handlePoolClick = async (pool: any) => {
    if (pool.status === "vote") {
      console.log("pool", pool)
      console.log("token", pool.token)
      changeToken(pool.token)
      console.log("pool id", pool.id)
      changePoolId(pool.id)
      router.push(`/Predict`);
    } else {
      const token = pool.token
      const tokenId = tokenIdMap[token as keyof typeof tokenIdMap]
      console.log("pool end date", pool.end_date)
      const endDate = new Date(pool.end_date * 1000).toISOString().slice(0, 10)
      console.log("endDate", endDate)
      console.log("tokenId", tokenId)
      // const response = await axios.get(`http://localhost:5000/byDate/`, {
      //   params: {
      //     token: tokenId,
      //     date : pool.end_date
      //   }
      // });
      // console.log("response", response.data.price)
      // const real_value = response.data.price;
      const bet_id = pool.id;
      const current_date = Math.floor(new Date().getTime() / 1000).toString();
      console.log("current date", current_date)
      const tx = claimBet(bet_id,userAddress,current_date,"88046.12")
      console.log("tx", tx)
    }
  };

  return (
    <div className="container mx-auto p-4">
      <h2 className="text-2xl font-bold mb-4">All Betting Pools</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {pools.map((pool, index) => (
          <Card 
            key={index}
            className="cursor-pointer transform transition-transform hover:scale-105"

          >
            <CardHeader>
              <CardTitle className="text-lg">Token: {pool.token}</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-2">


                <p>End Date: {new Date(pool.end_date * 1000).toLocaleDateString()}</p>
                <p>Deadline: {new Date(pool.deadline * 1000).toLocaleDateString()}</p>
                <p>Amount: {pool.total_amount / 1000000}</p>
                <p>Status: {pool.status}</p>

                <Button 
                  className="w-full mt-2"
                  onClick={(e) => {
                    e.stopPropagation();
                    handlePoolClick(pool);
                  }}
                >
                  {pool.status === "claim" ? "Claim" : "Bet"}
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
};

export default GetAllPool;
