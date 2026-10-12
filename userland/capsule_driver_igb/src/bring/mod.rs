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

//! Bringing the part up once setup holds every grant, as a machine of
//! stages (stage.rs), and standing it down when a stage fails.

pub mod address;
mod announce;
pub mod engines;
pub mod filters;
mod finish;
pub mod link;
mod machine;
pub mod phy;
pub mod port;
pub mod quiet;
pub mod reset;
pub mod rings;
pub mod stage;
mod stand_down;
pub mod wait;

pub use finish::finish;
