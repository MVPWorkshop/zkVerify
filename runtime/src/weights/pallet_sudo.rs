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

//! Autogenerated weights for `pallet_sudo`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 31.0.0
//! DATE: 2024-06-11, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `3e97efd37624`, CPU: `AMD EPYC 7571`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/nh-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-sudo
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
// /data/benchmark/runtime/src/weights/pallet_sudo.rs
// --template
// /data/benchmark/node/hl-deploy-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_sudo` using the New Horizen node and recommended hardware.
pub struct NHWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_sudo::WeightInfo for NHWeight<T> {
    /// Storage: `Sudo::Key` (r:1 w:1)
    /// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
    fn set_key() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `98`
        //  Estimated: `1517`
        // Minimum execution time: 22_031_000 picoseconds.
        Weight::from_parts(22_910_000, 1517)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Sudo::Key` (r:1 w:0)
    /// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
    fn sudo() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `98`
        //  Estimated: `1517`
        // Minimum execution time: 23_681_000 picoseconds.
        Weight::from_parts(24_340_000, 1517)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: `Sudo::Key` (r:1 w:0)
    /// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
    fn sudo_as() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `98`
        //  Estimated: `1517`
        // Minimum execution time: 23_570_000 picoseconds.
        Weight::from_parts(24_141_000, 1517)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: `Sudo::Key` (r:1 w:1)
    /// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
    fn remove_key() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `98`
        //  Estimated: `1517`
        // Minimum execution time: 20_350_000 picoseconds.
        Weight::from_parts(20_960_000, 1517)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}