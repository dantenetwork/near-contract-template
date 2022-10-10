const nearAPI = require('near-api-js');
const path = require('path');
const homedir = require('os').homedir();
const credentialsPath = path.join(homedir, '.near-credentials');
const networkId = 'testnet';
const greetingContractId =
  '99ff32da92227f302056389ce208d77e12f88a6ffd2cee1b238586cc4cc20bd7';
const computeContractId =
  'f738bf496d14e9c2d4b734be2905a319253a7cc5775511434991b9f212224aab';

const nodeUrl = `https://rpc.${networkId}.near.org`;
const gas = 30000000000000;

const nearConfig = {
  networkId,
  keyStore: new nearAPI.keyStores.UnencryptedFileSystemKeyStore(
    credentialsPath
  ),
  nodeUrl,
};

// destination contract information
const Chains = [
  // ROCOCO
  {
    greetingContract: '0x5e5e0249875e40b0f3275c62efb3620abaf825abd7a7092c68c1ade6640c1c6e',
    greetingActionName: '0x0c724dc2',
    computingContract: '0xd8785b20e687c9cafa2d0c1a634cdfd9cecab65c39533d4bc766a3d165f8eb6d',
    computingTaskActionName: '0x00000001',
    destinationChainName: 'ROCOCO',
  }
];

(async function init() {
  for (let i in Chains) {
    // Chains.forEach(async (chain) => {
    const near = await nearAPI.connect(nearConfig);
    let account = await near.account(greetingContractId);

    // Register contract info for sending messages to other chains
    await account.functionCall({
      contractId: greetingContractId,
      methodName: 'register_dst_contract',
      args: {
        action_name: 'send_greeting',
        chain_name: Chains[i].destinationChainName,
        contract_address: Chains[i].greetingContract,
        contract_action_name: Chains[i].greetingActionName,
      },
      gas,
    });

    await account.functionCall({
      contractId: greetingContractId,
      methodName: 'register_permitted_contract',
      args: {
        chain_name: Chains[i].destinationChainName,
        sender: Chains[i].greetingContract,
        action_name: 'receive_greeting',
      },
      gas,
    });

    account = await near.account(computeContractId);
    await account.functionCall({
      contractId: computeContractId,
      methodName: 'register_dst_contract',
      args: {
        action_name: 'receive_compute_task',
        chain_name: Chains[i].destinationChainName,
        contract_address: Chains[i].computingContract,
        contract_action_name: Chains[i].computingTaskActionName,
      },
      gas,
    });

    await account.functionCall({
      contractId: computeContractId,
      methodName: 'register_permitted_contract',
      args: {
        chain_name: Chains[i].destinationChainName,
        sender: Chains[i].computingContract,
        action_name: 'receive_compute_result',
      },
      gas,
    });

    await account.functionCall({
      contractId: computeContractId,
      methodName: 'register_permitted_contract',
      args: {
        chain_name: Chains[i].destinationChainName,
        sender: Chains[i].computingContract,
        action_name: 'receive_compute_task',
      },
      gas,
    });
  }
})();
