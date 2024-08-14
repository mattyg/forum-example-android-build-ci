{
  description = "Template for Holochain app development";

  inputs = {
    p2p-shipyard.url = "github:mattyg/p2p-shipyard?rev=8e58615c889c4a1a58d0a0cdb22055d2e95e97bc";
    versions.url  = "github:holochain/holochain?dir=versions/0_3";

    holochain-flake.url = "github:holochain/holochain";
    holochain-flake.inputs.versions.follows = "versions";

    nixpkgs.follows = "holochain-flake/nixpkgs";
    flake-parts.follows = "holochain-flake/flake-parts";
    
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holochain-flake.devShells;
        perSystem = { inputs', config, pkgs, system, ... }: {
        devShells.default = pkgs.mkShell {
          inputsFrom = [ inputs'.p2p-shipyard.devShells.holochainTauriDev ];
        };
        devShells.androidDev = pkgs.mkShell {
          inputsFrom = [ inputs'.p2p-shipyard.devShells.holochainTauriAndroidDev];
        };
      };
      
    };
}