// NONOS Operating System (AGPL-3.0-or-later)

//! What start-up says under this driver's name: the shared claim's reasons,
//! which name the RTL8821CE and reach the console through the shared give-up
//! line, and the line for a chip that is given back. Pure, so held on the
//! host.

use crate::pwr::NoMaster;

/// A shared claim reason under this driver's name; any other is kept.
pub fn own(why: &'static str) -> &'static str {
    match why {
        "rtl8821ce: claim failed" => "rtl8822ce: claim failed",
        "rtl8821ce: memory space enable failed" => "rtl8822ce: memory space enable failed",
        "rtl8821ce: mmio map failed" => "rtl8822ce: mmio map failed",
        other => other,
    }
}

/// The console line for a chip that is given back.
pub fn refusal(why: NoMaster) -> &'static [u8] {
    match why {
        NoMaster::PowerFailed => b"[rtl8822ce] power-on sequence failed; chip given back\n",
        NoMaster::LeftOn { .. } => b"[rtl8822ce] power-off did not take; chip given back\n",
        NoMaster::DeadMmio => b"[rtl8822ce] no response after power-on; chip given back\n",
        NoMaster::Refused => b"[rtl8822ce] bus master refused; chip given back\n",
    }
}
