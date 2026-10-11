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

//! The meter's wire. Every request and reply carries market_proto's 20-byte
//! header under this service's own magic; a reply's payload starts with a
//! status. All little-endian, every length explicit (plan/42).

#![no_std]

extern crate alloc;

mod account;
mod account_codec;
mod budget;
mod channel_ops;
mod cursor;
mod cursor_put;
mod drain;
mod drain_form;
mod drain_parse;
mod frame;
mod header;
mod open;
mod open_codec;
mod ops;
mod pay;
mod pay_encode;
mod status;
mod voucher;

pub use account::{Account, AccountRow, ACCOUNT_ROWS_MAX, ACCOUNT_ROW_LEN, KEPT, READY};
pub use budget::{SetBudget, MAX_LISTING_CAPS};
pub use channel_ops::{Close, TopUp};
pub use drain::Drain;
pub use drain_form::{DrainRequest, DrainSign, DRAIN_LIST, DRAIN_SIGN};
pub use frame::{reply, reply_body, request, ReplyError};
pub use header::{parse_header, Header, HDR_LEN, MAGIC, STATUS_LEN, VERSION};
pub use open::{OpenConfirm, OpenStart, OPEN_CONFIRM, OPEN_START};
pub use open_codec::parse_open;
pub use ops::*;
pub use pay::PayRequest;
pub use status::*;
pub use voucher::{VoucherRecord, DRAIN_ROWS_MAX, VOUCHER_RECORD_LEN};
