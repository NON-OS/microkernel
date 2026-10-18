// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's pure bring-up rules and wording, mounted from its source,
//! and a reader of the parts they say.

#[path = "../../../../capsule_acpi/src/sleep/shutdown_say.rs"]
pub mod say;
pub use say::Part;
#[path = "../../../../capsule_acpi/src/boot/bringup_say.rs"]
pub mod bringup_say;
#[path = "../../../../capsule_acpi/src/boot/load_rule.rs"]
pub mod load_rule;

pub fn text(write: impl FnOnce(&mut dyn FnMut(Part))) -> String {
    let mut s = String::new();
    write(&mut |p| match p {
        Part::Str(b) => s.push_str(core::str::from_utf8(b).unwrap()),
        Part::Hex(v) => s.push_str(&format!("{v:#x}")),
        Part::Dec(v) => s.push_str(&v.to_string()),
    });
    s
}
