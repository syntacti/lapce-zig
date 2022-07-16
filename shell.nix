{
  pkgs ?
    import <nixpkgs> {
      crossSystem = {
        config = "aarch64-unknown-linux-gnu";
      };
    },
}:
with pkgs;
  mkShell {
    buildInputs = [
      # Rust
      #pkgs.cargo
      #pkgs.rustc
      pkgs.rustup
      # Shells
      pkgs.zsh

      # to test github actions
      pkgs.act
      # Tools
      pkgs.docker
      pkgs.cargo-audit
      pkgs.nixfmt
      pkgs.cargo-cross
      # Dependencies
      pkgs.cacert
      pkgs.openssl
      pkgs.git
      pkgs.git-lfs
      pkgs.zlib
      pkgs.pkg-config
    ];
    RUST_BACKTRACE = 1;
  }
