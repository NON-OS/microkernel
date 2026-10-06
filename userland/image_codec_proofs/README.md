# image_codec_proofs

Host-runnable proofs for the image codec service, on the real source
included through `#[path]`.

- `decode_fuzz_tests`: everything an image's bytes decide
  (`server/handlers/decode_sized.rs`: the size from the header, the output
  buffer and the toolkit decoder) driven with damaged and arbitrary images.

The outputs image_codec holds for its clients (`server/outputs.rs`):

- An output is let go when its client asks for another decode, when its
  client has ended, or 30 s after it was made. Outputs of other living
  clients are kept.
- At most four are held at once, and a fifth lets the oldest go, so no run
  of decodes makes image_codec hold more. Nothing held is lost from the
  books: every output made is either still held or handed back exactly once.

What is not proven here: that the unmap gives the surface slot back (the
kernel's rule for that is held by `kernel_proofs`), that the serve loop
sweeps before each decode, and that `mk_pid_alive` answers truly. Those
need a boot.

Run: `cargo test` in this directory.

The image codec is described in [Audio and media](../../docs/handbook/audio-and-media.md).
