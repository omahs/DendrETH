pragma circom 2.0.3;

include "../../../node_modules/circomlib/circuits/comparators.circom";

// This template checks if a bit array contains at least 2/3 1s
template IsSuperMajority(N) {
  signal input bitmask[N];

  signal output out;

  var sum = 0;
  component lessThan[N];
  //count the number of 1s in the bitmask
  for(var i = 0; i < N; i++) {
    sum += bitmask[i];
  }
  // check if 1s are more then 2/3 of the bitmask
  component greaterThan = GreaterEqThan(252);
  greaterThan.in[0] <== sum * 3;
  greaterThan.in[1] <== 2 * N;

  out <== greaterThan.out;
}
