//! The kind byte each row starts with.

pub(super) const FOUND: u8 = 1;
pub(super) const STATUS: u8 = 2;
pub(super) const CURSOR: u8 = 3;
/// A deposit sent and not yet seen stored: the opening, which exists nowhere
/// else until the pool stores its leaf. A new kind, so older stores still read.
pub(super) const DEPOSIT: u8 = 4;
/// One thing the wallet did with the pool (`store::activity`). A new kind, so older stores still
/// read; an older build refuses a store holding one, as it must refuse any kind it does not know.
pub(super) const ACTIVITY: u8 = 5;
