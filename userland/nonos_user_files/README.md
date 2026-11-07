# nonos_user_files

The user-files broker's logic, no_std: the person's Documents, Downloads,
Music, Pictures and Videos
as a tree of chunked files over numbered objects, committed by alternating
manifest slots, and the wire clients ask it over. capsule_user_files runs it;
capsule_linux and native apps reach it through `wire::client`.

- **Paths** are components below the root joined by `/`. Empty, `.`, `..`,
  NUL, a `/` inside a name, or more than 255 bytes is refused before the tree
  is read (`path.rs`). The root holds `Documents` and `Downloads` only.
- **Files** are 1 MiB chunks, each an object written whole, 0 for a hole.
  Changed chunks wait in RAM, at most 16 across all files (`chunks/`).
- **Commits** write the changed chunks, the manifest body, then one of two
  slots, each sealed by BLAKE3 (`manifest/`, `store/`). Rename and fsync
  commit before they answer; any other change within 500 ms of the monotonic
  clock. A start takes the newest whole slot, removes what it said to, and
  removes objects past its next number, which only a cut save leaves.
- **Errors** are Linux errno values; the quota is EDQUOT, a full volume ENOSPC.

Tests: `userland/user_files_proofs`.
