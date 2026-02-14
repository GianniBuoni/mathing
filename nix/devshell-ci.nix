{
  flake.aspects.devshells.ci = {
    devshell.startup.enterTest.text = ''
      cargo -V;
      cargo clippy -V;
      just -V
    '';
  };
}
