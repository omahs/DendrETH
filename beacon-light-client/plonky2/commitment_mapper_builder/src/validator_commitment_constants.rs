use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorCommitmentConstants {
    pub validator_key: String,
    pub validator_proof_key: String,
    pub validator_proofs_queue: String,
}

pub fn get_validator_commitment_constants() -> ValidatorCommitmentConstants {
    serde_json::from_str(include_str!("../../constants/validator_commitment_constants.json")).unwrap()
}
