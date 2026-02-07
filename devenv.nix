{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
let
  gh-custodian = config.languages.rust.import ./. { };
in
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
    libressl
    just
    pinact
  ];

  languages = {
    rust = {
      enable = true;
      components = [
        "rustc"
        "cargo"
        "clippy"
        "rustfmt"
        "rust-analyzer"
      ];
    };
  };

  git-hooks = {
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

  outputs = {
    inherit gh-custodian;
  };
}
