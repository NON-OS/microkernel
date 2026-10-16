# shell_trusted_proofs

Host-runnable proofs for the desktop shell's half of the trusted path, on the
shell's own rule files (`../capsule_desktop_shell/src/state/`):

| Tests | What they hold |
|---|---|
| `attention_tests` | Ctrl+Alt+Delete opens the system's prompt from anywhere but the lock, the prompt locks or goes back, and only the chord leaves the lock (`attention.rs`) |
| `band_tests` | the chrome is in the compositor's trusted band exactly while a prompt, the system's prompt or the lock is up, every move places before it takes, and input is believed from the router's pid alone (`trusted_rule.rs`, `input_source_rule.rs`) |

Run: `cargo test` in this directory.
