import { task } from 'hardhat/config';
import { BeaconApi } from '../../../relay/implementations/beacon-api';
import { getConstructorArgs } from './utils';
import { getNetworkConfig } from '../../../relay/utils/get_current_network_config';
import { getGenericLogger } from '../../../libs/typescript/ts-utils/logger';

const logger = getGenericLogger();

task('verify-contracts', 'Verify')
  .addParam('lightclient', 'The address of the BeaconLightClient contract')
  .addParam('slot', 'The slot ')
  .addParam('follownetwork', 'The network to follow')
  .setAction(async (args, { run }) => {
    if (args.follownetwork !== 'pratter' && args.follownetwork !== 'mainnet') {
      logger.warn('This follownetwork is not specified in networkconfig');
      return;
    }

    const currentConfig = getNetworkConfig(args.follownetwork);

    const beaconApi = new BeaconApi(currentConfig.BEACON_REST_API!);

    await run('verify:verify', {
      address: args.lightclient,
      constructorArguments: await getConstructorArgs(
        beaconApi,
        args.slot,
        currentConfig,
      ),
    });
  });
