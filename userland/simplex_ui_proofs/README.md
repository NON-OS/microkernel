# simplex_ui_proofs

Host-runnable proofs for the SimpleX chat window, with no compositor and no
core. The capsule's own `src/proto/`, `src/model/` and `src/link/` are
mounted through `#[path]` under the names the capsule gives them.

| Tests | What they hold |
|---|---|
| `json_tests` | the JSON reader's escapes, surrogate pairs, nesting bound and refusals, and the writer reading back |
| `proto_tests` | each command as the API writes it, the frame, and the recorded chat list's names, last items and unread counts |
| `resp_tests` | a chat's items in order, address and invitation links in both shapes, errors and unread frames |
| `model_tests` | the list opens and reads the newest chat; new items count unread and move their chat up; status updates |
| `key_tests` | Up, Down, Tab, Enter, Esc and Ctrl+K search from the keyboard alone |
| `flow_tests` | contact requests accepted from the compose line, its commands, and what Ctrl+C copies |
| `transcript_tests` | the capsule's loop over the recorded session: a send echoes, then is delivered |

Run: `cargo test` in this directory.
