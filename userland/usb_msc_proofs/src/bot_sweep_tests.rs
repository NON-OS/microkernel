// NONOS Operating System (AGPL-3.0-or-later)
//! BOT commands over seeded runs against the scripted device, sound and
//! then with one lie in one command: each ends as the rule in
//! bot_sweep/model.rs says, using the answers it says, with a reset only
//! after an error, the bytes that came where they belong, and as the lie
//! must leave it.

use crate::bot_parts::bot::command;
use crate::bot_parts::types::{Data, Disk};
use crate::bot_sweep::build::{Ans, Cmd, Dir};
use crate::bot_sweep::lie::{run as sound_run, tell, Want, LIES};
use crate::bot_sweep::model::{rule, Seen};
use crate::bot_sweep::rng::{rounds, Rng};
use crate::descriptors::MscBinding;
use crate::state::State;
use crate::xhci::{cbw_takes, device, script, Answer};

fn resets() -> usize {
    device(|d| d.controls.iter().filter(|c| (c.0, c.1) == (0x21, 0xFF)).count())
}

/// Each command run on a fresh script, the state carried: what it ended in
/// against the rule, and what the lie wants.
fn run(cmds: &[Cmd], wants: &[Want], at: &str) {
    let b = MscBinding { interface: 0, bulk_in: 0x81, bulk_out: 0x02, ..MscBinding::default() };
    let (disk, mut state, mut ignore) = (Disk::bound(1, 1, &b), State::default(), false);
    for (i, (c, want)) in cmds.iter().zip(wants).enumerate() {
        let to = |a: &Ans| match a {
            Ans::Data(b) => Answer::Data(b.clone()),
            Ans::Zlp => Answer::Zlp,
            Ans::Stall => Answer::Stall,
        };
        script(c.answers.iter().map(to).collect());
        cbw_takes(c.cbw_takes);
        let mut buf = vec![0u8; c.len];
        let data = match c.dir {
            Dir::In => Data::In(&mut buf),
            Dir::Out => Data::Out(&c.data),
            Dir::None => Data::None,
        };
        let result = command(&disk, &mut state, ([c.op; 16], 10), data);
        let used = c.answers.len() - device(|d| d.answers.len());
        let model = rule(c, i as u32 + 1, &mut ignore);
        assert_eq!(Seen { result, used }, model, "{at}, command {i}: the rule");
        assert_eq!(result, *want, "{at}, command {i}");
        assert_eq!(resets(), result.is_err() as usize, "{at}, command {i}: reset");
        if result.is_ok() && c.dir == Dir::In {
            let d = c.delivered;
            assert_eq!(buf[..d], c.data[..d], "{at}, command {i}: the data");
        }
    }
}

#[test]
fn every_sound_run_of_commands_ends_as_its_device_says() {
    for seed in 1..rounds(4_000) {
        let r = &mut Rng::new(seed);
        let (cmds, wants) = sound_run(r, None);
        run(&cmds, &wants, &format!("seed {seed}"));
    }
}

#[test]
fn after_any_lie_each_command_ends_as_the_lie_leaves_it() {
    let mut told = vec![0u32; LIES.len()];
    for seed in 1..rounds(40_000) {
        let (lie, cmds, wants) = tell(&mut Rng::new(seed));
        run(&cmds, &wants, &format!("seed {seed}: {lie:?}"));
        told[LIES.iter().position(|l| *l == lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 2_000, "{lie:?} was told only {n} times");
    }
}
