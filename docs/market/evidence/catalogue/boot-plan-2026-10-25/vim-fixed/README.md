# vim on the fixed image (install-oom 521c94528)

Booted on qemu-dev, against the relay on 10.0.2.2:8080, with steps in drive.steps.

- The install role's heap is the install heap, 320 MiB. Every package
  reports `of 327680 KiB` (serial.log). (booted)
- vim-common's 8 MB apk unpacked whole, at a peak of 51158 KiB: `[LINUX] heap
  unpacked vim-common: used=38865 peak=51158 of 327680 KiB`. The out-of-memory
  failure is gone. (booted)
- The install then failed on store names, not memory. 1348 files were "not
  written", and the 455 already written were taken out (`[LINUX] install
  failed, 1348 file(s) not written: took out 455 of 455 file(s), 0 would not
  go`). vim-common holds about 1800 files. capsule_vfs gives each owner 512
  names (NAMES_PER_OWNER = MAX_FILES / 4) and 2048 in all
  (userland/capsule_vfs/src/store/fdtable/budget.rs:42, types.rs:20).
  (booted; host-read)
- Nothing of the failed package stayed. (booted)

The next unit is the store's name budget for an install, or an install that
lands only the runtime files a listing needs. The maintainers should rule on which.
(design)
