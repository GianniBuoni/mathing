{moduleWithSystem, ...}: let
  mkEnv = name: value: {inherit name value;};
in {
  flake.aspects.devshells.default = moduleWithSystem ({pkgs, ...}: {
    devshell.startup.dbCheck.text = ''
      pg_check=$(pg_ctl status | grep "no server running");
      if [ "$pg_check" == "" ]; then
        echo "";
        echo "PG server running!";
        echo "";
      else
        echo "";
        echo "PG server offline.";
        echo "";
      fi
    '';

    packages = with pkgs; [
      rust-analyzer
      # db utilities
      postgresql
      sqlx-cli
    ];

    env = [
      (mkEnv "DATABASE_URL" "postgres://[::1]:5432/mathing")
      (mkEnv "LOG_LEVEL" "info")
      (mkEnv "SERVER_URI" "[::1]:50051")
      (mkEnv "PGDATA" ".postgres")
    ];
  });
}
