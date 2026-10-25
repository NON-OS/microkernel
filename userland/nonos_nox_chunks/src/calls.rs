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

//! The escrow's calls a buyer's side sends (plan/22, plan/27), as calldata:
//! fund(T), fundFrom(T,uint256), release(T), refund(T,address,bytes32) and
//! dispute(T,uint64,bytes,bytes32,bytes32[],bytes32[]). T is OrderTerms,
//! a static tuple, so it sits inline in the head. Selectors are pinned by
//! a test against settle_proto's kind table.

use alloc::vec::Vec;

use crate::terms::OrderTerms;

pub const FUND: u32 = 0xb18d590d;
pub const FUND_FROM: u32 = 0xaba72c43;
pub const DISPUTE: u32 = 0xd84d6ef9;
pub const REFUND: u32 = 0xa3814a14;
pub const RELEASE: u32 = 0x2fc866f5;

pub(crate) fn num(v: u128) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&v.to_be_bytes());
    w
}

pub(crate) fn head(sel: u32, t: &OrderTerms, rest: &[[u8; 32]]) -> Vec<u8> {
    let mut out = sel.to_be_bytes().to_vec();
    t.words().iter().chain(rest).for_each(|w| out.extend_from_slice(w));
    out
}

pub fn fund(t: &OrderTerms) -> Vec<u8> {
    head(FUND, t, &[])
}

pub fn fund_from(t: &OrderTerms, amount: u128) -> Vec<u8> {
    head(FUND_FROM, t, &[num(amount)])
}

pub fn release(t: &OrderTerms) -> Vec<u8> {
    head(RELEASE, t, &[])
}

pub fn refund(t: &OrderTerms, refund_to: &[u8; 20], refund_salt: &[u8; 32]) -> Vec<u8> {
    let mut to = [0u8; 32];
    to[12..].copy_from_slice(refund_to);
    head(REFUND, t, &[to, *refund_salt])
}
