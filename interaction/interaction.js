const { NibiruQuerier, NibiruTxClient, Testnet } = require('@nibiruchain/nibijs');
const toast = require('react-hot-toast');

const CHAIN_NAME = 'nibiru-testnet-1';
const chain = Testnet(1);

const useTransaction = (address, signer) => {
  const createPool = async (contractAddress, owner, deadline, token, amount) => {
    if (!signer) {
      throw new Error('Signer is not initialized');
    }

    const message = {
      create_pool: {
        owner,
        deadline,
        token,
        amount
      }
    };

    const signingClient = await NibiruTxClient.connectWithSigner(
      chain.endptTm,
      signer
    );

    const tx = await signingClient.wasmClient.execute(
      address,
      contractAddress,
      message,
      "auto"
    );

    toast(`Transaction Hash: \n https://explorer.nibiru.fi/nibiru-testnet-1/tx/${tx.transactionHash}`, {
      duration: 8000,
    });
  };

  const enterBet = async (contractAddress, id, amount, player, tokenToSend) => {
    if (!signer) {
      throw new Error('Signer is not initialized');
    }

    const message = {
      enter_bet: {
        id,
        amount,
        player
      }
    };

    const signingClient = await NibiruTxClient.connectWithSigner(
      chain.endptTm,
      signer
    );

    const tx = await signingClient.wasmClient.execute(
      address,
      contractAddress,
      message,
      "auto",
      "nexusfi",
      tokenToSend
    );

    toast(`Transaction Hash: \n https://explorer.nibiru.fi/nibiru-testnet-1/tx/${tx.transactionHash}`, {
      duration: 8000,
    });
  };

  const claimBet = async (contractAddress, betId, player) => {
    if (!signer) {
      throw new Error('Signer is not initialized');
    }

    const message = {
      claim_bet: {
        bet_id: betId,
        player
      }
    };

    const signingClient = await NibiruTxClient.connectWithSigner(
      chain.endptTm,
      signer
    );

    const tx = await signingClient.wasmClient.execute(
      address,
      contractAddress,
      message,
      "auto"
    );

    toast(`Transaction Hash: \n https://explorer.nibiru.fi/nibiru-testnet-1/tx/${tx.transactionHash}`, {
      duration: 8000,
    });
  };

  const queryAllPools = async (contractAddress) => {
    const querier = await NibiruQuerier.connect(chain.endptTm);
    const res = await querier.nibiruExtensions.wasm.queryContractSmart(
      contractAddress,
      { get_all_pool: {} }
    );
    return res;
  };

  const queryPoolByToken = async (contractAddress, token) => {
    const querier = await NibiruQuerier.connect(chain.endptTm);
    const res = await querier.nibiruExtensions.wasm.queryContractSmart(
      contractAddress,
      { get_pool_by_token: { token } }
    );
    return res;
  };

  const queryPoolByDate = async (contractAddress, date) => {
    const querier = await NibiruQuerier.connect(chain.endptTm);
    const res = await querier.nibiruExtensions.wasm.queryContractSmart(
      contractAddress,
      { get_pool_by_date: { date } }
    );
    return res;
  };

  return { 
    createPool,
    enterBet,
    claimBet,
    queryAllPools,
    queryPoolByToken,
    queryPoolByDate
  };
};

module.exports = useTransaction;

// Example usage with console.logs
const testFunction = async () => {
  // Mock address and signer for testing
  const mockAddress = "nibi1test123";
  const mockSigner = {/* mock signer implementation */};
  
  const transaction = useTransaction(mockAddress, mockSigner);

  // Test createPool
  console.log("Testing createPool...");
  try {
    await transaction.createPool(
      "nibi1contract123",
      "nibi1owner456", 
      "2024-01-01",
      "NIBI",
      1000
    );
  } catch (error) {
    console.log("createPool error:", error.message);
  }

  // Test enterBet
  console.log("\nTesting enterBet...");
  try {
    await transaction.enterBet(
      "nibi1contract123",
      "bet123",
      500,
      "nibi1player789",
      [{denom: "unibi", amount: "500000"}]
    );
  } catch (error) {
    console.log("enterBet error:", error.message);
  }

  // Test claimBet
  console.log("\nTesting claimBet...");
  try {
    await transaction.claimBet(
      "nibi1contract123",
      "bet123",
      "nibi1player789"
    );
  } catch (error) {
    console.log("claimBet error:", error.message);
  }

  // Test queryAllPools
  console.log("\nTesting queryAllPools...");
  try {
    const allPools = await transaction.queryAllPools("nibi1contract123");
    console.log("All pools:", allPools);
  } catch (error) {
    console.log("queryAllPools error:", error.message);
  }

  // Test queryPoolByToken
  console.log("\nTesting queryPoolByToken...");
  try {
    const poolByToken = await transaction.queryPoolByToken("nibi1contract123", "NIBI");
    console.log("Pool by token:", poolByToken);
  } catch (error) {
    console.log("queryPoolByToken error:", error.message);
  }

  // Test queryPoolByDate
  console.log("\nTesting queryPoolByDate...");
  try {
    const poolByDate = await transaction.queryPoolByDate("nibi1contract123", "2024-01-01");
    console.log("Pool by date:", poolByDate);
  } catch (error) {
    console.log("queryPoolByDate error:", error.message);
  }
};

testFunction();