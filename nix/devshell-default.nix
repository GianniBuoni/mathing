{moduleWithSystem, ...}: let
  mkEnv = name: value: {inherit name value;};
in {
  flake.aspects.devshells.default = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [
      rust-analyzer
      # db utilities
      postgresql
      sqlx-cli
    ];

    commands = [
      {
        name = "enterTest";
        help = "Test CI Build environment";
        command = ''
          cargo -V;
          cargo clippy -V;
          just -V
          pg_ctl -V
          protoc --version
          sqlx -V
        '';
      }
    ];

    env = [
      (mkEnv "DATABASE_URL" "postgres://[::1]:5432/mathing")
      (mkEnv "LOG_LEVEL" "info")
      (mkEnv "SERVER_URI" "[::1]:50051")
      (mkEnv "PGDATA" ".postgres")
    ];
  });
}
