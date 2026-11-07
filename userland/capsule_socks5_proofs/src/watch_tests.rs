// NONOS Operating System (AGPL-3.0-or-later)
use crate::watch::{Watch, PROVEN_SILENCE_MS, SILENCE_MS};

// The exit watch decides when a silent exit is walked away from. The rule
// under proof: only an unproven, undirected exit that has been sent to and
// answered nothing for the whole budget rotates; delivery, configuration,
// or having sent nothing each veto it on their own.
//
// The budgets are also bounded from below, not only in their relative
// behaviour. All guest traffic shares one exit, and a rotation tears down
// every connection riding it, so a budget short enough to fire on a
// live-but-slow exit turns one slow handshake into a storm that closes all
// connections and, exit after exit, never lets any prove itself. The bounds
// below lock the budgets long enough to outlast several slow-mixnet round
// trips while staying finite, so a truly dead exit is still rotated.

/// A single mixnet send (build the Sphinx packet, cross the internet to the
/// gateway, wait out the mix delays, read the reply) can take most of the
/// IPC call budget (`ipc::call::TIMEOUT_MS`, 15 s) on a slow or emulated
/// machine. The silence budgets are measured in whole round trips of this
/// size; the constant is restated here because `watch` has no dependency on
/// the IPC layer and must not grow one for a test.
const ROUND_TRIP_MS: i64 = 15_000;

#[test]
fn the_unproven_budget_outlasts_several_slow_round_trips() {
    // A first exit that is alive but slow must get long enough to answer
    // before it is declared absent and every connection on it is dropped.
    // Below one round trip (the old 12 s) a live first exit was abandoned
    // mid-handshake under emulation, the replacement got the same too-short
    // budget, and no exit could ever prove itself: the connection-closed
    // storm. At least three round trips closes that.
    assert!(
        SILENCE_MS >= 3 * ROUND_TRIP_MS,
        "the unproven silence budget must outlast several slow-mixnet round trips \
         so a live-but-slow first exit is not abandoned mid-handshake"
    );
}

#[test]
fn a_proven_exit_is_given_strictly_more_patience_than_an_unproven_one() {
    // An exit that has carried traffic is not dead because it went quiet for
    // a stretch (a long-poll waiting on a server push, a paused download).
    // It earns more patience than one that has never delivered, and
    // `should_rotate` relies on this ordering to apply the longer budget
    // once `proven` is set.
    assert!(
        PROVEN_SILENCE_MS > SILENCE_MS,
        "a proven exit must be given a strictly longer silence budget than an unproven one"
    );
}

#[test]
fn the_budgets_stay_finite_so_a_dead_exit_is_still_rotated() {
    // Longer budgets must not become "never": an exit that is genuinely gone
    // still has to be walked away from, or the privacy property that traffic
    // only leaves by a working mixnet exit degrades into traffic wedged on a
    // dead one. Both budgets remain ordinary finite silence windows.
    let mut unproven = Watch::new();
    unproven.on_send(1_000);
    assert!(unproven.should_rotate(1_000 + SILENCE_MS), "a dead unproven exit is still rotated");

    let mut proven = Watch::new();
    proven.on_send(1_000);
    proven.on_delivered();
    proven.on_send(2_000);
    assert!(
        proven.should_rotate(2_000 + PROVEN_SILENCE_MS),
        "a proven exit that has truly gone silent is still rotated"
    );
}

#[test]
fn an_exit_that_keeps_delivering_is_never_rotated_however_long_it_runs() {
    // The safety property behind the shared exit: as long as something is
    // coming back through it, the exit is not rotated and no connection on it
    // is torn down. A delivery on any connection calls `on_delivered`, which
    // clears the silence clock, so a steadily delivering exit never reaches
    // its budget, across a run far longer than either budget. This is why a
    // single healthy connection keeps the shared exit alive for all of them.
    let mut w = Watch::new();
    let mut now = 0i64;
    for _ in 0..1_000 {
        // A send goes out, then a reply comes back one slow round trip later,
        // over and over, for well past PROVEN_SILENCE_MS of wall clock.
        w.on_send(now);
        now += ROUND_TRIP_MS;
        w.on_delivered();
        assert!(
            !w.should_rotate(now),
            "an exit delivering every round trip must never be rotated, at t={now}"
        );
    }
    assert!(now > PROVEN_SILENCE_MS * 10, "the run must outlast the budget by a wide margin");
}

