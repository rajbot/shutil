use std::process::{Command, Stdio};

mod error;
use crate::error::{Error, ErrorKind};

// pipe()
//_____________________________________________________________________________

/// Execute a shell command pipeline and return stdout. The input is a Vec of
/// shell commands, where the output of the first command is connected via a
/// pipe to the input of the second. The stdout of the final command in the
/// pipeline is returned as a String, wrapped in a Result.

/// In the case of non-zero exit status, return a shutil::Error result
/// with kind() set to ExecError and code() set to the exit code.
///
/// In the case of an OS Error, such as command not found, it will return
/// shutil::Error with kind() set to OsError and code() set to the raw os error.
///
/// If the command succeeds, but stdout is not valid utf-8, it will return a
/// shutil::Error with kind() set to UnicodeDecodeError.
///
/// In other error cases, it will return a shutil::Error with kind() set to
/// UnknownError.

pub fn pipe(commands: Vec<Vec<&str>>) -> Result<String, Error> {
    if commands.len() < 1 {
        return Err(Error::new(
            ErrorKind::InvalidFormatError,
            Some(-1),
            "no commands supplied",
        ));
    }

    let mut last_command: Option<Command> = None;

    for i in 0..commands.len() {
        let command_str = &commands[i];

        if command_str.len() == 0 {
            return Err(Error::new(
                ErrorKind::InvalidFormatError,
                Some(-1),
                "no command binary supplied",
            ));
        }

        // Parse binary
        let mut command = Command::new(command_str[0]);

        // Parse args
        for j in 1..command_str.len() {
            command.arg(command_str[j]);
        }

        // Set stdout
        command.stdout(Stdio::piped());

        // Spawn previous command in the chain and set it as stdin for the next command
        if let Some(mut prev) = last_command {
            match prev.spawn() {
                Ok(r) => {
                    if let Some(stdout) = r.stdout {
                        command.stdin(stdout);
                    }
                }
                Err(e) => {
                    return Err(Error::new(
                        ErrorKind::ExecError,
                        Some(-1),
                        format!("spawning failed: {}", e.to_string()).as_str(),
                    ));
                }
            }
        }

        last_command = Some(command);
    }

    // Execute the last command in the chain and return the utf-8 decoded output
    match last_command {
        None => {
            return Err(Error::new(
                ErrorKind::InvalidFormatError,
                Some(-1),
                "no commands supplied",
            ));
        }
        Some(mut cmd) => match cmd.output() {
            Ok(result) => {
                if !result.status.success() {
                    return Err(Error::new(
                        ErrorKind::ExecError,
                        result.status.code(),
                        "non-zero exit code",
                    ));
                }
                match String::from_utf8(result.stdout) {
                    Ok(v) => Ok(v),
                    Err(_e) => Err(Error::new(
                        ErrorKind::UnicodeDecodeError,
                        None,
                        "utf-8 decode failed",
                    )),
                }
            }
            Err(e) => {
                if let Some(raw_os_err) = e.raw_os_error() {
                    return Err(Error::new(
                        ErrorKind::OsError,
                        Some(raw_os_err),
                        &e.to_string(),
                    ));
                } else {
                    return Err(Error::new(ErrorKind::UnknownError, None, &e.to_string()));
                }
            }
        },
    }
}

// Tests
//_____________________________________________________________________________

#[cfg(test)]
mod tests {
    use super::*;

    // Zero command tests

    #[test]
    fn test_no_commands() {
        let output = pipe(vec![]);

        assert_eq!(
            output.as_ref().unwrap_err().kind(),
            ErrorKind::InvalidFormatError
        );
        assert_eq!(output.as_ref().unwrap_err().code(), Some(-1));
    }

    #[test]
    fn test_no_binary() {
        let output = pipe(vec![vec![]]);

        assert_eq!(
            output.as_ref().unwrap_err().kind(),
            ErrorKind::InvalidFormatError
        );
        assert_eq!(output.as_ref().unwrap_err().code(), Some(-1));
    }

    // Single command tests

    #[test]
    fn test_single_not_found() {
        let output = pipe(vec![vec!["/does/not/exist"]]);

        assert_eq!(output.as_ref().unwrap_err().kind(), ErrorKind::OsError);

        // errno 2 is ENOENT No such file or directory.
        assert_eq!(output.as_ref().unwrap_err().code(), Some(2));
    }

    #[test]
    fn test_single_true() {
        let output = pipe(vec![vec!["/usr/bin/true"]]);
        assert!(output.unwrap().eq(""));
    }

    #[test]
    fn test_single_false() {
        let output = pipe(vec![vec!["/usr/bin/false"]]);
        assert_eq!(output.as_ref().unwrap_err().kind(), ErrorKind::ExecError);
        assert_eq!(output.as_ref().unwrap_err().code(), Some(1));
    }

    #[test]
    fn test_echo_one_arg() {
        let output = pipe(vec![vec!["echo", "hello"]]);
        assert!(output.unwrap().eq("hello\n"));
    }

    #[test]
    fn test_echo_two_args() {
        let output = pipe(vec![vec!["echo", "hello", "world"]]);
        assert!(output.unwrap().eq("hello world\n"));
    }

    // two command pipe tests

    #[test]
    fn test_echo_rev() {
        let output = pipe(vec![vec!["echo", "foo"], vec!["rev"]]);
        let unwrapped = output.unwrap();
        assert!(unwrapped.eq("oof\n"));
    }

    // three command pipe tests

    #[test]
    fn test_echo_rev_tr() {
        let output = pipe(vec![
            vec!["echo", "foo"],
            vec!["rev"],
            vec!["tr", "a-z", "A-Z"],
        ]);
        let unwrapped = output.unwrap();
        assert!(unwrapped.eq("OOF\n"));
    }
}
