// NONOS Operating System (AGPL-3.0-or-later)
//! QEMU's ECM configuration with bytes changed and cut, as a hostile
//! device could send it: the search never panics, and a function it finds
//! has a bulk IN and a bulk OUT pipe.

use nonos_usbnet::qemu_usb_net::CONFIG_ECM;

use crate::ecm::function::find_ecm;
use crate::hostile_tests::next;

#[test]
fn mutated_configurations_never_panic_and_a_found_function_has_its_pipes() {
    let mut seed = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..20_000 {
        let mut raw = CONFIG_ECM.to_vec();
        for _ in 0..1 + next(&mut seed) % 4 {
            let at = (next(&mut seed) as usize) % raw.len();
            raw[at] = next(&mut seed) as u8;
        }
        raw.truncate(1 + (next(&mut seed) as usize) % raw.len());
        if let Some(f) = find_ecm(&raw) {
            assert!(f.pipes.bulk_in & 0x80 != 0 && f.pipes.bulk_out & 0x80 == 0, "{f:?}");
        }
    }
}
