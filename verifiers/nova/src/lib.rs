#![cfg_attr(not(feature = "std"), no_std)]

use core::{hash::Hash, marker::PhantomData};
use frame_support::weights::Weight;
use hp_verifiers::Verifier;
use nova_verifier::verifier::verify;
use sp_core::*;

pub mod benchmarking;
mod verifier_should;
mod weight;
pub use weight::WeightInfo;

pub trait Config: 'static {
    /// Some parameter for Nova verifier
    type SomeParameter: Get<u8>;

    fn get_some_parameter() -> u8 {
        Self::SomeParameter::get()
    }
}

// TODO -> Check if this is MAX
pub const VK_MAX_LEN: usize = 10302515;

// TODO -> Maybe send VerificationKey as a struct, and not bytes?
pub type Vk = [u8; VK_MAX_LEN];
pub type Proof = Vec<u8>;

// TODO -> Additional info
pub type Pubs = [u8; 32];

#[pallet_verifiers::verifier]
pub struct Nova<T>;

impl<T: Config> Verifier for Nova<T> {
    type Vk = Vk;
    type Proof = Proof;
    type Pubs = Pubs;

    fn hash_context_data() -> &'static [u8] {
        b"nova"
    }

    fn verify_proof(
        vk: &Self::Vk,
        proof: &Self::Proof,
        pubs: &Self::Pubs,
    ) -> Result<(), hp_verifiers::VerifyError> {
        log::trace!("Verifying proof");

        nova_verifier::verifier::verify(&vk.to_vec(), &proof)
            .map_err(|_| log::debug!("Cannot verify Nova proof"))
            .map_err(|_| hp_verifiers::VerifyError::VerifyError)
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> hp_verifiers::Cow<[u8]> {
        hp_verifiers::Cow::Borrowed(pubs)
    }
}

pub struct NovaWeight<W: weight::WeightInfo>(PhantomData<W>);

impl<T: Config, W: weight::WeightInfo> pallet_verifiers::WeightInfo<Nova<T>> for NovaWeight<W> {
    fn submit_proof(
        _proof: &<Nova<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<Nova<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof()
    }

    fn submit_proof_with_vk_hash(
        _proof: &<Nova<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<Nova<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof_with_vk_hash()
    }

    fn register_vk(_vk: &<Nova<T> as hp_verifiers::Verifier>::Vk) -> Weight {
        W::register_vk()
    }

    fn unregister_vk() -> frame_support::weights::Weight {
        W::unregister_vk()
    }
}
