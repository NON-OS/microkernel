// NONOS Operating System (AGPL-3.0-or-later)
//! The PHY's page register, written only when the page changes.

use nonos_usbnet::mock::{Call, MockBus};

use crate::rtl::phy::{phy_read, phy_write};

fn echo(_: u32) -> MockBus {
    MockBus::new(Box::new(|call| match call {
        Call::In(..) => Ok(vec![0; 4]),
        _ => Ok(vec![]),
    }))
}

/// The register each write went to.
fn sent(bus: &MockBus) -> Vec<(nonos_usbnet::Setup, Vec<u8>)> {
    let calls = bus.0.borrow().calls.clone();
    calls
        .into_iter()
        .filter_map(|c| if let Call::Out(s, d) = c { Some((s, d)) } else { None })
        .collect()
}

#[test]
fn the_phy_page_is_chosen_once_per_page_and_again_after_a_failure() {
    let mut bus = echo(0);
    let mut page = None;
    phy_read(&mut bus, &mut page, 0xa420).unwrap();
    phy_write(&mut bus, &mut page, 0xa400, 0x1200).unwrap();
    phy_read(&mut bus, &mut page, 0xbc06).unwrap();
    let pages: Vec<_> = sent(&bus).into_iter().filter(|(s, _)| s.value == 0xe86c).collect();
    assert_eq!(pages.len(), 2, "{pages:?}");
    assert_eq!(page, Some(0xb000));
    let mut broken = MockBus::new(Box::new(|_| Err(-71)));
    assert_eq!(phy_write(&mut broken, &mut page, 0xa400, 0), Err(-71));
    assert_eq!(page, None, "a failed choice leaves the page unknown");
}
