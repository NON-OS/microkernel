// NONOS Operating System (AGPL-3.0-or-later)
/*
 * A consensus refresh serves from the relays in hand until the new set is
 * whole, and nothing is built once the old consensus stops being valid. On
 * the old manager a refresh dropped the client to its cold bootstrap, so
 * `a_refresh_keeps_serving_until_the_swap` fails there: every stream open
 * was refused from the hour mark until the new directory was joined.
 */

use crate::refresh_rule::{refresh_join, usable, RefreshJoin};

const HOUR: u64 = 3_600;
const VALID_UNTIL: u64 = 10 * HOUR;

#[test]
fn a_batch_decides_keep_swap_or_refetch() {
    assert_eq!(refresh_join(false, false), RefreshJoin::Keep);
    assert_eq!(refresh_join(false, true), RefreshJoin::Keep, "a partial set is not swapped in");
    assert_eq!(refresh_join(true, true), RefreshJoin::Swap);
    assert_eq!(refresh_join(true, false), RefreshJoin::Refetch);
}

#[test]
fn a_refresh_keeps_serving_until_the_swap() {
    // Ready, fresh: usable.
    assert!(usable(true, false, 900, HOUR, VALID_UNTIL));
    // The hour mark: the manager leaves Ready to refetch, and serves on.
    let mut now = HOUR;
    for _batch in 0..3 {
        assert!(usable(false, true, 900, now, VALID_UNTIL), "refused mid refresh at {now}");
        assert_eq!(refresh_join(false, true), RefreshJoin::Keep);
        now += 60;
    }
    // A refetch after a set that draws no path still serves.
    assert_eq!(refresh_join(true, false), RefreshJoin::Refetch);
    assert!(usable(false, true, 900, now, VALID_UNTIL));
    // The whole new set: swapped in, Ready again.
    assert_eq!(refresh_join(true, true), RefreshJoin::Swap);
    assert!(usable(true, false, 950, now, VALID_UNTIL + HOUR));
}

#[test]
fn nothing_is_built_once_the_old_consensus_is_invalid() {
    assert!(!usable(false, true, 900, VALID_UNTIL, VALID_UNTIL), "a refresh that never lands fails closed");
    assert!(!usable(true, false, 900, VALID_UNTIL + 1, VALID_UNTIL));
}

#[test]
fn a_cold_bootstrap_is_not_usable() {
    assert!(!usable(false, false, 900, 0, VALID_UNTIL), "not Ready and not refreshing");
    assert!(!usable(true, false, 0, 0, VALID_UNTIL), "no relays");
    assert!(!usable(false, true, 0, 0, VALID_UNTIL), "a refresh with nothing in hand");
}
