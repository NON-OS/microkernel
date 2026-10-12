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
//! The 82575 with no NVM, which needs an init script of SerDes, clock and
//! PCIe lane settings this driver does not carry.

use crate::bring::reset::run;
use crate::bring::reset::NO_NVM_82575;
use crate::facts::mac::*;
use crate::facts::parts::Part;
use crate::facts::regs::*;

use super::model::{regs, window};

/// This driver carries no such script and stops, after the reset, rather
/// than run the part without it.
#[test]
fn an_82575_with_no_nvm_is_refused_and_one_with_an_nvm_is_not() {
    let bar = window();
    bar.present32(REG_EECD, EECD_AUTO_RD);
    assert_eq!(run(&regs(&bar), Part::I82575), Err(NO_NVM_82575));
    assert!("igb: ".len() + NO_NVM_82575.len() < 128, "fits one console line");
    assert_ne!(bar.wrote32(REG_CTRL) & CTRL_RST, 0, "refused after the reset");
    let bar = window();
    bar.present32(REG_EECD, EECD_AUTO_RD | EECD_PRES);
    assert_eq!(run(&regs(&bar), Part::I82575), Ok(()));
    let bar = window();
    bar.present32(REG_EECD, EECD_AUTO_RD);
    assert_eq!(run(&regs(&bar), Part::I82576), Ok(()), "only the 82575 needs the script");
}
