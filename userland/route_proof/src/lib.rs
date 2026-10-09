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

//! What an anonymity transport says about the route it is carrying traffic
//! over, who may say it, and what an observer may conclude.
//!
//! The report is deliberately poor in identifying detail. It names no guard,
//! no gateway, no relay address and no key: the attest service hands it to any
//! local caller, and a guard's identity is exactly what a guard-discovery
//! attack is after. It carries counts and verdicts the transport itself
//! checked: how many directory authorities signed, whether every hop of an open
//! circuit authenticated, how long the directory stays valid.

#![no_std]

pub mod authorize;
pub mod board;
pub mod facts;
pub mod frame;
pub mod report;
pub mod verdict;

pub use authorize::may_report;
pub use facts::{
    anyone_report, nym_report, AnyoneBootstrap, AnyoneFacts, NymDirectory, NymFacts,
    NYM_REASON_NO_CLOCK, NYM_REASON_UNTRUSTED,
};
pub use frame::{
    ask_frame, post_frame, reply_body, ANSWER_FRAME_LEN, ATTEST_SERVICE, OP_PROOF_ROUTE,
    OP_ROUTE_REPORT, POST_LEN,
};
pub use board::{read_answer, Board, Latest, PostError, ANSWER_LEN};
pub use report::{decode, encode, Network, RouteReport, Stage, REPORT_LEN, REPORT_VERSION};
pub use verdict::{route_verdict, RouteVerdict, Stale, ROUTE_ANYONE, ROUTE_DIRECT, ROUTE_NYM};
