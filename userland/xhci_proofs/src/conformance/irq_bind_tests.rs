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

//! The interrupt a controller is brought up with: MSI-X, or MSI on a
//! controller that has only that (Intel PCH controllers), and each
//! controller's waits block on its own. One process-wide grant, set by
//! every bring-up, left the first controller blocked on the second's.

use nonos_devmodel::{run, LiveDevice};
use nonos_libc::MK_IRQ_BIND_MSIX;
use nonos_libc::{irq_tried, irq_waited_on, offer_irq, offer_mmio, MK_IRQ_BIND_MSI};

use super::bringup_device::{controller_that, Once};
use super::bringup_model::{bringup_controller, found, WINDOW};
use crate::controller::park::SPIN_BUDGET;
use crate::setup::Driver;

const SUCCESS: u8 = 1;

/// A controller brought up, and the model it runs against.
fn brought_up() -> (LiveDevice, Driver) {
    let bar = bringup_controller();
    let hc = run(
        &bar,
        controller_that(Once::AnswersNoop(SUCCESS), Default::default(), Default::default()),
    );
    offer_mmio(bar.base(), WINDOW as u64);
    (hc, crate::setup::run(found()).unwrap_or_else(|e| panic!("bring-up: {e:?}")))
}

#[test]
fn a_controller_with_msi_and_no_msi_x_binds_msi() {
    offer_irq(MK_IRQ_BIND_MSI);
    let (_hc, d) = brought_up();
    assert_eq!(irq_tried(), MK_IRQ_BIND_MSIX | MK_IRQ_BIND_MSI, "MSI-X first, then MSI");
    assert_ne!(d.handles.irq_grant_id(), 0, "an interrupt is bound");
}

#[test]
fn each_controller_waits_on_its_own_interrupt() {
    offer_irq(MK_IRQ_BIND_MSIX);
    let (_one, mut first) = brought_up();
    let (_two, mut second) = brought_up();
    assert_ne!(first.handles.irq_grant_id(), second.handles.irq_grant_id());
    first.event_ring.irq.park(SPIN_BUDGET + 1);
    assert_eq!(irq_waited_on(), first.handles.irq_grant_id(), "not the last bound");
    second.event_ring.irq.park(SPIN_BUDGET + 1);
    assert_eq!(irq_waited_on(), second.handles.irq_grant_id());
}
