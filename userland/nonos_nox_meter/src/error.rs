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

/// Why the meter will not charge. Each is a refusal the caller can name.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MeterError {
    /// The charge would take the channel past what was deposited for it.
    OverDeposit,
    /// The channel, or the voucher it would sign, is past its expiry.
    Expired,
    /// The owner's budget, global or for this listing, does not cover it.
    OverBudget,
    /// Price times quantity does not fit, or the sequence would wrap.
    Overflow,
    /// No channel is open for this payee and listing.
    NoChannel,
    /// Settle reported more settled on the channel than this meter signed.
    Frozen,
    /// The meter's ETH is short of gas for what it must send.
    NoGas,
    /// The channel table is full.
    Full,
    /// The market does not stand behind the release's commerce block.
    Untrusted,
    /// A PAY word that is not what the release sells.
    Mismatch,
    /// The listing is not sold by voucher.
    NotMetered,
    /// Less than the listing's minimum charge, or nothing.
    BelowMinimum,
    /// A budget naming one listing twice.
    Duplicate,
    /// Persisted state that does not read back as this format.
    Corrupt,
    /// Nothing for a settle() to carry: no named channel has a voucher above
    /// its floor, or none newer than the last settle carried (v1.4.7).
    NothingOwed,
    /// Too soon: a close before expiry plus the contract's grace.
    TooEarly,
    /// The AmountPolicy's paused() is true: no new order (v1.4.20).
    Paused,
}
