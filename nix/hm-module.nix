{
  crane,
  cranix,
  fenix,
}: {
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  inherit (attrsets) filterAttrs;
  sss = import ./. {
    inherit crane cranix fenix pkgs lib;
    system = pkgs.system;
  };
  cfgSSS = config.programs.sss;
  tomlFormat = pkgs.formats.toml {};
  configDir =
    if pkgs.stdenv.isDarwin
    then "Library/Application Support"
    else config.xdg.configHome;
  sharedConfig = import ./sharedConfig.nix {inherit lib;};
  # Temp config
  sssPackage = lists.optional cfgSSS.enable sss.packages.default;
  codePackage = lists.optional cfgSSS.code.enable sss.packages.code;
  filterConfig = cfg: filterAttrs (n: v: ((builtins.typeOf v) != "null") && n != "enable") cfg;
in {
  options.programs = {
    sss =
      {
        enable = mkEnableOption "cli to take screenshots";
        cli = mkOption {
          description = "";
          default = {};
          type = types.submodule {
            config = {};
            options = {
              current = mkEnableOption "Capture current screens";
              screen = mkEnableOption "Capture all screens";
              screen-id = mkOption {
                type = types.str;
                default = "";
                description = "ID or Name of screen to capture";
              };
            };
          };
        };
        code = mkOption {
          description = "";
          default = {};
          type = types.submodule {
            config = {};
            options = {
              enable = mkEnableOption "cli to take screenshots code";
              # Code Configs
              line-numbers = mkEnableOption "Show Line numbers";
              theme = mkOption {
                type = types.str;
                default = "base16-ocean.dark";
                example = "base16-ocean.dark";
                description = "Theme file to use. May be a path, or an embedded theme. Embedded themes will take precendence.";
              };
              vim-theme = mkOption {
                type = types.str;
                default = "";
                example = "";
                description = "[Not recommended for manual use] Set theme from vim highlights, format: group,bg,fg,style;group,bg,fg,style;";
              };
              extra-syntaxes = mkOption {
                type = types.str;
                default = "";
                example = "~/.config/extra-syntaxes";
                description = "Additional folder to search for .sublime-syntax files in";
              };
              tab-width = mkOption {
                type = types.int;
                default = 4;
                example = "4";
                description = "Tab width";
              };
            };
          };
        };
        general = mkOption {
          description = "";
          default = {};
          type = types.submodule {
            config = {};
            options = sharedConfig;
          };
        };
      };
  };

  config = mkIf cfgSSS.enable {
    home.packages = sssPackage ++ codePackage;

    home.file."${configDir}/sss/config.toml" = mkIf cfgSSS.enable {
      source =
        tomlFormat.generate "config.toml" (filterConfig cfgSSS);
    };
  };
}
