// NONOS Operating System (AGPL-3.0-or-later)
//! A capsule the spawn gate withheld Debug from says so, in the table and
//! in its agent block, naming the secret capabilities it holds; one it did
//! not withhold from says nothing, so the missing bit reads as the gate's
//! and never as a change to the manifest.

use super::model::{entry, kernel_bits};
use crate::command::builtin::audit::agent::agent;
use crate::command::builtin::audit::join::join;
use crate::command::builtin::audit::table::line;
use crate::command::builtin::audit::withheld::{note, SECRET_CAPS};

fn text(b: &[u8]) -> String {
    String::from_utf8(b.to_vec()).unwrap()
}

#[test]
fn a_withheld_debug_is_said_beside_the_capsule() {
    let k = kernel_bits();
    let names: Vec<&str> = SECRET_CAPS.iter().map(|n| core::str::from_utf8(n).unwrap()).collect();
    assert_eq!(names, ["InputSource", "Keyring", "DeviceSecret", "InputConsumer"]);
    let caps = k["IPC"] | k["InputSource"] | k["DeviceSecret"] | k["Irq"];
    assert_eq!(text(&note(caps)), "debug withheld (holds InputSource, DeviceSecret)");
    let regs = [entry(3, caps, 0), entry(4, caps, 0)].concat();
    let rows = join(&regs, &[(3, b"driver.x", true), (4, b"driver.y", false)]).unwrap();
    let (said, quiet) = (text(&line(&rows[0])), text(&line(&rows[1])));
    assert!(said.ends_with("; debug withheld (holds InputSource, DeviceSecret)"), "{said}");
    assert!(!quiet.contains("withheld"), "{quiet}");
    let block: Vec<String> = agent(&rows, b"driver.x").iter().map(|l| text(l)).collect();
    assert!(
        block
            .iter()
            .any(|l| l
                == "  debug      withheld by the kernel, as it holds InputSource, DeviceSecret")
    );
    assert!(!agent(&rows, b"driver.y").iter().any(|l| text(l).contains("withheld")));
    let unsigned = join(&[], &[(9, b"init", true)]).unwrap();
    assert!(text(&line(&unsigned[0])).contains("no signed ceiling"), "no ceiling, no note");
}
