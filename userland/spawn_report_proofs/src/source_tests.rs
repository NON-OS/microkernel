// NONOS Operating System (AGPL-3.0-or-later)
//! Where the count is kept and read, and the boot checks that read the
//! incomplete line as a failure, from the tree's own files.

use crate::source::{code, order};

const INCOMPLETE: &str = "[INIT] capsule spawn incomplete";

#[test]
fn every_boot_spawn_error_is_noted_before_it_is_logged() {
    let run = code("src/userspace/init/capsule_boot/run.rs");
    let boot = &run[..run.find("pub(crate) fn restart").expect("restart")];
    order(
        boot,
        &["match spawn_fn() {", "Err(e) => {", "super::failures::note(e);", "boot_log::error("],
    );
}

#[test]
fn init_ends_its_spawns_with_the_report_and_prints_no_marker_itself() {
    let entry = code("src/userspace/init/entry.rs");
    assert!(!entry.contains("Capsules spawned"), "the marker is the report's to print");
    order(&entry, &["spawn_plan::spawn_apps();", "super::spawn_report::report();", "init_loop()"]);
}

#[test]
fn a_capsule_built_out_of_the_image_is_not_a_failure() {
    let v = code("src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs");
    order(
        &v,
        &[
            "let embedded = [spec.elf, spec.nonos_id_cert_bytes, spec.manifest_bytes];",
            "if embedded.iter().all(|b| b.is_empty()) {",
            "return Err(SpawnError::FeatureDisabled);",
            "preflight::run(",
        ],
    );
}

#[test]
fn every_boot_check_fails_an_incomplete_spawn() {
    let ev = code("scripts/qemu_boot_evidence.sh");
    order(
        &ev,
        &[
            "\\[INIT\\] capsule spawn incomplete",
            "\"status\":\"fail\"",
            "exit 1",
            "\"status\":\"pass\"",
        ],
    );
    let verdict = code("scripts/bootmatrix/verdict.py");
    let fatal = &verdict[verdict.find("FATAL = (").expect("FATAL")..];
    assert!(fatal[..fatal.find(')').unwrap()].contains(INCOMPLETE), "the matrix holds it fatal");
    let runtime = code("nonos-verify/src/runtime.rs");
    order(
        &runtime,
        &["!joined.contains(\"spawn incomplete\")", "rpt.check(\"capsules-spawned\", st(capsules)"],
    );
    let qemu = code("tools/nonos_qemu/__main__.py");
    order(&qemu, &["def watch(", INCOMPLETE, "or 1", "if expect and not pending:"]);
}
