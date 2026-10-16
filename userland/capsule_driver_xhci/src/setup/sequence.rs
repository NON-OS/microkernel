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
use nonos_libc::mk_device_release;

use super::bring_up::{program, start_checked};
use super::{
    assemble::assemble, claim::claim, driver::Driver, irq_bind::irq_bind, layout::read_layout,
    mmio_map::mmio_map, pci::enable_bus_master, pci::enable_decode, take_over::take_over,
};
use crate::discover::Found;
use crate::error::XhciResult;
use crate::handles::BrokerHandles;

/// One bring-up attempt on the controller discovery found. Before
/// `BrokerHandles` exists a failure releases the claim itself. Until the
/// `Driver` exists the controller holds no DMA of ours, and dropping the
/// handles releases it. From then on a failure drops the `Driver`, whose
/// `Drop` stops the controller and releases it before any DMA frame it
/// was given is freed (`teardown`), so the next attempt can claim it again.
/// Bus Master goes on only once `take_over` saw the controller halted and
/// reset with CNR clear, so nothing a predecessor armed still runs, and
/// once `assemble`, the last step that can refuse, has allocated what it
/// will be given: every refusal releases the controller with it never set
///. It goes on before `program`: the ERSTBA write starts the
/// event ring, whose segment table the controller fetches then (xHCI 1.2
/// section 5.5.2.3.2), and qemu-xhci shows HCE if it cannot.
pub fn run(dev: Found) -> XhciResult<Driver> {
    let claim_epoch = claim(dev.device_id)?;
    if let Err(e) = enable_decode(dev.device_id, claim_epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let mmio = mmio_map(dev.device_id, claim_epoch, dev.bar0_size)?;
    let irq = irq_bind(dev, claim_epoch);
    let handles =
        BrokerHandles::new(dev.device_id, claim_epoch, mmio.grant_id, mmio.user_va, irq.grant_id);
    let layout = read_layout(&handles, mmio.length)?;
    take_over(&layout, mmio.user_va, mmio.length)?;
    let mut driver = assemble(handles, layout, claim_epoch)?;
    enable_bus_master(dev.device_id, claim_epoch)?;
    program(&mut driver);
    start_checked(&mut driver)?;
    Ok(driver)
}
