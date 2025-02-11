use plonky2::{
    field::extension::Extendable, hash::hash_types::RichField, iop::target::BoolTarget,
    plonk::circuit_builder::CircuitBuilder,
};
use plonky2_u32::gadgets::arithmetic_u32::U32Target;
use sha2::{Digest, Sha256};

use crate::biguint::{BigUintTarget, CircuitBuilderBiguint};

pub const ETH_SHA256_BIT_SIZE: usize = 256;
pub const POSEIDON_HASH_SIZE: usize = 4;

pub fn hash_bit_array(validator_pubkey: Vec<&str>) -> Vec<String> {
    // Concatenate the array into a single binary string
    let binary_string: String = validator_pubkey.join("");

    // Convert binary string to bytes
    let mut byte_string: Vec<u8> = binary_string
        .as_str()
        .chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|chunk| {
            let byte_str: String = chunk.into_iter().collect();
            u8::from_str_radix(&byte_str, 2).unwrap()
        })
        .collect();

    byte_string.resize(64, 0);

    let mut hasher = Sha256::new();
    hasher.update(byte_string);
    let result = hasher.finalize();

    let pubkey_binary_result: Vec<String> = result
        .iter()
        .map(|byte| {
            format!("{:08b}", byte)
                .chars()
                .map(|ch| ch.to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect();
    pubkey_binary_result
}

pub fn biguint_is_equal<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: &BigUintTarget,
    b: &BigUintTarget,
) -> BoolTarget {
    assert!(a.limbs.len() == b.limbs.len());

    let mut all_equal = Vec::new();
    all_equal.push(builder._true());

    for i in 0..a.limbs.len() {
        let equal = builder.is_equal(a.limbs[i].0, b.limbs[i].0);
        all_equal.push(builder.and(all_equal[i], equal));
    }

    all_equal[a.limbs.len()]
}

pub fn create_bool_target_array<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
) -> [BoolTarget; ETH_SHA256_BIT_SIZE] {
    (0..ETH_SHA256_BIT_SIZE)
        .map(|_| builder.add_virtual_bool_target_safe())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn bits_to_biguint_target<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    bits_target: Vec<BoolTarget>,
) -> BigUintTarget {
    let bit_len = bits_target.len();
    assert_eq!(bit_len % 32, 0);

    let mut u32_targets = Vec::new();
    for i in 0..bit_len / 32 {
        u32_targets.push(U32Target(
            builder.le_sum(bits_target[i * 32..(i + 1) * 32].iter().rev()),
        ));
    }
    u32_targets.reverse();
    BigUintTarget { limbs: u32_targets }
}

pub fn biguint_to_bits_target<F: RichField + Extendable<D>, const D: usize, const B: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: &BigUintTarget,
) -> Vec<BoolTarget> {
    let mut res = Vec::new();
    for i in (0..a.num_limbs()).rev() {
        let bit_targets = builder.split_le_base::<B>(a.get_limb(i).0, 32);
        for j in (0..32).rev() {
            res.push(BoolTarget::new_unsafe(bit_targets[j]));
        }
    }

    res
}

pub fn _right_rotate<const S: usize>(n: [BoolTarget; S], bits: usize) -> [BoolTarget; S] {
    let mut res = [None; S];
    for i in 0..S {
        res[i] = Some(n[((S - bits) + i) % S])
    }
    res.map(|x| x.unwrap())
}

pub fn _shr<F: RichField + Extendable<D>, const D: usize, const S: usize>(
    n: [BoolTarget; S],
    bits: i64,
    builder: &mut CircuitBuilder<F, D>,
) -> [BoolTarget; S] {
    let mut res = [None; S];
    for i in 0..S {
        if (i as i64) < bits {
            res[i] = Some(BoolTarget::new_unsafe(builder.constant(F::ZERO)));
        } else {
            res[i] = Some(n[(i as i64 - bits) as usize]);
        }
    }
    res.map(|x| x.unwrap())
}

pub fn uint32_to_bits<F: RichField + Extendable<D>, const D: usize>(
    value: u32,
    builder: &mut CircuitBuilder<F, D>,
) -> [BoolTarget; 32] {
    let mut bits = [None; 32];
    for i in 0..32 {
        if value & (1 << (31 - i)) != 0 {
            bits[i] = Some(BoolTarget::new_unsafe(builder.constant(F::ONE)));
        } else {
            bits[i] = Some(BoolTarget::new_unsafe(builder.constant(F::ZERO)));
        }
    }
    bits.map(|x| x.unwrap())
}

fn reverse_endianness(bits: &[BoolTarget]) -> Vec<BoolTarget> {
    bits.chunks(8).rev().flatten().cloned().collect()
}

pub fn ssz_num_to_bits<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    num: &BigUintTarget,
    bit_len: usize,
) -> Vec<BoolTarget> {
    assert!(bit_len <= ETH_SHA256_BIT_SIZE);

    let mut bits = reverse_endianness(&biguint_to_bits_target::<F, D, 2>(builder, num));
    bits.extend((bit_len..ETH_SHA256_BIT_SIZE).map(|_| builder._false()));

    bits
}

