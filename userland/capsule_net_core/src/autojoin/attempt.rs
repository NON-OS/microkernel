//! One pass: is a remembered network due, and if one is in range, join it.
//!
//! Nothing happens unless the policy store says this boot keeps state (it
//! does only once it restored what setup kept, and never in amnesic mode)
//! and the Wi-Fi switch is not off. The saved record is opened with the TPM
//! key; the passphrases are wiped when the list drops at the end of the pass.

use nonos_policy_proto::Field;
use nonos_wifi_client::{find, join_text, load, DriverStage, SavedError, ScanNetwork};

use super::tick::say;

pub enum Step {
    NotYet,
    NoneInRange,
    Tried(u8),
    Done,
}

pub fn attempt(tried: u8) -> Step {
    let Some(policy) = nonos_policy_client::lookup() else { return Step::NotYet };
    let get = |f| nonos_policy_client::get_bool(policy, f);
    if get(Field::Persistent) != Some(true) || get(Field::WifiRadio) == Some(false) {
        return Step::NotYet;
    }
    let Some(driver) = find() else { return Step::NotYet };
    match driver.stage() {
        Some(DriverStage::Ready) => {}
        Some(DriverStage::NoAirPath) => return Step::Done,
        _ => return Step::NotYet,
    }
    if driver.link().is_some_and(|l| l.associated) {
        return Step::Done;
    }
    let list = match load() {
        Ok(list) if list.is_empty() => return Step::Done,
        Ok(list) => list,
        Err(SavedError::NoStore) => return Step::NotYet,
        Err(e) => {
            say(b"[NET-CORE] saved Wi-Fi networks not opened: ");
            say(e.text().as_bytes());
            say(b"\n");
            return Step::Done;
        }
    };
    let mut heard = [ScanNetwork::EMPTY; 16];
    let (count, _, _) = driver.scan(&mut heard);
    let due = (0..list.len()).filter(|i| tried & (1 << i) == 0);
    let mut due = due.peekable();
    if due.peek().is_none() {
        return Step::Done;
    }
    let Some(i) = due.find(|&i| heard[..count].iter().any(|n| n.ssid() == list.ssid(i))) else {
        return Step::NoneInRange;
    };
    let r = driver.connect(list.ssid(i), list.passphrase(i));
    if r.code == 0 {
        say(b"[NET-CORE] joined a saved Wi-Fi network\n");
        return Step::Done;
    }
    say(b"[NET-CORE] saved Wi-Fi network not joined: ");
    say(join_text(r.code).as_bytes());
    say(b"\n");
    Step::Tried(tried | 1 << i)
}
