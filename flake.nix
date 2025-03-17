{
  description = "Rust Development Shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    nixvimConfig.url = "github:Olisnot/NixVimConfig";
  };

  outputs = { self, nixpkgs, nixvimConfig}: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages."${system}";
      nvim = nixvimConfig.packages."${system}".default.extend {
        plugins = {
          rustaceanvim.enable = true;
        };
      };
    in
    {
      devShells."${system}".default = pkgs.mkShell {
        buildInputs = /* bash */ with pkgs; [ nvim gtk4 pkg-config openssl];
        nativeBuildInputs = with pkgs; [ rustc cargo gcc rustfmt clippy ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        shellHook = ''
        export SHELL=/run/current-system/sw/bin/bash
        cd shoppinglistanalyzer || cd ..
        '';
      };
    };
}
