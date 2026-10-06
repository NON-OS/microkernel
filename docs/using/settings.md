# Settings

Every panel of the Settings window, what each row changes, and how long a change lasts.

## How Settings works

Settings (`app.settings`) is an editor for the [policy store](../overview/glossary.md#policy-store), the service that holds the system's choices (`userland/capsule_settings/README.md`). When the window opens it reads every value it shows. Each change you make is sent to the policy store at once, and on success the desktop shows the notice `settings applied` (`userland/capsule_settings/src/settings/ipc/notify_shell.rs`). If the policy store does not answer, the values shown are defaults, the status strip says `Policy service unavailable - values shown are not stored`, and Settings asks again on its own (`userland/capsule_settings/src/settings/ui/status_bar.rs`).

A row only exists when some code acts on its value. `ALL_FIELDS` lists the fields Settings shows, each with the code that reads it, and the build fails if a row names a field outside that list, or if a listed field has no row (`userland/capsule_settings/src/settings/schema/coverage.rs`, `userland/capsule_settings/src/settings/schema/coverage_listed.rs`).

## Moving around

| Key | What it does |
|---|---|
| `Up`, `Down`, `Home`, `End`, `PgUp`, `PgDn` | Move between rows. |
| `Left`, `Right` | Step a value down or up. |
| `Space`, `Enter` | Switch a toggle, or step a choice. |
| `Tab`, `]` | Next panel. |
| `[` | Previous panel. |
| `Ctrl+V`, `Shift+Insert` | Paste into the field being edited. |
| `Esc` | Close the window. |

Code: `on_event_browsing` in `userland/capsule_settings/src/settings/event/on_event_browsing.rs`. Click the search field to find a row by name. The Wi-Fi panel has keys of its own, listed below.
