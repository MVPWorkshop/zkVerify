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

//! Autogenerated weights for `pallet_im_online`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-12-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `55be928bbcca`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-im-online
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
// /data/benchmark/runtime/src/weights/pallet_im_online.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.w0huHsUPPk

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_im_online` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_im_online::WeightInfo for ZKVWeight<T> {
    /// Storage: `Session::Validators` (r:1 w:0)
    /// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Session::CurrentIndex` (r:1 w:0)
    /// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ImOnline::Keys` (r:1 w:0)
    /// Proof: `ImOnline::Keys` (`max_values`: Some(1), `max_size`: Some(320002), added: 320497, mode: `MaxEncodedLen`)
    /// Storage: `ImOnline::ReceivedHeartbeats` (r:1 w:1)
    /// Proof: `ImOnline::ReceivedHeartbeats` (`max_values`: None, `max_size`: Some(25), added: 2500, mode: `MaxEncodedLen`)
    /// Storage: `ImOnline::AuthoredBlocks` (r:1 w:0)
    /// Proof: `ImOnline::AuthoredBlocks` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// The range of component `k` is `[1, 1000]`.
    fn validate_unsigned_and_then_heartbeat(k: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `261 + k * (32 ±0)`
        //  Estimated: `321487 + k * (1761 ±0)`
        // Minimum execution time: 57_167_000 picoseconds.
        Weight::from_parts(59_850_179, 321487)
            // Standard Error: 176
            .saturating_add(Weight::from_parts(17_579, 0).saturating_mul(k.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(Weight::from_parts(0, 1761).saturating_mul(k.into()))
    }
}
