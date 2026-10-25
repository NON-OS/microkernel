# shell_trusted_proofs

Host-runnable proofs for the desktop shell's half of the trusted path, on the
shell's own rule files (`../capsule_desktop_shell/src/state/`):

| Tests | What they hold |
|---|---|
| `attention_tests` | Ctrl+Alt+Delete opens the system's prompt from anywhere but the hidden screen, the prompt hides the screen or goes back, and only the chord shows it again (`attention.rs`) |
| `band_tests` | the chrome is in the compositor's trusted band exactly while a prompt, the system's prompt or the hidden screen is up, and every move places before it takes (`trusted_rule.rs`) |
| `router_tests` | input is believed from the router the registry names alone; a window posing as it is refused; a restarted router is believed and its old pid is not; the router's token comes only with a frame it believes (`input_source_rule.rs`) |
| `power_off_tests` | a power off is sent only on the router's Power Off to a confirmation that is up, at most once per confirmation, over every sequence of up to ten asks and answers; its failure fits a toast (`power_off.rs`) |
| `power_off_frame_tests` | the 20-byte request to `power.off`, and that no answer is ever read as a power off (`../power_off_client/frame.rs`) |
| `sealed` (doctests) | the router's token and the permission to send cannot be built outside their modules, and the permission is used up by one send |

Run: `cargo test` in this directory.
