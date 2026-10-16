// NONOS Operating System (AGPL-3.0-or-later)

//! Each SO platform's transport values.

use crate::gen3::select::transport;

#[test]
fn each_so_platform_gets_its_transport_values() {
    let short = transport(0x7AF0).unwrap();
    assert_eq!(
        (short.ltr_delay, short.xtal_latency, short.low_latency_xtal, short.imr_enabled),
        (1, 500, false, false)
    );
    let long = transport(0x51F0).unwrap();
    assert_eq!(
        (long.ltr_delay, long.xtal_latency, long.low_latency_xtal, long.imr_enabled),
        (2, 12000, true, false)
    );
    assert!(transport(0x7A70).unwrap().imr_enabled && transport(0x51F1).unwrap().imr_enabled);
    assert!(transport(0x2723).is_none());
}
