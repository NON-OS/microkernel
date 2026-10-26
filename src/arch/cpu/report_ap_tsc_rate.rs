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

/// Say on the log which source gave the APs their TSC rate. Called by the
/// boot CPU once the APs are started. Only x86_64 calibrates a TSC per CPU;
/// elsewhere every CPU reads the one system counter, and there is nothing to say.
pub fn report_ap_tsc_rate() {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::cpu::report_ap_tsc_rate();
}
