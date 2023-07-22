{
  description = "lapce-zig";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachSystem ["x86_64-linux" "aarch64-linux" "aarch64-darwin"] (
      system: let
        pkgs = import nixpkgs {inherit system;};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            cargo-watch
            cargo-outdated
            cargo-edit
            cargo-audit
            jo
            rust-analyzer
            rustfmt
            # Rust
            # Shells
            nushell
            # Tools
            cargo-audit
            nixfmt
            # Dependencies
            cacert
            openssl
            git
            git-lfs
            zlib
            pkg-config
          ];
        };
      }
    );
}
