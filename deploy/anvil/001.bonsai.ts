import { HardhatRuntimeEnvironment } from 'hardhat/types';
import { DeployFunction } from 'hardhat-deploy/types';
import { Resolver__factory } from '../../typechain-types';
import { namehash } from 'ethers';

const func: DeployFunction = async function (hre: HardhatRuntimeEnvironment) {
  const [deployer] = await hre.ethers.getSigners();
  const chainId = await hre.ethers.provider.getNetwork().then((n) => n.chainId);
  const { address } = await hre.deployments.deploy('BonsaiTestRelay', {
    log: true,
    from: await deployer.getAddress(),
    args: [chainId],
  });

  const domain = 'relay';
  const node = namehash(domain);
  await hre.ensMock.setDomainResolver(domain, hre.ensMock.constants.ENS_OPEN_RESOLVER_ADDRESS);
  await Resolver__factory.connect(hre.ensMock.constants.ENS_OPEN_RESOLVER_ADDRESS, deployer)[
    'setAddr(bytes32,address)'
  ](node, address);
};
export default func;
