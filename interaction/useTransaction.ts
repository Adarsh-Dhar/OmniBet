import { useState, useEffect } from "react";
import { NibiruTxClient, newSignerFromMnemonic, Testnet, NibiruQuerier } from '@nibiruchain/nibijs';
import { useStore } from "../states/state";


export function useTransaction() {
  const offlineSigner = useStore((state : any) => state.offlineSigner);
  const chain = Testnet(1);
  const [isLoading, setIsLoading] = useState(false)
  const contractAddress = "nibi1ajy4v4745agfterkvmhet7atylw50ndegna8thaqv7f22e6qgafs8dnam9";

  const createPool = async (owner: string, date: string, token: string, amount: string) => {
    console.log("offlineSigner", offlineSigner)
    const client = await NibiruTxClient.connectWithSigner(
      chain.endptTm, // RPC endpoint
      offlineSigner
    );
    const executeMsg = {
      create_pool: {
        date: date,
        token: token,
        amount: amount
      }
      
    };
    console.log("client", client)
    console.log("executeMsg", executeMsg)
   

      const fee = "auto"; // You can specify the fee if needed
      setIsLoading(true);
      console.log("wasmClient", client?.wasmClient)
      const executeContract = await client?.wasmClient.execute(
        owner,
        contractAddress,
        executeMsg,
        fee
      );
      console.log("executeContract", executeContract)
  
      if(executeContract){
        console.log('Transaction result:', executeContract);
        setIsLoading(false);
        
    }
  
  };

  const getAllPools = async () => {
    const client = await NibiruTxClient.connectWithSigner(
      chain.endptTm,
      offlineSigner
    );
    console.log("client", client)

    const queryMsg = {
      get_all_pool: {}
    };

    try {
      const result = await client?.wasmClient.queryContractSmart(
        contractAddress,
        queryMsg
      );
      return result;
    } catch (err) {
      console.error('Get all pools failed:', err);
      return null;
    }
  };

  const getPoolByToken = async (token: string) => {
    const client = await NibiruTxClient.connectWithSigner(
      chain.endptTm,
      offlineSigner
    );

    const queryMsg = {
      get_pool_by_token: {
        token: token
      }
    };

    try {
      const result = await client?.wasmClient.queryContractSmart(
        contractAddress,
        queryMsg
      );
      return result;
    } catch (err) {
      console.error('Get pool by token failed:', err);
      return null;
    }
  };

  const getPoolByDate = async (date: string) => {
    const client = await NibiruTxClient.connectWithSigner(
      chain.endptTm,
      offlineSigner
    );

    const queryMsg = {
      get_pool_by_date: {
        date: date
      }
    };

    try {
      const result = await client?.wasmClient.queryContractSmart(
        contractAddress,
        queryMsg
      );
      return result;
    } catch (err) {
      console.error('Get pool by date failed:', err);
      return null;
    }
  };

  // const enterBet = async (id: string, amount: string, player: string) => {
  //   const client = await NibiruTxClient.connectWithSigner(
  //     chain.endptTm,
  //     await offlineSigner
  //   );
  //   const executeMsg = {
  //     enter_bet: {
  //       id: id,
  //       amount: amount,
  //       player: player
  //     }
  //   };
  //   try {
  //     const fee = "auto";
  //     setIsLoading(true);
  //     const executeContract = await client?.wasmClient.execute(
  //       player,
  //       contractAddress,
  //       executeMsg,
  //       fee
  //     );

  //     if(executeContract){
  //       console.log('Transaction result:', executeContract);
  //       setIsLoading(false);
  //     }
  //   } catch (err) {
  //     console.error('Enter bet failed:', err);
  //   }
  // };

  // const claimBet = async (betId: string, player: string) => {
  //   const client = await NibiruTxClient.connectWithSigner(
  //     chain.endptTm,
  //     offlineSigner
  //   );
  //   const executeMsg = {
  //     claim_bet: {
  //       bet_id: betId,
  //       player: player
  //     }
  //   };
  //   try {
  //     const fee = "auto";
  //     setIsLoading(true);
  //     const executeContract = await client?.wasmClient.execute(
  //       player,
  //       contractAddress,
  //       executeMsg,
  //       fee
  //     );

  //     if(executeContract){
  //       console.log('Transaction result:', executeContract);
  //       setIsLoading(false);
  //     }
  //   } catch (err) {
  //     console.error('Claim bet failed:', err);
  //   }
  // };

  return {
    createPool,
    getAllPools,
    getPoolByToken,
    getPoolByDate
  }
  
}
