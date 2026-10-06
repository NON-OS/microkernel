# theme_proofs

Host-runnable proofs for the toolkit's theme and the pieces it paints
with. The real theme source, text scale, protocol and several components,
fonts and design files are included through `#[path]` from
`../toolkit/src/`; `nonos_policy_proto` is a dependency for the theme
names.

| Tests | What they hold |
|---|---|
| `scheme_tests`, `scheme_contrast_tests`, `scheme_accent_tests` | every scheme meets the WCAG 2.1 contrast floors, checked in floating point (`wcag.rs`), which the capsule computes in integers |
| `contrast_tests`, `contrast_edge_tests` | the integer WCAG transfer function agrees with the formula, and the two ends of the range |
| `derive_tests`, `derive_alpha_tests` | mixing two colours, and setting alpha without touching the colour |
| `text_scale_tests`, `text_scale_range_tests` | the text scale steps and their range |
| `toolkit_paint_tests` | component render paints a panel, button or label inside the surface and the rectangle named, with no arithmetic past the top of a u32 |
| `toolkit_refusal_tests` | every frame decodes to its header or draws one whole reply naming its op and request id |

Run: `cargo test` in this directory. The toolkit is described in
`userland/toolkit/README.md`.
