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
  // MOONBASEALPHA
  {
    greetingContract: '0xC0F1706106D2d7208C6586d1C8Aec520d99E9F14',
    greetingActionName: '0x2d436822',
    computingContract: '0x711cb9B41Ae7862b5961a134703Cd6B5f16dAdF5',
    computingTaskActionName: '0x47e50a42',
    destinationChainName: 'MOONBASEALPHA',
  },
  // FUJI
  {
    greetingContract: '0x1723f39e05Ca8b14ACaf244bAFFBd79801d42A63',
    greetingActionName: '0x2d436822',
    computingContract: '0x7F5b6F5F7a786F63383E8681Da7ACCEed76Ab209',
    computingTaskActionName: '0x47e50a42',
    destinationChainName: 'FUJI',
  },
  // SHIBUYA
  {
    greetingContract: 'a1mydsZDKLQJh8mwB1NZ86XVJ8ApiyNVWikMrhoLwoGfZex',
    greetingActionName: '0x0c724dc2',
    computingContract: 'ZakeYTFPNkC9Cgceui2aBZ6G23nA6ieB3KVWfmdNDv6UfM1',
    computingTaskActionName: '0x00000001',
    destinationChainName: 'SHIBUYA',
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
