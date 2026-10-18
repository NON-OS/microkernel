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

//! ResourceSource text paths found in the namespace.

use super::rig::machine;
use crate::consts::at;
use crate::platform::resolve_text;
use crate::run::boot;

#[test]
fn text_paths() {
    let (aml, _) = boot(&machine(), 2);
    let pad = at(&aml, "\\_SB.PCI0.I2C1.TPD0").unwrap();
    let i2c = at(&aml, "\\_SB.PCI0.I2C1");
    assert_eq!(resolve_text(&aml.ns, pad, b"^"), None, "no segment");
    assert_eq!(resolve_text(&aml.ns, pad, b"I2C1"), i2c, "searched upward");
    assert_eq!(resolve_text(&aml.ns, pad, b"GPO0"), at(&aml, "\\_SB.GPO0"), "upward to \\_SB");
    assert_eq!(resolve_text(&aml.ns, pad, b"^^I2C1"), i2c, "parent prefixes");
    assert_eq!(
        resolve_text(&aml.ns, pad, b"PCI0.I2C1"),
        None,
        "a multi-segment path is not searched"
    );
    assert_eq!(resolve_text(&aml.ns, pad, b"\\_SB.PCI0.I2C1\0junk"), i2c, "up to the NUL");
    assert_eq!(resolve_text(&aml.ns, pad, b"\\_SB.TOOLONG"), None);
}
