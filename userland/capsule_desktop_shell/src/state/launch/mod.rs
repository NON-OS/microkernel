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

//! A Launchpad launch the installer is still working on. The load reads the
//! app's artifacts out of the store and runs the whole verify and attestation
//! before the installer answers, which takes seconds; the shell used to wait
//! for that answer inside its frame loop, up to 30 s with nothing drawn. The
//! call now gets a short budget, and a load still running when it ends is
//! followed from here, on the shell's own turns, until the app shows up or the
//! installer is known to have finished without it.
//!
//! The installer's late answer cannot be read: it lands in the shell's reply
//! inbox, where the next call the shell makes drops it as not its own. Two
//! things stand in for it. The app's service is in the registry from the
//! moment the kernel spawns it, so it is looked for on the shell's turns. And
//! the installer serves one request at a time, in order, so once a health
//! probe sent after the load is answered, the load is over: if no app came of
//! it by then, it failed.
//!
//! It is timed on uptime. It was timed on the wall clock, which reads an
//! error until the RTC is read and is stepped back by NTP: a launch started
//! while it read the error, or before a step back, was never due again, so
//! it neither found its app nor failed, and every later Launchpad launch of
//! an installed app was told to wait for it, for the rest of the session.

mod follow;
mod steps;
pub mod timing;

pub use follow::Launch;
pub use timing::Step;
