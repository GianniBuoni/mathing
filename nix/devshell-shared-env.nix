let
  mkEnv = name: value: {inherit name value;};
in {
  flake.aspects.devshells.sharedEnv = {
    env = [
      (mkEnv "SQLX_OFFLINE" "true")
    ];
  };
}
