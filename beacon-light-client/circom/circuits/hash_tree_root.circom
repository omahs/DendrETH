pragma circom 2.0.3;

include "../../../node_modules/circomlib/circuits/sha256/sha256.circom";
include "hash_two_256.circom";

// This template creates a merkle root from a N leaf tree
template HashTreeRoot(N) {
  signal input leaves[N][256];

  signal output out[256];

  component hashers[N - 1];

  for(var i = 0; i < N - 1; i++) {
    hashers[i] = HashTwo256();
  }

  for(var i = 0; i < N / 2; i++) {
    for(var j = 0; j < 256; j++) {
      hashers[i].in[0][j] <== leaves[i * 2][j];
      hashers[i].in[1][j] <== leaves[i * 2 + 1][j];
    }
  }

  var k = 0;

  for(var i = N / 2; i < N - 1; i++) {
    for(var j = 0; j < 256; j++) {
      hashers[i].in[0][j] <== hashers[k * 2].out[j];
      hashers[i].in[1][j] <== hashers[k * 2 + 1].out[j];
    }

    k++;
  }

  for(var i = 0; i < 256; i++) {
    out[i] <== hashers[N - 2].out[i];
  }
}
