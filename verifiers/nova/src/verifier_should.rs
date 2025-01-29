#![cfg(test)]

use super::*;

struct Mock;

pub const SOME_PARAMETER_CONST: u8 = 1;

impl Config for Mock {
    type SomeParameter = ConstU8<SOME_PARAMETER_CONST>; // arbitrary value for tests
}

include!("resources.rs");

#[test]
fn verify_valid_proof() {
    assert!(Nova::<Mock>::verify_proof(
        &VALID_VK.to_vec().into(),
        &VALID_PROOF.to_vec(),
        &VALID_PUBS.to_vec()
    )
    .is_ok());
}

mod reject {
    use hp_verifiers::VerifyError;

    use super::*;

    #[test]
    fn invalid_proof() {
        let mut invalid_proof = VALID_PROOF.clone();
        invalid_proof[11] = SOME_PARAMETER_CONST.saturating_sub(VALID_VK[0]);

        assert_eq!(
            Nova::<Mock>::verify_proof(
                &VALID_VK.to_vec().into(),
                &invalid_proof.to_vec(),
                &VALID_PUBS.to_vec()
            ),
            Err(VerifyError::VerifyError)
        )
    }
}
