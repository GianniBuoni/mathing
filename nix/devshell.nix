{
  inputs,
  config,
  ...
}: {
  flake-file.inputs.devshell.url = "github:numtide/devshell";
  imports = [inputs.devshell.flakeModule];

  perSystem.devshells = with config.flake.aspects.devshells; {
    default = {extraModulesPath, ...}: {
      imports = [
        "${extraModulesPath}/language/rust.nix"
        "${extraModulesPath}/language/c.nix"
        default
      ];
    };
    ci = {extraModulesPath, ...}: {
      imports = [
        "${extraModulesPath}/language/rust.nix"
        "${extraModulesPath}/language/c.nix"
        ci
      ];
    };
  };
}
