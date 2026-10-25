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

//! Every status the meter answers. Linux errno values where one fits, and
//! the meter's own below -1000, where none does.

pub const OK: i32 = 0;
pub const EPERM: i32 = -1;
/// No channel for this payee and listing, or no token on the chain.
pub const ENOENT: i32 = -2;
/// A state file or a reply that does not read back.
pub const EIO: i32 = -5;
/// Not ready yet: the store, the keyring or the bound root is not there.
pub const EAGAIN: i32 = -11;
/// Another job is running: the meter runs one at a time.
pub const EBUSY: i32 = -16;
pub const EINVAL: i32 = -22;
/// The channel table is full.
pub const ENFILE: i32 = -23;
/// Past the channel's deposit.
pub const ENOSPC: i32 = -28;
/// Below the listing's minimum charge.
pub const EDOM: i32 = -33;
/// Past the channel's or the budget's expiry.
pub const ETIME: i32 = -62;
/// A request word that is not what the release's signed block says.
pub const EPROTO: i32 = -71;
pub const EOVERFLOW: i32 = -75;
/// The listing is not sold by voucher.
pub const EOPNOTSUPP: i32 = -95;
/// Over the owner's budget.
pub const EDQUOT: i32 = -122;
/// The market does not stand behind the release.
pub const EKEYREJECTED: i32 = -129;
/// This boot keeps nothing, so the meter holds no key (plan/20).
pub const ENOPERSIST: i32 = -1001;
/// Settle reported more settled on the channel than this meter signed.
pub const EFROZEN: i32 = -1002;
/// The meter's ETH is short of gas for what OPEN must send.
pub const EGAS: i32 = -1003;
