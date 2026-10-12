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

//! Every table after FADT and MADT, read in one order: the platform's
//! timers and windows, NUMA, idle, watchdog, logo, console, channels and
//! secure devices, then the IOMMUs last, as they read what came before. Each parser keeps what it
//! found; a table that is absent or does not check out is passed over.

use super::super::state::TableRegistry;
use super::{
    parse_bgrt, parse_dmar, parse_hmat, parse_hpet, parse_ivrs, parse_lpit, parse_mcfg, parse_pcct,
    parse_sdev, parse_serial_console, parse_slit, parse_srat, parse_wdat,
};

pub fn parse_other_tables(registry: &mut TableRegistry) {
    parse_hpet(registry);
    parse_mcfg(registry);
    parse_srat(registry);
    parse_slit(registry);
    parse_hmat(registry);
    parse_lpit(registry);
    parse_wdat(registry);
    parse_bgrt(registry);
    parse_serial_console(registry);
    parse_pcct(registry);
    parse_sdev(registry);
    parse_dmar(registry);
    parse_ivrs(registry);
}
