# hda_proofs

Host proofs for the Intel HD Audio driver (`capsule_driver_hda`). The crate
includes the shipping controller source with `#[path]` (`src/controller/`) and
runs it against a register window in memory, with `nonos_devmodel::run`
putting a device model on another thread to answer the rings.

## What it proves

105 `#[test]` functions over what the HD Audio specification and Linux's
driver fix in writing. `src/sim.rs` puts codecs on the link: a device thread
reads each command off the CORB and answers it from a description of a real
codec (`proofs/fixtures.rs`: the HP 15s-fq0xxx's Realtek ALC236, an Intel
display codec, QEMU's duplex codec, a mixer-based ALC269), writing answers
into the RIRB with the codec's address as a controller does.

- the full reset, the codecs found after it, and the engines stopped before
  it (`reset_tests`); ring size, start and order (`corb_tests`,
  `corb_start_tests`, `order_corb_tests`, `order_tests`);
- the verb transport: unsolicited responses set aside, answers taken from the
  codec asked, bounded waits, the ring wrap (`verb_tests`,
  `verb_timeout_tests`, `verb_ring_tests`, `verb_word_tests`,
  `verb_payload_tests`);
- codec discovery at any address, the walk, connection lists in both forms
  with ranges, output planning through mixers and selectors, and the verbs
  that make the ALC236 play: power, Realtek coefficients, input selection,
  amps at 0 dB, pin controls, EAPD, HP GPIOs, the stream join
  (`codec_tests`, `codec_refusal_tests`, `walk_tests`, `conn_tests`,
  `plan_tests`, `program_tests`, `jack_tests`, `format_tests`);
- the verdict that names a machine that cannot play, Gemini Lake with a codec
  playing, and the vendor tables (`verdict_tests`, `intel_tests`,
  `output_status_tests`);
- the stream descriptor layout, its reset, the BDL and interrupt enable, and
  the position source (`layout_tests`, `bdl_tests`, `stream_tests`,
  `stream_dma_tests`, `stream_irq_tests`, `position_tests`);
- the interrupt plan, failure reasons and refused request headers
  (`irq_plan_tests`, `reason_tests`, `request_refusal_tests`).

## What it does not prove

Timing, and the halves of the capsule that talk to the broker rather than to
registers (`discover`, `setup`, `server`, `handles`). Sound out of a real
codec needs a boot.

## Run

```sh
cd userland/hda_proofs && cargo test --offline
```

See [drivers](../../docs/handbook/drivers.md),
[audio and media](../../docs/handbook/audio-and-media.md) and
[proofs](../../docs/handbook/verification/proofs.md).
