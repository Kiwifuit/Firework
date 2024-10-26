use anyhow::Context;
use fern_colored::colors::{Color, ColoredLevelConfig};
use owo_colors::OwoColorize;

pub fn init() -> anyhow::Result<()> {
  let colors_level = ColoredLevelConfig::new()
    .debug(Color::BrightMagenta)
    .trace(Color::BrightGreen)
    .info(Color::BrightCyan)
    .warn(Color::BrightYellow)
    .error(Color::BrightRed);

  fern_colored::Dispatch::new()
    .format(move |out, message, record| {
      out.finish(format_args!(
        "{} {}\t{}",
        chrono::Local::now().format("[%H:%M:%S]").black(),
        format_args!(
          "[{}/{}{}",
          record.module_path().unwrap_or("<module>").on_black(),
          colors_level.color(record.level()),
          "]".black()
        )
        .black(),
        message,
      ));
    })
    // set the default log level. to filter out verbose log messages from dependencies, set
    // this to Warn and overwrite the log level for your crate.
    .level(log::LevelFilter::Info)
    // output to stdout
    .chain(std::io::stdout())
    .apply()
    .context("while initializing logger")
}
