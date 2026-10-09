# setup_layout_proofs

Host-runnable proofs for the layouts of first-boot setup and the
installer. The installer brand crate's scale, palette, faces, type set,
labels and release text, the compositor's canvas rule
(`compositor/src/setup/canvas_scale.rs`), the setup wizard's layout and
theme, and the installer's UI metrics, text, wrap and full-screen layout
are the real sources, included through `#[path]`. The toolkit,
app_skeleton, `nonos_policy_proto` and `nonos_keymap` are dependencies.

Every display NONOS targets, from 1366 by 768 to 3840 by 2160
(`displays.rs`), is checked against the real Geist and JetBrains Mono
faces:

| Tests | What they hold |
|---|---|
| `scale_tests` | the brand scale and the compositor's half-size canvas never scale the same screen twice |
| `wizard_layout_tests`, `wizard_fit_tests` | nothing in the setup wizard overlaps or leaves the canvas, and every line fits its box |
| `installer_layout_tests`, `installer_fit_tests` | the same for the installer |
| `qwen_default_tests` | the Qwen step over the real pins: for an install the largest Qwen3 that fits, the smallest under QEMU's software CPU; on an amnesic stick only what fits the session's memory (0.5B, 0.6B and 4B by name), starting on none; with no disk nothing |

`literals.rs` and `installer.rs` hold the strings and the module tree the
included files expect.

Run: `cargo test` in this directory. The installer and setup are
described in [Installer](../../docs/handbook/apps/installer.md).
