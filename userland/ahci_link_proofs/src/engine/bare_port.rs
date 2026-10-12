// NONOS Operating System (AGPL-3.0-or-later)
//! A port with no DMA memory behind it, at register offset 0, for the
//! proofs that run recovery on a register file in host memory.

use super::port::Port;
use super::region::DmaRegion;
use crate::identity::Names;

pub(super) fn bare_port() -> Port {
    let none = || DmaRegion::at(0, 0);
    Port {
        clb: none(),
        ctba: none(),
        _fb: none(),
        data: none(),
        base: 0,
        capacity_sectors: 0,
        names: Names::empty(),
        last_tfd: 0,
        wedged: false,
        command_ms: 0,
        reset_window: None,
    }
}
