const nearAPI = require("near-api-js");
const path = require("path");
const homedir = require("os").homedir();
const credentialsPath = path.join(homedir, ".near-credentials");
const networkId = "testnet";
const accoutId = "georgecross.testnet";
const greetingContractId =
  "9f9350eb575cae7aac7f85a8c62b08d94dcac70a84e3c765464ff87c669fa4e5";
const computeContractId =
  "a7d1736372266477e0d0295d34ae47622ba50d007031a009976348f954e681fe";

const nodeUrl = `https://rpc.${networkId}.near.org`;
const AvalancheGreetingContractAddress =
  "0xc17a00D5e657fd8E5766A4E1C13599ea4d31E563";
const AvalancheComputingContractAddress =
  "0x04b697c09243D8aD48DE703E10F6b82A931456D9"; 
// destination chain name
const destinationChainName = "PLATONEVMDEV";
const gas = 30000000000000;

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

  // Register contract info for sending messages to other chains
  await account.functionCall({
    contractId: greetingContractId,
    methodName: "register_dst_contract",
    args: {
      chain_name: destinationChainName,
      contract_address: AvalancheGreetingContractAddress,
      action_name: "receiveGreeting",
    },
    gas,
  });

  await account.functionCall({
    contractId: greetingContractId,
    methodName: "register_permitted_contract",
    args: {
      chain_name: destinationChainName,
      sender: AvalancheGreetingContractAddress,
      action_name: "receive_greeting",
    },
    gas,
  });

  await account.functionCall({
    contractId: computeContractId,
    methodName: "register_dst_contract",
    args: {
      action_name: "receive_compute_task",
      chain_name: destinationChainName,
      contract_address: AvalancheComputingContractAddress,
      contract_action_name: "receiveComputeTask",
    },
    gas,
  });

  await account.functionCall({
    contractId: computeContractId,
    methodName: "register_dst_contract",
    args: {
      action_name: "receive_compute_result",
      chain_name: destinationChainName,
      contract_address: AvalancheComputingContractAddress,
      contract_action_name: "receiveComputeTaskCallback",
    },
    gas,
  });
  
  await account.functionCall({
    contractId: computeContractId,
    methodName: "register_permitted_contract",
    args: {
      chain_name: destinationChainName,
      sender: AvalancheComputingContractAddress,
      action_name: "receive_compute_result",
    },
    gas,
  });

  await account.functionCall({
    contractId: computeContractId,
    methodName: "register_permitted_contract",
    args: {
      chain_name: destinationChainName,
      sender: AvalancheComputingContractAddress,
      action_name: "receive_compute_task",
    },
    gas,
  });
})();
