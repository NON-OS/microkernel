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

//! The refusals while the halted controller's memory is allocated
//! (`assemble`): a page size the driver cannot serve, and a scratchpad page
//! the broker will not map. With bus mastering turned on before that
//! allocation, either one released the controller with it on, which a
//! refusal never may.

use nonos_devmodel::FakeBar;
use nonos_libc::{refuse_dma_after, Call};

use super::bringup_model::PAGESIZE;
use super::bus_master_log::{first_bus_master_on, released};
use super::left_running::attempt;
use super::model::CAP_LEN;
use crate::error::XhciError;

/// What the broker answers when it has no DMA memory left to give.
const ENOMEM: i64 = -12;

/// The attempt on a controller `shape` set up is refused with `refused`,
/// and the claim given back with bus mastering never on. How many DMA
/// grants were given back.
fn refused_without_bus_mastering(shape: fn(&FakeBar), refused: XhciError) -> usize {
    let (got, log) = attempt(shape, true);
    assert_eq!(got, Err(refused));
    assert_eq!(first_bus_master_on(&log), None, "{refused:?}: bus mastering on");
    assert!(released(&log), "{refused:?}: the claim given back");
    log.iter().filter(|l| matches!(l.call, Call::DmaUnmap(_))).count()
}

/// PAGESIZE naming no page size at all.
fn no_page_size(bar: &FakeBar) {
    bar.present32(CAP_LEN + PAGESIZE, 0);
}

/// 64 KiB pages, past the largest a scratchpad buffer is allocated at.
fn pages_too_large(bar: &FakeBar) {
    bar.present32(CAP_LEN + PAGESIZE, 1 << 4);
}

#[test]
fn a_controller_whose_page_size_is_refused_never_gets_bus_mastering() {
    for shape in [no_page_size, pages_too_large] {
        let freed = refused_without_bus_mastering(shape, XhciError::ControllerUnsupported);
        assert_eq!(freed, 0, "refused before anything was mapped");
    }
}

/// The broker maps the scratchpad array, the first grant of the attempt,
/// and refuses the first scratchpad page.
fn no_scratchpad_page(_: &FakeBar) {
    refuse_dma_after(1);
}

#[test]
fn a_scratchpad_page_the_broker_refuses_never_gets_bus_mastering() {
    let freed =
        refused_without_bus_mastering(no_scratchpad_page, XhciError::BrokerCallFailed(ENOMEM));
    assert_eq!(freed, 1, "the scratchpad array, mapped before the refusal, given back");
}
