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

//! The capsule's links against a controller that lies: it resets itself
//! with or without a word, gives an open link's handle to another, keeps
//! ACL buffers, stops answering, or floods events. Each ends the link it
//! touches within a bound, every key still down goes up, and nothing hangs.

mod buffer_tests;
mod forget_tests;
mod handle_tests;
mod peer;
mod reset_tests;
mod restart_tests;
mod sealed_tests;
mod silence_tests;

use peer::{opened, A_DOWN, KEYS};
