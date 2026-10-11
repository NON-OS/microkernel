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

//! ORDER_OFFER's job (plan/29) as the flow holds it: what was asked, the
//! buyer key drawn for it, the FETCH 1 body built with its proof of work,
//! and the sealed answer as its pages arrive.

use alloc::vec::Vec;

use super::want::Want;

#[derive(Clone, PartialEq, Eq)]
pub struct OfferWork {
    pub want: Want,
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    pub buyer_sk: [u8; 32],
    pub buyer_pk: [u8; 32],
    /// FETCH 1 ESCROW's body, form byte first.
    pub request: Vec<u8>,
    pub made_at: u64,
    /// The sealed answer so far, page by page.
    pub fetched: Vec<u8>,
}
