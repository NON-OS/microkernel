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
use super::state::{Channel, NO_SIG};
use crate::voucher::Voucher;

impl Channel {
    /// The newest voucher on this channel, paid by `payer` (the meter). Its
    /// expiry is the channel's by construction: the contract refuses a
    /// voucher whose expiry differs, so the caller never chooses one.
    pub fn voucher(&self, payer: &[u8; 20]) -> Voucher {
        Voucher {
            channel_id: self.id,
            payer: *payer,
            payee: self.payee,
            listing_id: self.listing_id,
            unit: self.unit,
            cumulative: self.cumulative,
            seq: self.seq,
            expiry: self.expiry,
            usage_root: self.usage_root,
        }
    }

    /// Whether `v` is this channel's newest voucher, field for field.
    pub fn is_newest(&self, v: &Voucher) -> bool {
        *v == self.voucher(&v.payer)
    }

    /// The newest voucher with its signature, once the keyring returned one.
    pub fn signed(&self, payer: &[u8; 20]) -> Option<(Voucher, [u8; 65])> {
        (self.sig != NO_SIG).then(|| (self.voucher(payer), self.sig))
    }
}
