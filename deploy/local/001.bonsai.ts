import { HardhatRuntimeEnvironment } from 'hardhat/types';
import { DeployFunction } from 'hardhat-deploy/types';

const func: DeployFunction = async function (hre: HardhatRuntimeEnvironment) {
  const [deployer] = await hre.ethers.getSigners();
  const chainId = await hre.ethers.provider.getNetwork().then((n) => n.chainId);
  await hre.deployments.deploy('BonsaiTestRelay', {
    log: true,
    from: await deployer.getAddress(),
    args: [chainId],
  });
};
export default func;
