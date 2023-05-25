// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-23, STEPS: `50`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("manta-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=manta-dev
// --steps=50
// --repeat=40
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_collective.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_collective.
pub trait WeightInfo {
    fn set_members(m: u32, n: u32, p: u32, ) -> Weight;
    fn execute(b: u32, m: u32, ) -> Weight;
    fn propose_execute(b: u32, m: u32, ) -> Weight;
    fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight;
    fn vote(m: u32, ) -> Weight;
    fn close_early_disapproved(m: u32, p: u32, ) -> Weight;
    fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight;
    fn close_disapproved(m: u32, p: u32, ) -> Weight;
    fn close_approved(b: u32, m: u32, p: u32, ) -> Weight;
    fn disapprove_proposal(p: u32, ) -> Weight;
}

/// Weights for pallet_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for SubstrateWeight<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Prime (r:0 w:1)
	// Storage: Council Voting (r:100 w:100)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Minimum execution time: 19_119 nanoseconds.
		Weight::from_ref_time(19_599_000)
			// Standard Error: 53_196
			.saturating_add(Weight::from_ref_time(6_096_439).saturating_mul(m.into()))
			// Standard Error: 53_196
			.saturating_add(Weight::from_ref_time(8_544_422).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Council Members (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 21_154 nanoseconds.
		Weight::from_ref_time(22_127_938)
			// Standard Error: 117
			.saturating_add(Weight::from_ref_time(1_265).saturating_mul(b.into()))
			// Standard Error: 1_209
			.saturating_add(Weight::from_ref_time(13_876).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, _m: u32, ) -> Weight {
		// Minimum execution time: 23_240 nanoseconds.
		Weight::from_ref_time(30_162_006)
			// Standard Error: 468
			.saturating_add(Weight::from_ref_time(355).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 29_605 nanoseconds.
		Weight::from_ref_time(30_237_587)
			// Standard Error: 89
			.saturating_add(Weight::from_ref_time(4_880).saturating_mul(b.into()))
			// Standard Error: 935
			.saturating_add(Weight::from_ref_time(27_456).saturating_mul(m.into()))
			// Standard Error: 923
			.saturating_add(Weight::from_ref_time(170_959).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Minimum execution time: 34_241 nanoseconds.
		Weight::from_ref_time(35_604_588)
			// Standard Error: 437
			.saturating_add(Weight::from_ref_time(41_942).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 32_555 nanoseconds.
		Weight::from_ref_time(35_152_528)
			// Standard Error: 908
			.saturating_add(Weight::from_ref_time(29_272).saturating_mul(m.into()))
			// Standard Error: 885
			.saturating_add(Weight::from_ref_time(161_586).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 44_322 nanoseconds.
		Weight::from_ref_time(46_110_044)
			// Standard Error: 94
			.saturating_add(Weight::from_ref_time(2_959).saturating_mul(b.into()))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(22_709).saturating_mul(m.into()))
			// Standard Error: 974
			.saturating_add(Weight::from_ref_time(178_996).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 35_353 nanoseconds.
		Weight::from_ref_time(38_252_558)
			// Standard Error: 932
			.saturating_add(Weight::from_ref_time(31_885).saturating_mul(m.into()))
			// Standard Error: 908
			.saturating_add(Weight::from_ref_time(155_887).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 46_335 nanoseconds.
		Weight::from_ref_time(50_047_179)
			// Standard Error: 98
			.saturating_add(Weight::from_ref_time(2_011).saturating_mul(b.into()))
			// Standard Error: 1_040
			.saturating_add(Weight::from_ref_time(18_097).saturating_mul(m.into()))
			// Standard Error: 1_014
			.saturating_add(Weight::from_ref_time(176_368).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Minimum execution time: 20_977 nanoseconds.
		Weight::from_ref_time(24_427_354)
			// Standard Error: 1_225
			.saturating_add(Weight::from_ref_time(154_372).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Prime (r:0 w:1)
	// Storage: Council Voting (r:100 w:100)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Minimum execution time: 19_119 nanoseconds.
		Weight::from_ref_time(19_599_000)
			// Standard Error: 53_196
			.saturating_add(Weight::from_ref_time(6_096_439).saturating_mul(m.into()))
			// Standard Error: 53_196
			.saturating_add(Weight::from_ref_time(8_544_422).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(RocksDbWeight::get().writes(2))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Council Members (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 21_154 nanoseconds.
		Weight::from_ref_time(22_127_938)
			// Standard Error: 117
			.saturating_add(Weight::from_ref_time(1_265).saturating_mul(b.into()))
			// Standard Error: 1_209
			.saturating_add(Weight::from_ref_time(13_876).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, _m: u32, ) -> Weight {
		// Minimum execution time: 23_240 nanoseconds.
		Weight::from_ref_time(30_162_006)
			// Standard Error: 468
			.saturating_add(Weight::from_ref_time(355).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 29_605 nanoseconds.
		Weight::from_ref_time(30_237_587)
			// Standard Error: 89
			.saturating_add(Weight::from_ref_time(4_880).saturating_mul(b.into()))
			// Standard Error: 935
			.saturating_add(Weight::from_ref_time(27_456).saturating_mul(m.into()))
			// Standard Error: 923
			.saturating_add(Weight::from_ref_time(170_959).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Minimum execution time: 34_241 nanoseconds.
		Weight::from_ref_time(35_604_588)
			// Standard Error: 437
			.saturating_add(Weight::from_ref_time(41_942).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 32_555 nanoseconds.
		Weight::from_ref_time(35_152_528)
			// Standard Error: 908
			.saturating_add(Weight::from_ref_time(29_272).saturating_mul(m.into()))
			// Standard Error: 885
			.saturating_add(Weight::from_ref_time(161_586).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 44_322 nanoseconds.
		Weight::from_ref_time(46_110_044)
			// Standard Error: 94
			.saturating_add(Weight::from_ref_time(2_959).saturating_mul(b.into()))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(22_709).saturating_mul(m.into()))
			// Standard Error: 974
			.saturating_add(Weight::from_ref_time(178_996).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 35_353 nanoseconds.
		Weight::from_ref_time(38_252_558)
			// Standard Error: 932
			.saturating_add(Weight::from_ref_time(31_885).saturating_mul(m.into()))
			// Standard Error: 908
			.saturating_add(Weight::from_ref_time(155_887).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 46_335 nanoseconds.
		Weight::from_ref_time(50_047_179)
			// Standard Error: 98
			.saturating_add(Weight::from_ref_time(2_011).saturating_mul(b.into()))
			// Standard Error: 1_040
			.saturating_add(Weight::from_ref_time(18_097).saturating_mul(m.into()))
			// Standard Error: 1_014
			.saturating_add(Weight::from_ref_time(176_368).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Minimum execution time: 20_977 nanoseconds.
		Weight::from_ref_time(24_427_354)
			// Standard Error: 1_225
			.saturating_add(Weight::from_ref_time(154_372).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}