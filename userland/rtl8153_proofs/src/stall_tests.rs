// NONOS Operating System (AGPL-3.0-or-later)
//! A stalled pipe, IN or OUT, reset on the host and its halt cleared.

use nonos_usbnet::mock::Call;
use nonos_usbnet::Nic;

use crate::helpers::up;

#[test]
fn a_stalled_in_pipe_is_reset_and_its_halt_cleared() {
    let (p, mut r) = up(0x6010);
    p.bus.0.borrow_mut().bulk_in.push_back(Err(-32));
    assert_eq!(r.recv(&mut [0u8; 1514]), Err(-32));
    let calls = p.bus.0.borrow().calls.clone();
    assert!(calls.contains(&Call::ResetBulk(true)));
    assert!(calls
        .iter()
        .any(|c| matches!(c, Call::Out(s, _) if s.request == 0x01 && s.index == 0x81)));
}

#[test]
fn a_stalled_out_pipe_is_reset_and_its_halt_cleared() {
    let (p, mut r) = up(0x6000);
    p.bus.0.borrow_mut().bulk_out_fails = Some(-32);
    assert_eq!(r.send(&[1; 64]), Err(-32));
    let calls = p.bus.0.borrow().calls.clone();
    assert!(calls.contains(&Call::ResetBulk(false)));
    assert!(calls
        .iter()
        .any(|c| matches!(c, Call::Out(s, _) if s.request == 0x01 && s.index == 0x02)));
}
