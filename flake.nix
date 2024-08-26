{
  description = "Telegram Notification Bot";

  # Define the inputs of the flake
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";  # Use NixOS's package set
    flake-utils.url = "github:numtide/flake-utils";  # Utility functions for flakes
    rust-overlay.url = "github:oxalica/rust-overlay";  # Rust overlay for latest Rust toolchains
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];  # Add Rust overlay to Nixpkgs
        };
      in
      {
        # Define a Nix package for your Rust application
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "my-rust-app";  # Change to your application's name
          version = "1.0.0";      # Set your app's version

          src = self;

          cargoLock = {
            lockFile = ./Cargo.lock;  # Path to your Cargo.lock file
          };

          cargoToml = ./Cargo.toml;   # Path to your Cargo.toml file

          meta = with pkgs.lib; {
            description = "A Rust application for NixOS";
            license = licenses.mit;  # Change to your app's license
            maintainers = [ maintainers.yourGithubHandle ];  # Replace with your GitHub handle
          };
        };

        # Define how to run the application
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/my-rust-app";
        };

        # DevShell for development with Rust
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
          ];
        };
      }
    );
}
