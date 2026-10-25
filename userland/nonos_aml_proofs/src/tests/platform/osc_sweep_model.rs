// NONOS Operating System (AGPL-3.0-or-later)
//! The model the _OSC sweep is held to, apart from the code: an answer's
//! words, what a call returns for them, and each refusal by name.

use crate::exec::Fault;
use crate::object::Object;
use crate::platform::OscError;

pub fn words(o: &Object) -> Option<Vec<u32>> {
    let Object::Buffer(b) = o else { return None };
    let b = b.borrow();
    Some(
        b.chunks(4)
            .filter(|c| c.len() == 4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect(),
    )
}

pub fn model(sent: &[u32], got: &Result<Object, Fault>) -> Result<Vec<u32>, &'static str> {
    let w = got.as_ref().map_err(|_| "fault")?;
    let w = words(w).filter(|w| w.len() >= sent.len()).ok_or("shape")?;
    for (bit, why) in [(4, "uuid"), (8, "revision"), (2, "failure")] {
        if w[0] & bit != 0 {
            return Err(why);
        }
    }
    Ok((0..sent.len()).map(|i| if i == 0 { w[0] } else { w[i] & sent[i] }).collect())
}

pub fn named(e: &OscError) -> &'static str {
    match e {
        OscError::Fault(_) => "fault",
        OscError::Shape => "shape",
        OscError::UnrecognizedUuid => "uuid",
        OscError::UnrecognizedRevision => "revision",
        OscError::Failure => "failure",
    }
}
