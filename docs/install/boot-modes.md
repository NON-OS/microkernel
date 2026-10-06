# Boot modes

How to pick an entry in the NONOS boot menu, what each of its seven entries changes, and how the kernel learns which one you chose.

## The menu

The loader draws the menu on every boot, from the stick and from an installed disk alike, whenever the firmware gives it a graphics display. Without a display it returns `MenuAction::Timeout`, which boots as `Standard` (`nonos-bootloader/src/bootmenu/run.rs:35-38`, `nonos-bootloader/src/entry/action.rs:28`).

The entries, in order, are the `ENTRIES` table (`nonos-bootloader/src/bootmenu/entries.rs:33-56`):

| Key | Entry | What the menu says about it |
|---|---|---|
| 1 | Standard | Boots when the kernel is signed, attested and current. |
| 2 | Hardened | Standard, and refuses to boot without Secure Boot and a TPM. |
| 3 | Safe Mode | No network, no audio, no optional apps. |
| 4 | Air-Gapped | No network driver or service starts. |
| 5 | Recovery | A terminal and files. No network, no setup. |
| 6 | `Install NØNOS` | Writes to the disk you choose, after you confirm. |
| 7 | Shut down | Power off. |

The default entry is `Standard`, or `Hardened` on an image whose build floor is Hardened (`default_index` in `nonos-bootloader/src/bootmenu/keys.rs:42-47`). A countdown of 10 seconds runs on it (`TIMEOUT_S` in `nonos-bootloader/src/bootmenu/run.rs:31`), and the foot of the menu shows it, for example `STANDARD IN 10 S`. Any key stops the countdown, and the foot then reads `TIMER HELD`.

### Keys

| Key | What it does |
|---|---|
| Up and Down, or `w`, `s`, `k`, `j` | move the selection, wrapping at either end |
| Home or Page Up | the first entry |
| End or Page Down | the last entry |
| `1` to `7` | jump to that entry |
| Enter | start the selected entry |
| Escape, or any other key | hold the countdown |

The keys are `special` and `printable` in `nonos-bootloader/src/bootmenu/input.rs:36-55`, and `apply` in `nonos-bootloader/src/bootmenu/keys.rs:23-35`.

### What the menu tells you before you choose

Above the list the loader shows four facts about this machine: `SECURE BOOT` on or off, `TPM 2.0` measuring or not found, `ROLLBACK` with a TPM counter or none, and the `BUILD FLOOR` (`secure_boot_enabled` and `measured_boot_active` in `nonos-bootloader/src/bootmenu/platform.rs:31-48`).

Under the selected entry it shows one sentence, a line naming the checks the loader makes for that entry, and a verdict: `READY ON THIS MACHINE`, or `REFUSED HERE: NO` followed by what is missing. The names it can list are crypto self-test, signing keys, hardware RNG, Secure Boot, PK, db and TPM 2.0 (`missing` in `nonos-bootloader/src/bootmenu/ready.rs:51-65`). The verdict names what the loader's own checks will refuse, and decides nothing itself.
