//! `RoleStore`: the table under the pinned key and no older than the host
//! last accepted, the list under that table and never older than the one
//! it replaces; a failed install keeps the list held; and a store that can
//! no longer vouch fails every session closed.

use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::run::Run;
use nsh_roles::PolicyError;
use nsh_server::{EndReason, Event as S, Why};

#[test]
fn a_store_that_cannot_vouch_stops_every_session() {
    let table = Table::shipped();
    let handle = table.clone();
    let mut run = Run::new(&GrantB::default(), table);
    run.pump();
    handle.fail.set(Some(PolicyError::Signature));
    run.host(|s, now, r| s.tick(now, r));
    run.host(|s, now, r| s.tick(now, r));
    assert_eq!(run.s_events.iter().filter(|e| matches!(e, S::Stop(_))).count(), 1);
    assert!(run.s_events.contains(&S::Stop(EndReason::Host)));
    assert_eq!(run.refused(), [Why::Policy(PolicyError::Signature)], "logged once");
}
