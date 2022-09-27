// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

use crate::*;
use frame_support::pallet_prelude::*;

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	pub fn do_set_attribute(
		maybe_check_owner: Option<T::AccountId>,
		collection: T::CollectionId,
		maybe_item: Option<T::ItemId>,
		key: BoundedVec<u8, T::KeyLimit>,
		value: BoundedVec<u8, T::ValueLimit>,
	) -> DispatchResult {
		let mut collection_details =
			Collection::<T, I>::get(&collection).ok_or(Error::<T, I>::UnknownCollection)?;

		if let Some(check_owner) = &maybe_check_owner {
			ensure!(check_owner == &collection_details.owner, Error::<T, I>::NoPermission);
		}

		let collection_settings = Self::get_collection_settings(&collection)?;
		let maybe_is_frozen = match maybe_item {
			None => Ok(collection_settings.contains(CollectionSetting::LockedAttributes)),
			Some(item) => Self::get_item_settings(&collection, &item)
				.map(|v| v.contains(ItemSetting::LockedAttributes)),
		}?;
		ensure!(!maybe_is_frozen, Error::<T, I>::Frozen);

		let attribute = Attribute::<T, I>::get((collection, maybe_item, &key));
		if attribute.is_none() {
			collection_details.attributes.saturating_inc();
		}
		let old_deposit = attribute.map_or(Zero::zero(), |m| m.1);
		collection_details.total_deposit.saturating_reduce(old_deposit);
		let mut deposit = Zero::zero();
		if !collection_settings.contains(CollectionSetting::FreeHolding) &&
			maybe_check_owner.is_some()
		{
			deposit = T::DepositPerByte::get()
				.saturating_mul(((key.len() + value.len()) as u32).into())
				.saturating_add(T::AttributeDepositBase::get());
		}
		collection_details.total_deposit.saturating_accrue(deposit);
		if deposit > old_deposit {
			T::Currency::reserve(&collection_details.owner, deposit - old_deposit)?;
		} else if deposit < old_deposit {
			T::Currency::unreserve(&collection_details.owner, old_deposit - deposit);
		}

		Attribute::<T, I>::insert((&collection, maybe_item, &key), (&value, deposit));
		Collection::<T, I>::insert(collection, &collection_details);
		Self::deposit_event(Event::AttributeSet { collection, maybe_item, key, value });
		Ok(())
	}

	pub fn do_clear_attribute(
		maybe_check_owner: Option<T::AccountId>,
		collection: T::CollectionId,
		maybe_item: Option<T::ItemId>,
		key: BoundedVec<u8, T::KeyLimit>,
	) -> DispatchResult {
		let mut collection_details =
			Collection::<T, I>::get(&collection).ok_or(Error::<T, I>::UnknownCollection)?;
		if let Some(check_owner) = &maybe_check_owner {
			ensure!(check_owner == &collection_details.owner, Error::<T, I>::NoPermission);
		}

		let collection_settings = Self::get_collection_settings(&collection)?;
		let maybe_is_frozen = match maybe_item {
			None => Ok(collection_settings.contains(CollectionSetting::LockedAttributes)),
			Some(item) => Self::get_item_settings(&collection, &item)
				.map(|v| v.contains(ItemSetting::LockedAttributes)),
		}?;
		ensure!(!maybe_is_frozen, Error::<T, I>::Frozen);

		if let Some((_, deposit)) = Attribute::<T, I>::take((collection, maybe_item, &key)) {
			collection_details.attributes.saturating_dec();
			collection_details.total_deposit.saturating_reduce(deposit);
			T::Currency::unreserve(&collection_details.owner, deposit);
			Collection::<T, I>::insert(collection, &collection_details);
			Self::deposit_event(Event::AttributeCleared { collection, maybe_item, key });
		}
		Ok(())
	}
}
