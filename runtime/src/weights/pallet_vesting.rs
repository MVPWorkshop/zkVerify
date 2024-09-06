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

// TODO: Autogenerate with remote machine and add details + executed command

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct ZKVWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for ZKVWeight<T> {
    /// Storage: Vesting Vesting (r:1 w:1)
    /// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
    /// Storage: Balances Locks (r:1 w:1)
    /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
    /// Storage: Balances Freezes (r:1 w:0)
    /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[1, 28]`.
    fn vest_locked(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `348 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 35_225_000 picoseconds.
        Weight::from_parts(34_420_748, 0)
            .saturating_add(Weight::from_parts(0, 4764))
            // Standard Error: 2_341
            .saturating_add(Weight::from_parts(41_794, 0).saturating_mul(l.into()))
            // Standard Error: 4_166
            .saturating_add(Weight::from_parts(114_507, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Vesting Vesting (r:1 w:1)
    /// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
    /// Storage: Balances Locks (r:1 w:1)
    /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
    /// Storage: Balances Freezes (r:1 w:0)
    /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[1, 28]`.
    fn vest_unlocked(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `348 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 38_507_000 picoseconds.
        Weight::from_parts(38_552_717, 0)
            .saturating_add(Weight::from_parts(0, 4764))
            // Standard Error: 2_406
            .saturating_add(Weight::from_parts(42_332, 0).saturating_mul(l.into()))
            // Standard Error: 4_282
            .saturating_add(Weight::from_parts(67_638, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Vesting Vesting (r:1 w:1)
    /// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
    /// Storage: Balances Locks (r:1 w:1)
    /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
    /// Storage: Balances Freezes (r:1 w:0)
    /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[1, 28]`.
    fn vest_other_locked(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `451 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 36_919_000 picoseconds.
        Weight::from_parts(35_087_984, 0)
            .saturating_add(Weight::from_parts(0, 4764))
            // Standard Error: 2_435
            .saturating_add(Weight::from_parts(66_131, 0).saturating_mul(l.into()))
            // Standard Error: 4_333
            .saturating_add(Weight::from_parts(125_178, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: Vesting Vesting (r:1 w:1)
    /// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
    /// Storage: Balances Locks (r:1 w:1)
    /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
    /// Storage: Balances Freezes (r:1 w:0)
    /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[1, 28]`.
    fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `451 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 40_393_000 picoseconds.
        Weight::from_parts(39_522_987, 0)
            .saturating_add(Weight::from_parts(0, 4764))
            // Standard Error: 1_958
            .saturating_add(Weight::from_parts(46_626, 0).saturating_mul(l.into()))
            // Standard Error: 3_484
            .saturating_add(Weight::from_parts(94_547, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: Vesting Vesting (r:1 w:1)
    /// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Balances Locks (r:1 w:1)
    /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
    /// Storage: Balances Freezes (r:1 w:0)
    /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[0, 27]`.
    fn vested_transfer(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `522 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 72_925_000 picoseconds.
        Weight::from_parts(75_858_529, 0)
            .saturating_add(Weight::from_parts(0, 4764))
            // Standard Error: 3_995
            .saturating_add(Weight::from_parts(70_032, 0).saturating_mul(l.into()))
            // Standard Error: 7_108
            .saturating_add(Weight::from_parts(160_507, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: Vesting Vesting (r:1 w:1)
    /// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
    /// Storage: System Account (r:2 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Balances Locks (r:1 w:1)
    /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
    /// Storage: Balances Freezes (r:1 w:0)
    /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[0, 27]`.
    fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `625 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `6196`
        // Minimum execution time: 74_405_000 picoseconds.
        Weight::from_parts(78_253_087, 0)
            .saturating_add(Weight::from_parts(0, 6196))
            // Standard Error: 3_708
            .saturating_add(Weight::from_parts(56_748, 0).saturating_mul(l.into()))
            // Standard Error: 6_598
            .saturating_add(Weight::from_parts(146_713, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(4))
    }
    /// Storage: Vesting Vesting (r:1 w:1)
    /// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
    /// Storage: Balances Locks (r:1 w:1)
    /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
    /// Storage: Balances Freezes (r:1 w:0)
    /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[2, 28]`.
    fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `449 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 37_715_000 picoseconds.
        Weight::from_parts(36_483_330, 0)
            .saturating_add(Weight::from_parts(0, 4764))
            // Standard Error: 2_146
            .saturating_add(Weight::from_parts(55_976, 0).saturating_mul(l.into()))
            // Standard Error: 3_964
            .saturating_add(Weight::from_parts(116_455, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: Vesting Vesting (r:1 w:1)
    /// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
    /// Storage: Balances Locks (r:1 w:1)
    /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
    /// Storage: Balances Freezes (r:1 w:0)
    /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[2, 28]`.
    fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `449 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 42_102_000 picoseconds.
        Weight::from_parts(41_671_515, 0)
            .saturating_add(Weight::from_parts(0, 4764))
            // Standard Error: 2_743
            .saturating_add(Weight::from_parts(47_496, 0).saturating_mul(l.into()))
            // Standard Error: 5_065
            .saturating_add(Weight::from_parts(95_785, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }

    /// Storage: `Vesting::Vesting` (r:1 w:1)
    /// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[2, 28]`.
    fn force_remove_vesting_schedule(l: u32, s: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `555 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 41_497_000 picoseconds.
        Weight::from_parts(38_763_834, 4764)
            // Standard Error: 2_030
            .saturating_add(Weight::from_parts(99_580, 0).saturating_mul(l.into()))
            // Standard Error: 3_750
            .saturating_add(Weight::from_parts(132_188, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
}