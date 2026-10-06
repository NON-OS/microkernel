# nonos-utils

Small host-side utilities that feed build inputs into NONOS. Nothing
here runs on the target; each script produces an artifact the build
consumes.

`extract_root_cas.py` and `generate_ca_store.py` build the CA store the
TLS stack ships, from a pinned Mozilla bundle rather than whatever the
host happens to trust. `png_to_lz4raw.py` converts boot and wallpaper
imagery into the raw LZ4 frames the kernel decodes without a PNG
parser in the trusted path. `sign_kernel.py` is the minimal reference
signer for the kernel image format, useful when reproducing the
pipeline outside make. `setup-nonos-vm.sh` prepares a host VM for
QEMU-based development.

Python utilities follow the house rule: python3 with argparse, no
shell parsing of structured data.

## License

AGPL-3.0, like the rest of NONOS.
