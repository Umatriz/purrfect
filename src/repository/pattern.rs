use owo_colors::OwoColorize;

use crate::{config::LoggerColors, prelude::*};

use super::color::LoggerColor;

#[derive(Debug, Default)]
pub struct MessagePattern {
    pub pattern: String,
    pub time_format: String,
    pub with_colors: bool,
    pub lc_color: Option<LoggerColors>,
}

#[derive(Debug, Default)]
pub struct MessagePatternBuilder {
    pub pattern: String,
    pub time_format: String,
    pub with_colors: bool,
    pub lc_color: Option<LoggerColors>,
}

impl MessagePatternBuilder {
    pub fn new() -> MessagePatternBuilder {
        MessagePatternBuilder::default()
    }

    pub fn pattern(&mut self, pattern: String) -> &mut MessagePatternBuilder {
        self.pattern = pattern;
        self
    }

    pub fn time_format(&mut self, time_format: String) -> &mut MessagePatternBuilder {
        self.time_format = time_format;
        self
    }

    pub fn with_colors(&mut self, with_colors: bool) -> &mut MessagePatternBuilder {
        self.with_colors = with_colors;
        self
    }

    pub fn lc_color(&mut self, lc_color: LoggerColors) -> &mut MessagePatternBuilder {
        self.lc_color = Some(lc_color);
        self
    }

    pub fn build(self) -> MessagePattern {
        MessagePattern {
            pattern: self.pattern,
            time_format: self.time_format,
            with_colors: self.with_colors,
            lc_color: self.lc_color,
        }
    }
}

pub fn log_by_pattern<F>(record: log::Record, fun: F)
where
    F: FnOnce(log::Record) -> Result<()>,
{
}

impl MessagePattern {
    pub fn new(
        pattern: String,
        time_format: String,
        with_colors: bool,
        lc_color: Option<LoggerColors>,
    ) -> Self {
        Self {
            pattern,
            time_format,
            with_colors,
            lc_color,
        }
    }

    pub fn parse(self, record: log::Record<'static>) -> Result<String> {
        let colors_re = regex::Regex::new(r"(?P<color>[A-Z][a-z]*)<(?P<content>[^>]+)>")?;
        let lc_re = regex::Regex::new(r"lc<(?P<content>[^>]+)>")?;

        let mut colors = vec![];

        for (origin, [color, content]) in
            colors_re.captures_iter(&self.pattern).map(|c| c.extract())
        {
            colors.push((
                origin,
                color.parse::<LoggerColor>().unwrap_or_default(),
                content,
            ))
        }

        let mut lcs = vec![];

        for (origin, [content]) in lc_re.captures_iter(&self.pattern).map(|c| c.extract()) {
            lcs.push((origin, content))
        }

        let mut replaced = self.pattern.clone();

        colors.iter().for_each(|color| {
            replaced = replaced.replace(color.0, color.2);
        });

        lcs.iter()
            .for_each(|lc| replaced = replaced.replace(lc.0, lc.1));

        if self.with_colors {
            colors.iter().for_each(|color| {
                let new = replaced.split(color.2).collect::<Vec<&str>>();
                replaced = format!("{}{}{}", new[0], color.2.color(color.1 .0), new[1]);
            });

            if let Some(lc_color) = self.lc_color {
                let level = match record.metadata().level() {
                    log::Level::Error => lc_color.error,
                    log::Level::Warn => lc_color.warn,
                    log::Level::Info => lc_color.info,
                    log::Level::Debug => lc_color.debug,
                    log::Level::Trace => lc_color.trace,
                };

                lcs.iter().for_each(|lc| {
                    let new = replaced.split(lc.1).collect::<Vec<&str>>();
                    replaced = format!("{}{}{}", new[0], lc.1.color(level.color.0), new[1]);
                });
            }
        }

        Ok(replaced)
    }

    pub fn format_args(self, record: log::Record<'static>) -> String {
        let mut replaced = self.pattern;

        let args = "{args}";
        if replaced.contains(args) {
            replaced = replaced.replace(args, &record.args().to_string())
        }

        let level = "{level}";
        if replaced.contains(level) {
            replaced = replaced.replace(level, record.metadata().level().as_str())
        }

        let module_path = "{module_path}";
        if replaced.contains(module_path) {
            if let Some(module) = record.module_path() {
                replaced = replaced.replace(module_path, module)
            }
        }

        let file_str = "{file}";
        if replaced.contains(file_str) {
            if let Some(file) = record.file() {
                replaced = replaced.replace(file_str, file)
            }
        }

        let line_str = "{line}";
        if replaced.contains(line_str) {
            if let Some(line) = record.line() {
                replaced = replaced.replace(line_str, &line.to_string())
            }
        }

        let time = "{time}";
        if replaced.contains(time) {
            replaced = replaced.replace(
                file_str,
                &chrono::Local::now().format(&self.time_format).to_string(),
            )
        }

        replaced
    }
}
