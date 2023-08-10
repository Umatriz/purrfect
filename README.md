# Purrfect

Logging library for Rust. All configuration in TOML file!

**WORK IN PROGRESS!**

> <picture>
>   <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/Mqxx/GitHub-Markdown/main/blockquotes/badge/light-theme/warning.svg">
>   <img alt="Warning" src="https://raw.githubusercontent.com/Mqxx/GitHub-Markdown/main/blockquotes/badge/dark-theme/warning.svg">
> </picture><br>
>
> This crate yet unstable. You may install it as a workspace.

## Examples

Init in code
```toml
# cargo.toml
[dependencies]
log = "0.4.19"
purrfect = { path = "./purrfect"}

[workspace]
members = ["purrfect"]
```

```rust
fn main {
    purrfect::PurrfectBuilder::new()
        .config("path/to/config.toml")
        .build()
        .unwrap();

    // then you can use log crate

    log::error!("Error");
    log::info!("info");
    log::debug!("debug");
    log::warn!("warn");
    log::trace!("trace")
}
```

All configuration must store in TOML file.

This is a full featured config example:

> <picture>
>   <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/Mqxx/GitHub-Markdown/main/blockquotes/badge/light-theme/warning.svg">
>   <img alt="Warning" src="https://raw.githubusercontent.com/Mqxx/GitHub-Markdown/main/blockquotes/badge/dark-theme/warning.svg">
> </picture><br>
>
> `background` field in `level_colors` does not work

```toml
[root]
pattern = "{time}lc<[{level}]>Red<[{module_path}]> {file} {line}: {args}"
time = "Blue<[%Y-%m-%d]>lc<[%H:%M:%S]>"

[level_colors.error]
background = "Blue"
color = "Yellow"

[level_colors.warn]
background = "Blue"
color = "Yellow"

[level_colors.info]
background = "Blue"
color = "Yellow"

[level_colors.debug]
background = "Blue"
color = "Yellow"

[level_colors.trace]
background = "Blue"
color = "Yellow"

[[loggers]]

[loggers.File]
path = "File.log"
level = "Trace"

[[loggers]]

[loggers.Console]
level = "Trace"
```

#### Formatting snippets:

- `pattern` field
    * `{time}` - will be replaced with log time following by pattern provided in `time` field
    * `{level}` - will be replaced with log level
    * `{module_path}` - will be replaced with path to module where log was called
    * `{file}` - will be replaced with path to file where log was called
    * `{line}` - will be replaced with line where log was called
    * `{args}` - will be replaced with log message
    * allows you to use color formatting
- `time` field
    * time format
    * allows you to use color formatting

#### Color formatting

- `lc<content>` - will wrap you content into level color
- `ColorName<content>` - will wrap you content into provided color

`ColorName` can be replaced with one of the following names

- Black
- Blue
- BrightBlack
- BrightBlue
- BrightCyan
- BrightGreen
- BrightMagenta
- BrightRed
- BrightWhite
- BrightYellow
- Cyan
- Default
- Green
- Magenta
- Red
- White
- Yellow

This names also uses in the `level_colors`

##### Example

`lc<{level}>: Blue<{args}>`

## ToDo

- [ ] Write docs