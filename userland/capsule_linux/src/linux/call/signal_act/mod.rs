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

//! `struct sigaction` as the guest passes it, the signal rt_sigaction may
//! name, and the mask a waiting call waits under. Pure, over the guest's
//! memory where it reads, so the host proofs hold all three.

mod action;
mod mask;

pub use action::{decode, encode, signal_of, SIGACTION_LEN};
pub use mask::{wait_mask, wait_set, SIGSET_LEN};
