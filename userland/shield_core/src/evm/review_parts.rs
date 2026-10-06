//! The pieces of a review: what is asked for, and what comes back.

use super::network::Chain;
use super::nox;
use super::tx::Eip1559;
use crate::net::asset::Coin;

/// A send as the user asked for it, amounts in base units.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Order {
    pub coin: Coin,
    pub to: [u8; 20],
    pub amount: u128,
}

/// What reaches the recipient, and the token's cut.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Arrival {
    pub arrives: u128,
    pub fee: u128,
    pub bps: u16,
}

/// A send ready to sign, and what it will do.
pub struct Review {
    pub tx: Eip1559,
    pub arrival: Arrival,
    /// Gas limit times the fee ceiling: the most the network can charge.
    pub max_network_fee: u128,
    pub to_is_contract: bool,
}

pub enum Checked {
    Ready(Review),
    Refused(&'static str),
}

/// Where the transaction goes, what it carries and its value.
pub(super) struct Plan {
    pub target: [u8; 20],
    pub value: u128,
    pub data: Vec<u8>,
}

impl Plan {
    pub(super) fn of(chain: &Chain, order: &Order) -> Plan {
        match chain.token(order.coin) {
            None => Plan { target: order.to, value: order.amount, data: Vec::new() },
            Some(token) => {
                Plan { target: token, value: 0, data: nox::transfer(&order.to, order.amount) }
            }
        }
    }
}
