import { VerifyFromPaths } from './../../libs/typescript/verify-utils/verify-given-proof-ffjavascript';
import { getRootDir } from '../../libs/typescript/ts-utils/common-utils';

describe('Check verifier build on ffjavascript', () => {
  let keyPath: string;
  let proofPath: string;
  let updateOldPath: string;
  let updatePath: string;

  beforeAll(async () => {
    const rootDir = await getRootDir();

    keyPath =
      rootDir +
      '/vendor/eth2-light-client-updates/prater/capella-updates-94/vk.json';

    proofPath =
      rootDir +
      '/vendor/eth2-light-client-updates/prater/capella-updates-94/proof_5609044_5609069.json';

    updateOldPath =
      rootDir +
      '/vendor/eth2-light-client-updates/prater/capella-updates-94/update_5601823_5609044.json';

    updatePath =
      rootDir +
      '/vendor/eth2-light-client-updates/prater/capella-updates-94/update_5609044_5609069.json';
  });

  test('Check "Verifier"', async () => {
    const res = await VerifyFromPaths(
      keyPath,
      proofPath,
      updateOldPath,
      updatePath,
    );
    expect(res).toEqual(true);
  }, 1000);
});
