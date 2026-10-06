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

extern crate alloc;

pub mod app;
pub mod body;
pub mod header;
pub mod install;
pub mod listing;
mod lp;
pub mod ops;
pub mod readiness;
pub mod reason;
pub mod release;
pub mod reply;
pub mod request;
pub mod status;

pub use app::{parse_app, App};
pub use body::{listing_body, pair_body};
pub use header::{HDR_LEN, MAGIC, STATUS_LEN, VERSION};
pub use install::Stage;
pub use listing::{parse_list, Entry};
pub use ops::{
    OP_GET_APP, OP_GET_RELEASE, OP_HEALTHCHECK, OP_INSTALL_READY, OP_LIST_APPS, OP_LOAD_INDEX,
};
pub use readiness::{parse_readiness, Readiness, GATES};
pub use reason::{installed_line, reason, removal_only, Reason};
pub use release::{parse_release, Release};
pub use reply::{reply_body, ReplyError};
pub use request::request;
pub use status::{status_name, SERVICE};
