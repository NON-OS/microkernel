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

use nonos_policy_proto::Field;

use super::cache::Cached;
use super::choose::{choose, HOST_FALLBACK};
use super::sanitize::hostname_len;

static HOST: Cached = Cached::new(Field::Hostname, hostname_len);

/// The configured hostname, asked for once per capsule.
pub fn hostname() -> &'static [u8] {
    choose(HOST.get(), HOST_FALLBACK)
}
