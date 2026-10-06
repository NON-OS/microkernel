# One cargo build in the sandbox: offline, frozen to its lock, against a vendor
# directory holding exactly that lock's crates. dontFixup because these are
# NONOS binaries, and stripping or patching them would change the bytes that
# get measured.
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
  nativeBuildInputs = [ rust pins.python ] ++ nativeBuildInputs;
  dontConfigure = true;
  dontFixup = true;
  dontInstall = true;
  buildPhase = ''
    runHook preBuild
    ${cargo.setup vendor}
    ${script}
    runHook postBuild
  '';
  passthru = { inherit vendor; };
} // extra)
