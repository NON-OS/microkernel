// NONOS Operating System (AGPL-3.0-or-later)
//! What depends on the bus speed and the chip's code: the slower timers
//! of a high-speed adapter, and the 6010's flow-control task.

use crate::chip::Access;
use crate::helpers::{first, last, link_to, up, wrote, GIGA_UP};

#[test]
fn the_6010_holds_its_flow_control_task_off_and_back_on() {
    let (p, _r) = up(0x6010);
    let off = first(&p, |a| matches!(*a, Access::Write(0, 0xd4e8, _, v) if v & 0x0002 == 0));
    let on = last(&p, wrote(0, 0xd4e8, 0x0002));
    assert!(off.unwrap() < on.unwrap());
    let (q, _r) = up(0x6000);
    assert!(first(&q, |a| matches!(*a, Access::Write(0, 0xd4e8, _, _))).is_none());
}

#[test]
fn a_high_speed_adapter_gets_its_slower_timers() {
    use crate::adapter::{vendor_high_speed, CONFIG_ECM};
    let configs = vec![vendor_high_speed(), CONFIG_ECM.to_vec()];
    let p = crate::chip::plug_with(0x5c10, (0x0bda, 0x8153), configs);
    let nonos_usbnet::Bind::Ours(mut r) = crate::helpers::try_bind(&p) else { panic!("not bound") };
    link_to(&p, &mut r, GIGA_UP).unwrap();
    let chip = p.chip.borrow();
    assert_eq!(chip.stored(0, 0xd418) >> 16 & 0x0c, 0x04, "500 ms low-power timer");
    assert_eq!(chip.stored(0, 0xd42c) & 0xffff, 250_000 / 8);
}
