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

use nonos_marketplace_abi::{commerce_bytes, DecodeError, SettleMode};

use super::blobs::{blob, decode, find};
use super::fixtures::sold;
use super::refusals::SETTLE_AT;

#[test]
fn an_escrow_listing_must_pin_every_release() {
    // Written as a voucher listing, which needs no chunk tree, then turned
    // into escrow in the bytes: the decoder must notice the missing pin.
    let mut voucher = sold();
    let entry = &mut voucher.entries[0];
    entry.commerce.as_mut().unwrap().settle_mode = SettleMode::Voucher;
    entry.releases[1].ext.as_mut().unwrap().chunk_root = [0; 32];
    let listed = commerce_bytes(entry.commerce.as_ref().unwrap());
    let mut b = blob(voucher);
    let at = find(&b, &listed) + SETTLE_AT;
    b[at] = SettleMode::Escrow as u8;
    assert_eq!(decode(&b).err(), Some(DecodeError::Inconsistent));
}
