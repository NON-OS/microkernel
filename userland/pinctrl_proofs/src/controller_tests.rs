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

use nonos_pinctrl::{controller, Controller};

fn intel_name(hid: &[u8; 8]) -> Option<&'static str> {
    match controller(hid)? {
        Controller::Intel(l) => Some(l.name),
        Controller::Amd => None,
    }
}

#[test]
fn seven_character_ids_need_the_trailing_zero() {
    assert_eq!(intel_name(b"INT344B\0"), Some("Sunrise Point-LP"));
    assert_eq!(intel_name(b"INT344BX"), None);
    assert_eq!(intel_name(b"INT34BB\0"), Some("Cannon Point-LP"));
}

#[test]
fn alder_lake_p_uses_the_tiger_lake_lp_layout() {
    // Linux pinctrl-tigerlake.c binds INTC1055 to tgllp_soc_data.
    assert_eq!(intel_name(b"INTC1055"), Some("Tiger Lake-LP"));
    assert_eq!(intel_name(b"INT34C5\0"), Some("Tiger Lake-LP"));
}

#[test]
fn amd_ids_and_unknown_ids() {
    for id in [b"AMD0030\0", b"AMDI0030", b"AMDI0031", b"AMDI0033"] {
        assert!(matches!(controller(id), Some(Controller::Amd)));
    }
    assert!(controller(b"AMDI0010").is_none(), "the AMD I2C controller is not GPIO");
    assert!(controller(b"\0\0\0\0\0\0\0\0").is_none());
}
