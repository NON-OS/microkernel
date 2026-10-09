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

//! The LPSS wrapper and core bring-up against the modelled core, where the
//! order of register writes can be read back as a sequence.

use nonos_i2cmodel::regs::{IC_ENABLE, LPSS_PRIV_RESETS};
use nonos_i2cmodel::{Config, Designware, Bus};

use super::fixture::{bench, core};
use crate::bench::LPSS_PHYS;
use crate::regs::{self, Regs};
use crate::transaction::{control, TransferError};

#[test]
fn the_function_is_put_in_reset_then_released_and_remapped_before_the_core_is_touched() {
    let _b = bench(Config::LPSS);
    core(|c| {
        assert_eq!(c.writes_to(LPSS_PRIV_RESETS), [0, 0x7], "intel_lpss_init_dev order");
        assert_eq!(c.writes_to(0x240), [LPSS_PHYS as u32]);
        assert_eq!(c.writes_to(0x244), [(LPSS_PHYS >> 32) as u32]);
        let first_core = c.events().iter().position(|e| e.offset < 0x100).expect("core written");
        let released = c.events().iter().rposition(|e| e.offset == LPSS_PRIV_RESETS).unwrap();
        assert!(released < first_core, "core written before the reset was released");
        assert!(c.violations().is_empty(), "{:?}", c.violations());
    });
}

#[test]
fn a_core_that_never_reports_disabled_is_given_up_on_in_bounded_time() {
    /*
     * A functional clock gated by firmware leaves IC_ENABLE_STATUS frozen.
     * The wait is bounded in uptime (the shim's clock moves on each read),
     * so the driver gives up with a timeout instead of spinning a count
     * that is long on QEMU and short on silicon, and it keeps writing the
     * disable while it waits, as Linux __i2c_dw_disable does.
     */
    let _attached = regs::attach(Designware::new(Config::LPSS, Bus::default()).never_acknowledges_enable());
    regs::with(|c| c.write32(LPSS_PRIV_RESETS, 0x7));
    let before = nonos_libc::mk_uptime_ms();
    let outcome = control::disable(Regs::new(0));
    let waited = nonos_libc::mk_uptime_ms() - before;
    assert!(matches!(outcome, Err(TransferError::Timeout)));
    assert!((25..200).contains(&waited), "waited {waited} ms of uptime");
    regs::with(|c| assert!(c.writes_to(IC_ENABLE).len() > 1, "the disable was written once"));
}
