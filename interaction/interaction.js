// Contract interaction functions
const CONTRACT_ADDRESS = "nibi130rr94tv0gmt705t22u05vmv98dkt4zv6rn7uy5xz3j7x4f59u8sskszpy"; // Replace with deployed contract address

// Helper function to connect Keplr wallet
const connectKeplr = async () => {
  try {
    const chainId = "nibiru-testnet-1";
    if (!window.keplr) {
      throw new Error("Keplr wallet not found! Please install Keplr extension.");
    }
    
    await window.keplr.enable(chainId);
    const offlineSigner = window.keplr.getOfflineSigner(chainId);
    const accounts = await offlineSigner.getAccounts();
    
    if (!accounts || accounts.length === 0) {
      throw new Error("No accounts found");
    }

    return {
      signer: offlineSigner,
      address: accounts[0].address
    };
  } catch (error) {
    console.error("Error connecting to Keplr:", error);
    throw error;
  }
};

// Query functions
const getAllPools = async (client) => {
  const queryMsg = {
    get_all_pool: {}
  };
  
  try {
    const response = await client.wasmClient.queryContractSmart(
      CONTRACT_ADDRESS,
      queryMsg
    );
    console.log("getAllPools response:", response);
    return response;
  } catch (error) {
    console.error("Error querying all pools:", error);
    throw error;
  }
};

const getPoolByToken = async (client, token) => {
  const queryMsg = {
    get_pool_by_token: { token }
  };

  try {
    const response = await client.wasmClient.queryContractSmart(
      CONTRACT_ADDRESS, 
      queryMsg
    );
    console.log("getPoolByToken response:", response);
    return response;
  } catch (error) {
    console.error("Error querying pool by token:", error);
    throw error;
  }
};

const getPoolByDate = async (client, date) => {
  const queryMsg = {
    get_pool_by_date: { date }
  };

  try {
    const response = await client.wasmClient.queryContractSmart(
      CONTRACT_ADDRESS,
      queryMsg
    );
    console.log("getPoolByDate response:", response);
    return response;
  } catch (error) {
    console.error("Error querying pool by date:", error);
    throw error;
  }
};

// Execute functions
const createPool = async (client, owner, deadline, token, amount) => {
  try {
    const { signer, address } = await connectKeplr();
    
    const executeMsg = {
      create_pool: {
        owner,
        deadline,
        token,
        amount
      }
    };

    const response = await client.wasmClient.execute(
      address,
      CONTRACT_ADDRESS,
      executeMsg,
      "auto"
    );
    console.log("createPool response:", response);
    return response;
  } catch (error) {
    console.error("Error creating pool:", error);
    throw error;
  }
};

const enterBet = async (client, id, amount, player) => {
  try {
    const { signer, address } = await connectKeplr();

    const executeMsg = {
      enter_bet: {
        id,
        amount,
        player
      }
    };

    const response = await client.wasmClient.execute(
      address,
      CONTRACT_ADDRESS,
      executeMsg,
      "auto"
    );
    console.log("enterBet response:", response);
    return response;
  } catch (error) {
    console.error("Error entering bet:", error);
    throw error;
  }
};

const claimBet = async (client, betId, player) => {
  try {
    const { signer, address } = await connectKeplr();

    const executeMsg = {
      claim_bet: {
        bet_id: betId,
        player
      }
    };

    const response = await client.wasmClient.execute(
      address,
      CONTRACT_ADDRESS,
      executeMsg,
      "auto"
    );
    console.log("claimBet response:", response);
    return response;
  } catch (error) {
    console.error("Error claiming bet:", error);
    throw error;
  }
};

// Test the functions
const main = async () => {
  try {
    // Initialize client
    const client = {}; // Replace with actual client initialization
    
    // Test getAllPools
    const pools = await getAllPools(client);
    console.log("All pools:", pools);

    // Test getPoolByToken
    const btcPool = await getPoolByToken(client, "BTC");
    console.log("BTC pool:", btcPool);

    // Test getPoolByDate
    const todayPools = await getPoolByDate(client, "2024-02-15");
    console.log("Today's pools:", todayPools);

    // Test createPool
    const newPool = await createPool(client, "owner123", "2024-03-01", "ETH", "1000");
    console.log("New pool created:", newPool);

    // Test enterBet
    const bet = await enterBet(client, 1, "500", "player123");
    console.log("Bet placed:", bet);

    // Test claimBet
    const claim = await claimBet(client, 1, "player123");
    console.log("Bet claimed:", claim);

  } catch (error) {
    console.error("Test failed:", error);
  }
};

// Run the tests
main();
