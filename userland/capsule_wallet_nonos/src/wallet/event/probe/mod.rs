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
//! The refresh: when it runs, and what its read does to the window.

pub(in crate::wallet::event) mod probe_apply;
pub(in crate::wallet::event) mod probe_came;
pub(in crate::wallet::event) mod probe_finish;
pub(in crate::wallet::event) mod probe_follow;
pub(in crate::wallet::event) mod probe_kick;
pub(in crate::wallet::event) mod probe_tick;
pub(in crate::wallet::event) mod reading;
pub(in crate::wallet::event) mod tx_freshen;
