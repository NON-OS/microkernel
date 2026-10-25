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
mod home_key_tests;
mod hosts;
mod ids;
mod linux_policy_answer_tests;
mod linux_policy_gate_tests;
mod linux_policy_kernel;
mod linux_policy_pin;
mod linux_policy_wire;
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
#[path = "../../../../src/security/market_capsule/client/linux_policy_wire.rs"]
mod kernel_linux_policy;
#[path = "../../../capsule_market/src/server/handlers/get_linux_policy/answer.rs"]
mod linux_policy_answer;
#[path = "../../../capsule_market/src/server/handlers/get_linux_policy/encode.rs"]
mod linux_policy_encode;
/* answer.rs names its sibling as the capsule does. */
use linux_policy_encode as encode;
