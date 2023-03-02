import { exec as exec_, execSync, spawn } from 'node:child_process';

async function main() {
  try {
    const command = execSync(
      `snarkit2 check aggregate_bitmask_N1 --backend native -f `,
    );

    console.log('\n aggregate_bitmask_N1 is okey \n');
  } catch (e) {
    console.log('\n aggregate_bitmask_N1 failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check aggregate_bitmask_N3 --backend native -f `,
    );
    console.log('\n aggregate_bitmask_N3 is okey \n');
  } catch (e) {
    console.log('\n aggregate_bitmask_N3 failed \n');
  }
  try {
    const command = execSync(`snarkit2 check compress --backend native -f `);
    console.log('\n compress is okey \n');
  } catch (e) {
    console.log('\n compress failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check compute_domain --backend native -f `,
    );
    console.log('\n compute_domain is okey \n');
  } catch (e) {
    console.log('\n compute_domain failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check compute_signing_root --backend native -f `,
    );
    console.log('\n compute_signing_root is okey \n');
  } catch (e) {
    console.log('\n compute_signing_root failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check expand_message --backend native -f `,
    );
    console.log('\n expand_message is okey \n');
  } catch (e) {
    console.log('\n expand_message failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check hash_to_field --backend native -f `,
    );
    console.log('\n hash_to_field is okey \n');
  } catch (e) {
    console.log('\n hash_to_field failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check hash_tree_root --backend native -f `,
    );
    console.log('\n hash_tree_root is okey \n');
  } catch (e) {
    console.log('\n hash_tree_root failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check hash_tree_root_beacon_header --backend native -f `,
    );
    console.log('\n hash_tree_root_beacon_header is okey \n');
  } catch (e) {
    console.log('\n hash_tree_root_beacon_header failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check hash_two_256 --backend native -f `,
    );
    console.log('\n hash_two_256 is okey \n');
  } catch (e) {
    console.log('\n hash_two_256 failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check are_equal_hashes --backend native -f `,
    );
    console.log('\n are_equal_hashes is okey \n');
  } catch (e) {
    console.log('\n are_equal_hashes failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check is_supermajority --backend native -f `,
    );
    console.log('\n is_supermajority is okey \n');
  } catch (e) {
    console.log('\n is_supermajority failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check numbersTo256Bits --backend native -f `,
    );
    console.log('\n numbersTo256Bits is okey \n');
  } catch (e) {
    console.log('\n numbersTo256Bits failed \n');
  }
  try {
    const command = execSync(`snarkit2 check ssz_num --backend native -f `);
    console.log('\n ssz_num is okey \n');
  } catch (e) {
    console.log('\n ssz_num failed \n');
  }
  try {
    const command = execSync(
      `snarkit2 check sync_committee_hash_tree_root --backend native -f `,
    );
    console.log('\n sync_committee_hash_tree_root is okey \n');
  } catch (e) {
    console.log('\n sync_committee_hash_tree_root failed \n');
  }
}

main();
