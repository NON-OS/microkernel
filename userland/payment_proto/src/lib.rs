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

//! The meter's wire: market_proto's 20-byte header under this service's own
//! magic, a status first in each reply, little-endian, lengths explicit.

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
mod job;
mod open;
mod open_codec;
mod ops;
mod order_delivered;
mod order_made;
mod order_more;
mod order_new;
mod order_new_parse;
mod offer;
mod offer_made;
mod pay;
mod plain_page;
mod plaintext;
mod purchase;
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
pub use job::{job_id, parse_job_id, JOB_ID_LEN};
pub use open::{OpenConfirm, OpenStart, OPEN_CONFIRM, OPEN_START};
pub use open_codec::parse_open;
pub use ops::*;
pub use order_delivered::{DeliveredAsk, DeliveredMade, DELIVERED_ASK_LEN};
pub use order_made::OrderMade;
pub use order_more::{parse_refund_reply, refund_reply, OrderStatus, ORDER_STATUS_LEN};
pub use order_new::{OrderNew, ORDER_MADE_LEN, ORDER_NEW_LEN};
pub use offer::{OfferAsk, MAX_URL};
pub use offer_made::{OfferMade, OFFER_MADE_LEN};
pub use pay::PayRequest;
pub use plain_page::{PlainPage, PLAINTEXT_PAGE};
pub use plaintext::{PlainAsk, PLAIN_ASK_LEN};
pub use purchase::*;
pub use status::*;
pub use voucher::{VoucherRecord, DRAIN_ROWS_MAX, VOUCHER_RECORD_LEN};
