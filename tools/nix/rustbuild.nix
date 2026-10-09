# One cargo build in the sandbox: offline, frozen to its lock, against a vendor
# directory holding exactly that lock's crates. dontFixup because these are
# NONOS binaries, and the blanket strip a fixup would run changes the bytes that
# get measured. The one section a fixup is not trusted with but that must go is
# .comment: rust-lld writes its own provenance there, and that string is the
# LLVM source path on a Linux toolchain but the LLVM source URL on a macOS one,
# so the same NONOS target built on the two hosts differed byte for byte for no
# reason that is loaded or run. postBuild below removes exactly that section,
# deterministically, from every ELF the build emits, which is what lets two
# independent builds on unlike hosts name the same artifacts by hash.
{ pkgs, lib, pins, cargo }:
{
  name,
  src,
  lockFiles,
  script,
  buildStd ? false,
  rust ? pins.rust,
  nativeBuildInputs ? [ ],
  # Host tools link with the host C toolchain; NONOS targets link with
  # rust-lld and need none.
  cc ? true,
  extra ? { },
}:
let
  vendor = cargo.vendor { inherit name lockFiles buildStd; };
in
(if cc then pkgs.stdenv else pkgs.stdenvNoCC).mkDerivation ({
  inherit name src;
  nativeBuildInputs = [ rust pins.python pins.llvm.llvm ] ++ nativeBuildInputs;
  dontConfigure = true;
  dontFixup = true;
  dontInstall = true;
  buildPhase = ''
    runHook preBuild
    ${cargo.setup vendor}
    ${script}
    runHook postBuild
  '';
  # Strip the host-dependent .comment from every ELF the script left in $out, so
  # the measured bytes are the same whichever host built them. llvm-objcopy is
  # the same tool on either host (from pins), the magic test skips anything that
  # is not an ELF (a macOS host tool is Mach-O and is left alone), and removing a
  # non-allocated section changes nothing that is mapped or executed.
  postBuild = ''
    if [ -d "$out" ]; then
      find "$out" -type f 2>/dev/null | while IFS= read -r f; do
        if [ "$(od -An -N4 -tx1 "$f" 2>/dev/null | tr -d ' \n')" = "7f454c46" ]; then
          llvm-objcopy --remove-section .comment "$f" "$f"
        fi
      done
    fi
  '';
  passthru = { inherit vendor; };
} // extra)
