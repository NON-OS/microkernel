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
//! Every group of usage rows, in the order `help <command>` searches them.

use super::apps::APPS;
use super::files::FILES;
use super::jobs::JOBS;
use super::net::NET;
use super::settings::SETTINGS;
use super::system::SYSTEM;
use super::text::TEXT;

/// One group's rows: name, usage, what it does.
type Group = &'static [(&'static [u8], &'static [u8], &'static [u8])];

/// Every group, in order; rows keep their order within each.
pub(crate) const USAGE: &[Group] = &[FILES, TEXT, JOBS, SYSTEM, SETTINGS, NET, APPS];
