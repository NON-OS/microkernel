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

//! The market service's wire format, from the client's side: how a request
//! is framed, which replies are believed, and how each reply body reads.
//!
//! Every reader bounds every length by the bytes that arrived, never by
//! the length a field claims, and returns None rather than panicking.

#![no_std]
#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation
)]

extern crate alloc;

pub mod app;
pub mod app_ids;
#[cfg(test)]
mod app_ids_tests;
pub mod body;
pub mod bond_code;
mod bytes;
pub mod commerce;
mod commerce_parse;
pub mod header;
pub mod install;
pub mod linux_policy;
pub mod listing;
mod lp;
pub mod ops;
pub mod readiness;
#[cfg(test)]
mod readiness_tests;
pub mod reason;
pub mod release;
pub mod release_escrow;
pub mod reply;
pub mod request;
pub mod status;

pub use app::{parse_app, App};
pub use app_ids::{parse_app_ids, AppIds};
pub use body::{listing_body, pair_body, ready_body};
pub use bond_code::BondCode;
pub use commerce::{Commerce, BINDING_VALID, COMMERCE_MATCHES};
pub use commerce_parse::parse_commerce;
pub use header::{HDR_LEN, MAGIC, STATUS_LEN, VERSION};
pub use install::Stage;
pub use linux_policy::{linux_policy_body, parse_linux_policy, LinuxPolicy, MARKET_ARG_LEN};
pub use listing::{parse_list, Entry};
pub use ops::*;
pub use readiness::{parse_readiness, Readiness, GATES, READINESS_LEN};
pub use reason::{installed_line, reason, removal_only, Reason};
pub use release::{parse_release, Release};
pub use release_escrow::{parse_release_escrow, ReleaseEscrow};
pub use reply::{reply_body, ReplyError};
pub use request::request;
pub use status::{status_name, SERVICE};
