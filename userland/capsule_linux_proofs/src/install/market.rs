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

//! The market install's pure parts, at the capsule's paths: the request's
//! fourth word, what a policy is held to, and how a held program is written
//! and taken out.

#[path = "../../../capsule_linux/src/linux/install/market/arg.rs"]
pub mod arg;
#[path = "../../../capsule_linux/src/linux/install/market/check.rs"]
pub mod check;
#[path = "../../../capsule_linux/src/linux/install/market/minted.rs"]
pub mod minted;
#[path = "../../../capsule_linux/src/linux/install/market/take.rs"]
pub mod take;
