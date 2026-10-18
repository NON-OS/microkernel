// NONOS Operating System (AGPL-3.0-or-later)
//! The console finder: register layout, clock and speed, and what it passes
//! over because no driver could use it.

use crate::arch::fdt::find::uart::UartKind;
use crate::uart_tree::{console, Props};

#[test]
fn a_16550_states_its_layout_clock_and_speed() {
    let wide: Props = &[
        ("reg-shift", &[2]),
        ("reg-io-width", &[4]),
        ("clock-frequency", &[1_843_200]),
        ("current-speed", &[9600]),
    ];
    let u = console(&[("ns16550a", wide)], None).unwrap();
    assert_eq!((u.kind, u.base, u.reg_shift, u.io_width), (UartKind::Ns16550, 0x1000, 2, 4));
    assert_eq!((u.clock_hz, u.baud), (Some(1_843_200), Some(9600)));
    let plain = console(&[("ns16550", &[])], None).unwrap();
    assert_eq!((plain.reg_shift, plain.io_width, plain.clock_hz, plain.baud), (0, 1, None, None));
}

#[test]
fn stdout_options_win_over_current_speed() {
    let props: Props = &[("current-speed", &[9600])];
    let u = console(&[("ns16550a", props)], Some("/serial@0:57600n8")).unwrap();
    assert_eq!(u.baud, Some(57_600));
    let u = console(&[("ns16550a", props)], Some("/serial@0:n8")).unwrap();
    assert_eq!(u.baud, Some(9600));
}

#[test]
fn a_layout_no_driver_handles_is_passed_over() {
    let odd: Props = &[("reg-io-width", &[2])];
    let far: Props = &[("reg-shift", &[3])];
    let u = console(&[("ns16550a", odd), ("ns16550a", far), ("arm,pl011", &[])], Some("/serial@0"));
    assert_eq!(u.map(|u| (u.kind, u.base)), Some((UartKind::Pl011, 0x3000)));
}

#[test]
fn only_a_fixed_clock_gives_a_rate() {
    let fixed = console(&[("arm,pl011", &[("clocks", &[1])])], None).unwrap();
    assert_eq!(fixed.clock_hz, Some(50_000_000));
    let managed = console(&[("arm,pl011", &[("clocks", &[2, 0x13])])], None).unwrap();
    assert_eq!(managed.clock_hz, None);
    let wide = console(&[("arm,pl011", &[("clock-frequency", &[0, 48_000_000])])], None);
    assert_eq!(wide.unwrap().clock_hz, Some(48_000_000));
    let huge = console(&[("arm,pl011", &[("clock-frequency", &[1, 0])])], None);
    assert_eq!(huge.unwrap().clock_hz, None);
}

#[test]
fn the_mini_uart_layout_is_the_chips() {
    let u = console(&[("brcm,bcm2835-aux-uart", &[("reg-shift", &[0])])], None).unwrap();
    assert_eq!((u.kind, u.reg_shift, u.io_width), (UartKind::MiniUart, 2, 4));
}
