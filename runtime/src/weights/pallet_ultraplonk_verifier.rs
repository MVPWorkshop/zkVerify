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

//! Autogenerated weights for `pallet_ultraplonk_verifier`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-11-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `933de9f0c728`, CPU: `AMD Ryzen 7 7700X 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-ultraplonk-verifier
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
// /data/benchmark/runtime/src/weights/pallet_ultraplonk_verifier.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.E6qfmFsjYB

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_ultraplonk_verifier` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_ultraplonk_verifier::WeightInfo for ZKVWeight<T> {
    /// Storage: `SettlementUltraplonkPallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementUltraplonkPallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    /// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
    /// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `361`
        //  Estimated: `177995`
        // Minimum execution time: 3_677_836_000 picoseconds.
        Weight::from_parts(3_694_856_000, 177995)
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `SettlementUltraplonkPallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementUltraplonkPallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    /// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
    /// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_1() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `361`
        //  Estimated: `177995`
        // Minimum execution time: 3_654_275_000 picoseconds.
        Weight::from_parts(3_670_425_000, 177995)
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `SettlementUltraplonkPallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementUltraplonkPallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    /// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
    /// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_8() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `361`
        //  Estimated: `177995`
        // Minimum execution time: 3_672_856_000 picoseconds.
        Weight::from_parts(3_694_195_000, 177995)
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `SettlementUltraplonkPallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementUltraplonkPallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    /// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
    /// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_16() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `361`
        //  Estimated: `177995`
        // Minimum execution time: 3_678_055_000 picoseconds.
        Weight::from_parts(3_696_466_000, 177995)
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `SettlementUltraplonkPallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementUltraplonkPallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    /// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
    /// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_32() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `361`
        //  Estimated: `177995`
        // Minimum execution time: 3_691_376_000 picoseconds.
        Weight::from_parts(3_706_345_000, 177995)
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `SettlementUltraplonkPallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementUltraplonkPallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementUltraplonkPallet::Vks` (r:1 w:0)
    /// Proof: `SettlementUltraplonkPallet::Vks` (`max_values`: None, `max_size`: Some(1751), added: 4226, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    /// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
    /// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2140`
        //  Estimated: `177995`
        // Minimum execution time: 3_674_516_000 picoseconds.
        Weight::from_parts(3_690_625_000, 177995)
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `SettlementUltraplonkPallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementUltraplonkPallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementUltraplonkPallet::Vks` (r:1 w:0)
    /// Proof: `SettlementUltraplonkPallet::Vks` (`max_values`: None, `max_size`: Some(1751), added: 4226, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    /// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
    /// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_32_with_vk_hash() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2140`
        //  Estimated: `177995`
        // Minimum execution time: 3_678_195_000 picoseconds.
        Weight::from_parts(3_694_785_000, 177995)
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `SettlementUltraplonkPallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementUltraplonkPallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementUltraplonkPallet::Vks` (r:0 w:1)
    /// Proof: `SettlementUltraplonkPallet::Vks` (`max_values`: None, `max_size`: Some(1751), added: 4226, mode: `MaxEncodedLen`)
    fn register_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `1486`
        // Minimum execution time: 17_490_000 picoseconds.
        Weight::from_parts(17_950_000, 1486)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `SettlementUltraplonkPallet::Tickets` (r:1 w:1)
    /// Proof: `SettlementUltraplonkPallet::Tickets` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
    /// Storage: `SettlementUltraplonkPallet::Vks` (r:1 w:1)
    /// Proof: `SettlementUltraplonkPallet::Vks` (`max_values`: None, `max_size`: Some(1759), added: 4234, mode: `MaxEncodedLen`)
    fn unregister_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1952`
        //  Estimated: `5224`
        // Minimum execution time: 174_756_000 picoseconds.
        Weight::from_parts(174_756_000, 5224)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
    }
}
