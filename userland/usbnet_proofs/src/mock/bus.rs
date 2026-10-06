// NONOS Operating System (AGPL-3.0-or-later)
//! Every call a driver makes is recorded; control requests are answered by
//! the test's responder, bulk IN by the queue the test fills.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use super::script::{Call, Responder, Script};
use crate::bus::{Bus, Pipes};
use crate::setup::Setup;

#[derive(Clone)]
pub struct MockBus(pub Rc<RefCell<Script>>);

impl MockBus {
    pub fn new(respond: Responder) -> Self {
        let s = Script { calls: vec![], bulk_in: VecDeque::new(), bulk_out_fails: None, respond };
        Self(Rc::new(RefCell::new(s)))
    }

    fn control(&mut self, call: Call) -> Result<Vec<u8>, i32> {
        let mut s = self.0.borrow_mut();
        s.calls.push(call.clone());
        (s.respond)(&call)
    }
}

impl Bus for MockBus {
    fn control_in(&mut self, setup: Setup, out: &mut [u8]) -> Result<usize, i32> {
        let got = self.control(Call::In(setup, out.len()))?;
        let n = got.len().min(out.len());
        out[..n].copy_from_slice(&got[..n]);
        Ok(n)
    }
    fn control_out(&mut self, setup: Setup, data: &[u8]) -> Result<(), i32> {
        self.control(Call::Out(setup, data.to_vec())).map(|_| ())
    }
    fn configure_bulk(&mut self, pipes: &Pipes) -> Result<(), i32> {
        self.0.borrow_mut().calls.push(Call::Configure(*pipes));
        Ok(())
    }
    fn bulk_out(&mut self, data: &[u8]) -> Result<usize, i32> {
        let mut s = self.0.borrow_mut();
        s.calls.push(Call::BulkOut(data.to_vec()));
        s.bulk_out_fails.map_or(Ok(data.len()), Err)
    }
    fn bulk_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        let Some(next) = self.0.borrow_mut().bulk_in.pop_front() else { return Ok(None) };
        let Some(bytes) = next? else { return Ok(None) };
        let n = bytes.len().min(out.len());
        out[..n].copy_from_slice(&bytes[..n]);
        Ok(Some(n))
    }
    fn reset_bulk(&mut self, dir_in: bool) -> Result<(), i32> {
        self.0.borrow_mut().calls.push(Call::ResetBulk(dir_in));
        Ok(())
    }
}
