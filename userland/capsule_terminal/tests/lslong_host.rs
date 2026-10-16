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
// Host-side harness for the `ls -l` columns: the mtime stamp, the byte total
// and the `.`/`..` rows. Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/lslong_host.rs -o /tmp/lslong_host && /tmp/lslong_host
//
// The modules are pulled in by path so this cannot pass against a copy that
// has drifted from what the capsule runs.

extern crate alloc;

#[path = "lslong_host/calendar.rs"]
mod calendar;
#[path = "lslong_host/dots.rs"]
mod dots;
#[path = "lslong_host/mounts.rs"]
mod mounts;
#[path = "lslong_host/stamps.rs"]
mod stamps;
#[path = "lslong_host/support.rs"]
mod support;
#[path = "lslong_host/totals.rs"]
mod totals;

use mounts::{format_u64, ls_date, ls_dots, ls_long, ls_total, term};
