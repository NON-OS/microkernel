// NONOS Operating System (AGPL-3.0-or-later)
//! What the scripted device records and answers with.

use std::collections::VecDeque;

use crate::bus::Pipes;
use crate::setup::Setup;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Call {
    In(Setup, usize),
    Out(Setup, Vec<u8>),
    Configure(Pipes),
    BulkOut(Vec<u8>),
    ResetBulk(bool),
}

/// The responder sees each control call and answers it: the bytes in for
/// `Call::In`, anything for `Call::Out`, or an errno.
pub type Responder = Box<dyn FnMut(&Call) -> Result<Vec<u8>, i32>>;

pub struct Script {
    pub calls: Vec<Call>,
    pub bulk_in: VecDeque<Result<Option<Vec<u8>>, i32>>,
    pub bulk_out_fails: Option<i32>,
    pub respond: Responder,
}
