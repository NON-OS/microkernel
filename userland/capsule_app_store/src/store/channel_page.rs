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

//! ACCOUNT's pages as the store reads them, and an install once asked.

use alloc::vec::Vec;

use nonos_payment_proto::{Account, AccountRow};

use super::super::asked::Asked;
use super::super::listing::Listing;
use super::super::progress::Progress;

/// One ACCOUNT page from channel `from`: its rows and the next page's first
/// channel, or None for an error or a reply that does not read.
pub fn page(reply: Result<Vec<u8>, i32>, from: u8) -> Option<(Vec<AccountRow>, Option<u8>)> {
    let a = Account::parse(&reply.ok()?)?;
    let next = usize::from(from) + a.rows.len();
    let more = !a.rows.is_empty() && next < usize::from(a.total);
    Some((a.rows, u8::try_from(next).ok().filter(|_| more)))
}

/// An install asked for and queued shows as queued.
pub fn queued(l: &mut Listing, asked: Asked) {
    if asked == Asked::Queued {
        (l.progress, l.removing) = (Progress::Queued, false);
    }
}
