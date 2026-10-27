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

//! An Intel HD Audio function reporting class 0x0480, its DSP, which only
//! SOF runs: never claimed, and the machine is named as needing SOF, not
//! as having no controller. Intel's other 0x0480 functions, the IPU
//! cameras, name nothing.

use nonos_libc::{offer_devices, Bar, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::controller::sst::intel_dsp;
use crate::controller::verdict::Verdict;
use crate::discover::survey;
use crate::setup::machine_verdict;

/// A PCI multimedia "other" function, filed by the broker as other.
fn other(device_id: u64, vendor: u16, device: u16) -> DeviceRecord {
    let mut r = DeviceRecord { device_id, vendor, device, ..Default::default() };
    (r.bus_kind, r.class, r.pci_class, r.pci_subclass) = (BUS_KIND_PCI, 0x0080, 0x04, 0x80);
    (r.bar_count, r.bars[0]) = (1, Bar { size: 0x4000, kind: BAR_KIND_MMIO });
    r
}

#[test]
fn an_intel_dsp_function_names_the_machine_as_needing_sof() {
    // Lunar Lake's HD Audio function, reporting 0x0480.
    offer_devices(vec![other(1, 0x8086, 0xa828)]);
    let s = survey();
    assert!(s.is_empty(), "the DSP was taken for an HD Audio controller");
    assert_eq!((s.intel_sst, s.bus_verdict()), (Some(0xa828), Some(Verdict::NeedsSof)));
    assert_eq!(machine_verdict(&s, s.bus_verdict()), Verdict::NeedsSof);
}

#[test]
fn an_ipu_camera_or_another_vendor_names_nothing() {
    // Tiger Lake's IPU6 is Intel's, class 0x0480, and no audio.
    offer_devices(vec![other(2, 0x8086, 0x9a19), other(3, 0x10ec, 0xa828)]);
    let s = survey();
    assert_eq!((s.intel_sst, s.bus_verdict()), (None, None));
    assert!(intel_dsp(0x8086, 0x7e28, 0x04, 0x80));
    assert!(!intel_dsp(0x8086, 0x7e28, 0x04, 0x03), "an HD Audio class is a controller");
    assert!(!intel_dsp(0x8086, 0x9a19, 0x04, 0x80));
}
