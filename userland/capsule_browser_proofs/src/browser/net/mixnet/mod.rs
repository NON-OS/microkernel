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

//! The reader's choice of network, what its refusals mean, and the pure
//! half of a conversation with a proxy: its frames, its state, how often it
//! is asked and how a failed call is read.

#[path = "../../../../../capsule_browser/src/browser/net/mixnet/choice.rs"]
mod choice;
#[path = "../../../../../capsule_browser/src/browser/net/mixnet/conv.rs"]
pub mod conv;
#[path = "../../../../../capsule_browser/src/browser/net/mixnet/fault.rs"]
pub mod fault;
#[path = "../../../../../capsule_browser/src/browser/net/mixnet/frames.rs"]
pub mod frames;
#[path = "../../../../../capsule_browser/src/browser/net/mixnet/pace.rs"]
pub mod pace;
#[path = "../../../../../capsule_browser/src/browser/net/mixnet/refusal.rs"]
mod refusal;
#[path = "../../../../../capsule_browser/src/browser/net/mixnet/status.rs"]
pub mod status;
#[path = "../../../../../capsule_browser/src/browser/net/mixnet/streams.rs"]
pub mod streams;
#[path = "../../../../../capsule_browser/src/browser/net/mixnet/way.rs"]
pub mod way;

pub use way::{Routes, Way};

pub use choice::{choose, chosen, Network};
pub use conv::Broke;
pub use status::{still, Heard};
