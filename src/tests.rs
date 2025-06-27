/// Unit tests for KCL plugin helpers.
///
/// These tests check the behavior of running, formatting, and validating KCL files
/// using the KCL CLI. All tests are skipped if the `kcl` binary is not installed.
#[cfg(test)]
mod tests {
    // use super::*;
    use crate::helpers::{format_kcl_file, run_kcl_command, validate_kcl_project};
    use std::io::Write;
    use std::process::Command;
    use tempfile::{NamedTempFile, tempdir};

    /// Returns true if the `kcl` CLI is installed and available in PATH.
    fn kcl_installed() -> bool {
        Command::new("kcl").arg("--version").output().is_ok()
    }

    /// Test that running a valid KCL file with `run_kcl_command` succeeds.
    #[test]
    fn test_run_kcl_command_success() {
        if !kcl_installed() {
            return;
        }
        let mut file = NamedTempFile::new().expect("Failed to create temp KCL file");
        writeln!(file, "a = 1").expect("Failed to write KCL code to temp file");
        let path = file
            .path()
            .to_str()
            .expect("Temp file path is not valid UTF-8");
        let res = run_kcl_command(path, "yaml", &None, &[]);
        assert!(res.is_ok(), "Expected Ok, got: {:?}", res);
        let out = res.expect("run_kcl_command returned Err unexpectedly");
        assert!(out.contains("a = 1") || out.contains("✅") || out.contains("a: 1"));
    }

    /// Test that formatting a valid KCL file with `format_kcl_file` succeeds.
    #[test]
    fn test_format_kcl_file_success() {
        if !kcl_installed() {
            return;
        }
        let mut file = NamedTempFile::new().expect("Failed to create temp KCL file");
        writeln!(file, "a = 1").expect("Failed to write KCL code to temp file");
        let path = file
            .path()
            .to_str()
            .expect("Temp file path is not valid UTF-8");
        let res = format_kcl_file(path);
        assert!(res.is_ok(), "Expected Ok, got: {:?}", res);
        let out = res.expect("format_kcl_file returned Err unexpectedly");
        assert!(out.contains("formatted"));
    }

    /// Test that validating a directory with a valid KCL file using `validate_kcl_project` succeeds.
    #[test]
    fn test_validate_kcl_project_success() {
        if !kcl_installed() {
            return;
        }
        let dir = tempdir().expect("Failed to create temp dir");
        let file_path = dir.path().join("test.k");
        std::fs::write(&file_path, "a = 1").expect("Failed to write KCL code to temp file");
        let res = validate_kcl_project(
            dir.path()
                .to_str()
                .expect("Temp dir path is not valid UTF-8"),
        );
        assert!(res.is_ok(), "Expected Ok, got: {:?}", res);
        let out = res.expect("validate_kcl_project returned Err unexpectedly");
        assert!(out.contains("valid") || out.contains("✅"));
    }

    /// Test that running a nonexistent KCL file with `run_kcl_command` returns an error.
    #[test]
    fn test_run_kcl_command_fail() {
        if !kcl_installed() {
            return;
        }
        let res = run_kcl_command("nonexistent.k", "yaml", &None, &[]);
        assert!(res.is_err(), "Expected Err, got: {:?}", res);
    }
}
