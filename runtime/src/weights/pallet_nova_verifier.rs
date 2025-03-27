#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_nova_verifier` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_plonky2_verifier::WeightInfo for ZKVWeight<T> {
    fn verify_proof() -> Weight {
        Weight::from_parts(10_000_000, 1000)
    }
    fn get_vk() -> Weight {
        Weight::from_parts(10_000_000, 1000)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    fn validate_vk() -> Weight {
        Weight::from_parts(10_000_000, 1000)
    }
    fn compute_statement_hash() -> Weight {
        Weight::from_parts(10_000_000, 1000)
    }
    fn register_vk() -> Weight {
        Weight::from_parts(10_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    fn unregister_vk() -> Weight {
        Weight::from_parts(10_000_000, 1000)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
}
