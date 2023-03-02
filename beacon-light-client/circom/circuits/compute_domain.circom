pragma circom 2.0.3;

include "hash_two_256.circom";

// https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#compute_domain
// Return the domain for the ``domain_type`` and ``fork_version``.
template ComputeDomain() {
  signal input fork_version[32];
  signal input GENESIS_VALIDATORS_ROOT[256];
  // should be "DOMAIN_TYPE"
  signal input DOMAIN_SYNC_COMMITTEE[32];
  // var GENESIS_VALIDATORS_ROOT[256] = [0,1,0,0,1,0,1,1,0,0,1,1,0,1,1,0,0,0,1,1,1,1,0,1,1,0,1,1,1,0,0,1,0,1,0,0,1,1,1,0,0,0,1,0,1,0,0,0,0,1,1,0,0,0,0,1,0,0,1,0,0,0,0,0,1,1,0,1,0,1,1,1,0,1,1,0,1,1,1,0,1,0,1,1,1,0,0,1,0,0,0,0,0,1,0,1,0,0,1,1,0,1,0,0,0,0,0,0,1,1,1,1,1,1,0,1,1,1,0,1,0,1,0,0,1,1,1,0,0,1,0,1,0,1,0,0,1,0,1,1,1,1,1,1,1,1,1,0,1,0,0,1,1,1,1,1,0,0,0,0,0,1,1,0,1,0,1,1,1,1,1,1,0,0,1,1,0,0,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,0,0,1,1,1,1,0,1,0,1,1,0,1,0,1,1,0,1,0,0,1,0,0,1,1,1,1,1,1,1,0,1,0,1,0,0,0,1,0,0,0,1,1,0,1,1,1,1,1,1,1,1,1,0,1,0,0,1,0,1,0,1];
  // var DOMAIN_SYNC_COMMITTEE[32] = [0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

  signal output domain[256];

  component hashTwo256 = HashTwo256();

  for(var i = 0; i < 32; i++) {
    hashTwo256.in[0][i] <== fork_version[i];
  }

  for(var i = 32; i < 256; i++) {
    hashTwo256.in[0][i] <== 0;
  }

  for(var i = 0; i < 256; i++) {
    hashTwo256.in[1][i] <== GENESIS_VALIDATORS_ROOT[i];
  }

  for(var i = 0; i < 32; i++) {
    domain[i] <== DOMAIN_SYNC_COMMITTEE[i];
  }

  for(var i = 32; i < 256; i++) {
    domain[i] <== hashTwo256.out[i - 32];
  }
}
