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

//! The touchpad read whole: _STA before _CRS, the run-time register, the
//! controller paths resolved, and the properties.

use super::rig::machine;
use crate::consts::at;
use crate::platform::resolve_text;
use crate::probe::Probe;
use crate::run::boot;
use crate::props::Value;
use crate::serial::{serial_device, Bus};
use crate::tests::platform::run::evaluator;

#[test]
fn touchpad_from_the_evaluated_tables() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let pad = at(&aml, "\\_SB.PCI0.I2C1.TPD0").unwrap();
    let d = serial_device(&mut aml, &mut evaluator(&mut probe), pad, true).unwrap();
    let c = &d.connections[0];
    assert_eq!(
        c.bus,
        Bus::I2c { address: 0x2C, speed_hz: 400_000, ten_bit: false },
        "_STA ran first"
    );
    assert!(c.consumer);
    assert_eq!(resolve_text(&aml.ns, pad, &c.source), at(&aml, "\\_SB.PCI0.I2C1"));
    assert_eq!(d.hid_register, Some(0x20), "computed by _DSM at run time");
    let g = &d.gpios[0];
    assert_eq!((g.interrupt, g.edge, g.polarity, g.pins.as_slice()), (true, true, 1, &[0x55][..]));
    assert_eq!(resolve_text(&aml.ns, pad, &g.source), at(&aml, "\\_SB.GPO0"));
    let names: Vec<&[u8]> = d.properties.iter().map(|p| p.name.as_slice()).collect();
    assert_eq!(
        names,
        [&b"compatible"[..], b"max-fingers", b"reset-gpios"],
        "the other UUID's set is left"
    );
    assert_eq!(d.properties[1].value, Value::Integer(5));
    let gpo = at(&aml, "\\_SB.GPO0").unwrap();
    let want = Value::Package(vec![
        Value::Reference(gpo),
        Value::Integer(0),
        Value::Integer(1),
        Value::Integer(0),
    ]);
    assert_eq!(d.properties[2].value, want);
    assert_eq!(
        serial_device(&mut aml, &mut evaluator(&mut probe), gpo, false),
        None,
        "no serial bus"
    );
}
