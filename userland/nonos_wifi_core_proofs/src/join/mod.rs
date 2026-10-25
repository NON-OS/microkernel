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

//! The join as one path from the shell's bytes to the chosen AKM against a
//! hostile access point: a seeded sweep that a network saved as WPA3 is
//! never joined with WPA2 (`select_sweep`), the end-to-end downgrade
//! refusal (`downgrade_tests`), the refusal of SAE without management-frame
//! protection for a saved-WPA3 network (`pmf_tests`), and a sweep of hostile
//! connect bodies that never leak the passphrase or panic (`body_tests`).

mod body_tests;
mod downgrade_tests;
mod pmf_tests;
mod select_sweep;
