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

//! No notice stays on screen. "Terminal opened", "Files closed" and the rest
//! leave `TOAST_LIFETIME_MS` after they were first shown: with nothing else
//! happening, after the app that caused them ended, when they are said again
//! and again, and when the clock reads behind the time they were shown. The
//! serve loop is modelled as `server/runner/run.rs` and `drain.rs` run it: it
//! waits on its inbox until `wake_at`, expires the queue on every pass, and
//! syncs the panel whenever the queue moved.

mod clock;
mod expiry;
mod held;
mod rig;
mod wake;
