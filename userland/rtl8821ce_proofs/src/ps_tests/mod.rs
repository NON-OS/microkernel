// NONOS Operating System (AGPL-3.0-or-later)
//! Leisure power save and ASPM: what the host enabled on the link, the chip's
//! own enables following sleep, and the watch that decides when to sleep.

mod dbi_card;
mod host;
mod sleep;
mod watch;
