# hda_proofs

Host proofs for the Intel HD Audio driver (`capsule_driver_hda`). The crate
includes the shipping source with `#[path]` (`src/controller/`, and
`discover`, `setup`, `handles`, `audio` and the request handlers and refill
of `server`) and runs it against a register window in memory, with
`nonos_devmodel::run` putting a device model on another thread to answer the
rings and `libc_shim` standing in for the broker, recording every call.

## What it proves

179 `#[test]` functions over what the HD Audio specification and Linux's
driver fix in writing. `src/sim.rs` puts codecs on the link: a device thread
reads each command off the CORB and answers it from a description of a real
codec (`proofs/fixtures.rs`: the HP 15s-fq0xxx's Realtek ALC236, an Intel
display codec, QEMU's duplex codec, a mixer-based ALC269), writing answers
into the RIRB with the codec's address as a controller does.

- the full reset, the codecs found after it, and the engines stopped and
  seen stopped before it (`reset_tests`, `lifecycle/stop_tests`); ring size,
  start and order (`corb_tests`, `corb_start_tests`, `order_corb_tests`,
  `order_tests`);
- bring-up and teardown through `setup::run` on QEMU's ICH6 controller
  (`proofs/lifecycle/`): bus mastering off from the claim until the engines
  a dead owner left armed read stopped, never on when the command word or
  BAR0 is refused, and an interrupt refused in every form leaving the
  attempt polled (`claim_tests`, `claim_refused_tests`); every way an
  attempt ends short of playing (an HDMI-only codec, rings or a stream
  refused or out of 32-bit reach, a controller refused after its reset) and
  a playing driver dropped, each read off the broker log: no DMA grant
  unmapped by hand, and the release last, after a Bus Master clear made with
  every engine stopped, the position buffer off and CRST low
  (`attempt_tests`, `attempt_reach_tests`); the order inside the quiesce,
  streams, then rings, then DPLBASE, then CRST, witnessed by a model holding
  one part while the driver waits (`quiesce_order_tests`); a stuck engine, a
  controller held out of reset and a dead one (`teardown_tests`,
  `teardown_fault_tests`, `reset_hold_tests`); the survey of the device list
  (`discover_tests`) and the console lines (`mark_tests`);
- the serving half (`server/runner/`): every query handler answering whole
  from the loop's reply buffer, PCM writes at and past every bound, stream
  start and stop opening and closing the outputs, the tone, the refill under
  any LPIB or position buffer reading with nothing written past the ring,
  underruns counted once, the completion check, and the jack poll following
  the headphones, leaving a dead codec alone through its back-off and
  logging once a codec lost under a stream of jack events (`serve_tests`,
  `pcm_tests`, `stream_serve_tests`, `refill_tests`, `refill_count_tests`,
  `poll_irq_tests`, `jack_poll_tests`, `jack_lost_tests`);
- the verb transport: unsolicited responses set aside, answers taken from
  the codec asked, bounded waits that spin a whole millisecond and sleep
  past it, on a clock set by hand (`wait_spin_tests`), the ring wrap
  (`verb_tests`, `verb_timeout_tests`, `verb_idle_tests`, `verb_ring_tests`,
  `verb_word_tests`, `verb_payload_tests`), and the jack read backing off
  from a codec that stopped answering and giving it up as lost after six
  reads (`jack_schedule_tests`);
- codec discovery at any address, the walk, connection lists in both forms
  with ranges, output planning through mixers and selectors, and the verbs
  that make the ALC236 play: power, Realtek coefficients, input selection,
  amps at 0 dB, pin controls, EAPD, HP GPIOs, the stream join
  (`codec_tests`, `codec_refusal_tests`, `walk_tests`, `conn_tests`,
  `plan_tests`, `program_*tests`, `jack_tests`, `format_tests`); a hostile
  codec graph given up on by name within a budget, a looping one ending as
  no route, and no real codec near the budget (`path_budget_tests`,
  `path_loop_tests`), and no route or amp verb through an input amp past the
  sixteenth (`amp_index_tests`, `amp_route_tests`, `route_sel_tests`);
- the verdict that names a machine that cannot play, Gemini Lake with a
  codec playing, and the vendor tables (`verdict_tests`, `intel_tests`,
  `output_status_tests`);
- the stream descriptor layout, its reset, the BDL and interrupt enable, and
  the position source (`layout_tests`, `bdl_tests`, `stream_tests`,
  `stream_dma_tests`, `stream_irq_tests`, `position_tests`,
  `position_hostile_tests`);
- the interrupt plan, failure reasons and refused request headers
  (`irq_plan_tests`, `reason_tests`, `request_refusal_tests`).

## What it does not prove

Timing, and the loops that wait on IPC for ever (`server/runner/run.rs`,
`status.rs`) and the start that leads to them (`start.rs`): their handlers
and refill are proved, their dispatch is not. The jack poll's give-up after
half a minute is proved on the schedule's own clock; through the poll, only
under jack events, which skip the back-off. Sound out of a real codec needs
a boot.

## Run

```sh
cd userland/hda_proofs && cargo test --offline
```

See [drivers](../../docs/handbook/drivers.md),
[audio and media](../../docs/handbook/audio-and-media.md) and
[proofs](../../docs/handbook/verification/proofs.md).
