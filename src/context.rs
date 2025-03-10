use regex::Regex;
use std::env;
use std::io::{self, Error};

pub use crate::command_history::*;

/// Context about the environment in which this shell is being run. 
pub struct Context {
  pub uname: String,
  pub shell: String,
  pub os: String,
  pub pwd: String,
  pub history: CommandHistory
}

/// Determine the environment's current working directory.
pub fn
get_current_working_dir () -> Result<String, Error> 
{
  let current_dir = env::current_dir()?;
  current_dir.into_os_string().into_string().map_err(|_| {
    std::io::Error::new(std::io::ErrorKind::InvalidData, "Current directory contains invalid Unicode data")
  })
}

/// Sanitizes stdout read from executed `Command`s. 
pub fn 
sanitize_stdout (stdout: &str) -> String
{
  // Current santization is eliminating unecessary whitespace
  let re = Regex::new(r"\s+").unwrap();
  re.replace_all(stdout.trim(), " ").into_owned()
}

impl Context 
{
  pub fn 
  init (uname: Vec<u8>, shell: &str, os: Vec<u8>, stateless: bool) -> io::Result<Context> 
  {
    Ok(Context {
      uname: sanitize_stdout(std::str::from_utf8(&uname).expect("failed to convert stdout to String")),
      shell: shell.to_string(),
      os: sanitize_stdout(std::str::from_utf8(&os).expect("failed to convert stdout to String")),
      pwd: get_current_working_dir().unwrap(),
      history: CommandHistory::init(shell, !stateless)?
    })
  }

  /// Conditionally updates the given `Context`, depending on the nature of the sucessfullly-executed command string.
  pub fn
  update (&mut self, cmd_input: &str) -> Result<(), Box<dyn std::error::Error>>
  {
    let mut parts: Vec<&str> = cmd_input.split_whitespace().collect();
    let cmd = parts.remove(0);

    if cmd.to_owned().to_lowercase().eq("cd") && parts.len() >= 1 {
      // If this was a change-directory command, set the current environment to
      // cd's subsequent argument and update `context.pwd`
      env::set_current_dir(parts.remove(0))?;
      self.pwd = get_current_working_dir()?;
    }

    // Possibly update command history with this most recent command
    self.update_command(cmd_input)?;

    Ok(())
  }

  pub fn 
  update_command (&mut self, cmd: &str) -> io::Result<()>
  {
    self.history.maybe_append_command(cmd)
  }

  pub fn 
  get_command_history (&self) -> Vec<String>
  {
    self.history.get_history()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;
  use tempfile;

  #[test]
  fn test_get_current_working_dir_success() {
    let cwd = get_current_working_dir().unwrap();
    assert_eq!(cwd, env::current_dir().unwrap().into_os_string().into_string().unwrap());
  }

  #[test]
  fn test_sanitize_stdout() {
    let input = "This is    a  test\n";
    let expected = "This is a test";
    assert_eq!(sanitize_stdout(input), expected);
  }

  #[test]
  fn test_context_update_cd_command() {
    // Setup - create a temporary directory and initialize Context
    let temp_dir = tempfile::tempdir().unwrap();
    let mut context = Context {
      uname: "Darwin".to_string(),
      shell: "/bin/zsh".to_string(),
      os: "Darwin 23.3.0 arm64".to_string(),
      pwd: "/home".to_string(),
      history: CommandHistory::init("/bin/zsh", false).unwrap(),
    };

    // Test - change directory to temp_dir
    let cmd_input = format!("cd {}", temp_dir.path().to_str().unwrap());
    context.update(&cmd_input).unwrap();

    // This is a contains rather than an equality check because of the
    // usage of /private symlink for temp files on Mac OS
    assert!(context.pwd.contains(temp_dir.path().to_str().unwrap()));
  }
}
