//! Step 5, the firmware's log: replayed, it must give exactly the quoted
//! values of the PCRs firmware extends. In PCR 4 every application must be
//! the release's loader or a boot application the policy lists, and the
//! loader must be among them. Which one ran last is not enough: an
//! earlier application could load the loader and never start it.

use crate::consts::REPLAYED_PCRS;
use crate::log::{replay, Replayed};
use crate::{Doc, Policy, Refusal, Release};

pub fn log(doc: &Doc<'_>, release: &Release, policy: &Policy) -> Result<Replayed, Refusal> {
    let r = replay(doc.log).map_err(Refusal::Log)?;
    for i in REPLAYED_PCRS {
        if doc.pcr(i) != r.pcrs.get(i as usize) {
            return Err(Refusal::Replay(i));
        }
    }
    let listed = |a: &[u8; 32]| *a == release.loader || policy.boot_application(a);
    if !r.applications.iter().all(listed) {
        return Err(Refusal::BootApplication);
    }
    match r.applications.contains(&release.loader) {
        true => Ok(r),
        false => Err(Refusal::Loader),
    }
}
