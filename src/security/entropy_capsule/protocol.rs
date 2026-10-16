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

// Kernel-side mirror of `userland/capsule_entropy/src/protocol/*`.
// Bit-for-bit identical layout: a reply that drifted fails to decode, and
// ask::poll drops it.

use alloc::vec::Vec;

use crate::services::lifecycle::transport;

pub(super) const MAGIC: u32 = 0x4E4F_454E; // "NOEN"
pub(super) const VERSION: u16 = 1;

pub(super) const OP_GET_RANDOM: u16 = 1;

pub(super) const MAX_RANDOM_BYTES: u32 = 4096;
pub(super) const MAX_PAYLOAD_BYTES: u32 = 4096;

pub(super) use crate::services::lifecycle::transport::DecodedResponse;

pub(super) fn encode_request(op: u16, flags: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    transport::encode_request(MAGIC, VERSION, op, flags, request_id, body)
}

pub(super) fn decode_response(buf: &[u8]) -> Option<DecodedResponse<'_>> {
    transport::decode_v1_response(buf, MAGIC, VERSION, MAX_PAYLOAD_BYTES)
}
