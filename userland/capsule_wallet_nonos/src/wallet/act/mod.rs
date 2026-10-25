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

//! Network work a press starts: reading the nonce, fee and gas for a
//! payment's review, and broadcasting a signed transaction. A press only
//! begins it; the window steps it a slice a tick (`net::step`) and applies
//! the result once it finishes, its screen saying it is working meanwhile.
//!
//! One at a time: a press while an action runs does nothing, so nothing is
//! read twice over itself and nothing is sent twice. An action also holds
//! the network to itself. Both proxies keep one stream per program, so a
//! refresh under way is dropped when an action begins and none begins while
//! one runs; the refresh takes up again once it is done.

mod action;
mod broadcast;
mod broadcast_finish;
mod broadcast_record;
mod broadcast_show;
mod review;
mod review_finish;
mod review_refuse;
mod step;

pub use action::{running, working, Action};
pub use broadcast::{broadcast, Purpose};
pub use review::{review, review_stake};
pub use step::step;
