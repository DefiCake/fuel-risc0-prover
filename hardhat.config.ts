import 'dotenv/config';
import { HardhatUserConfig } from 'hardhat/config';
import '@nomicfoundation/hardhat-toolbox';
import '@nomicfoundation/hardhat-ethers';
import 'hardhat-ens-mock';
import 'hardhat-deploy';
import './tasks';

const { RPC_URL } = process.env;

const config: HardhatUserConfig = {
  defaultNetwork: 'hardhat',
  solidity: {
    compilers: [
      {
        version: '0.8.17',
        settings: {
          optimizer: {
            enabled: false,
            runs: 10000,
          },
        },
      },
      {
        version: '0.6.12',
        settings: {
          optimizer: {
            enabled: false,
            runs: 10000,
          },
        },
      },
    ],
  },
  networks: {
    hardhat: {
      chainId: 5, // Allows better integration with ENS mock
      deploy: ['deploy/hardhat'],
    },
    local: {
      deploy: ['deploy/local'],
      url: 'http://localhost:8545',
    },
    anvil: {
      url: RPC_URL || 'http://anvil:8545',
      deploy: ['deploy/anvil'],
    },
  },
};

export default config;
