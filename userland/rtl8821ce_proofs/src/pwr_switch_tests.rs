// NONOS Operating System (AGPL-3.0-or-later)
//! rtw88's power-on path on the modeled chip: a cold card is not powered off,
//! a warm one is, a failed power-off does not stop the power-on (mac.c:388),
//! the system config runs first, running firmware is woken, and a stalled
//! poll is retried once after the BIT_PFM_WOWL pulse.

use crate::pwr::command::PwrCmd;
use crate::pwr::{power_on, run_pwr_seq, PowerOn};
use crate::pwr_mock::Chip;

#[test]
fn a_cold_card_is_powered_on_without_a_power_off() {
    let c = Chip::new(0xEA);
    assert!(matches!(power_on(&c), PowerOn::FromCold));
    assert!(!c.wrote(0x0093), "card disable's first write never ran");
    assert_eq!(c.writes.borrow()[0], (0x001C, 0), "REG_RSV_CTRL is cleared first");
    assert_eq!(c.writes.borrow()[2], (0x0075, 0x01), "BIT_USB_SUS_DIS is set next");
    assert!(c.wrote(0x0300), "carddis-to-cardemu runs on every power-on");
}

#[test]
fn a_warm_card_is_powered_off_then_on() {
    let c = Chip::new(0x00);
    assert!(matches!(power_on(&c), PowerOn::WasLeftPowered { .. }));
    assert!(c.wrote(0x0093), "card disable ran");
}

#[test]
fn the_power_off_reports_reg_cr_after_it() {
    let c = Chip::new(0x00); // the model never powers REG_CR down
    assert!(matches!(power_on(&c), PowerOn::WasLeftPowered { cr_after_off: 0x00 }));
}

#[test]
fn init_system_cfg_follows_power_on() {
    let c = Chip::new(0xEA);
    c.regs.borrow_mut()[0x1103] = 0xA3;
    power_on(&c);
    let r = c.regs.borrow();
    assert_eq!(r[0x1082] & 0x01, 0x01, "BIT_WL_PLATFORM_RST");
    assert_eq!(r[0x1081] & 0x01, 0x01, "BIT_DDMA_EN");
    assert_eq!(r[0x03] & 0xD8, 0xD8, "sys_func_en 0xD8 in REG_SYS_FUNC_EN+1");
    assert_eq!(r[0x1103], 0xAC, "REG_CR_EXT+3 keeps its high nibble, low is 0xC");
}

#[test]
fn a_failed_power_off_still_powers_on() {
    let mut c = Chip::new(0x00);
    c.stuck = Some((0x05, 0x02)); // the power-off request never clears
    assert!(matches!(power_on(&c), PowerOn::WasLeftPowered { .. }));
}

#[test]
fn running_firmware_is_woken_before_the_switch() {
    let c = Chip::new(0xEA);
    c.regs.borrow_mut()[0x80] = 0x78;
    c.regs.borrow_mut()[0x81] = 0xC0;
    power_on(&c);
    assert!(c.writes.borrow().contains(&(0x03D9, 0x80)), "RPWM toggle bit flipped");
    let cold = Chip::new(0xEA);
    power_on(&cold);
    assert!(!cold.wrote(0x03D9), "no firmware, no RPWM write");
}

#[test]
fn a_stalled_poll_is_retried_after_the_wowl_pulse() {
    let mut c = Chip::new(0xEA);
    c.on_wowl = Some((0x0010, 0x01));
    assert!(run_pwr_seq(&c, &[PwrCmd::poll(0x0010, 0x01, 0x01), PwrCmd::end()]));
    let w = c.writes.borrow();
    assert_eq!(&w[..], &[(0x04, 0x08), (0x04, 0x00)], "value|BIT3 then value&~BIT3");
}
