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

//! HTTP to a mirror, over the network the person chose.

pub(super) mod anyone;
/* Waiting for Anyone before an install downloads, the fetcher's own rule. */
#[path = "../../../../../capsule_model_fetch/src/get/anyone_wait.rs"]
mod anyone_wait;
#[cfg(feature = "dev-mirror")]
pub(super) mod direct;
pub(super) mod get;
pub(super) mod reach;
pub(super) mod route;
pub(super) mod stream;
pub(super) use get::*;
pub(super) use reach::*;
