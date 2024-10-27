import { NibiruTxClient, Testnet } from '@nibiruchain/nibijs';


// Step 1: Query the Oracle Price
import { getOraclePrice } from './oracle.js'; // assuming getOraclePrice is in a file named oracle.js
import { send } from 'process';

const chainInfo = {
    chainId: "nibiru-testnet-1", // Replace with "nibiru-testnet-1" for testnet
    chainName: "nibiru-testnet-1", // Replace with "nibiru-testnet-1" for testnet
    rpc: "https://rpc.testnet-1.nibiru.fi:443", // Replace with testnet URL if needed
    rest: "https://lcd.testnet-1.nibiru.fi:443", // Replace with testnet URL if needed
    stakeCurrency: {
      coinDenom: "NIBI",
      coinMinimalDenom: "unibi",
      coinDecimals: 6,
    },
    bip44: {
      coinType: 118,
    },
    bech32Config: {
      bech32PrefixAccAddr: "nibi",
      bech32PrefixAccPub: "nibipub",
      bech32PrefixValAddr: "nibivaloper",
      bech32PrefixValPub: "nibivaloperpub",
      bech32PrefixConsAddr: "nibivalcons",
      bech32PrefixConsPub: "nibivalconspub",
    },
    currencies: [
      {
        coinDenom: "NIBI",
        coinMinimalDenom: "unibi",
        coinDecimals: 6,
      },
    ],
    feeCurrencies: [
      {
        coinDenom: "NIBI",
        coinMinimalDenom: "unibi",
        coinDecimals: 6,
        gasPriceStep: {
          low: 0.025,
          average: 0.05,
          high: 0.1,
        },
      },
    ],
  };
  

// Step 2: Connect to the Transaction Client
const connectToTxClient = async () => {
   // Ensure Keplr is installed
   //@ts-ignore
if (!window.getOfflineSigner || !window.keplr) {
    alert("Please install Keplr extension");
    return;
  }
  // Suggest the chain to Keplr
   //@ts-ignore

  await window.keplr.experimentalSuggestChain(chainInfo);
  
  // Enable Keplr
   //@ts-ignore

  await window.keplr.enable(chainInfo.chainId);
  
  // Get the offline signer from Keplr
   //@ts-ignore

  const offlineSigner = window.getOfflineSigner(chainInfo.chainId);
  
  const txClient = await NibiruTxClient.connectWithSigner(
    chainInfo.rpc, // RPC endpoint
    offlineSigner
  );

  return txClient;
  
};

// Step 3: Send Oracle Price to Smart Contract
export const sendOraclePriceToContract = async (token, contractAddress) => {
  try {
    // Get the Oracle Price
    const oraclePrice = await getOraclePrice(token);
    
    // Connect to the Tx Client
    const txClient = await connectToTxClient();
    
    // Get the address of the signer (the account interacting with the contract)
    //@ts-ignore
    const [{ address }] = await txClient?.signer.getAccounts();
    
    // Prepare the message to send to the contract
    const msg = {
      update_price: { // You can replace this with your smart contract's message handler
        token,
        price: oraclePrice
      }
    };

    // Construct MsgExecuteContract to send to the contract
    const msgExecuteContract = {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: {
        sender: address,
        contract: contractAddress,
        msg: msg, // This will be JSON-encoded automatically
        funds: [], // If needed, you can send tokens along with the message
      },
    };

    // Step 4: Sign and Broadcast the Transaction
    const result = await txClient?.signAndBroadcast(address, [msgExecuteContract], 'auto');
    
    if (result?.code !== 0) {
      console.error("Transaction failed:", result?.rawLog);
    } else {
      console.log("Transaction successful:", result.transactionHash);
    }
  } catch (error) {
    console.error("Error while sending oracle price to contract:", error);
  }
};

sendOraclePriceToContract("unibi", "contractAddress"); // Replace "contractAddress" with the address of your smart contract
