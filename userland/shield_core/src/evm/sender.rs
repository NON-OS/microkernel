//! Who asks and over what: the route to the node, the network, and the
//! sending address, as one argument for the steps that need all three.

use super::network::Chain;
use crate::net::tor::Tor;

#[derive(Clone, Copy)]
pub(super) struct Sender<'a> {
    pub(super) tor: &'a Tor,
    pub(super) chain: &'a Chain,
    pub(super) from: &'a [u8; 20],
}
