use nu_plugin::{
    EngineInterface, EvaluatedCall, MsgPackSerializer, Plugin, PluginCommand, SimplePluginCommand,
    serve_plugin,
};
use nu_protocol::{Category, Example, LabeledError, Signature, SyntaxShape, Type, Value};

use anyhow::Result;
mod helpers;

#[cfg(test)]
mod tests;

use crate::helpers::{format_kcl_file, run_kcl_command, validate_kcl_project};

/// Nushell plugin for running, formatting, and validating KCL files using the KCL CLI.
///
/// This plugin provides three commands:
/// - `kcl-run`: Execute KCL files and return their output.
/// - `kcl-format`: Format KCL files.
/// - `kcl-validate`: Validate all KCL files in a directory.
///
/// See each command struct for more details and usage examples.
struct KclWrapperPlugin;

/// Implements the Nushell Plugin trait for the KCL wrapper plugin.
impl Plugin for KclWrapperPlugin {
    fn version(&self) -> String {
        // This automatically uses the version of your package from Cargo.toml as the plugin version
        // sent to Nushell
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(KclRun), Box::new(KclFormat), Box::new(KclValidate)]
    }
}

/// Command to execute KCL files using the KCL CLI.
///
/// # Usage
/// ```nu
/// kcl-run myfile.k -D foo=bar -f json
/// ```
///
/// See `examples()` for more.
struct KclRun;

impl SimplePluginCommand for KclRun {
    type Plugin = KclWrapperPlugin;

    fn name(&self) -> &str {
        "kcl-run"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .input_output_type(Type::Any, Type::String)
            .required("file", SyntaxShape::Filepath, "KCL file to execute")
            .named(
                "format",
                SyntaxShape::String,
                "Output format (yaml/json)",
                Some('f'),
            )
            .named("output", SyntaxShape::Filepath, "Output file", Some('o'))
            .named(
                "define",
                SyntaxShape::String,
                "Variables to define (key=value)",
                Some('D'),
            )
            .category(Category::Experimental)
    }
    fn description(&self) -> &str {
        "Execute KCL files using the CLI wrapper"
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "kcl-run myfile.k -D foo=bar -f json",
            description: "Run 'myfile.k' with variable 'foo=bar' and output as JSON.",
            result: Some(Value::test_string("{\n  \"foo\": \"bar\"\n}")),
        }]
    }

    fn run(
        &self,
        _plugin: &KclWrapperPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let file_path: String = call.req(0)?;
        let format = call
            .get_flag_value("format")
            .and_then(|v| v.as_str().ok().map(|s| s.to_string()))
            .unwrap_or_else(|| "yaml".to_string());
        let output = call
            .get_flag_value("output")
            .and_then(|v| v.as_str().ok().map(|s| s.to_string()));
        let defines: Vec<String> = call
            .get_flag_value("define")
            .and_then(|v| v.as_list().ok().map(|list| list.to_vec()))
            .map(|list| {
                list.into_iter()
                    .filter_map(|v| v.as_str().ok().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        match run_kcl_command(&file_path, &format, &output, &defines) {
            Ok(result) => Ok(Value::string(result, call.head)),
            Err(e) => {
                Err(LabeledError::new("Error executing KCL").with_label(e.to_string(), call.head))
            }
        }
    }
}

/// Command to format KCL files using the KCL CLI.
///
/// # Usage
/// ```nu
/// kcl-format myfile.k
/// ```
///
/// See `examples()` for more.
struct KclFormat;

impl SimplePluginCommand for KclFormat {
    type Plugin = KclWrapperPlugin;

    fn name(&self) -> &str {
        "kcl-format"
    }

    fn description(&self) -> &str {
        "Format KCL files"
    }
    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .input_output_type(Type::String, Type::String)
            .required("file", SyntaxShape::Filepath, "KCL file to format")
            .category(Category::Experimental)
    }
    fn run(
        &self,
        _plugin: &KclWrapperPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let file_path: String = call.req(0)?;

        match format_kcl_file(&file_path) {
            Ok(result) => Ok(Value::string(result, call.head)),
            Err(e) => {
                Err(LabeledError::new("Error formatting KCL").with_label(e.to_string(), call.head))
            }
        }
    }
    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "kcl-format myfile.k",
            description: "Format the KCL file 'myfile.k'.",
            result: Some(Value::test_string("✅ File formatted: myfile.k")),
        }]
    }
}

/// Command to validate all KCL files in a directory using the KCL CLI.
///
/// # Usage
/// ```nu
/// kcl-validate ./project_dir
/// ```
///
/// See `examples()` for more.
struct KclValidate;

impl SimplePluginCommand for KclValidate {
    type Plugin = KclWrapperPlugin;
    fn name(&self) -> &str {
        "kcl-validate"
    }
    fn description(&self) -> &str {
        "kcl validate"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .input_output_type(Type::Any, Type::String)
            .optional("dir", SyntaxShape::Directory, "Directory to validate")
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &KclWrapperPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let dir = call.opt::<String>(0)?.unwrap_or_else(|| ".".to_string());

        match validate_kcl_project(&dir) {
            Ok(result) => Ok(Value::string(result, call.head)),
            Err(e) => Err(LabeledError::new("Error validating KCL project")
                .with_label(e.to_string(), call.head)),
        }
    }
    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "kcl-validate ./project_dir",
            description: "Validate all KCL files in the directory './project_dir'.",
            result: Some(Value::test_string(
                "✅ All 3 files are valid\n\n✅ ./project_dir/main.k\n✅ ./project_dir/vars.k\n✅ ./project_dir/other.k",
            )),
        }]
    }
}

/// Entry point for the KCL Nushell plugin.
///
/// This function registers the plugin and its commands with Nushell.
fn main() {
    serve_plugin(&KclWrapperPlugin, MsgPackSerializer);
}
