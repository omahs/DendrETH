pragma circom 2.0.3;

include "compute_domain.circom";

// https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#compute_signing_root
// Return the signing root for the corresponding signing data.
template ComputeSigningRoot() {
  // In the "consensus-specs" they pass ssz-object(of type SSZObject) to "compute_signing_root"
  // then they hash it. We use the hash of the header(SSZObject) directly
  signal input headerHash[256];
  signal input domain[256];

  signal output signing_root[256];

  component hashTwo256 = HashTwo256();

  for(var i = 0; i < 256; i++) {
    hashTwo256.in[0][i] <== headerHash[i];
  }

  for(var i = 0; i < 256; i++) {
    hashTwo256.in[1][i] <== domain[i];
  }


  for(var i = 0; i < 256; i++) {
    signing_root[i] <== hashTwo256.out[i];
  }
}
