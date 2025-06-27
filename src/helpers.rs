// Helper functions using KCL CLI
use anyhow::Result;
use std::process::Command;

/// Run a KCL file using the KCL CLI.
///
/// # Arguments
/// * `file` - Path to the KCL file to execute.
/// * `format` - Output format (e.g., "yaml" or "json").
/// * `output` - Optional output file path.
/// * `defines` - List of variable definitions (e.g., ["foo=bar"]).
///
/// # Returns
/// * `Ok(String)` with the output or output file path on success.
/// * `Err(anyhow::Error)` if the KCL command fails.
pub(crate) fn run_kcl_command(
    file: &str,
    format: &str,
    output: &Option<String>,
    defines: &[String],
) -> Result<String> {
    let mut cmd = Command::new("kcl");
    cmd.arg("run").arg(file).arg("--format").arg(format);

    // Add defined variables
    for define in defines {
        cmd.arg("-D").arg(define);
    }

    // Add output file if specified
    if let Some(output_file) = output {
        cmd.arg("-o").arg(output_file);
    }

    let output_res = cmd
        .output()
        .map_err(|e| anyhow::anyhow!("Error executing kcl: {}", e))?;

    if output_res.status.success() {
        if let Some(output_file) = output {
            Ok(format!("✅ {}", output_file))
        } else {
            Ok(format!(
                "✅ {}",
                String::from_utf8_lossy(&output_res.stdout)
            ))
        }
    } else {
        Err(anyhow::anyhow!(
            "❌: {}",
            String::from_utf8_lossy(&output_res.stderr)
        ))
    }
}

/// Format a KCL file using the KCL CLI.
///
/// # Arguments
/// * `file` - Path to the KCL file to format.
///
/// # Returns
/// * `Ok(String)` with a success message if formatting succeeds.
/// * `Err(anyhow::Error)` if formatting fails.
pub(crate) fn format_kcl_file(file: &str) -> Result<String> {
    let output = Command::new("kcl")
        .arg("fmt")
        .arg(file)
        .output()
        .map_err(|e| anyhow::anyhow!("Error executing kcl fmt: {}", e))?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "KCL format failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(format!("✅ File formatted: {}", file))
}

/// Validate all KCL files in a directory using the KCL CLI.
///
/// # Arguments
/// * `dir` - Path to the directory to search for KCL files.
///
/// # Returns
/// * `Ok(String)` with a summary of validation results if all files are valid or no files found.
/// * `Err(anyhow::Error)` if validation fails for any file or if the find command fails.
pub(crate) fn validate_kcl_project(dir: &str) -> Result<String> {
    // Find KCL files in directory
    let find_output = Command::new("find")
        .arg(dir)
        .arg("-name")
        .arg("*.k")
        .arg("-type")
        .arg("f")
        .output()
        .map_err(|e| anyhow::anyhow!("Error finding KCL files: {}", e))?;

    let files = String::from_utf8_lossy(&find_output.stdout);
    let kcl_files: Vec<&str> = files.lines().filter(|line| !line.is_empty()).collect();

    if kcl_files.is_empty() {
        return Ok(format!("No KCL files found in {}", dir));
    }

    let mut results = Vec::new();
    let mut all_valid = true;

    for file in &kcl_files {
        let output = Command::new("kcl")
            .arg("run")
            .arg(file)
            .arg("--format")
            .arg("yaml")
            .output();

        match output {
            Ok(output) if output.status.success() => {
                results.push(format!("✅ {}", file));
            }
            Ok(output) => {
                results.push(format!(
                    "❌ {}: {}",
                    file,
                    String::from_utf8_lossy(&output.stderr)
                ));
                all_valid = false;
            }
            Err(e) => {
                results.push(format!("❌ {}: Execution error: {}", file, e));
                all_valid = false;
            }
        }
    }

    let summary = if all_valid {
        format!("✅ All {} files are valid", kcl_files.len())
    } else {
        format!("❌ Errors found in some files")
    };

    Ok(format!("{}\n\n{}", summary, results.join("\n")))
}
