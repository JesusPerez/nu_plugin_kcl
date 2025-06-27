# nu_plugin_kcl

A [Nushell](https://nushell.sh/) plugin to use with [KCL](https://www.kcl-lang.io/) CLI wrapper

## Installing

> [!CAUTION]  
> Require to have [KCL](https://www.kcl-lang.io/) CLI wrapper
> use [KLC installation documentation](https://www.kcl-lang.io/docs/user_docs/getting-started/install)

Clone this repository 

> [!WARNING]  
> **nu_plugin_kcl** has dependencies to nushell source via local path in Cargo.toml
> Nushell and plugins require to be **sync** with same **version** 

Clone [Nushell](https://nushell.sh/) to plugin to use [Tera templates](https://keats.github.io/tera/docs/) or change dependecies in [Cargo.toml](Cargo.toml)

This plugin is also included as submodule in [nushell-plugins](https://repo.jesusperez.pro/jesus/nushell-plugins) 
as part of plugins collection for [Provisioning project](https://rlung.librecloud.online/jesus/provisioning)

Is used in

Build from source 

```nushell
> cd nu_plugin_tcl
> cargo install --path .
```

### Nushell 

In a [Nushell](https://nushell.sh/)

```nushell
> plugin add ~/.cargo/bin/nu_plugin_kcl
```

## Exec KCL files 

Exec [KCL files](https://www.kcl-lang.io/docs/user_docs/getting-started/kcl-quick-start) and return result in [YAML](https://en.wikipedia.org/wiki/YAML)

```nushell
> kcl-exec <file> (work_dir)
```

Flags:
-  **-h**, **--help**: Display the help message for this command

Parameters:
- file <path>: KCL file to execute
- work_dir <directory>: Work directory (optional)

### Examples:

Execute the KCL file './src/myfile.k'
```nushell
> kcl-exec ./src/myfile.k
```

## Validate KCL files 

Validate [KCL files](https://www.kcl-lang.io/docs/user_docs/getting-started/kcl-quick-start)

```nushell
> kcl-validate (dir)
```

Flags:
-**h**, **--help**: Display the help message for this command

Parameters:
- dir <directory>: Directory to validate (optional)

### Examples

Validate all KCL files in the directory './project_dir'.

```nushell
> kcl-validate ./project_dir
✅ All 3 files are valid
✅ ./project_dir/main.k
```

## Format KCL files

Format [KCL files](https://www.kcl-lang.io/docs/user_docs/getting-started/kcl-quick-start)

```nushell
> kcl-format (dir)
```

Flags:
- **-h**, **--help**: Display the help message for this command

Parameters:
- file <path>: KCL file to format

### Examples

Format the KCL file 'myfile.k'.
```nushell
> kcl-format myfile.k
✅ File formatted: myfile.k
```
