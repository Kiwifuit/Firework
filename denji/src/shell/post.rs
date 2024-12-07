use anyhow::Context;
use log::{debug, error, info, warn};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use std::fs::File;
use std::io::{BufReader, BufWriter};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

pub(super) fn add_run_sh<P: AsRef<Path>, S: super::ServerSoftwareMeta>(
  root_dir: P,
  server_type: S,
) -> anyhow::Result<()> {
  let filename = root_dir.as_ref().join("run.sh");

  info!("writing initializer script at {:?}", filename.display());
  let mut lines = get_lines(&filename).unwrap_or_else(|e| {
    let selected_default = server_type.run_sh_content();

    error!("failed to get lines: {}", e);
    warn!("run.sh opts will be defaulted to: {:?}", selected_default);

    selected_default
  });

  if let Some(line) = lines.last_mut() {
    *line = get_modded_line(line);
  }

  // dunno what to name this, plus it looks really
  // dirty inside `BufWriter::new` 🤡
  let write_file_handle = File::options()
    .mode(0o755)
    .create(true)
    .truncate(true)
    .write(true)
    .open(filename)
    .context("while creating run.sh file")?;

  let mut write_file = BufWriter::new(write_file_handle);

  for line in lines {
    writeln!(write_file, "{}", line).context("while writing run.sh file")?;
  }

  Ok(())
}

pub fn agree_eula<P: AsRef<Path>>(base_dir: P) -> anyhow::Result<usize> {
  let mut file =
    File::create(base_dir.as_ref().join("eula.txt")).context("while eula.txt creating file")?;

  file
    .write(b"eula=true")
    .context("while writing eula.txt to file")
}

pub(super) fn write_user_jvm_args<T: ToString, P: AsRef<Path>>(
  base_dir: P,
  args: T,
) -> anyhow::Result<()> {
  let filename = base_dir.as_ref().join("user_jvm_args.txt");
  info!("writing JVM args at {:?}", filename.display());

  let mut file = File::create(filename).context("while creating jvm args file")?;

  file
    .write_all(args.to_string().as_bytes())
    .context("while writing jvm args file")
}

fn get_lines(filename: &PathBuf) -> anyhow::Result<Vec<String>> {
  let read_file = BufReader::new(File::open(filename).context("while reading run.sh lines")?);
  let lines: Vec<String> = read_file
    .lines()
    .collect::<Result<_, _>>()
    .context("while reading run.sh lines")?;

  Ok(lines)
}

fn get_modded_line(line: &String) -> String {
  debug!("recieved line: {}", line);

  let mut args = line.split(' ').collect::<Vec<&str>>();
  // index of the second to the last argument, typically a `"$@"`
  let sttl_arg_index = args.len() - 1;

  //
  if !line.contains("user_jvm_args.txt") {
    args.insert(2, "@user_jvm_args.txt");
  } else if !line.contains("\"$@\"") {
    args.push("\"$@\"");
  }

  args.insert(sttl_arg_index, "--nogui");

  args.join(" ").to_string()
}
