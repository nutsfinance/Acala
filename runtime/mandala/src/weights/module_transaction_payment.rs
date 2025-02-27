// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

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

//! Autogenerated weights for module_transaction_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ea4c8813bb44`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/mandala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_transaction_payment.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_transaction_payment::WeightInfo for WeightInfo<T> {
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: TransactionPayment AlternativeFeeSwapPath (r:0 w:1)
	fn set_alternative_fee_swap_path() -> Weight {
		// Minimum execution time: 28_837 nanoseconds.
		Weight::from_ref_time(30_308_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: TransactionPayment PoolSize (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Storage: Dex LiquidityPool (r:2 w:0)
	// Storage: StableAsset Pools (r:1 w:0)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: TransactionPayment TokenExchangeRate (r:0 w:1)
	// Storage: TransactionPayment SwapBalanceThreshold (r:0 w:1)
	fn enable_charge_fee_pool() -> Weight {
		// Minimum execution time: 95_410 nanoseconds.
		Weight::from_ref_time(98_176_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: TransactionPayment TokenExchangeRate (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: TransactionPayment SwapBalanceThreshold (r:0 w:1)
	// Storage: TransactionPayment GlobalFeeSwapPath (r:0 w:1)
	// Storage: TransactionPayment PoolSize (r:0 w:1)
	fn disable_charge_fee_pool() -> Weight {
		// Minimum execution time: 72_774 nanoseconds.
		Weight::from_ref_time(74_127_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: TransactionPause PausedTransactions (r:1 w:0)
	fn with_fee_path() -> Weight {
		// Minimum execution time: 10_342 nanoseconds.
		Weight::from_ref_time(10_708_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: TransactionPause PausedTransactions (r:1 w:0)
	fn with_fee_currency() -> Weight {
		// Minimum execution time: 11_428 nanoseconds.
		Weight::from_ref_time(11_999_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: TransactionPause PausedTransactions (r:1 w:0)
	fn with_fee_aggregated_path() -> Weight {
		// Minimum execution time: 11_480 nanoseconds.
		Weight::from_ref_time(12_366_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: TransactionPause PausedTransactions (r:1 w:0)
	fn with_fee_paid_by() -> Weight {
		// Minimum execution time: 7_469 nanoseconds.
		Weight::from_ref_time(7_782_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:1)
	fn on_finalize() -> Weight {
		// Minimum execution time: 9_165 nanoseconds.
		Weight::from_ref_time(9_514_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
