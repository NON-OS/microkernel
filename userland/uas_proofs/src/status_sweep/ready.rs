// NONOS Operating System (AGPL-3.0-or-later)
//! A READY told out of turn.

use nonos_uas::DataDir;

use super::build::{iu, Exchange, READ_READY, WRITE_READY};
use super::model::Outcome;
use super::rng::Rng;

/// A READY under the command's tag out of turn: the other way, on a device
/// with streams or a command with no data, or once the data has moved.
pub fn return_ready(r: &mut Rng, x: &mut Exchange) -> Outcome {
    let wrong = match x.dir {
        DataDir::In => WRITE_READY,
        DataDir::Out => READ_READY,
        DataDir::None => r.one_of(&[READ_READY as usize, WRITE_READY as usize]) as u8,
    };
    match x.ius.len() {
        2 if r.range(0, 1) == 0 => x.ius.insert(1, x.ius[0].clone()),
        2 => x.ius[0][0] = wrong,
        _ => x
            .ius
            .insert(0, iu(r.one_of(&[READ_READY as usize, WRITE_READY as usize]) as u8, x.tag, 4)),
    }
    Outcome::Failed(0x02)
}
