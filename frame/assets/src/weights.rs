// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_assets
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP VALUES: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_assets
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/assets/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_assets.
pub trait WeightInfo {
	fn create() -> Weight;
	fn force_create() -> Weight;
	fn destroy(c: u32, s: u32, a: u32, ) -> Weight;
	fn mint() -> Weight;
	fn burn() -> Weight;
	fn transfer() -> Weight;
	fn transfer_keep_alive() -> Weight;
	fn force_transfer() -> Weight;
	fn freeze() -> Weight;
	fn thaw() -> Weight;
	fn freeze_asset() -> Weight;
	fn thaw_asset() -> Weight;
	fn transfer_ownership() -> Weight;
	fn set_team() -> Weight;
	fn set_metadata(n: u32, s: u32, ) -> Weight;
	fn clear_metadata() -> Weight;
	fn force_set_metadata(n: u32, s: u32, ) -> Weight;
	fn force_clear_metadata() -> Weight;
	fn force_asset_status() -> Weight;
	fn approve_transfer() -> Weight;
	fn transfer_approved() -> Weight;
	fn cancel_approval() -> Weight;
	fn force_cancel_approval() -> Weight;
}

/// Weights for pallet_assets using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn create() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1715` bytes
		(27_428_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn force_create() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1191` bytes
		(15_512_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:5002 w:5001)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	/// Storage: System Account (r:5000 w:5000)
	/// Proof Skipped: System Account (max_values: None, max_size: None)
	/// Storage: Assets Metadata (r:1 w:0)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	/// Storage: Assets Approvals (r:501 w:500)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2708)
	/// The range of component `c` is `[0, 5000]`.
	/// The range of component `s` is `[0, 5000]`.
	/// The range of component `a` is `[0, 500]`.
	fn destroy(c: u32, s: u32, a: u32, ) -> Weight {
		// Proof Size
		// Worst Case: `10840` bytes
		// Base: `5149` bytes
		//     PoV component: `c * 240` with slope stddev `0`.
		//     PoV component: `s * 240` with slope stddev `0`.
		//     PoV component: `a * 122` with slope stddev `0`.
		(0 as Weight)
			// Standard Error: `36_000`
			.saturating_add((17_579_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: `36_000`
			.saturating_add((20_074_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: `363_000`
			.saturating_add((18_244_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	fn mint() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `1865` bytes
		(31_653_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	fn burn() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `2093` bytes
		(36_030_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	/// Storage: System Account (r:1 w:1)
	/// Proof Skipped: System Account (max_values: None, max_size: None)
	fn transfer() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `2132` bytes
		(49_245_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	/// Storage: System Account (r:1 w:1)
	/// Proof Skipped: System Account (max_values: None, max_size: None)
	fn transfer_keep_alive() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `1960` bytes
		(42_245_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	/// Storage: System Account (r:1 w:1)
	/// Proof Skipped: System Account (max_values: None, max_size: None)
	fn force_transfer() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `2132` bytes
		(48_359_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	fn freeze() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `1569` bytes
		(22_026_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	fn thaw() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `1569` bytes
		(22_429_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn freeze_asset() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1495` bytes
		(18_530_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn thaw_asset() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1495` bytes
		(18_480_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Metadata (r:1 w:0)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	fn transfer_ownership() -> Weight {
		// Proof Size
		// Worst Case: `5470` bytes
		// Base: `1461` bytes
		(20_225_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn set_team() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1461` bytes
		(18_531_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(_n: u32, _s: u32, ) -> Weight {
		// Proof Size
		// Worst Case: `5470` bytes
		// Base: `1985` bytes
		(33_742_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	fn clear_metadata() -> Weight {
		// Proof Size
		// Worst Case: `5470` bytes
		// Base: `2181` bytes
		(33_033_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(_n: u32, _s: u32, ) -> Weight {
		// Proof Size
		// Worst Case: `5470` bytes
		// Base: `1461` bytes
		(20_082_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	fn force_clear_metadata() -> Weight {
		// Proof Size
		// Worst Case: `5470` bytes
		// Base: `2181` bytes
		(32_332_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn force_asset_status() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1461` bytes
		(17_694_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2708)
	fn approve_transfer() -> Weight {
		// Proof Size
		// Worst Case: `5478` bytes
		// Base: `2019` bytes
		(36_936_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2708)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	/// Storage: System Account (r:1 w:1)
	/// Proof Skipped: System Account (max_values: None, max_size: None)
	fn transfer_approved() -> Weight {
		// Proof Size
		// Worst Case: `8140` bytes
		// Base: `2302` bytes
		(62_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2708)
	fn cancel_approval() -> Weight {
		// Proof Size
		// Worst Case: `5478` bytes
		// Base: `2189` bytes
		(37_159_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2708)
	fn force_cancel_approval() -> Weight {
		// Proof Size
		// Worst Case: `5478` bytes
		// Base: `2189` bytes
		(38_118_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn create() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1715` bytes
		(27_428_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn force_create() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1191` bytes
		(15_512_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:5002 w:5001)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	/// Storage: System Account (r:5000 w:5000)
	/// Proof Skipped: System Account (max_values: None, max_size: None)
	/// Storage: Assets Metadata (r:1 w:0)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	/// Storage: Assets Approvals (r:501 w:500)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2708)
	/// The range of component `c` is `[0, 5000]`.
	/// The range of component `s` is `[0, 5000]`.
	/// The range of component `a` is `[0, 500]`.
	fn destroy(c: u32, s: u32, a: u32, ) -> Weight {
		// Proof Size
		// Worst Case: `10840` bytes
		// Base: `5149` bytes
		//     PoV component: `c * 240` with slope stddev `0`.
		//     PoV component: `s * 240` with slope stddev `0`.
		//     PoV component: `a * 122` with slope stddev `0`.
		(0 as Weight)
			// Standard Error: `36_000`
			.saturating_add((17_579_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: `36_000`
			.saturating_add((20_074_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: `363_000`
			.saturating_add((18_244_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	fn mint() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `1865` bytes
		(31_653_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	fn burn() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `2093` bytes
		(36_030_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	/// Storage: System Account (r:1 w:1)
	/// Proof Skipped: System Account (max_values: None, max_size: None)
	fn transfer() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `2132` bytes
		(49_245_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	/// Storage: System Account (r:1 w:1)
	/// Proof Skipped: System Account (max_values: None, max_size: None)
	fn transfer_keep_alive() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `1960` bytes
		(42_245_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	/// Storage: System Account (r:1 w:1)
	/// Proof Skipped: System Account (max_values: None, max_size: None)
	fn force_transfer() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `2132` bytes
		(48_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	fn freeze() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `1569` bytes
		(22_026_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	fn thaw() -> Weight {
		// Proof Size
		// Worst Case: `5432` bytes
		// Base: `1569` bytes
		(22_429_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn freeze_asset() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1495` bytes
		(18_530_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn thaw_asset() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1495` bytes
		(18_480_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Metadata (r:1 w:0)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	fn transfer_ownership() -> Weight {
		// Proof Size
		// Worst Case: `5470` bytes
		// Base: `1461` bytes
		(20_225_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn set_team() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1461` bytes
		(18_531_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(_n: u32, _s: u32, ) -> Weight {
		// Proof Size
		// Worst Case: `5470` bytes
		// Base: `1985` bytes
		(33_742_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	fn clear_metadata() -> Weight {
		// Proof Size
		// Worst Case: `5470` bytes
		// Base: `2181` bytes
		(33_033_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(_n: u32, _s: u32, ) -> Weight {
		// Proof Size
		// Worst Case: `5470` bytes
		// Base: `1461` bytes
		(20_082_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2700)
	fn force_clear_metadata() -> Weight {
		// Proof Size
		// Worst Case: `5470` bytes
		// Base: `2181` bytes
		(32_332_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	fn force_asset_status() -> Weight {
		// Proof Size
		// Worst Case: `2770` bytes
		// Base: `1461` bytes
		(17_694_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2708)
	fn approve_transfer() -> Weight {
		// Proof Size
		// Worst Case: `5478` bytes
		// Base: `2019` bytes
		(36_936_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2708)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2662)
	/// Storage: System Account (r:1 w:1)
	/// Proof Skipped: System Account (max_values: None, max_size: None)
	fn transfer_approved() -> Weight {
		// Proof Size
		// Worst Case: `8140` bytes
		// Base: `2302` bytes
		(62_989_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2708)
	fn cancel_approval() -> Weight {
		// Proof Size
		// Worst Case: `5478` bytes
		// Base: `2189` bytes
		(37_159_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2770)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2708)
	fn force_cancel_approval() -> Weight {
		// Proof Size
		// Worst Case: `5478` bytes
		// Base: `2189` bytes
		(38_118_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
