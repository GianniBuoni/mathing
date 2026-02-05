{inputs, ...}: {
  flake-file.inputs.devshell.url = "github:numtide/devshell";
  imports = [inputs.devshell.flakeModule];

  perSystem = {pkgs, ...}: {
    devshells.default = {extraModulesPath, ...}: {
      imports = [
        "${extraModulesPath}/language/rust.nix"
        "${extraModulesPath}/language/c.nix"
      ];

      packages = with pkgs; [
        rust-analyzer
        grpc-tools
      ];
    };
  };
}
