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

pub const DOMAIN: &[u8] = b"NONOSREL2";

pub fn canonical_manifest(
    version: u64,
    commit: &[u8; 20],
    bootloader_hash: &[u8; 32],
    kernel_hash: &[u8; 32],
    zk_root: &[u8; 32],
    epoch: u64,
) -> Vec<u8> {
    let mut m = Vec::with_capacity(141);
    m.extend_from_slice(DOMAIN);
    m.extend_from_slice(&version.to_be_bytes());
    m.extend_from_slice(commit);
    m.extend_from_slice(bootloader_hash);
    m.extend_from_slice(kernel_hash);
    m.extend_from_slice(zk_root);
    m.extend_from_slice(&epoch.to_be_bytes());
    m
}
