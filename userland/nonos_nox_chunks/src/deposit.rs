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

//! An order's deposit address: CREATE2 by the escrow, salt =
//! orderId, of an EIP-1167 clone of the DepositForwarder. The OS computes it
//! itself; the escrow's depositAddress() view is only a cross-check.

use nonos_marketplace_abi::Keccak;

const CLONE_HEAD: [u8; 20] = [
    0x3d, 0x60, 0x2d, 0x80, 0x60, 0x0a, 0x3d, 0x39, 0x81, 0xf3, 0x36, 0x3d, 0x3d, 0x37, 0x3d, 0x3d,
    0x3d, 0x36, 0x3d, 0x73,
];
const CLONE_TAIL: [u8; 15] =
    [0x5a, 0xf4, 0x3d, 0x82, 0x80, 0x3e, 0x90, 0x3d, 0x91, 0x60, 0x2b, 0x57, 0xfd, 0x5b, 0xf3];

/// keccak256 of the EIP-1167 creation code for a clone of `implementation`.
pub fn clone_initcode_hash(k: &impl Keccak, implementation: &[u8; 20]) -> [u8; 32] {
    let mut code = [0u8; 55];
    code[..20].copy_from_slice(&CLONE_HEAD);
    code[20..40].copy_from_slice(implementation);
    code[40..].copy_from_slice(&CLONE_TAIL);
    k.keccak256(&code)
}

/// EIP-1014: keccak256(0xff | deployer | salt | initcode_hash)[12..].
pub fn create2(
    k: &impl Keccak,
    deployer: &[u8; 20],
    salt: &[u8; 32],
    initcode_hash: &[u8; 32],
) -> [u8; 20] {
    let mut pre = [0u8; 85];
    pre[0] = 0xff;
    pre[1..21].copy_from_slice(deployer);
    pre[21..53].copy_from_slice(salt);
    pre[53..].copy_from_slice(initcode_hash);
    let h = k.keccak256(&pre);
    let mut out = [0u8; 20];
    out.copy_from_slice(&h[12..]);
    out
}

/// Where the buyer sends an order's NOX.
pub fn deposit_address(
    k: &impl Keccak,
    escrow: &[u8; 20],
    order_id: &[u8; 32],
    forwarder_initcode_hash: &[u8; 32],
) -> [u8; 20] {
    create2(k, escrow, order_id, forwarder_initcode_hash)
}
