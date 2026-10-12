// NONOS Operating System (AGPL-3.0-or-later)
//! Each host keeps its own port book, and a host that never registers
//! costs nothing but its lookup.

use crate::hosts::Hosts;
use crate::hosts_model::{Model, NAMES};
use crate::scan::{search, Book, Port, Tried};

#[test]
fn a_port_given_up_on_one_host_stays_open_on_another() {
    let mut m = Model::default();
    m.up.extend([("driver.xhci0", 10), ("driver.ehci0", 20)]);
    m.devices.insert((10, 1), "camera");
    m.devices.insert((20, 1), "camera");
    let mut hosts = Hosts::new(NAMES, Book::default);
    hosts.find(|n| m.lookup(n));
    let mut tried = Vec::new();
    let got = search(
        &mut hosts,
        |x| m.ports(x),
        |x, p| {
            tried.push((x, p));
            Tried::<()>::NotOurs
        },
    );
    assert_eq!((got, tried), (None, vec![(10, 1), (20, 1)]));
    // Each host's book decided its own port 1; a reconnect on one reopens
    // only that one.
    let again = |h: &mut Hosts<Book>, changed: u32| {
        let mut seen = Vec::new();
        let ports = |x: u32| Some(vec![Port { id: 1, owner: 0, changed: x == changed }]);
        search(h, ports, |x, p| {
            seen.push((x, p));
            Tried::<()>::Busy
        });
        seen
    };
    assert_eq!(again(&mut hosts, 20), [(20, 1)]);
}

#[test]
fn a_host_that_never_registers_costs_only_its_lookup() {
    let mut hosts = Hosts::new(NAMES, Book::default);
    let mut asked = Vec::new();
    assert_eq!(
        hosts.find(|n| {
            asked.push(n.to_string());
            None
        }),
        0
    );
    assert_eq!(asked, NAMES);
    let visited = search(&mut hosts, |_| panic!("no host to list"), |_, _| Tried::<()>::Busy);
    assert_eq!(visited, None);
}
