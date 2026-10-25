// NONOS Operating System (AGPL-3.0-or-later)
//! The thread trampoline reads what the spawner writes, and the layer's
//! install copies and declares both files that hold its registers.

use crate::std_trap_tests::{read, OWNERS, STD};

#[test]
fn the_trampoline_reads_what_the_spawner_writes() {
    let regs = read(OWNERS[1]);
    for arm in [
        "\"mov rdi, [rsp + 8]\", \"call qword ptr [rsp]\"",
        "\"ldr x0, [sp, #8]\", \"ldr x16, [sp]\", \"blr x16\"",
        "\"ld a0, 8(sp)\", \"ld t0, 0(sp)\", \"jalr t0\"",
    ] {
        assert!(regs.contains(arm), "trampoline: {arm}");
    }
    let spawn = read("sys/thread/nonos.rs");
    assert!(spawn.contains("let entry_sp = top - 16;"));
    assert!(spawn.contains("(entry_sp as *mut u64).write(start as usize as u64);"));
    assert!(spawn.contains("((top - 8) as *mut u64).write((data as usize) as u64);"));
    assert!(spawn.contains("thread_spawn(entry, entry_sp)"));
}

#[test]
fn the_layer_installs_every_pal_file() {
    let apply = read("apply.sh");
    let pal = read("sys/pal/nonos/mod.rs");
    let dir = std::fs::read_dir(format!("{STD}/sys/pal/nonos")).unwrap();
    let mut names: Vec<String> =
        dir.map(|e| e.unwrap().file_name().into_string().unwrap()).collect();
    names.sort();
    assert!(OWNERS.iter().all(|o| names.iter().any(|n| o.ends_with(n.as_str()))));
    for name in names {
        let from = format!("cp \"$HERE/sys/pal/nonos/{name}\"");
        let to = format!("\"$SYS/pal/nonos/{name}\"");
        let copied = apply.lines().any(|l| l.trim_start().starts_with(&from) && l.contains(&to));
        assert!(copied, "apply.sh does not install sys/pal/nonos/{name}");
    }
    for name in ["trap", "thread_regs"] {
        assert!(pal.contains(&format!("pub mod {name};")), "mod.rs: {name}");
    }
}
