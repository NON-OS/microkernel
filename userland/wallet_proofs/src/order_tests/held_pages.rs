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


//! ACCOUNT pages the channel table (v1.4.48): a listing's open channel is
//! looked for a page a tick until the table's end.

use super::fixture::NOW;
use super::held::{page, row};
use super::voucher::voucher;
use crate::order::held::{after_page, Held};

#[test]
fn the_table_is_read_a_page_at_a_time_until_its_end() {
    let v = voucher();
    let rows = vec![row([0xee; 32], v.chain_id, NOW + 1, false); 32];
    let first = page(40, rows.clone());
    assert_eq!(after_page(Some(&first), 0, &v, NOW), Held::Asking(32));
    let last = page(40, rows[..8].to_vec());
    assert_eq!(after_page(Some(&last), 32, &v, NOW), Held::Free);
    let shrunk = page(40, vec![]);
    assert_eq!(after_page(Some(&shrunk), 32, &v, NOW), Held::Free, "rows gone since");
}
