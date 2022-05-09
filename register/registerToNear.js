const nearAPI = require("near-api-js");
const path = require("path");
const homedir = require("os").homedir();
const credentialsPath = path.join(homedir, ".near-credentials");
const networkId = "testnet";
const accoutId = "georgecross.testnet";
const contractId =
  "98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de";
const nodeUrl = `https://rpc.${networkId}.near.org`;
const AvalancheGreetingContractAddress =
  "0x71F985781d5439E469384c483262b24085Fc08aC";
const crossChainContractId = "d-hub.testnet";
// destination chain name
const destinationChainName = "AVALANCHE";
const destinationActionName = "receiveGreeting";

const nearConfig = {
  networkId,
  keyStore: new nearAPI.keyStores.UnencryptedFileSystemKeyStore(
    credentialsPath
  ),
  nodeUrl,
};

(async function init() {

  const near = await nearAPI.connect(nearConfig);
  const account = await near.account(accoutId);

  // init greeting contract
  let functionCallResponse = await account.functionCall({
    contractId,
    methodName: "new",
    args: {
      owner_id: contractId,
      cross_chain_contract_id: crossChainContractId,
    },
    gas: 30000000000000,
  });

  let result = await nearAPI.providers.getTransactionLastResult(
    functionCallResponse
  );
  console.log(result);
  
  // Register contract info for sending messages to other chains
  functionCallResponse = await account.functionCall({
    contractId,
    methodName: "register_dst_contract",
    args: {
      chain_name: destinationChainName,
      contract_address: AvalancheGreetingContractAddress,
      action_name: destinationActionName,
    },
    gas: 30000000000000,
  });

  result = await nearAPI.providers.getTransactionLastResult(
    functionCallResponse
  );
  console.log(result);
})();
