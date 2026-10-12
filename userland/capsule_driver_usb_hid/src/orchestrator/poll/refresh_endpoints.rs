use crate::orchestrator::enumerate::{enumerate, scan_hubs};
use crate::xhci::{host, Hosts};

use super::held::Held;

/// Look at every root port and every hub port of every host again: a host
/// registered since is searched, a device plugged in since is bound, one
/// pulled out has its endpoints dropped.
pub(super) fn refresh_endpoints(hosts: &mut Hosts<Held>) {
    hosts.find(host);
    hosts.each(|xhci_port, held| {
        enumerate(xhci_port, &mut held.tries, &mut held.devs);
        scan_hubs(xhci_port, &mut held.devs);
    });
}
