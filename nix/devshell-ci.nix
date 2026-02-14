{moduleWithSystem, ...}: {
  flake.aspects.devshells.ci = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [
      just
    ];

    commands = [
      {
        name = "enterTest";
        help = "Test CI lint enviornment";
        command = ''
          cargo -V;
          cargo clippy -V;
          just -V
        '';
      }
    ];
  });
}
