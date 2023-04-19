import { task } from 'hardhat/config';
import { BeaconApi } from '../../../relay/implementations/beacon-api';
import { Redis } from '../../../relay/implementations/redis';
import { SolidityContract } from '../../../relay/implementations/solidity-contract';
import { publishProofs } from '../../../relay/on_chain_publisher';
import { checkConfig } from '../../../libs/typescript/ts-utils/common-utils';
import * as networkConfig from '../../../relay/constants/network_config.json';
import { Config } from '../../../relay/constants/constants';
import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers';

task('start-publishing', 'Run relayer')
  .addParam('lightclient', 'The address of the BeaconLightClient contract')
  .addParam('follownetwork', 'The network the contract follows')
  .addParam(
    'privatekey',
    'The private key that will be used to publish',
    undefined,
    undefined,
    true,
  )
  .setAction(async (args, { ethers }) => {
    const config = {
      REDIS_HOST: process.env.REDIS_HOST,
      REDIS_PORT: Number(process.env.REDIS_PORT),
    };

    checkConfig(config);

    if (!networkConfig[args.follownetwork]) {
      console.warn('This follownetwork is not specified in networkconfig');
      return;
    }

    const currentConfig = networkConfig[args.follownetwork] as Config;

    let publisher;

    if (!args.privatekey) {
      [publisher] = await ethers.getSigners();
    } else {
      publisher = new ethers.Wallet(args.privatekey, ethers.provider);
    }

    console.log('Publishing updates with the account:', publisher.address);
    console.log('Account balance:', (await publisher.getBalance()).toString());

    console.log(`Contract address ${args.lightclient}`);

    const lightClientContract = await ethers.getContractAt(
      'BeaconLightClient',
      args.lightclient,
      publisher,
    );

    const redis = new Redis(config.REDIS_HOST!, config.REDIS_PORT);
    const beaconApi = new BeaconApi(currentConfig.BEACON_REST_API);
    const contract = new SolidityContract(lightClientContract);

    publishProofs(redis, beaconApi, contract);

    // never resolving promise to block the task lets see if it works
    return new Promise(() => {});
  });
