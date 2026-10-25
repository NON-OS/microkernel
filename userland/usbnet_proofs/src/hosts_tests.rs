// NONOS Operating System (AGPL-3.0-or-later)
//! Every USB host searched, fastest first: two modelled hosts, a device on
//! each, both served, and a host that registers late picked up next pass.

use crate::hosts::Hosts;
use crate::hosts_model::{Model, NAMES};
use crate::scan::{search, Book};

#[test]
fn a_device_on_a_later_slower_host_is_found_through_that_host() {
    let mut m = Model::default();
    m.up.push(("driver.xhci0", 10));
    m.devices.insert((10, 1), "storage");
    let mut hosts = Hosts::new(NAMES, Book::default);
    assert_eq!(hosts.find(|n| m.lookup(n)), 1);
    let tablet = |h: &mut Hosts<Book>, m: &Model| {
        search(h, |x| m.ports(x), |x, p| m.try_port(x, p, "tablet"))
    };
    assert_eq!(tablet(&mut hosts, &m), None, "only storage on the xHCI");
    // The OHCI comes up after: port 1 there is not the xHCI's port 1.
    m.up.push(("driver.ohci0", 30));
    m.devices.insert((30, 1), "tablet");
    assert_eq!(hosts.find(|n| m.lookup(n)), 2);
    assert_eq!(tablet(&mut hosts, &m), Some((30, 1)));
}

#[test]
fn two_hosts_with_a_device_each_are_both_served() {
    let mut m = Model::default();
    m.up.extend([("driver.xhci0", 10), ("driver.uhci0", 40)]);
    m.devices.insert((10, 2), "keyboard");
    m.devices.insert((40, 1), "tablet");
    let mut hosts = Hosts::new(NAMES, Vec::new);
    hosts.find(|n| m.lookup(n));
    // As usb_hid's rescan: every host's ports, each device kept by its host.
    hosts.each(|x, bound| bound.extend(m.ports(x).unwrap().iter().map(|p| m.devices[&(x, p.id)])));
    let mut served = Vec::new();
    hosts.each(|x, bound| served.extend(bound.iter().map(|k| (x, *k))));
    assert_eq!(served, [(10, "keyboard"), (40, "tablet")], "fastest host first");
    let mut seen = Vec::new();
    hosts.each(|x, _| seen.push(x));
    assert_eq!(seen, [10, 40], "the EHCI and OHCI never registered");
}
