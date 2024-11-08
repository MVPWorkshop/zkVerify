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

//! Autogenerated weights for `crate::parachains::hrmp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-09-03, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `drhorizen`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /home/daniele/horizen/git/zkVerify/target/release/zkv-relay
// benchmark
// pallet
// --chain
// dev
// --pallet
// crate::parachains::hrmp
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /home/daniele/horizen/git/zkVerify/HEADER-APACHE2
// --output
// /home/daniele/horizen/git/zkVerify/runtime/src/weights/parachains/hrmp.rs
// --template
// /home/daniele/horizen/git/zkVerify/node/zkv-deploy-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `crate::parachains::hrmp` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> crate::parachains::hrmp::WeightInfo for ZKVWeight<T> {
    /// Storage: `Paras::ParaLifecycles` (r:1 w:0)
    /// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn hrmp_init_open_channel() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `311`
        //  Estimated: `3776`
        // Minimum execution time: 53_593_000 picoseconds.
        Weight::from_parts(55_489_000, 3776)
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn hrmp_accept_open_channel() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `302`
        //  Estimated: `3767`
        // Minimum execution time: 46_774_000 picoseconds.
        Weight::from_parts(47_802_000, 3767)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpCloseChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpCloseChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpCloseChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpCloseChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn hrmp_close_channel() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `415`
        //  Estimated: `3880`
        // Minimum execution time: 44_787_000 picoseconds.
        Weight::from_parts(45_789_000, 3880)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:254 w:254)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:0 w:1)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannelContents` (r:0 w:254)
    /// Proof: `Hrmp::HrmpChannelContents` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:0 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `i` is `[0, 127]`.
    /// The range of component `e` is `[0, 127]`.
    fn force_clean_hrmp(i: u32, e: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `159 + e * (100 ±0) + i * (100 ±0)`
        //  Estimated: `3621 + e * (2575 ±0) + i * (2575 ±0)`
        // Minimum execution time: 1_253_800_000 picoseconds.
        Weight::from_parts(117_547_985, 3621)
            // Standard Error: 40_044
            .saturating_add(Weight::from_parts(9_157_328, 0).saturating_mul(i.into()))
            // Standard Error: 40_044
            .saturating_add(Weight::from_parts(9_715_578, 0).saturating_mul(e.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(i.into())))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(e.into())))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(i.into())))
            .saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(e.into())))
            .saturating_add(Weight::from_parts(0, 2575).saturating_mul(e.into()))
            .saturating_add(Weight::from_parts(0, 2575).saturating_mul(i.into()))
    }
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:128 w:128)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Paras::ParaLifecycles` (r:256 w:0)
    /// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:128 w:128)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:128 w:128)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:0 w:128)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `c` is `[0, 128]`.
    fn force_process_hrmp_open(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `386 + c * (136 ±0)`
        //  Estimated: `1841 + c * (5086 ±0)`
        // Minimum execution time: 6_027_000 picoseconds.
        Weight::from_parts(10_806_286, 1841)
            // Standard Error: 16_716
            .saturating_add(Weight::from_parts(20_760_558, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(c.into())))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(c.into())))
            .saturating_add(Weight::from_parts(0, 5086).saturating_mul(c.into()))
    }
    /// Storage: `Hrmp::HrmpCloseChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpCloseChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:128 w:128)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpCloseChannelRequests` (r:0 w:128)
    /// Proof: `Hrmp::HrmpCloseChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannelContents` (r:0 w:128)
    /// Proof: `Hrmp::HrmpChannelContents` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `c` is `[0, 128]`.
    fn force_process_hrmp_close(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `230 + c * (124 ±0)`
        //  Estimated: `1690 + c * (2600 ±0)`
        // Minimum execution time: 4_996_000 picoseconds.
        Weight::from_parts(10_527_420, 1690)
            // Standard Error: 9_799
            .saturating_add(Weight::from_parts(12_589_411, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(c.into())))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(c.into())))
            .saturating_add(Weight::from_parts(0, 2600).saturating_mul(c.into()))
    }
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `c` is `[0, 128]`.
    fn hrmp_cancel_open_request(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `921 + c * (13 ±0)`
        //  Estimated: `4190 + c * (15 ±0)`
        // Minimum execution time: 17_224_000 picoseconds.
        Weight::from_parts(23_598_687, 4190)
            // Standard Error: 1_558
            .saturating_add(Weight::from_parts(69_541, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 15).saturating_mul(c.into()))
    }
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:128 w:128)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `c` is `[0, 128]`.
    fn clean_open_channel_requests(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `138 + c * (63 ±0)`
        //  Estimated: `1617 + c * (2538 ±0)`
        // Minimum execution time: 3_188_000 picoseconds.
        Weight::from_parts(7_181_307, 1617)
            // Standard Error: 3_361
            .saturating_add(Weight::from_parts(3_197_688, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
            .saturating_add(Weight::from_parts(0, 2538).saturating_mul(c.into()))
    }
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Paras::ParaLifecycles` (r:1 w:0)
    /// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:2 w:2)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:2 w:2)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `c` is `[0, 1]`.
    fn force_open_hrmp_channel(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `311 + c * (235 ±0)`
        //  Estimated: `6251 + c * (235 ±0)`
        // Minimum execution time: 47_519_000 picoseconds.
        Weight::from_parts(49_730_679, 6251)
            // Standard Error: 86_396
            .saturating_add(Weight::from_parts(11_190_620, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(12_u64))
            .saturating_add(T::DbWeight::get().writes(8_u64))
            .saturating_add(Weight::from_parts(0, 235).saturating_mul(c.into()))
    }
    /// Storage: `Paras::ParaLifecycles` (r:1 w:0)
    /// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:2 w:2)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:2 w:2)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn establish_system_channel() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `311`
        //  Estimated: `6251`
        // Minimum execution time: 47_061_000 picoseconds.
        Weight::from_parts(48_510_000, 6251)
            .saturating_add(T::DbWeight::get().reads(12_u64))
            .saturating_add(T::DbWeight::get().writes(8_u64))
    }
    /// Storage: `Hrmp::HrmpChannels` (r:1 w:1)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn poke_channel_deposits() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `158`
        //  Estimated: `3623`
        // Minimum execution time: 11_444_000 picoseconds.
        Weight::from_parts(11_767_000, 3623)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}