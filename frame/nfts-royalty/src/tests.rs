// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
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

//! Tests for the NFT Royalties pallet.

use crate::{mock::*, NftWithRoyalty};
use frame_support::{assert_ok};
use pallet_nfts::{CollectionAccount, Account, ItemSettings, CollectionConfig, MintSettings, CollectionSettings, CollectionSetting};
pub use sp_runtime::{Perbill, Permill};

type AccountIdOf<Test> = <Test as frame_system::Config>::AccountId;
fn account(id: u8) -> AccountIdOf<Test> {
	[id; 32].into()
}

#[test]
fn nft_minting_with_royalties_should_work() {
	new_test_ext().execute_with(|| {
		// Create a collection, calling directly the NFT pallet
		assert_ok!(Nfts::force_create(
			RuntimeOrigin::root(),
			account(1),
			CollectionConfig {
                settings: CollectionSettings::from_disabled(CollectionSetting::DepositRequired.into()),
                max_supply: None,
                mint_settings: MintSettings::default(),
            }
		));
        let mut collections: Vec<_> = CollectionAccount::<Test>::iter().map(|x| (x.0, x.1)).collect();
        collections.sort();
		assert_eq!(collections, vec![(account(1), 0)]);
		assert_ok!(NftsRoyalty::mint_item_with_royalty(
			RuntimeOrigin::signed(account(1)),
			0, 42, account(1), 
			ItemSettings::all_enabled(),
			Permill::from_percent(5),
			account(1)
		));
		// Get the items directly from the NFT pallet, to see if has been created there
        let mut items: Vec<_> = Account::<Test>::iter().map(|x| x.0).collect();
	    items.sort();
		assert_eq!(items, vec![(account(1), 0, 42)]);
		// Read royalties pallet storage.
        let nft_with_royalty = NftWithRoyalty::<Test>::get((0,42)).unwrap();
        assert_eq!(nft_with_royalty.royalty_percentage, Permill::from_percent(5));
        assert_eq!(nft_with_royalty.royalty_recipient, account(1));
	});
}
