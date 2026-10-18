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

//! Listed controllers with their evaluated windows and interrupt; the rest
//! passed over, those with a reason named.

use super::rig::machine;
use crate::consts::at;
use crate::legacy::Range;
use crate::platform::inventory;
use crate::platform_device::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

const LIST: &[u8] =
    b"AMDI0010\ti2c\tu\nINT33FC\tgpio\tu\n80860F14\tsd\tu\nINT3432\ti2c\tu\nINT33C2\ti2c\tu\nINT33C3\ti2c\tu\n";

#[test]
fn listed_controllers_with_their_resources() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let (devices, _) = inventory(&mut aml, &mut evaluator(&mut probe));
    let (list, _) = known(LIST);
    let f = platform_devices(&mut aml, &mut evaluator(&mut probe), &devices, &list);
    let id = |p: &str| at(&aml, p).unwrap();
    let r = |base, len| Range { base, len };
    let got: Vec<_> = f.devices.iter().map(|d| (d.id, d.class, d.matched.as_slice())).collect();
    assert_eq!(
        got,
        [
            (id("\\_SB.I2CA"), Class::I2c, &b"AMDI0010"[..]),
            (id("\\_SB.GPO0"), Class::Gpio, b"INT33FC"),
            (id("\\_SB.SDHA"), Class::Sd, b"80860F14"),
        ],
        "not enabled, no template, no window and not listed are left out"
    );
    let i2c = &f.devices[0];
    assert_eq!(i2c.windows, [r(0xFEDC_2000, 0x1000)]);
    let irq = i2c.irq.unwrap();
    assert_eq!((irq.gsiv, irq.edge, irq.active_low, irq.shared), (10, false, true, true));
    assert_eq!(f.devices[1].windows.len(), 2);
    assert_eq!(f.devices[1].irq.map(|i| i.gsiv), Some(49));
    let sd = &f.devices[2];
    assert_eq!((sd.uid, sd.windows.as_slice(), sd.irq), (3, &[r(0x1_0000_0000, 0x1000)][..], None));
    assert_eq!(
        f.skipped,
        [(id("\\_SB.I2CC"), Skip::NoTemplate), (id("\\_SB.I2CD"), Skip::NoWindow)]
    );
}
