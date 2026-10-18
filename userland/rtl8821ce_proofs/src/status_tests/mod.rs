// NONOS Operating System (AGPL-3.0-or-later)
//! No status reply carries the factory address. The efuse model burns a
//! known address at its logical offset, and the reply is built through the
//! driver's own `status_body` from the driver's own efuse diagnosis: after a
//! read that stalls on any byte, and after a whole read.

mod card;
mod stall;
mod whole;
