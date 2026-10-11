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

//! capsule_nox_settle's wire (plan/26, plan/28, plan/42): market_proto's
//! 20-byte header under "NSTL", every body LE with every length explicit.
//! READ and INTENT name fixed tables, so no caller chooses what settle calls.

#![no_std]
extern crate alloc;

mod async_frame;
mod callers;
mod chain;
mod channel_state;
mod delivered_log;
mod fetch;
mod frame;
mod gas;
mod header;
mod intent;
mod intent_check;
mod kinds;
mod listing;
mod opened;
mod opened_log;
mod ops;
mod page;
mod quote;
mod read;
mod registry_current;
mod registry_log;
mod registry_topics;
mod settled;
mod url;
mod view;
mod views;

pub use async_frame::{admits_flags, async_answer, request_async, ASYNC_CALLERS, FLAG_ASYNC};
pub use async_frame::{WALLET_ASYNC, WALLET_ASYNC_OPS};
pub use callers::*;
pub use chain::{chain_hash, parse_chain_hash, ChainHash};
pub use channel_state::{ChannelState, CHANNEL_STATE_LEN};
pub use delivered_log::{delivered, DeliveredKeys, DELIVERED, DELIVERED_TOPIC, MAX_KEYS};
pub use fetch::*;
pub use frame::{reply, reply_body, request, ReplyError};
pub use gas::{Gas, GAS_LEN};
pub use header::{parse_header, Header, HDR_LEN, MAGIC, STATUS_LEN, VERSION};
pub use intent::{Intent, MAX_CALLDATA};
pub use intent_check::{approve_spender, claim_ok, kind_of};
pub use kinds::{Kind, KINDS, ORDER_TERMS_TUPLE, VOUCHER_TUPLE};
pub use listing::{listing_reply, parse_listing_reply};
pub use opened::{Opened, OPENED_LEN};
pub use opened_log::{CHANNEL_OPENED, CHANNEL_OPENED_TOPIC};
pub use ops::*;
pub use page::Page;
pub use quote::{padded, Quote, QUOTE_LEN};
pub use read::{parse_read_reply, read_reply, Read};
pub use registry_current::{current, LISTED_MAX};
pub use registry_log::{withdrawn, Listed, PUBLISHED, PUBLISHED_TOPIC, WITHDRAWN, WITHDRAWN_TOPIC};
pub use settled::{Settled, SETTLED_MAX};
pub use url::https_target;
pub use view::{view, view_calldata, Arg, View};
pub use views::VIEWS;
