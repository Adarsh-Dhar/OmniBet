const { Chain, NibiruQuerier, Testnet } = require("@nibiruchain/nibijs") ;

const connectToQuerier = async () => {
  /**
   * A "Chain" object exposes all endpoints for a Nibiru node, such as the
   * gRPC server, Tendermint RPC endpoint, and REST server.
   *
   * The most important endpoint for nibijs is "Chain.endptTM", the Tendermint RPC.
   **/
  const chain = Testnet(1);

  const querier = await NibiruQuerier.connect(chain.endptTm);
  return querier;
};


const getOraclePrice = async () => {
  const querier = await connectToQuerier();
try
 { 
  const price = await querier.nibiruExtensions.query.oracle.aggregatePrevotes(); 
  console.log(`Oracle Price `, price);
  return price;
}
  catch (error) 
  {
    console.error("Error getting Oracle Price: ", error);
  }
}


getOraclePrice();



