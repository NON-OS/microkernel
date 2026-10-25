// NONOS Operating System (AGPL-3.0-or-later)
//! Who reads a CPU index on aarch64: one strict roster index; a secondary
//! checked, then registered at it; every generic per-CPU table fed it;
//! tree-ordered data reached by tree position (the hardware id: hw_id_tests).

use std::fs;

pub(crate) fn read(path: &str) -> String {
    let src = fs::read_to_string(format!("../../src/{path}")).unwrap();
    src.lines().map(|l| l.split("//").next().unwrap_or("").trim()).collect::<Vec<_>>().join("\n")
}

fn before(src: &str, first: &str, then: &str) -> bool {
    matches!((src.find(first), src.find(then)), (Some(a), Some(b)) if a < b)
}

#[test]
fn one_strict_index() {
    let id = read("arch/aarch64/cpu/id/index.rs");
    assert!(id.contains("None if roster::len() == 0 => 0,"));
    assert!(id.contains("None => park_outside_roster(affinity),"));
    let smp = read("smp/cpu_id.rs");
    assert!(smp.contains("crate::arch::cpu::dense_cpu_index().or_else(|| apic_to_cpu_id(apic_id))"));
    let dense = read("arch/cpu/dense_cpu_index.rs");
    assert!(dense.contains(
        "#[cfg(target_arch = \"aarch64\")]\nreturn Some(crate::arch::aarch64::cpu::cpu_id());"
    ));
    let park = read("arch/aarch64/boot/multicore/outside.rs");
    assert!(park.contains("loop {") && park.contains("msr daifset, #0xf"));
    assert!(!read("arch/aarch64/boot/multicore/roster.rs").contains("static mut"));
}

#[test]
fn a_secondary_is_checked_then_registered_at_its_index() {
    let entry = read("arch/aarch64/boot/multicore/ap_entry.rs");
    let check = "let Some(index) = roster::index_of(cpu::mpidr_affinity()) else {";
    assert!(before(&entry, check, "security::pac::secondary_key()"));
    assert!(before(&entry, check, "cpu::percpu::install_here();"));
    let register = "crate::smp::register_secondary(index, cpu::cpu_affinity());";
    assert!(before(&entry, register, "CPUS_ONLINE.fetch_add(1, Ordering::AcqRel);"));
    let reg = read("smp/register_secondary.rs");
    assert!(reg.contains("let Some(descriptor) = CPU_DESCRIPTORS.get(index) else {"));
    assert!(before(
        &reg,
        "descriptor.set_identity(index as u32, hw_id, 0);",
        "set_state(CpuState::Online)"
    ));
}

#[test]
fn every_generic_table_takes_the_dense_index_on_aarch64() {
    let api = read("process/scheduler/api.rs");
    assert!(api.contains("dense.map_or_else(crate::arch::cpu::get_cpu_id, |index| index as u32)"));
    let pending = read("kernel_core/process_spawn/pending_stack_free.rs");
    assert!(pending.contains(
        "crate::arch::cpu::dense_cpu_index().unwrap_or(Arch::current_cpu_id() as usize)"
    ));
    for file in ["context/current.rs", "interrupts/safety/context.rs"] {
        let src = read(file);
        assert!(src.contains("crate::arch::cpu::dense_cpu_index().unwrap_or(0)"), "{file}");
    }
    let restored = read("arch/aarch64/context/restored.rs");
    assert!(restored.contains("crate::arch::aarch64::cpu::cpu_id()\n}"));
}

#[test]
fn tree_ordered_data_goes_through_the_tree_position() {
    let one = read("arch/aarch64/boot/multicore/start_one.rs");
    assert!(
        one.contains("roster::tree_index(index).and_then(|at| info.cpu_release.get(at).copied())")
    );
    let idle = read("arch/aarch64/idle_states/enter.rs");
    assert!(idle.contains("tree.cpus.get(roster::tree_index(cpu)?)?"));
}
