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

//! The panel's brightness behind its trait: a level out of firmware's PWM
//! period on Gen9's eDP panel, and nothing for an output with no backlight
//! here, a generation not cited, or a PWM firmware left unset.

use nonos_gfx_wire::errno::{EINVAL, ENOSYS, ENXIO};

use super::two_rig::two_rig;
use crate::display::Brightness;
use crate::engine::backlight::{BLC_PWM_PCH_CTL1, BLC_PWM_PCH_CTL2, PWM_ENABLE};
use crate::engine::gen::GEN11;

#[test]
fn the_edp_panel_reads_and_sets_a_level_out_of_firmwares_period() {
    let (mut d, model, _) = two_rig();
    model.borrow_mut().set(BLC_PWM_PCH_CTL2, (4800 << 16) | 1200);
    assert_eq!(d.brightness(0), Ok((1200, 4800)), "every PWM step a level");
    assert_eq!(d.set_brightness(0, 3600), Ok(()));
    assert_eq!(model.borrow().get(BLC_PWM_PCH_CTL2), (4800 << 16) | 3600, "period kept");
    assert_eq!(model.borrow().get(BLC_PWM_PCH_CTL1) & PWM_ENABLE, PWM_ENABLE);
    assert_eq!(d.brightness(0), Ok((3600, 4800)));
    assert_eq!(d.set_brightness(0, 0), Ok(()), "dark is a level too");
    assert_eq!(d.set_brightness(0, 4801), Err(EINVAL));
    assert_eq!(model.borrow().get(BLC_PWM_PCH_CTL2) & 0xFFFF, 0);
}

#[test]
fn outputs_without_a_backlight_here_answer_enosys_or_enxio() {
    let (mut d, model, _) = two_rig();
    assert_eq!(d.brightness(0), Err(ENOSYS), "firmware set no period");
    model.borrow_mut().set(BLC_PWM_PCH_CTL2, 4800 << 16);
    assert_eq!(d.brightness(1), Err(ENOSYS), "DisplayPort monitor on pipe B");
    assert_eq!(d.set_brightness(2, 1), Err(ENXIO), "no third output");
    d.gen = &GEN11;
    assert_eq!(d.set_brightness(0, 1), Err(ENOSYS), "Gen11's registers are not cited");
    assert_eq!(model.borrow().get(BLC_PWM_PCH_CTL2), 4800 << 16, "nothing written");
}
