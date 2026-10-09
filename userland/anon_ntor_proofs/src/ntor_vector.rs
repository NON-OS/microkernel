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

//! The ntor vector, as the reference implementation produces it.

/*
 * Produced by the field order in src/test/ntor_ref.py, client_part2, from fixed
 * scalars. The two shared points and the three public keys are the curve
 * outputs for those scalars; everything after them is concatenation, which is
 * what the ntor proofs check.
 */
pub const NODE_ID: &str = "000102030405060708090a0b0c0d0e0f10111213";
pub const B: &str = "bb50ff9e82a574cfbf820e97f60fb9c143ec7415cf514f8cfd98eff59e059614";
pub const X: &str = "14ee0e07f34e3a17030c6eecf66d3705e80d1e153f7a0ca5b345e83ce9d62643";
pub const Y: &str = "8344397780de3e2add150c06052386de1d6b22722c18c2c55fdbf6834946515e";
pub const XY: &str = "4ede3e37e9aa09fbf1915284812a4529e944a4aa6c7fd57ab06962208d09250a";
pub const XB: &str = "32dfa776c16b37746565aff7031b504c4208d9a0e4b23076a0887249aaf7b50e";
pub const VERIFY: &str = "17be643598e00e400ac1cd6c60b2b6156de28db195ff1e3f980fadf7cd169c1c";

/// PROTOID, hex encoded, so the expected strings below are spelled out rather
/// than rebuilt from the same constant the code reads.
pub const PROTOID_HEX: &str = "6e746f722d637572766532353531392d7368613235362d31";

/// "Server", hex encoded, for the same reason.
pub const SERVER_HEX: &str = "536572766572";
