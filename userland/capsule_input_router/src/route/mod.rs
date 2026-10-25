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

pub mod bond;
pub mod chord;
pub mod deliver;
pub mod dispatch;
mod fanout;
mod focus_now;
pub mod grab_rule;
mod held;
mod key_deliver;
pub mod key_target;
mod keyboard;
mod kinds;
mod notice;
pub mod pad;
mod pointer;
pub mod preempt;
mod press_kinds;
mod refuse;
pub mod shell_keys;
pub mod trusted_hold;

pub use dispatch::route_event;
