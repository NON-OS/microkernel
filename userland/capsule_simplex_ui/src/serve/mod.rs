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

//! The window's own channel to simplex.core. The window starts the core
//! (its listing, MkAppLaunch, as model/start.rs rules) and asks for
//! SERVE_ONE's endpoint each tick until it answers; the core stops when the
//! window closes. Until then, and if it never answers, the window is offline
//! and says so: it never shows a recorded session as if it were yours.

mod call;
mod launch;
mod lazy;
mod pipe;

pub use lazy::open;
