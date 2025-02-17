use std::ops::Range;

use clap::Parser;
use clap_stdin::FileOrStdin;
use merge2::{bool::overwrite_false, Merge};
use serde::{Deserialize, Serialize};
use sss_lib::{default_bool, swap_option};

use crate::error::CodeScreenshotError;

#[derive(Clone, Debug, Deserialize, Merge, Parser, Serialize)]
#[clap(author, version, about)]
struct ClapConfig {
    #[clap(flatten)]
    #[merge(strategy = swap_option)]
    pub code: Option<CodeConfig>,
    // lib configs
    #[clap(flatten)]
    #[serde(rename = "general")]
    pub lib_config: sss_lib::GenerationSettingsArgs,
}

#[derive(Clone, Debug, Deserialize, Merge, Parser, Serialize)]
pub struct CodeConfig {
    #[clap(help = "Content to take screenshot. It accepts stdin or File")]
    #[serde(skip)]
    #[merge(skip)]
    pub content: Option<FileOrStdin<String>>,
    #[clap(
        long,
        short,
        default_value = "base16-ocean.dark",
        help = "Theme file to use. May be a path, or an embedded theme. Embedded themes will take precendence."
    )]
    pub theme: Option<String>,
    #[clap(
        long,
        help = "[Not recommended for manual use] Set theme from vim highlights, format: group,bg,fg,style;group,bg,fg,style;"
    )]
    pub vim_theme: Option<String>,
    // Setting synctect
    #[clap(long, short = 'l', help = "Lists supported file types")]
    #[merge(strategy = overwrite_false)]
    #[serde(skip)]
    pub list_file_types: bool,
    #[clap(long, short = 'L', help = "Lists themes")]
    #[merge(strategy = overwrite_false)]
    #[serde(skip)]
    pub list_themes: bool,
    #[clap(
        long,
        help = "Additional folder to search for .sublime-syntax files in"
    )]
    pub extra_syntaxes: Option<String>,
    #[clap(long, short, help = "Set the extension of language input")]
    #[serde(skip)]
    pub extension: Option<String>,
    // Render options
    #[clap(
        long,
        help = "[default: #323232] Support: '#RRGGBBAA' 'h;#RRGGBBAA;#RRGGBBAA' 'v;#RRGGBBAA;#RRGGBBAA' or file path"
    )]
    #[merge(strategy = swap_option)]
    pub code_background: Option<String>,
    #[clap(long, default_value="..", help = "Lines range to take screenshot, format start..end", value_parser=parse_range)]
    #[serde(skip)]
    pub lines: Option<Range<usize>>,
    #[clap(long, default_value="..", help = "Lines to highlight over the rest, format start..end", value_parser=parse_range)]
    #[serde(skip)]
    pub highlight_lines: Option<Range<usize>>,
    #[clap(long, short = 'n', default_value = "false", help = "Show Line numbers")]
    #[merge(strategy = overwrite_false)]
    #[serde(default = "default_bool")]
    pub line_numbers: bool,
    #[clap(long, default_value = "4", help = "Tab width")]
    pub tab_width: Option<u8>,
}

pub fn get_config() -> (CodeConfig, sss_lib::GenerationSettings) {
    let config_path = directories::BaseDirs::new()
        .unwrap()
        .config_dir()
        .join("sss");

    let _ = std::fs::create_dir_all(config_path.clone());

    let config_path = config_path.join("config.toml");
    // println!("Reading configs from path: {config_path:?}");

    if let Ok(cfg_content) = std::fs::read_to_string(config_path) {
        // println!("Merging from config file");
        let mut config: ClapConfig = toml::from_str(&cfg_content).unwrap();
        let mut args = ClapConfig::parse();

        config.merge(&mut args);
        return (config.code.unwrap(), config.lib_config.into());
    }
    let config = ClapConfig::parse();
    (config.code.unwrap(), config.lib_config.into())
}

fn parse_range(s: &str) -> Result<Range<usize>, CodeScreenshotError> {
    let Some(other) = s.chars().find(|c| !c.is_numeric()) else {
        return Err(CodeScreenshotError::InvalidFormat("range", "start..end"));
    };

    let Some((start_str, end_str)) = s.split_once(&other.to_string()) else {
        return Err(CodeScreenshotError::InvalidFormat("range", "start..end"));
    };

    let (start, end) = (
        start_str
            .replace(other, "")
            .parse::<usize>()
            .map(|s| if s >= 1 { s - 1 } else { s })
            .unwrap_or_default(),
        end_str
            .replace(other, "")
            .parse::<usize>()
            .map(|s| s + 1)
            .unwrap_or(usize::MAX),
    );

    Ok(Range { start, end })
}
