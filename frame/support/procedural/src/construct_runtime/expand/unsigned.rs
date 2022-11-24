// This file is part of Substrate.

// Copyright (C) 2021-2022 Parity Technologies (UK) Ltd.
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
// limitations under the License

use crate::construct_runtime::Pallet;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn expand_outer_validate_unsigned(
	runtime: &Ident,
	pallet_decls: &[Pallet],
	scrate: &TokenStream,
) -> TokenStream {
	let mut pallet_names = Vec::new();
	let mut query_validate_unsigned_part_macros = Vec::new();

	for pallet_decl in pallet_decls {
		if pallet_decl.exists_part("ValidateUnsigned") {
			let name = &pallet_decl.name;
			let path = &pallet_decl.path;

			pallet_names.push(name);
			query_validate_unsigned_part_macros.push(quote! {
				#path::__substrate_validate_unsigned_check::is_validate_unsigned_part_defined!(#name);
			});
		}
	}

    let pallettos: Vec<_> = pallet_names.iter().map(|m| m.to_string()).collect();

	quote! {
		#( #query_validate_unsigned_part_macros )*

		impl #scrate::unsigned::ValidateUnsigned for #runtime {
			type Call = Call;

			fn pre_dispatch(call: &Self::Call) -> Result<(), #scrate::unsigned::TransactionValidityError> {
				#[allow(unreachable_patterns)]
				match call {
					#( Call::#pallet_names(inner_call) => #pallet_names::pre_dispatch(inner_call), )*
					// pre-dispatch should not stop inherent extrinsics, validation should prevent
					// including arbitrary (non-inherent) extrinsics to blocks.
					_ => Ok(()),
				}
			}

			fn validate_unsigned(
				#[allow(unused_variables)]
				source: #scrate::unsigned::TransactionSource,
				call: &Self::Call,
			) -> #scrate::unsigned::TransactionValidity {
				#[allow(unreachable_patterns)]
				match call {
					#( Call::#pallet_names(inner_call) => #pallet_names::validate_unsigned(source, inner_call), )*
					_ => {
                        frame_support::log::error!("TJDEBUG REDIIE?? GO!!");
                        for meh in [#(#pallettos),*].iter() {
                            frame_support::log::error!("TJDEBUG palleto nameo {:?}", meh);
                        }
                        #scrate::unsigned::UnknownTransaction::NoUnsignedValidator.into()
                    }
				}
			}
		}
	}
}
