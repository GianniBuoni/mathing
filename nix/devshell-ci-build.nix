let
  mkEnv = name: value: {inherit name value;};
in {
  flake.aspects.devshells.ciBuild = {
    env = [
      (mkEnv "SQLX_OFFLINE" "true")
    ];
  };
}
