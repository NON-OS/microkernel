// NONOS Operating System (AGPL-3.0-or-later)
//! An interrupt endpoint's context is typed by its direction: an odd DCI
//! Interrupt IN (7), an even one Interrupt OUT (3), a security key's
//! request pipe. Before, every interrupt endpoint was typed IN.

use super::silicon_tests::{dw, region};
use crate::contexts::{write_configure_endpoint_input, EndpointConfig};

fn ep_type(dci: u8) -> u32 {
    let csz = 32usize;
    let (input, output) = (region(33 * 32), region(32 * 32));
    let cfg =
        EndpointConfig { context_size: 32, dci, ring_phys: 0x1000, max_packet: 64, interval: 4 };
    write_configure_endpoint_input(&input, &output, cfg);
    dw(&input, (dci as usize + 1) * csz + 4) >> 3 & 0x7
}

#[test]
fn an_odd_dci_is_interrupt_in_and_an_even_one_interrupt_out() {
    assert_eq!(ep_type(3), 7, "EP1 IN");
    assert_eq!(ep_type(4), 3, "EP2 OUT");
    assert_eq!(ep_type(5), 7, "EP2 IN");
    assert_eq!(ep_type(2), 3, "EP1 OUT");
}
