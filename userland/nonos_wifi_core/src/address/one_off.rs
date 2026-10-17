// NONOS Operating System (AGPL-3.0-or-later)

//! An address for one hunt or one scan burst, tied to no network and no key:
//! drawn fresh each time, so nothing sent under it links to the station's
//! other addresses or to another hunt.

use nonos_mac::{apply, MAC_LEN};

/// `random`, from the kernel's randomness, made local and unicast.
pub fn one_off(random: &[u8; MAC_LEN]) -> [u8; MAC_LEN] {
    let mut mac = *random;
    apply(&mut mac);
    mac
}
