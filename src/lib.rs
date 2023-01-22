//use std::io::{Error, ErrorKind, Result};
use std::process::Command;

mod error;
use crate::error::{Error, ErrorKind};

// pipe()
//_____________________________________________________________________________

pub fn pipe(cmd: &str) -> Result<String, Error> {
    let output = Command::new(cmd).output();

    match output {
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
    }
}

// Tests
//_____________________________________________________________________________

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_not_found() {
        let output = pipe("/does/not/exist");

        assert_eq!(output.as_ref().unwrap_err().kind(), ErrorKind::OsError);

        // errno 2 is ENOENT No such file or directory.
        assert_eq!(output.as_ref().unwrap_err().code(), Some(2));
    }

    #[test]
    fn test_single_true() {
        let output = pipe("/usr/bin/true");
        assert!(output.unwrap().eq(""));
    }

    #[test]
    fn test_single_false() {
        let output = pipe("/usr/bin/false");
        assert_eq!(output.as_ref().unwrap_err().kind(), ErrorKind::ExecError);
        assert_eq!(output.as_ref().unwrap_err().code(), Some(1));
    }
}
