// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

mod attest;
mod attest_fixture;
mod baseline;
mod binding;
mod blobs;
mod bond;
mod bond_gate;
mod commerce;
mod digest;
mod document;
mod encoder;
mod entry_document;
pub(crate) mod fixtures;
mod hosts;
mod ids;
mod pinning;
mod policy;
mod policy_fixture;
mod price;
mod qwen_vector;
mod refusals;
mod roundtrip;
mod signing;
mod wire;
mod wire_cut;

#[path = "../../../capsule_market/src/chain_hosts/mod.rs"]
pub(crate) mod chain_hosts;
#[path = "../../../capsule_market/src/server/handlers/get_commerce/encode.rs"]
mod commerce_encode;
