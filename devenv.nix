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
    libsodium
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

    python = {
      enable = true;
      venv.enable = true;
      uv.enable = true;
    };
  };

  treefmt = {
    enable = true;
    config.programs = {
      nixfmt.enable = true;
      rustfmt.enable = true;
    };
  };

  git-hooks = {
    hooks = {
      shellcheck.enable = true;
      cargo-check.enable = true;
      clippy.enable = true;
      markdownlint = {
        settings.configuration = {
          MD013 = {
            line_length = 120;
          };
          MD033 = false;
        };
      };
      treefmt.enable = true;
    };
  };

  outputs = {
    inherit gh-custodian;
  };
}
