import { HardhatRuntimeEnvironment } from 'hardhat/types';
import { DeployFunction } from 'hardhat-deploy/types';

const func: DeployFunction = async function (hre: HardhatRuntimeEnvironment) {
  await hre.ensMock.setupEnsMock(hre); // I am so surprised anvil accepts hardhat commands!!
};
export default func;

func.tags = ['ensmock'];
