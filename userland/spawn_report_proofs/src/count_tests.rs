// NONOS Operating System (AGPL-3.0-or-later)
//! The boot's failure count and the line init ends its spawns with. The count
//! is one static for the whole process, so this is one test, step by step.

use crate::console::take;
use crate::failures::{failed, note};
use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;
use crate::security::capsule_manifest::ManifestVerifyError;
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::spawn_report::report;

const MARKER: &str = "[INIT] Capsules spawned";

#[test]
fn the_marker_is_printed_only_when_no_capsule_failed() {
    take();
    report();
    assert_eq!(take(), format!("{MARKER}\n"), "no failure: the marker, alone");

    // A capsule this image was built without, or one its profile refuses, is
    // a choice and leaves the count alone.
    note(SpawnError::FeatureDisabled);
    note(SpawnError::ProfileRefused);
    assert_eq!(failed(), 0);
    report();
    assert_eq!(take(), format!("{MARKER}\n"));

    // Verification refusals count.
    note(SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy));
    note(SpawnError::ManifestRejected(ManifestVerifyError::PayloadHash));
    note(SpawnError::AttestationRejected);
    assert_eq!(failed(), 3);
    // So do the failures after verification: loading, the address space,
    // the endpoint, the inbox, the ceiling.
    for e in [
        SpawnError::ElfLoad,
        SpawnError::ProcessCreation,
        SpawnError::AddressSpace,
        SpawnError::EndpointCollision,
        SpawnError::InboxName,
        SpawnError::ImageCeiling,
    ] {
        note(e);
    }
    assert_eq!(failed(), 9);

    report();
    let out = take();
    assert!(!out.contains(MARKER), "a failed spawn never prints the marker: {out}");
    assert!(out.contains("[INIT] capsule spawn incomplete: 9 failed\n"), "{out}");
    assert!(out.contains("[ERROR] capsule spawn incomplete"), "{out}");
}
