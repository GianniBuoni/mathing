{
  flake.aspects.devshells.enterTest.commands = [
    {
      name = "enterTest";
      help = "Test CI lint enviornment";
      command = ''
        cargo -V;
        cargo clippy -V;
        just -V
        protoc --version
      '';
    }
  ];
}
