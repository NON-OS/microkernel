use super::rule::{unlogged, DEBUG, SECRETS};

const ROOT: &str = "../..";

/// Every capsule manifest's widest grant, required with optional, as a
/// standard image's kernel sets it: none can log and also see a secret.
#[test]
fn no_standard_capsule_logs_and_sees_a_secret() {
    let (mut seen, mut both) = (0, 0);
    for d in std::fs::read_dir(format!("{ROOT}/userland")).unwrap().flatten() {
        let Ok(mk) = std::fs::read_to_string(d.path().join("Capsule.mk")) else { continue };
        let get = |k: &str| {
            mk.lines()
                .find(|l| l.starts_with(k))
                .and_then(|l| l.split(":=").nth(1))
                .and_then(|v| u64::from_str_radix(v.trim().trim_start_matches("0x"), 16).ok())
                .unwrap_or(0)
        };
        let widest = get("CAPSULE_REQUIRED_CAPS") | get("CAPSULE_OPTIONAL_CAPS");
        both += usize::from(widest & DEBUG != 0 && widest & SECRETS != 0);
        let std = unlogged(widest, false);
        assert!(std & DEBUG == 0 || std & SECRETS == 0, "{:?}", d.path());
        seen += 1;
    }
    let gate = std::fs::read_to_string(format!(
        "{ROOT}/src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs"
    ))
    .unwrap();
    assert!(gate.contains("secret_log::unlogged(caps, cfg!(feature = \"nonos-dev-attest\"))"));
    assert!(gate.contains("unlogged(caps, cfg!(feature = \"nonos-dev-attest\")) != caps"));
    let install = std::fs::read_to_string(format!(
        "{ROOT}/src/kernel_core/process_spawn/capsule_spawn/runner/install/install_caps.rs"
    ))
    .unwrap();
    assert!(install.contains("if super::super::profile_gate::withholds_debug(caps_bits) {"));
    assert!(install.contains("proc_caps::mark_debug_withheld(pid);"));
    println!("{seen} manifests, {both} asking for Debug beside a secret, all withheld");
    assert!(seen >= 130 && both >= 6, "{seen} {both}");
}
