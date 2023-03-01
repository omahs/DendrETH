pragma circom 2.0.3;

include "hash_two.circom";
include "../../../node_modules/circomlib/circuits/comparators.circom";

// This template checks if a leaf is part of a merkle tree where index is path to the root and branch are its neighbours
template IsValidMerkleBranch(N) {
  signal input branch[N][256];
  signal input leaf[256];
  signal input root[256];
  signal input index;

  component hashers[N];
  component isZero[N];

  for(var i = 0; i < N; i++) {
    hashers[i] = HashTwo();
    isZero[i] = IsZero();

    isZero[i].in <-- (index \ (2**i)) % 2;

    var current[256];

    for(var j = 0; j < 256; j++) {
      current[j] = i == 0 ? leaf[j] : hashers[i - 1].out[j];
    }

    for(var j = 0; j < 256; j++) {
      hashers[i].in[0][j] <== (current[j] - branch[i][j]) * isZero[i].out + branch[i][j];
      hashers[i].in[1][j] <== (branch[i][j] - current[j]) * isZero[i].out + current[j];
    }

  component isEqual[N];

  for(var i = 0; i < N; i++) {
    isEqual[i] = IsEqual();
    isEqual[i].in[0] <== root[i];
    isEqual[i].in[1] <== hashers[N-1].out[i];
    isEqual[i].out == 1;
  }
  }
}
