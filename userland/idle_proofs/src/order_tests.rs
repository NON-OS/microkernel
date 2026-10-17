// NONOS Operating System (AGPL-3.0-or-later)
//! The idle path's order, read from the kernel's source: CPU_SUSPEND entered
//! masked, a refusal remembered, the wake taken behind an ISB, every caller
//! masked going in, and the format known before any state is used.

const ENTER: &str = include_str!("../../../src/arch/aarch64/idle_states/enter.rs");
const PREPARE: &str = include_str!("../../../src/arch/aarch64/idle_states/prepare.rs");
const FORMAT: &str = include_str!("../../../src/arch/aarch64/idle_states/psci_format.rs");
const CALL: &str = include_str!("../../../src/arch/aarch64/idle_states/call.rs");
const IDLE: &str = include_str!("../../../src/arch/idle.rs");
const IDLE_CPU: &str = include_str!("../../../src/arch/cpu/idle_cpu.rs");

fn code(src: &str) -> String {
    let lines = src.lines().map(|l| l.split("//").next().unwrap_or("").trim());
    lines.filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n")
}

fn after<'a>(src: &'a str, mark: &str) -> &'a str {
    &src[src.find(mark).unwrap_or_else(|| panic!("no `{mark}`")) + mark.len()..]
}

#[test]
fn suspend_is_entered_masked_and_its_wake_taken() {
    let wait = code(after(ENTER, "pub fn wait()").split("\nfn chosen").next().unwrap());
    let suspend = after(&wait, "return;");
    let (before, rest) = suspend.split_once("if enter_state(&chosen).is_err()").expect("no entry");
    assert!(!before.contains("daifclr"), "IRQs opened before CPU_SUSPEND");
    let call = code(CALL);
    assert!(
        call.contains("psci::cpu_suspend(") && call.contains("powerdown::enter(cpu, state.param)")
    );
    assert!(!call.contains("daif"), "IRQs opened around the call");
    let (refusal, tail) = rest.split_once("unsafe { asm!(\"wfi\"").expect("no WFI on refusal");
    assert!(refusal.contains("fetch_or("));
    let tail = after(tail, "}");
    assert!(tail.contains(r#"asm!("msr daifclr, #2", "isb", "msr daifset, #2""#), "{tail}");
}

#[test]
fn every_caller_waits_masked() {
    let arm = after(IDLE, "#[cfg(target_arch = \"aarch64\")]");
    let arm = arm.split("#[cfg(").next().unwrap();
    assert!(code(arm).contains("crate::arch::aarch64::idle_states::wait();"));
    let cpu = code(after(IDLE_CPU, "#[cfg(target_arch = \"aarch64\")]"));
    let cpu = cpu.split("#[cfg(").next().unwrap();
    let mask = cpu.find("msr daifset, #2").expect("idle_cpu does not mask");
    let wait = cpu.find("idle_states::wait()").expect("idle_cpu does not wait");
    let open = cpu.rfind("msr daifclr, #2").expect("idle_cpu does not reopen");
    assert!(mask < wait && wait < open);
}

#[test]
fn the_format_is_known_before_any_state_is_used() {
    let prepare = code(after(PREPARE, "pub fn prepare()"));
    assert_eq!(prepare.matches("EXTENDED.call_once").count(), 1);
    let ok = prepare.find("Ok(extended) =>").expect("no Ok arm");
    assert!(prepare.find("EXTENDED.call_once").unwrap() > ok);
    let format = code(FORMAT);
    let version = format.find("(version.major, version.minor) < (0, 2)").expect("no 0.2 floor");
    let features = format.find("cpu_suspend_features()").unwrap();
    assert!(version < features && format.contains("flags & 0b10 != 0"));
    assert!(code(ENTER).contains("let extended = *EXTENDED.get()?;"));
    assert!(code(ENTER).contains("budget_us(), allow(cpu))?;"));
}
