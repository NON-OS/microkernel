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

//! Devices with _PRW as a Name, as a Method through a shared helper, and
//! in the GPE block form, beside the power resource they name.

use super::more::*;
use crate::enc::*;
use crate::rig::Rig;

pub fn rig() -> Rig {
    let pwr = power_res(
        "\\_SB.PWRR",
        0,
        0,
        &cat(&[&method("_STA", 0, &ret(&int(1))), &method("_ON", 0, &[]), &method("_OFF", 0, &[])]),
    );
    let gprw = cat(&[
        &name_obj("\\PRWP", &package(2, &[int(0), int(0)])),
        &method(
            "\\GPRW",
            2,
            &cat(&[
                &store(&arg(0), &e(0x88, &[&name("\\PRWP"), &int(0), &[0x00]])),
                &store(&arg(1), &e(0x88, &[&name("\\PRWP"), &int(1), &[0x00]])),
                &ret(&name("\\PRWP")),
            ]),
        ),
    ]);
    let xhci = device(
        "\\_SB.XHCI",
        &name_obj("_PRW", &package(3, &[int(0x6D), int(3), name("\\_SB.PWRR")])),
    );
    let lan =
        device("\\_SB.LAN0", &method("_PRW", 0, &ret(&invoke("\\GPRW", &[int(0x6E), int(4)]))));
    let block = device("\\_SB.GPE1", &[]);
    let usb = device(
        "\\_SB.USB1",
        &name_obj("_PRW", &package(2, &[package(2, &[name("\\_SB.GPE1"), int(3)]), int(4)])),
    );
    Rig::new(&cat(&[&device("\\_SB", &[]), &pwr, &gprw, &xhci, &lan, &block, &usb])).connected()
}
