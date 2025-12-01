{ pkgs, lib, config, inputs, ... }:

{
  packages = with pkgs; [
      cargo-nextest
  ];

  languages.rust = {
    enable = true;
    channel = "stable"; # or "nightly"
    components = ["rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-src"];
  };
  enterShell = ''
    RR_TOOLCHAIN_DIR="$XDG_DATA_HOME/rust-rover"
    mkdir -p "$RR_TOOLCHAIN_DIR"

    RUSTC_PATH=$(type -p rustc)
    TOOLCHAIN_ROOT=$(dirname $(dirname $(readlink -f "$RUSTC_PATH")))
    ln -sfn "$TOOLCHAIN_ROOT" "$RR_TOOLCHAIN_DIR/toolchain"

    echo "🔗 RustRover toolchain linked to: $RR_TOOLCHAIN_DIR/toolchain"
    echo "🔗 RustRover standard library linked to: $RR_TOOLCHAIN_DIR/toolchain/lib/rustlib/src/rust"
  '';
}
