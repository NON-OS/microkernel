use crate::orchestrator::enumerate::enumerate;
use crate::xhci::{host, Hosts};

use super::held::Held;

/// Look at every port of every host again, root and virtual: a host
/// registered since is searched, a device plugged in since is bound, one
/// pulled out has its endpoints dropped.
pub(super) fn refresh_endpoints(hosts: &mut Hosts<Held>) {
    hosts.find(host);
    hosts.each(|xhci_port, held| {
        enumerate(xhci_port, &mut held.tries, &mut held.devs);
    });
}
