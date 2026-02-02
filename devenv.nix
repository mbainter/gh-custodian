{ pkgs, lib, config, inputs, ... }:
{
  packages = with pkgs; [
    actionlint
    cargo-outdated
    cargo-machete
    cargo-edit
    cargo-insta
    cargo-nextest
    cargo-deny
    git
    just
    pinact
  ];

  languages = {
    rust = {
      enable = true;
      components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
    };
  };

  git-hooks = {
    enable = true;
    hooks = {
      shellcheck.enable = true;
      cargo-check.enable = true;
      rustfmt.enable = true;
      clippy.enable = true;
      markdownlint = {
        settings.configuration = {
          MD013 = {
            line_length = 120;
          };
          MD033 = false;
        };
      };
    };
  };
}
