import { JsonRpcProvider, ethers, namehash, Wallet } from 'ethers';
import { task, types } from 'hardhat/config';
import * as IAddrResolverAbi from '@ensdomains/ens-contracts/build/contracts/Resolver.json';

task('deploy_bonsai', 'deploys bonsai test relay')
  .addOptionalParam('privateKey', 'deployer private key', process.env.PRIVATE_KEY || '', types.string)
  .addOptionalParam('rpc', 'url of the RPC', 'http://localhost:8545', types.string)
  .addOptionalParam('ens', 'optional domain to register, only for hardhat networ', '', types.string)
  .setAction(async ({ rpc, ens: domain, privateKey }, hre) => {
    const provider = new JsonRpcProvider(rpc);

    const { chainId } = await provider.getNetwork();

    let deployer = privateKey
      ? new Wallet(privateKey, provider)
      : await hre.ethers.getSigners().then(([deployer]) => deployer.connect(provider));

    const bonsaiRelay = await hre.ethers
      .getContractFactory('BonsaiTestRelay', deployer)
      .then((factory) => factory.deploy(chainId));
    await bonsaiRelay.deploymentTransaction()?.wait();

    // ENS resolution for hardhat-ens-mock
    if (domain && typeof domain === 'string') {
      const resolverAddr = await provider.resolveName('resolver');
      if (!resolverAddr) return;

      const node = namehash(domain);
      const resolver = new ethers.Contract(resolverAddr, IAddrResolverAbi, deployer);
      await hre.ensMock.setDomainResolver(domain, resolverAddr, provider);
      await hre.ensMock.setDomainOwner(domain, await deployer.getAddress(), provider);
      await resolver.setAddr(node, bonsaiRelay.getAddress());

      // await hre.ensMock.setDo
      console.log('Resolver', resolverAddr);
    }
  });
