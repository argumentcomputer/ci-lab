{
  description = "Lean Nix flake";

  inputs = {
    # System packages
    nixpkgs.follows = "lean4-nix/nixpkgs";

    # Lean 4 & Lake
    lean4-nix.url = "github:lenianiva/lean4-nix";

    # Helper: flake-parts for easier outputs
    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix = {
      url = "github:nix-community/fenix";
      # Follow lean4-nix nixpkgs so we stay in sync
      inputs.nixpkgs.follows = "lean4-nix/nixpkgs";
    };
  };

  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    lean4-nix,
    fenix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      # Systems we want to build for
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      perSystem = {
        system,
        pkgs,
        ...
      }: let
        # Pins the Rust toolchain
        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-2eWc3xVTKqg5wKSHGwt1XoM/kUBC6y3MWfKg74Zn+fY=";
        };
      in {
        # Lean overlay
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [(lean4-nix.readToolchainFile ./lean-toolchain)];
        };

        packages.default =
          ((lean4-nix.lake {inherit pkgs;}).mkPackage {
            src = ./.;
            roots = ["Main" "Template"];
          })
          .executable;

        # Provide a unified dev shell with Lean + Rust
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            lean.lean-all # Includes Lean compiler, lake, stdlib, etc.
            rustToolchain
          ];
        };
      };
    };
}
