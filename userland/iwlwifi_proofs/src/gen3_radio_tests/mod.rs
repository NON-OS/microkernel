// NONOS Operating System (AGPL-3.0-or-later)
//! The serving-loop side of the gen3 radio, against the modeled device: the
//! background sweep (completes, ignores another scan's completion, gives up
//! a sweep past its budget, stops on a firmware error, refuses an empty
//! channel list), what it hears into the shared scan list, the device masked
//! once ALIVE is in and stopped after a failed boot, the secure-boot status
//! read through the UMAC window, the bounds every grant access is checked
//! against, how each failure is reported, the control family's replies, and
//! which legacy ops are refused once the radio owns the card.

mod boot;
mod control;
mod heard;
mod outcome;
mod rig;
mod routing;
mod scanlist;
mod sweep;
mod sweep_ends;
