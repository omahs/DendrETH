pragma circom 2.0.3;

include "../../../node_modules/circomlib/circuits/comparators.circom";

// This template insures that a given array is a bit array - consists only of 0s and 1s
template BitmaskContainsOnlyBools(N) {
  signal input bitmask[N];

  component lessThan[N];

  for(var i = 0; i < N; i++) {
    lessThan[i] = LessEqThan(1);
    lessThan[i].in[0] <== bitmask[i];
    lessThan[i].in[1] <== 1;

    lessThan[i].out === 1;
  }
}
