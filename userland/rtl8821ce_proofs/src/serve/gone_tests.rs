// NONOS Operating System (AGPL-3.0-or-later)

//! The serve loop's check for a card that left the bus: a live card is left
//! up, one reading all ones is taken down and reported `DeadMmio`.

use super::gone::check_gone;
use super::host_radio::host_radio;
use super::radio::Radio;
use super::Stage;
use crate::constants::regs::REG_SYS_CFG1;
use crate::regs::Mmio;

#[test]
fn a_live_card_stays_up() {
    let (mut radio, host) = host_radio();
    host.regs.write32(REG_SYS_CFG1, 0x0010_C0DE);
    let mut stage = Stage::Ready;
    check_gone(&mut radio, &mut None, &mut stage);
    assert!(matches!(radio, Radio::Up(_)) && stage == Stage::Ready);
}

#[test]
fn a_card_reading_all_ones_is_taken_down_as_dead() {
    let (mut radio, host) = host_radio();
    host.regs.write32(REG_SYS_CFG1, u32::MAX);
    let mut stage = Stage::Ready;
    check_gone(&mut radio, &mut None, &mut stage);
    assert!(matches!(radio, Radio::Down), "the radio is down");
    assert!(stage == Stage::DeadMmio, "and the panel reads why");
}
