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

//! Wire magic, header width and the table sizes net.anon holds itself to.

use crate::cell::RELAY_BODY_BYTES;
use crate::circuit::window::{CIRCUIT_INCREMENT, STREAM_INCREMENT};

pub const MAGIC: u32 = 0x414E_4F31;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

/// Largest body one request or reply carries. A reassembled response is not
/// bounded by a cell, so anything longer is split across reads, not refused.
pub const IPC_PAYLOAD_MAX: usize = 32 * 1024;

/// Three: one working circuit, one being built, one being torn down. Each costs
/// three ntor handshakes and a TLS session, and a client holding dozens is both
pub const CIRCUIT_MAX: usize = 3;

pub const STREAM_MAX: usize = 32;

/*
 * How far behind the caller may fall before the far end is stalled. A SENDME is
 * the only thing that lets a relay send more, so withholding one is the whole of
 * the backpressure this transport has, and without a bound the buffer is whatever
 * the exit decides to send. One increment of undelivered payload is the most a
 * single grant can bring in, so a caller reading at any steady rate never meets
 * the limit while one that has stopped reading cannot be used to fill the heap.
 */

/// Bytes buffered on one stream before its SENDME is withheld.
pub const STREAM_HIGH_WATER: usize = STREAM_INCREMENT as usize * RELAY_BODY_BYTES;

/// Bytes buffered across one circuit's streams before its SENDME is withheld.
pub const CIRCUIT_HIGH_WATER: usize = CIRCUIT_INCREMENT as usize * RELAY_BODY_BYTES;

/// Seconds a circuit accepts new streams for, from tor's MaxCircuitDirtiness
/// default (dir-spec, or.h `MAX_CIRCUIT_DIRTINESS` 600). A circuit that took
pub const CIRCUIT_DIRTY_SECONDS: u64 = 600;

/// Failed streams before a circuit is retired rather than retried. Two, because
/// one failure is a site and two on a circuit that has never delivered is the
pub const CIRCUIT_FAILURES_MAX: u8 = 2;

/// Not configurable. Two hops lets the guard see both ends, and the consensus
/// bandwidth weights are computed for three.
pub const HOPS: usize = 3;
