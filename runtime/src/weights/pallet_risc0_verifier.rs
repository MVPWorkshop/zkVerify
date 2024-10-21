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

//! Autogenerated weights for `pallet_risc0_verifier`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-10-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `b209d8e1964e`, CPU: `AMD Ryzen 7 7700X 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-risc0-verifier
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
// /data/benchmark/runtime/src/weights/pallet_risc0_verifier.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.Xbh2my8yxI

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_risc0_verifier` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_risc0_verifier::WeightInfo for ZKVWeight<T> {
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_12() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 16_432_378_000 picoseconds.
        Weight::from_parts(16_446_168_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_13() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 16_427_368_000 picoseconds.
        Weight::from_parts(16_434_838_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_14() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 17_380_139_000 picoseconds.
        Weight::from_parts(17_396_859_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_15() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 17_377_800_000 picoseconds.
        Weight::from_parts(17_384_929_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_16() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 17_381_619_000 picoseconds.
        Weight::from_parts(17_392_730_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_17() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 17_375_989_000 picoseconds.
        Weight::from_parts(17_389_830_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_18() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 18_345_191_000 picoseconds.
        Weight::from_parts(18_358_021_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_19() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 19_415_663_000 picoseconds.
        Weight::from_parts(19_426_183_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_20() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 20_675_895_000 picoseconds.
        Weight::from_parts(20_690_055_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_21() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 36_493_802_000 picoseconds.
        Weight::from_parts(36_515_792_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_22() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 55_629_314_000 picoseconds.
        Weight::from_parts(55_646_695_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_23() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 94_896_041_000 picoseconds.
        Weight::from_parts(94_923_421_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_24() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `150`
        //  Estimated: `3537`
        // Minimum execution time: 171_399_390_000 picoseconds.
        Weight::from_parts(171_417_950_000, 3537)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_12() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 16_435_447_000 picoseconds.
        Weight::from_parts(16_443_888_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_13() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 16_435_638_000 picoseconds.
        Weight::from_parts(16_444_188_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_14() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 17_385_019_000 picoseconds.
        Weight::from_parts(17_391_779_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_15() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 17_371_029_000 picoseconds.
        Weight::from_parts(17_392_599_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_16() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 17_386_909_000 picoseconds.
        Weight::from_parts(17_395_430_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_17() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 17_374_140_000 picoseconds.
        Weight::from_parts(17_390_479_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_18() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 18_348_881_000 picoseconds.
        Weight::from_parts(18_361_851_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_19() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 19_427_203_000 picoseconds.
        Weight::from_parts(19_433_343_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_20() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 20_684_445_000 picoseconds.
        Weight::from_parts(20_697_815_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_21() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 36_507_172_000 picoseconds.
        Weight::from_parts(36_520_482_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_22() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 55_633_014_000 picoseconds.
        Weight::from_parts(55_651_454_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_23() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 94_915_210_000 picoseconds.
        Weight::from_parts(94_948_600_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_24() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3537`
        // Minimum execution time: 171_449_639_000 picoseconds.
        Weight::from_parts(171_500_828_000, 3537)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementRisc0Pallet::Vks` (r:0 w:1)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    fn register_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `76`
        //  Estimated: `1486`
        // Minimum execution time: 7_000_000 picoseconds.
        Weight::from_parts(7_290_000, 1486)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}
