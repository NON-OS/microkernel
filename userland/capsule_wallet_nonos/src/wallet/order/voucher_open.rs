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

/*
 * OPEN 12/0's request for a voucher listing (B8, v1.4.38), once the person
 * sets the deposit and the expiry. close() pays the unspent deposit to
 * reclaimTo, a fresh Shield address, so a lost meter key strands nothing.
 */

use nonos_payment_proto::OpenStart;

use super::voucher::Voucher;

impl Voucher {
    /// OPEN START with `amount` deposited until `expiry` (s), asked at `now`.
    pub fn open(
        &self,
        amount: u128,
        expiry: u64,
        reclaim_to: [u8; 20],
        now: u64,
    ) -> Result<OpenStart, &'static str> {
        if amount == 0 || amount < self.unit_price.max(self.min_charge) {
            return Err("The deposit is less than one use of it costs.");
        }
        if expiry <= now {
            return Err("The channel would expire before it opens.");
        }
        if reclaim_to == [0; 20] {
            return Err("There is no address for the unspent deposit to go back to.");
        }
        let (chain_id, payee, listing_id, unit) =
            (self.chain_id, self.payee, self.listing_id, self.unit);
        Ok(OpenStart { chain_id, payee, listing_id, unit, amount, expiry, reclaim_to })
    }
}
