{
  description = "Rust development environment for Iced GUI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        runtimeLibs = with pkgs; [
          libxkbcommon
          wayland
          libX11
          libXcursor
          libXrandr
          libXi
          vulkan-loader
          libGL
          openssl
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            pkg-config
            clang
            mold
          ];

          buildInputs = runtimeLibs;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      }
    );
}
