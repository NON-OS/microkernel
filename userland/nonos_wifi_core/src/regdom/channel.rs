// NONOS Operating System (AGPL-3.0-or-later)

//! The channel a join runs on, from the one the beacon was heard on and the
//! one the beacon names. A beacon is unauthenticated: the channel it names is
//! taken only where the world domain allows it and it could have been heard
//! where the radio listened (2.4 GHz channels overlap four apart; 5 GHz ones
//! do not), and otherwise the channel the radio was on is kept.

use super::world_use::world;

/// The channel to join on, or `None` when the radio heard the beacon on a
/// channel the world domain does not allow at all.
pub fn join_channel(heard_on: u8, named: u8) -> Option<u8> {
    world(heard_on)?;
    let both_2g = heard_on <= 14 && named <= 14;
    let near = if both_2g { heard_on.abs_diff(named) <= 4 } else { heard_on == named };
    Some(if world(named).is_some() && near { named } else { heard_on })
}
