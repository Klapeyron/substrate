// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

#![cfg(test)]

//use crate::GetMigrations;
use codec::{Decode, Encode};
use frame_support::{
	migrations::*,
	traits::{ConstU16, ConstU64},
	weights::{Weight, WeightMeter},
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Migrations: crate,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

/// Succeeds after `inner` steps.
pub struct MockedMigrate(u32);
impl SteppedMigration for MockedMigrate {
	fn step(
		&self,
		cursor: &Option<SteppedMigrationCursor>,
		meter: &mut WeightMeter,
	) -> Result<Option<SteppedMigrationCursor>, SteppedMigrationError> {
		let mut count: u32 =
			cursor.as_ref().and_then(|c| Decode::decode(&mut &c[..]).ok()).unwrap_or(0);
		if count == self.0 {
			return Ok(None)
		}
		count += 1;
		Ok(Some(count.encode().try_into().unwrap()))
	}
}

frame_support::parameter_types! {
	pub const ServiceWeight: Weight = Weight::MAX;
	/// Stepped migrations need to be allocated as objects.
	///
	/// This is different from the normal compile-time tuple config, but allows them to carry
	/// configuration.
	pub SteppedMigrations: Vec<Box<dyn SteppedMigration>> = vec![
		Box::new(MockedMigrate(1)),
		Box::new(MockedMigrate(2)),
	];
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Migrations = SteppedMigrations;
	type Suspender = frame_system::Pallet<T>;
	type ServiceWeight = ServiceWeight;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
