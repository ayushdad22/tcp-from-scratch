{ pkgs, lib, config, inputs, ... }:

{
  # Packages available in your dev shell
  packages = [
    pkgs.rustc
    pkgs.cargo
    pkgs.tcpdump
    pkgs.gdb
  ];

  # Optional: Shell environment
  env = {
    RUST_BACKTRACE = "1";
  };
}
