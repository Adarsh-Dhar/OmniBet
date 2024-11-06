import { useState, useEffect } from "react";
import { NibiruTxClient, newSignerFromMnemonic, Testnet, NibiruQuerier } from '@nibiruchain/nibijs';
import { useStore } from "../states/state";


export function useTransaction() {
  const offlineSigner = useStore((state : any) => state.offlineSigner);
  const chain = Testnet(1);
  const [isLoading, setIsLoading] = useState(false)
  const contractAddress = "nibi169wj4wcspujv697lw4rjm6rnamaacpjfnw2k9ddwmtghrf6j02aqxetzen";

  const createPool = async (owner: string, start_date: string, end_date: string, token: string, amount: string, deadline: string) => {
    console.log("offlineSigner", offlineSigner)
    const client = await NibiruTxClient.connectWithSigner(
      chain.endptTm, // RPC endpoint
      offlineSigner
    );
    const executeMsg = {
      CreatePool: {
        start_date: start_date,
        end_date: end_date,
        token: token,
        amount: amount,
        deadline: deadline
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
    const client = await NibiruQuerier.connect(chain.endptTm);
    
    const queryMsg = {
        get_all_pool: {}
    };

    try {
        const result = await client.wasmClient.queryContractSmart(
            contractAddress,
            queryMsg
        );
        console.log('Query result:', result);
        return result;
    } catch (err) {
        console.error('Get all pools failed:', err);
        throw err;
    }
  };

  const getPoolByToken = async (token: string) => {
    const client = await NibiruQuerier.connect(chain.endptTm);
    
    const queryMsg = {
        get_pool_by_token: {
            token: token
        }
    };

    try {
        const result = await client.wasmClient.queryContractSmart(
            contractAddress,
            queryMsg
        );
        console.log('Query result:', result);
        return result;
    } catch (err) {
        console.error('Get pool by token failed:', err);
        throw err;
    }
  };

  const getPoolByDate = async (date: string) => {
    const client = await NibiruQuerier.connect(chain.endptTm);
    
    const queryMsg = {
        get_pool_by_date: {
            date: date
        }
    };

    try {
        const result = await client.wasmClient.queryContractSmart(
            contractAddress,
            queryMsg
        );
        console.log('Query result:', result);
        return result;
    } catch (err) {
        console.error('Get pool by date failed:', err);
        throw err;
    }
  };

  

  const enterBet = async (id: string, amount: string, bet: string,current_date : string, player: string) => {
    try {
      console.log("offlineSigner", offlineSigner)
      const client = await NibiruTxClient.connectWithSigner(
        chain.endptTm,
        offlineSigner
      );
      const executeMsg = {
        EnterBet: {
          id: id,
          current_date: current_date,
          bet: bet
        }
      };
      console.log("client", client)
      console.log("executeMsg", executeMsg)

      const funds = [{
        denom: "unibi",
        amount: amount
      }];

      const fee = "auto";
      setIsLoading(true);
      console.log("wasmClient", client?.wasmClient)
      const executeContract = await client?.wasmClient.execute(
        player,
        contractAddress,
        executeMsg,
        fee,
        "enter_bet",
        funds
      );
      console.log("executeContract", executeContract)

      if(executeContract){
        console.log('Transaction result:', executeContract);
        setIsLoading(false);
      }
    } catch (err) {
      console.error('Failed to retrieve accounts from signer:', err);
      setIsLoading(false);
      throw err;
    }
  };

  const claimBet = async (betId: string, player: string, currentDate: string, realValue: string) => {
    const client = await NibiruTxClient.connectWithSigner(
      chain.endptTm,
      offlineSigner
    );
    const executeMsg = {
      claim_bet: {
        bet_id: betId,
        current_date: currentDate,
        real_value: realValue
      }
    };
    try {
      const fee = "auto";
      setIsLoading(true);
      const executeContract = await client?.wasmClient.execute(
        player,
        contractAddress,
        executeMsg,
        fee
      );

      if(executeContract){
        console.log('Transaction result:', executeContract);
        setIsLoading(false);
      }
    } catch (err) {
      console.error('Claim bet failed:', err);
    }
  };

  return {
    createPool,
    getAllPools,
    getPoolByToken,
    getPoolByDate,
    enterBet,
    claimBet
  }
  
}
