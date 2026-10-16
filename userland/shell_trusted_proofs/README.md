# shell_trusted_proofs

Host-runnable proofs for the desktop shell's half of the trusted path, on the
shell's own rule files (`../capsule_desktop_shell/src/state/`):

| Tests | What they hold |
|---|---|
| `attention_tests` | Ctrl+Alt+Delete opens the system's prompt from anywhere but the hidden screen, the prompt hides the screen or goes back, and only the chord shows it again (`attention.rs`) |
| `band_tests` | the chrome is in the compositor's trusted band exactly while a prompt, the system's prompt or the hidden screen is up, and every move places before it takes (`trusted_rule.rs`) |
| `router_tests` | input is believed from the first router found alone; a window posing as it is refused; once it is gone no later holder of the name is believed (`input_source_rule.rs`) |

Run: `cargo test` in this directory.
