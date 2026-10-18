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

//! The PCI path to a device: its _ADR and its bridges', up to the host
//! bridge and its _SEG and _BBN.

use crate::consts::at;
use crate::enc::*;
use crate::platform::device_pci_path;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn up_to_the_host_bridge() {
    let adr = |v: u64| name_obj("_ADR", &int(v));
    let nvme = device("NVME", &adr(0));
    let rp = device("RP01", &cat(&[&adr(0x001C_0000), &nvme]));
    let host = cat(&[&name_obj("_HID", &int(0x080A_D041)), &name_obj("_BBN", &int(2)), &rp]);
    let src = cat(&[&device("\\_SB.PCI0", &host), &device("\\_SB.LOOS", &adr(3))]);
    let (mut aml, _) = boot(&src, 2);
    let mut probe = Probe::default();
    let dev = at(&aml, "\\_SB.PCI0.RP01.NVME").unwrap();
    let p = device_pci_path(&mut aml, &mut evaluator(&mut probe), dev).unwrap();
    assert_eq!((p.segment, p.bus, p.adrs.as_slice()), (0, 2, &[0x001C_0000, 0][..]));
    let loose = at(&aml, "\\_SB.LOOS").unwrap();
    assert!(
        device_pci_path(&mut aml, &mut evaluator(&mut probe), loose).is_none(),
        "no host bridge"
    );
    let pci0 = at(&aml, "\\_SB.PCI0").unwrap();
    assert!(device_pci_path(&mut aml, &mut evaluator(&mut probe), pci0).is_none(), "no _ADR");
}
