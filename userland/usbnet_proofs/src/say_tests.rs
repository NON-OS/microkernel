// NONOS Operating System (AGPL-3.0-or-later)
//! What reaches the serial log when a device comes up: how the drawn
//! address was taken, never the address.

use nonos_libc::lines;

use crate::say::say_up;
use crate::station::Taken;

#[test]
fn the_line_a_bind_writes_says_how_the_address_was_taken_and_not_what_it_is() {
    for (taken, how) in [
        (Taken::Programmed, "drawn station address programmed"),
        (Taken::Promiscuous, "drawn station address, device filter opened"),
        (Taken::Unconfirmed, "drawn station address not confirmed: device filter would not open"),
    ] {
        say_up(b"cdc-ecm", 3, taken);
        let line = lines().pop().expect("a line");
        assert_eq!(String::from_utf8_lossy(&line), format!("[usbnet cdc-ecm] port 3 up, {how}\n"));
    }
}
