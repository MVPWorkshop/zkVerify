// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_groth16_verifier`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-10-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `e6dabcdac20d`, CPU: `AMD Ryzen 7 7700X 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-groth16-verifier
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /data/benchmark/HEADER-APACHE2
// --output
// /data/benchmark/runtime/src/weights/pallet_groth16_verifier.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.UFBk8aQwaf

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_groth16_verifier` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_groth16_verifier::WeightInfo for ZKVWeight<T> {
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bn254(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 25_002_832_000 picoseconds.
        Weight::from_parts(25_117_910_298, 3537)
            // Standard Error: 597_321
            .saturating_add(Weight::from_parts(1_550_475_342, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bls12_381(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 25_534_672_000 picoseconds.
        Weight::from_parts(25_673_128_076, 3537)
            // Standard Error: 1_179_829
            .saturating_add(Weight::from_parts(1_954_779_833, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(3948), added: 6423, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bn254_with_vk_hash(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `663 + n * (66 ±0)`
        //  Estimated: `7413`
        // Minimum execution time: 19_117_852_000 picoseconds.
        Weight::from_parts(19_215_654_116, 7413)
            // Standard Error: 519_274
            .saturating_add(Weight::from_parts(1_048_389_408, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(3948), added: 6423, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bls12_381_with_vk_hash(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `919 + n * (98 ±0)`
        //  Estimated: `7413`
        // Minimum execution time: 23_054_597_000 picoseconds.
        Weight::from_parts(23_107_300_522, 7413)
            // Standard Error: 629_763
            .saturating_add(Weight::from_parts(1_572_766_339, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:0 w:1)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(3948), added: 6423, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn register_vk_bn254(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 5_951_580_000 picoseconds.
        Weight::from_parts(6_029_142_659, 0)
            // Standard Error: 1_467_227
            .saturating_add(Weight::from_parts(504_319_890, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:0 w:1)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(3948), added: 6423, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn register_vk_bls12_381(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_481_354_000 picoseconds.
        Weight::from_parts(3_683_552_606, 0)
            // Standard Error: 6_177_860
            .saturating_add(Weight::from_parts(294_677_727, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}
