# nonos_keymap

Keyboard layout tables and the resolver that turns a key into a character,
shared by the input drivers, the setup wizard and the proofs. `no_std`, no
dependencies.

## API

- `Layout`: `Us`, `Uk`, `De`, `Fr`, `Es`, `It`, with `index`,
  `from_index` and `next`.
- `resolve(base, shift, caps, altgr, layout) -> u32`: `base` is the US
  character the key produces unshifted. The AltGr table is tried first,
  then letters (cased by Shift xor Caps Lock), then symbols. On French
  AZERTY the US `;` position gives `m` and the US `m` position gives `,`
  and `?`. Values outside printable ASCII pass through unchanged.
- `KEY_ISO` (0xE100) and `iso(layout, shift, altgr)`: the extra key left
  of Z on ISO boards. US has nothing there.
- `POLICY_LAYOUTS` and `Layout::from_policy`: the policy store's keyboard
  layout indices that have a table here; any other index is `None`.
- `HeldKeys` and `KeyPosts`: the code each held key went down with.
  `event(key, is_release, resolved)` says what a key event posts: a release
  carries its press's code whatever the modifiers and layout resolve it to
  by then, and a repeat that resolves to another code first releases the
  old one. A press held with code 0 (the layout chord the drivers consume)
  posts no release.

## Tables

`src/tables/{us,uk,de,fr,es,it}.rs` hold the letter and symbol tables;
`src/tables/altgr/` the AltGr level for German, French, Spanish and
Italian.

## Users

`capsule_driver_ps2_input`, `capsule_driver_usb_hid`,
`capsule_setup_wizard`, and the host proof crates `input_proofs`,
`setup_layout_proofs` and `capsule_browser_proofs`.

## Tests

`tests/italian.rs`, `tests/levels.rs` and `tests/policy.rs` run with
`cargo test` in this directory. The drivers are described in
[Drivers](../../docs/handbook/drivers.md).
