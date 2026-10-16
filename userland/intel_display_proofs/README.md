# intel_display_proofs

Host proofs for the Intel display capsule's engine code
(`capsule_driver_intel_display/src/engine`) and its device id table,
mounted by `#[path]` so the tests run the shipping source.

The engine is proved against `src/model`, a register-level model of a Gen9
display engine written from the same Skylake PRM registers the engine
cites: transcoder timings, TRANS_CONF state, the eDP transcoder's pipe
select, PIPESRC, the primary plane and cursor with their double-buffered
registers armed by PLANE_SURF and CUR_BASE and latched together at a
vertical blank, PIPE_FRMCNT and PLANE_SURFLIVE moving only at that blank,
the GGTT and the flush after it, and a scanout that reads each pixel
through the GGTT out of IOVA pages. Time in the model is the reads the code
makes; a wait's deadline there is a count of checks, where the shipping one
is a clock.

What is proved: the takeover reads only; a lit eDP or HDMI output and its
timing; outputs refused for a stopped transcoder, a dark or tiled plane, or
a timing with sync outside its blanking; a programmed plane appears whole
at the next blank with PLANE_SURF written last; refusals write nothing;
flips return only once the new surface is live and time out on a pipe that
never blanks; GGTT entries, their flush, the table size from GGMS and runs
placed clear of firmware's surface; the cursor's sign-magnitude position
and its arming; and a whole takeover from start through two presents to
the hand-back, which restores firmware's picture and removes the driver's
entries. It proves the code against the PRM as this driver reads it, not
against silicon: booting a Gen9 machine is the owner's step.

DDC/CI (`tests/ddc/`) is proved against `model/mccs.rs`, a monitor's end
written from the message layout and not from the driver: requests byte for
byte against hand-worked checksums, every one-byte change to a reply
refused, the allowlist over all 256 codes, the 40 ms and 50 ms waits and the
second's memory on the model clock, both carriers at 0x37 and nothing else
(GMBUS through the register model, I2C over AUX through `model/mccs_aux.rs`),
and the controls through F 12's bytes on the two-output rig with pipe B on
HDMI.

```sh
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
cargo clippy --release --all-targets -- -D warnings
```
