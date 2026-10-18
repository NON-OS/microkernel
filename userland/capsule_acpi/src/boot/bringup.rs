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

//! Bringing the platform up from the loaded namespace, in the order the
//! specification gives: the region handlers (and with them _REG, 6.5.4),
//! \_PIC (5.8.1) before anything reads _PRT, _OSC (6.2.11) before _INI so
//! firmware initializes for what NONOS supports, then _INI (6.5.1).

use super::bringup_say::{ini_line, pic_line, reg_line};
use super::eval::evaluator;
use super::osc::negotiate_all;
use crate::host::CapsuleHost;
use crate::sys::log_parts::emit;
use nonos_aml::exec::{ExecLimits, Machine};
use nonos_aml::load::Aml;
use nonos_aml::platform::{initialize, set_pic, PIC_APIC};
use nonos_aml::region::Space;

/// The spaces the capsule serves; EmbeddedControl only once the EC is
/// found, so its _REG says the EC is there only when it is (12.11).
const SPACES: [Space; 4] =
    [Space::SYSTEM_MEMORY, Space::SYSTEM_IO, Space::PCI_CONFIG, Space::EMBEDDED_CONTROL];

/// Each host bridge's _OSC grant goes to the kernel, which answers
/// MkAcpiSlot's hot-plug ports from it.
pub fn bring_up(aml: &mut Aml, host: &mut CapsuleHost) {
    let ec_found = host.ec.is_some();
    for space in SPACES.into_iter().filter(|s| *s != Space::EMBEDDED_CONTROL || ec_found) {
        let failed = Machine::new(aml, host, ExecLimits::DEFAULT).connect(aml, space);
        emit(|out| reg_line(failed.len(), space.0, out));
    }
    let pic = set_pic(aml, &mut evaluator(host), PIC_APIC);
    emit(|out| pic_line(pic.map(|r| r.map_err(|f| f.at)), out));
    negotiate_all(aml, host);
    let r = initialize(aml, &mut evaluator(host));
    let failed = r.ran.iter().filter(|(_, x)| x.is_err()).count();
    emit(|out| ini_line([r.ran.len(), failed, r.absent.len(), r.sta_failed.len()], out));
}
