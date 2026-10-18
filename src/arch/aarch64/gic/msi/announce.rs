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

use crate::sys::serial::Line;

/// Bring the MSI controller up and say what PCI functions can bind as a
/// result. Returns the routes it installed: one controller, or none.
pub fn bring_up_and_report() -> usize {
    match super::bring_up() {
        Ok(name) => {
            let mut line = Line::new();
            line.str(b"[MSI] ").str(name.as_bytes());
            line.str(b" in service; PCI functions may bind MSI and MSI-X");
            // A v2m frame raises whichever SPI it is written, whoever wrote it.
            if name == "gic-v2m" {
                line.str(b"; the frame cannot tell one device's messages from another's");
            }
            line.end();
            1
        }
        Err(reason) => {
            Line::new().str(b"[MSI] ").str(reason.as_bytes()).str(b"; PCI has INTx alone").end();
            0
        }
    }
}
