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

//! The Wi-Fi panel's state and its steps: entering the panel, its status,
//! scans and joins handed to the worker and their answers, the radio, the
//! passphrase, saved networks, and the refusals the panel gives.

pub mod answer;
pub mod apply;
pub mod cursor;
pub mod diag;
pub mod enter;
pub mod extra;
pub mod join;
pub mod kinds;
pub mod pass;
pub mod pending;
pub mod radio;
pub mod refusal;
pub mod remember;
pub mod saved;
pub mod scan;
pub mod scan_now;
pub mod status;
pub mod step;
pub mod worker;
