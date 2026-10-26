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

mod abandoned;
mod clear_pid;
mod death;
mod entry;
mod give_up_rule;
mod given_up;
mod mark;
mod pop;
mod push;
mod remove;
mod reply_rule;
mod request_ids;
mod served_rule;
mod share;
mod state;
mod take_named;
mod take_reply;

pub(crate) use abandoned::forget as forget_dead;
pub(in crate::syscall::microkernel::ipc) use abandoned::take as abandoned;
pub(crate) use clear_pid::clear_pid;
pub(super) use given_up::{give_up, GiveUp};
pub(crate) use mark::mark_served;
pub(super) use pop::pop;
pub(super) use push::{push, Refused};
pub(super) use remove::remove_latest;
pub(super) use take_named::{delivered, take_named, Named};
pub(super) use take_reply::take_for_reply;
