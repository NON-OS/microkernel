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

use super::health_type::SmartHealth;

impl SmartHealth {
    /// The snapshot served when the SMART / health log could not be read:
    /// every counter zero. The log is diagnostics, and a drive that refuses
    /// it still stores data, so its refusal does not stop the bring-up.
    pub const fn unread() -> Self {
        Self {
            critical_warning: 0,
            temperature_kelvin: 0,
            available_spare: 0,
            available_spare_threshold: 0,
            percentage_used: 0,
            endurance_group_warning: 0,
            data_units_read: 0,
            data_units_written: 0,
            host_read_commands: 0,
            host_write_commands: 0,
            controller_busy_time: 0,
            power_cycles: 0,
            power_on_hours: 0,
            unsafe_shutdowns: 0,
            media_errors: 0,
            error_log_entries: 0,
            warning_temp_time: 0,
            critical_temp_time: 0,
        }
    }
}
