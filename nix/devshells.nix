{inputs, ...}: let
  mkEnv = name: value: {inherit name value;};
in {
  flake-file.inputs.devshell.url = "github:numtide/devshell";
  imports = [inputs.devshell.flakeModule];

  perSystem = {pkgs, ...}: {
    devshells.default = {extraModulesPath, ...}: {
      imports = [
        "${extraModulesPath}/language/rust.nix"
        "${extraModulesPath}/language/c.nix"
      ];

      packages = with pkgs; [
        # general shell utilities
        just
        rust-analyzer
        # api tools
        grpc-tools
        # db utilities
        postgresql
        sqlx-cli
      ];

      env = [
        (mkEnv "SERVER_URI" "[::1]:50051")
        (mkEnv "DATABASE_URL" "postgres://[::1]:5432/mathing")
        (mkEnv "PGDATA" ".postgres")
      ];
    };
  };
}
