// NONOS Operating System (AGPL-3.0-or-later)
//! A port at register offset 0, a page of broker grant for each region,
//! for the proofs that run recovery on a register file in host memory.

use crate::engine::port::Port;
use crate::engine::region::DmaRegion;
use crate::identity::Names;

pub fn bare_port() -> Port {
    let page = || DmaRegion::map(0, 3, 4096, 0).expect("a DMA grant");
    Port {
        clb: page(),
        ctba: page(),
        _fb: page(),
        data: page(),
        base: 0,
        capacity_sectors: 0,
        names: Names::empty(),
        last_tfd: 0,
        wedged: false,
        command_ms: 0,
        reset_window: None,
    }
}
