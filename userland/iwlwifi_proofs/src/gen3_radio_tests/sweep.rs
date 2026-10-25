// NONOS Operating System (AGPL-3.0-or-later)
//! A sweep completes on its own uid and is given up past its budget.


use crate::gen3::sweep::{budget_ms, Sweep, Tick};
use crate::gen3_model::{reply, Out, Seen};
use super::rig::{booted, rig};

fn completion(uid: u32, status: u8) -> Out {
    let mut p = vec![0u8; 16];
    p[0..4].copy_from_slice(&uid.to_le_bytes());
    p[6] = status;
    Out { cmd: 0x0F, group: 0, seq: 0x8000, payload: p }
}

#[test]
fn a_sweep_completes_on_its_own_uid_and_ignores_another() {
    let r = rig();
    r.model.s.borrow_mut().responder = Some(Box::new(|s: &Seen| {
        if (s.group, s.cmd) == (1, 0x0D) {
            vec![reply(s, vec![]), completion(7, 1)]
        } else {
            vec![reply(s, vec![])]
        }
    }));
    booted(&r, |dev, c| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(dev, c, &[1, 6, 11], 0, &mut |_| {}), Ok(Tick::Running));
        assert_eq!(sweep.pump(dev, 5, &mut |_| {}), Tick::Running, "uid 7 is not this driver's scan");
        r.model.queue(completion(0, 1));
        assert_eq!(sweep.pump(dev, 6, &mut |_| {}), Tick::Completed(1));
        assert!(!sweep.running());
        assert_eq!(sweep.pump(dev, 7, &mut |_| {}), Tick::Idle, "nothing in flight");
    });
}

#[test]
fn a_sweep_past_its_budget_is_given_up() {
    let r = rig();
    booted(&r, |dev, c| {
        let mut sweep = Sweep::new();
        let channels = [1u8, 6, 11];
        let budget = u64::from(budget_ms(channels.len()));
        assert_eq!(budget, 3 * 150 + 2000);
        assert_eq!(sweep.start(dev, c, &channels, 1000, &mut |_| {}), Ok(Tick::Running));
        assert_eq!(sweep.pump(dev, 1000 + budget, &mut |_| {}), Tick::Running, "still within budget");
        assert_eq!(sweep.pump(dev, 1001 + budget, &mut |_| {}), Tick::Stalled);
        assert_eq!((sweep.stalled, sweep.completed, sweep.running()), (1, 0, false));
    });
}
