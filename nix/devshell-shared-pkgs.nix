{moduleWithSystem, ...}: {
  flake.aspects.devshells.sharedPkgs = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [
      grpc-tools
      just
    ];
  });
}
