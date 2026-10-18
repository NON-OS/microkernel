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

//! The configuration walk's edges, by name: lengths and records. Each case
//! is one input at a boundary of a rule and the one outcome the rule allows
//! there. A failure names its case.

use super::{Binding, ConfigCase, KBD};
use crate::config_blob::summaries;
use crate::config_blob::{config, keyboard, Blob, HID};
use crate::descriptors::hid_bindings;

fn cut(blob: Blob, total: u16) -> Vec<u8> {
    blob.build_with_total(total)
}

#[test]
fn configuration_descriptor_edges() {
    let header_only = config().build();
    let cases: Vec<ConfigCase> = vec![
        ("empty buffer", vec![], Err(())),
        ("eight bytes", header_only[..8].to_vec(), Err(())),
        ("header alone", header_only.clone(), Ok(vec![])),
        ("wTotalLength 8", cut(config(), 8), Err(())),
        ("wTotalLength 0", cut(config(), 0), Err(())),
        ("wTotalLength 0xffff on nine bytes", cut(config(), 0xffff), Err(())),
        ("wTotalLength one past the buffer", cut(keyboard(0), 35), Err(())),
        ("wTotalLength one short of the endpoint", cut(keyboard(0), 33), Err(())),
        ("wTotalLength exact", keyboard(0).build(), Ok(vec![KBD])),
        ("bLength 0", keyboard(0).raw(&[0, 0x24]).build(), Err(())),
        ("bLength 1", keyboard(0).raw(&[1, 0x24]).build(), Err(())),
        ("bLength 2", keyboard(0).raw(&[2, 0x24]).build(), Ok(vec![KBD])),
        ("bLength 255 in 4 bytes", keyboard(0).raw(&[255, 0x24, 0, 0]).build(), Err(())),
        ("one stray byte", keyboard(0).raw(&[7]).build(), Ok(vec![KBD])),
        (
            "interface bLength 8",
            config().raw(&[8, 4, 0, 0, 1, HID, 1, 1]).ep(0x81, 3, 8, 10).build(),
            Ok(vec![]),
        ),
        (
            "endpoint bLength 6",
            config().iface(0, HID, 1, 1).raw(&[6, 5, 0x81, 3, 8, 0]).build(),
            Ok(vec![]),
        ),
    ];
    for (name, raw, want) in cases {
        let got: Result<Vec<Binding>, ()> = hid_bindings(&raw).map(|b| summaries(&b));
        assert_eq!(got, want, "{name}");
    }
}
