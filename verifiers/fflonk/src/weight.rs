// Copyright 2024, Horizen Labs, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_settlement_fflonk`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 31.0.0
//! DATE: 2024-05-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `miklap`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /home/mdamico/devel/NH-core/target/production/nh-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-settlement-fflonk
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /home/mdamico/devel/NH-core/HEADER-APACHE2
// --output
// pallets/settlement-fflonk/src/weight.rs
// --template
// /home/mdamico/devel/NH-core/node/hl-pallets-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub trait WeightInfo {
    fn submit_proof() -> Weight;
    fn submit_proof_with_vk_hash() -> Weight;
    fn register_vk() -> Weight;
}

// For backwards compatibility and tests.
impl WeightInfo for () {
    fn submit_proof() -> Weight {
        Weight::from_parts(26_647_286_000, 4010)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    fn submit_proof_with_vk_hash() -> Weight {
        Weight::from_parts(27_201_242_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    fn register_vk() -> Weight {
        Weight::from_parts(3_695_874_000, 0)
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}