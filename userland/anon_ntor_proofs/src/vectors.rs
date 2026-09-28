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

//! The live documents the directory proofs run against.

/*
 * Served by authority 49.13.145.234:9230 on 2026-09-18. The consensus is
 * trimmed to six relays and the whole footer so it can live in the tree;
 * nothing about its syntax is altered.
 */
/// A microdescriptor flavour consensus as the network served it.
pub const CONSENSUS: &[u8] = include_bytes!("../vectors/consensus-microdesc.txt");

/// Three microdescriptors, fetched for three Guard flagged relays in it.
pub const MICRODESCS: &[u8] = include_bytes!("../vectors/microdescs.txt");