#[test]
fn a_fresh_exit_never_rotates_before_anything_is_sent() {
    let w = Watch::new();
    assert!(!w.should_rotate(0));
    assert!(!w.should_rotate(i64::MAX));
}

#[test]
fn silence_past_the_budget_rotates_and_a_moment_less_does_not() {
    let mut w = Watch::new();
    w.on_send(1_000);
    assert!(!w.should_rotate(1_000 + SILENCE_MS - 1));
    assert!(w.should_rotate(1_000 + SILENCE_MS));
}

#[test]
fn the_budget_runs_from_the_first_send_not_the_newest() {
    let mut w = Watch::new();
    w.on_send(1_000);
    w.on_send(1_000 + SILENCE_MS - 1);
    assert!(w.should_rotate(1_000 + SILENCE_MS), "a later send must not reset the clock");
}

#[test]
fn one_delivery_proves_the_exit_for_good() {
    let mut w = Watch::new();
    w.on_send(1_000);
    w.on_delivered();
    assert!(!w.should_rotate(1_000 + SILENCE_MS * 100));
}

#[test]
fn a_configured_exit_is_never_rotated_away() {
    let mut w = Watch::new();
    w.configured = true;
    w.on_send(1_000);
    assert!(!w.should_rotate(1_000 + SILENCE_MS * 100));
}

#[test]
fn rotation_starts_the_next_exit_with_a_clean_record() {
    let mut w = Watch::new();
    w.on_send(1_000);
    assert!(w.should_rotate(1_000 + SILENCE_MS));
    w.on_rotate();
    assert!(!w.should_rotate(1_000 + SILENCE_MS * 2), "no send yet on the new exit");
    w.on_send(50_000);
    assert!(!w.should_rotate(50_000 + SILENCE_MS - 1));
    assert!(w.should_rotate(50_000 + SILENCE_MS));
}

#[test]
fn uptime_zero_edge_does_not_wedge_the_first_send() {
    // A send stamped at uptime 0 is indistinguishable from "never sent",
    // so the watch treats it as unset; the next send stamps for real. The
    // cost is one extra budget window on a machine that sends in the very
    // first millisecond of its life, never a wedge.
    let mut w = Watch::new();
    w.on_send(0);
    assert!(!w.should_rotate(SILENCE_MS * 2));
    w.on_send(5);
    assert!(w.should_rotate(5 + SILENCE_MS));
}

#[test]
fn an_answer_without_payload_restarts_the_budget_but_proves_nothing() {
    let mut w = Watch::new();
    w.on_send(1_000);
    // The connect is accepted late in the budget, on a slow machine.
    w.on_answered(1_000 + SILENCE_MS - 1);
    assert!(!w.should_rotate(1_000 + SILENCE_MS), "the exit answered: not silent");
    assert!(!w.proven, "an accepted connect is not a delivery");
    // Silent again for a whole budget after its last answer: walked away from.
    assert!(w.should_rotate(1_000 + 2 * SILENCE_MS - 1));
}

#[test]
fn an_answer_before_any_send_starts_no_clock() {
    let mut w = Watch::new();
    w.on_answered(5_000);
    assert!(!w.should_rotate(5_000 + SILENCE_MS * 10));
}

#[test]
fn a_proven_exit_that_goes_quiet_is_walked_away_from_after_the_longer_budget() {
    use crate::watch::PROVEN_SILENCE_MS;
    let mut w = Watch::new();
    w.on_send(1_000);
    w.on_delivered();
    assert!(!w.should_rotate(1_000 + PROVEN_SILENCE_MS * 10), "a delivery ends the silence");
    w.on_send(5_000);
    assert!(!w.should_rotate(5_000 + SILENCE_MS), "not on the unproven budget");
    assert!(!w.should_rotate(5_000 + PROVEN_SILENCE_MS - 1));
    assert!(w.should_rotate(5_000 + PROVEN_SILENCE_MS), "a proven exit no longer holds the session for good");
}
