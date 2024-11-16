import { useState, useEffect } from "react";
import { NibiruTxClient, newSignerFromMnemonic, Testnet, NibiruQuerier } from '@nibiruchain/nibijs';
import { useStore } from "../states/state";


export function useTransaction() {
  const offlineSigner = useStore((state : any) => state.offlineSigner);
  const chain = Testnet(1);
  const [isLoading, setIsLoading] = useState(false)
  const contractAddress = "nibi1qx3hw4relp7mp8x3h84xjkcp7h85l0jkrv8y4cnv49mtu3wl8f8s6ne8a0";
  const current_time = Math.floor(new Date().getTime() / 1000).toString();
  console.log("current_time", current_time)

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
        get_all_pool: {
          current_time: current_time
        },
  
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
            token: token,
            current_time: current_time
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
            date: date,
            current_time: current_time
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

  

  const enterBet = async (id: string, amount: string, bet: string,current_date : string,player: string) => {
    try {
      if (!offlineSigner) {
        throw new Error("Offline signer not initialized");
      }

      const accounts = await offlineSigner.getAccounts();
      if (!accounts || accounts.length === 0) {
        throw new Error("No accounts found in signer");
      }

      console.log("offlineSigner", offlineSigner)
      const client = await NibiruTxClient.connectWithSigner(
        chain.endptTm,
        offlineSigner
      );

      if (!client) {
        throw new Error("Failed to initialize client");
      }

      const executeMsg = {
        EnterBet: {
          id: id,
          current_date: current_date,
          bet: bet
        }
      };
      console.log("client", client)
      console.log("executeMsg", executeMsg)

      let real_amount = parseFloat(amount) * 1000000;

      const funds = [{
        denom: "unibi",
        amount: real_amount.toString()
      }];

      const fee = "auto";
      setIsLoading(true);
      console.log("wasmClient", client.wasmClient)
      const executeContract = await client.wasmClient.execute(
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
      console.error('Transaction failed:', err);
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
      ClaimBet: {
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