// use std::fs::File;
// use std::io::Read;
// use std::path::PathBuf;
// use std::process::{Command, Stdio};
// use std::str::FromStr;

// use anyhow::Context;
// use lazy_static::lazy_static;

// lazy_static! {
//   static ref GIT_PATH: PathBuf =
//     PathBuf::from_str("../.git/").expect("expected path to be created");
// }

static mut FEATURES: Vec<&'static str> = vec![];

macro_rules! detect_feature {
  ($feat:literal) => {
    #[cfg(feature = $feat)]
    unsafe {
      FEATURES.push($feat)
    }
  };
}

fn main() -> anyhow::Result<()> {
  // Grab the HEAD commit hash and set it to
  // these 2 compile-time variables:

  //   let bruh = get_git_sha_file()
  //     .or_else(get_git_sha_command)
  //     .context("while attempting to resolve git HEAD commit")
  //     .context("this means that the build script was unable to get the HEAD commit");

  //   eprintln!("bruh: {:?}", bruh);

  //   let git_sha = bruh?;

  //   println!("cargo::rustc-env=GIT_SHA_LONG={}", git_sha);
  //   println!(
  //     "cargo::rustc-env=GIT_SHA_SHORT={}",
  //     git_sha.get(..8).unwrap()
  //   );

  // Feature listing
  detect_feature!("dms");
  detect_feature!("all-providers");
  detect_feature!("all-parsers");

  #[cfg(not(feature = "all-providers"))]
  detect_feature!("provider-modrinth");

  #[cfg(not(feature = "all-providers"))]
  detect_feature!("provider-hangar");

  #[cfg(not(feature = "all-providers"))]
  detect_feature!("provider-curse");

  #[cfg(not(feature = "all-parsers"))]
  detect_feature!("parser-modpack");
  #[cfg(not(feature = "all-parsers"))]
  detect_feature!("parser-modjar");

  println!("cargo::rustc-env=CRATE_FEATURES={}", unsafe {
    FEATURES.join(",")
  });

  Ok(())
}

// fn get_git_sha_command(error: anyhow::Error) -> anyhow::Result<String> {
//   println!(
//     "cargo::warning=Failed to grab HEAD by file: {}. attempting git command",
//     error
//   );

//   let command = Command::new("git")
//     .args(vec!["rev-parse", "HEAD"])
//     .stdout(Stdio::piped())
//     .spawn()
//     .context("while trying to spawn `git rev-parse HEAD`")?;
//   let commit_hash = String::from_utf8(
//     command
//       .wait_with_output()
//       .context("while waiting for `git` output")?
//       .stdout,
//   )
//   .context("while parsing command output")?;

//   eprintln!("commit hash empty: {}", commit_hash.is_empty());

//   Ok(commit_hash)
// }

// fn get_git_sha_file() -> anyhow::Result<String> {
//   let head = GIT_PATH.join("head");

//   let mut ref_file =
//     File::open(&head).context(format!("while opening ref file: {}", head.display()))?;
//   let mut ref_content = String::new();

//   ref_file
//     .read_to_string(&mut ref_content)
//     .context(format!("while reading ref file: {}", head.display()))?;

//   ref_content.truncate(ref_content.len() - 1);

//   let head_ref = ref_content.strip_prefix("ref: ").unwrap();
//   let git_head_commit = GIT_PATH.join(head_ref);
//   let mut git_head = File::open(&git_head_commit).context(format!(
//     "while opening git head file: {:?}",
//     git_head_commit.display()
//   ))?;
//   let mut git_sha = String::new();

//   git_head.read_to_string(&mut git_sha).context(format!(
//     "while reading git head file: {:?}",
//     git_head_commit.display()
//   ))?;

//   Ok(git_sha)
// }
