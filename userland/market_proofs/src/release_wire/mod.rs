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

//! The release encoder the market capsule replies with, so the clients'
//! reader is held to the bytes the server writes.

#[path = "../../../capsule_market/src/server/handlers/get_release/write_lp_string.rs"]
mod write_lp_string;

#[path = "../../../capsule_market/src/server/handlers/get_release/encode_release.rs"]
mod encode_release;

pub fn encode(rel: &nonos_marketplace_abi::CapsuleRelease) -> alloc::vec::Vec<u8> {
    encode_release::encode_release(rel)
}
