import { DeployFunction } from 'hardhat-deploy/types';

const BLOCK_TIME_MS = Number(process.env.BLOCK_TIME) || 15000;

const func: DeployFunction = async function (hre) {
  if (process.env.MINING_ENABLED === 'true') {
    await hre.network.provider.send('evm_setAutomine', [true]);
    await hre.network.provider.send('evm_setIntervalMining', [BLOCK_TIME_MS]);
  }
};
export default func;
func.tags = ['mining'];
