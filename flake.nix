{
  description = "E2E Email [prototype]";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
	let
	system = "x86_64-linux";
	pkgs = nixpkgs.legacyPackages.${system};
	in{
		devShells.${system}.default = pkgs.mkShell {
			buildInputs = [
			  pkgs.rustc
				pkgs.rust-analyzer
				pkgs.cargo
				pkgs.rustfmt
			];
		};
  };
}