pub fn ssz_num_from_bits<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    bits: &[BoolTarget],
) -> BigUintTarget {
    bits_to_biguint_target(builder, reverse_endianness(bits))
}

pub fn if_biguint<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    b: BoolTarget,
    x: &BigUintTarget,
    y: &BigUintTarget,
) -> BigUintTarget {
    let not_b = builder.not(b);

    let maybe_x = builder.mul_biguint_by_bool(x, b);

    let maybe_y = builder.mul_biguint_by_bool(y, not_b);

    let mut result = builder.add_biguint(&maybe_y, &maybe_x);

    // trim the carry
    result.limbs.pop();

    result
}

#[cfg(test)]
mod test_ssz_num_from_bits {
    use anyhow::Result;
    use itertools::Itertools;
    use num::{BigUint, Num};
    use plonky2::{
        field::goldilocks_field::GoldilocksField,
        iop::witness::{PartialWitness, WitnessWrite},
        plonk::{
            circuit_builder::CircuitBuilder, circuit_data::CircuitConfig,
            config::PoseidonGoldilocksConfig,
        },
    };
    use serde::Deserialize;
    use std::{fs, iter::repeat, println};

    use crate::{biguint::CircuitBuilderBiguint, utils::ssz_num_from_bits};

    #[derive(Debug, Default, Deserialize)]
    #[serde(default)]
    struct Config {
        test_cases: Vec<TestCase>,
    }

    #[derive(Debug, Deserialize, Clone)]
    struct TestCase {
        r#type: String,
        valid: bool,
        value: String,
        ssz: Option<String>,
        tags: Vec<String>,
    }

    fn get_test_cases(path: &str) -> Result<Vec<TestCase>> {
        let yaml_str = fs::read_to_string(path).expect("Unable to read config file");
        let config: Config = serde_yaml::from_str(&yaml_str)?;

        Ok(config.test_cases)
    }

    #[test]
    fn test_ssz_num_from_bits() -> Result<()> {
        let bound_test_cases = get_test_cases("../../../vendor/eth2.0-tests/ssz/uint_bounds.yaml")?
            .iter()
            .cloned()
            .filter(|x| x.valid)
            .collect_vec();

        let random_test_cases =
            get_test_cases("../../../vendor/eth2.0-tests/ssz/uint_random.yaml")?
                .iter()
                .cloned()
                .filter(|x| x.valid)
                .collect_vec();

        let test_cases = bound_test_cases
            .iter()
            .chain(random_test_cases.iter())
            .cloned()
            .collect_vec();

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = GoldilocksField;

        let grouped_test_cases = test_cases
            .iter()
            .group_by(|x| x.r#type.clone())
            .into_iter()
            .map(|(k, v)| (k, v.cloned().collect_vec()))
            .collect_vec();

        for (type_, test_cases) in grouped_test_cases {
            let num_bits = type_
                .split("uint")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            if num_bits % 32 != 0 {
                // For  now lets test only multiples of 32
                continue;
            }

            for test_case in test_cases {
                println!(
                    "Running test case: {}_{}",
                    test_case.r#type, test_case.tags[2]
                );

                let mut pw = PartialWitness::new();

                let mut builder =
                    CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());

                let bits = (0..num_bits)
                    .map(|_| builder.add_virtual_bool_target_safe())
                    .collect::<Vec<_>>();

                let target = ssz_num_from_bits(&mut builder, &bits);

                let value = test_case.value.parse::<BigUint>().expect(
                    format!(
                        "Unable to parse value: {}_{}",
                        test_case.r#type, test_case.tags[2]
                    )
                    .as_str(),
                );

                let expected_target = builder.constant_biguint(&value);

                builder.connect_biguint(&target, &expected_target);

                let data = builder.build::<C>();

                let bits_value = BigUint::from_str_radix(&test_case.ssz.unwrap()[2..], 16)
                    .unwrap()
                    .to_str_radix(2)
                    .chars()
                    .map(|x| x == '1')
                    .collect_vec();

                let padding_length = num_bits - bits_value.len();

                let expected_bits = repeat(false)
                    .take(padding_length)
                    .chain(bits_value.iter().cloned())
                    .collect_vec();

                for i in 0..num_bits {
                    pw.set_bool_target(bits[i], expected_bits[i]);
                }

                let proof = data.prove(pw).expect(
                    format!(
                        "Prove failed for {}_{}",
                        test_case.r#type, test_case.tags[2]
                    )
                    .as_str(),
                );

                data.verify(proof).expect(
                    format!(
                        "Prove failed for {}_{}",
                        test_case.r#type, test_case.tags[2]
                    )
                    .as_str(),
                );
            }
        }

        Ok(())
    }
}
