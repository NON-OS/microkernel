/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The world-safe regulatory default, and the channel a join may run on.
//!
//! With no country the person confirmed, a station may transmit first only
//! where every country allows it: 2.4 GHz channels 1 to 11. Channels 12 and
//! 13, and the 5 GHz channels of the world list, are passive: listened on, and
//! joined once an access point's beacon was heard there, but never probed.
//! Channel 14, the 5 GHz channels above 165 and anything else are never used.
//! Every channel is held to 20 dBm EIRP, the world domain's limit (Linux
//! regulatory database, country 00).

mod channel;
mod world;

pub use channel::join_channel;
pub use world::{max_eirp_dbm, world, Use};
