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

    pub fn pattern(&mut self, pattern: &String) -> &mut MessagePatternBuilder {
        self.pattern = pattern.to_string();
        self
    }

    pub fn time_format(&mut self, time_format: &String) -> &mut MessagePatternBuilder {
        self.time_format = time_format.to_string();
        self
    }

    pub fn with_colors(&mut self, with_colors: bool) -> &mut MessagePatternBuilder {
        self.with_colors = with_colors;
        self
    }

    pub fn lc_color(&mut self, lc_color: &LoggerColors) -> &mut MessagePatternBuilder {
        self.lc_color = Some(lc_color.clone());
        self
    }

    pub fn build(&self) -> MessagePattern {
        MessagePattern {
            pattern: self.pattern.clone(),
            time_format: self.time_format.clone(),
            with_colors: self.with_colors,
            lc_color: self.lc_color.clone(),
        }
    }
}

impl MessagePattern {
    pub fn parse(self) -> Result<Message> {
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

        let mut time_lcs = vec![];

        for (origin, [content]) in lc_re.captures_iter(&self.time_format).map(|c| c.extract()) {
            time_lcs.push((origin.to_string(), content.to_string()))
        }

        let mut time_colors = vec![];

        for (origin, [color, content]) in colors_re
            .captures_iter(&self.time_format)
            .map(|c| c.extract())
        {
            time_colors.push((
                origin,
                color.parse::<LoggerColor>().unwrap_or_default(),
                content,
            ))
        }

        let mut lcs = vec![];

        for (origin, [content]) in lc_re.captures_iter(&self.pattern).map(|c| c.extract()) {
            lcs.push((origin.to_string(), content.to_string()))
        }

        let mut replaced = self.pattern.clone();

        colors.iter().for_each(|color| {
            replaced = replaced.replace(color.0, color.2);
        });

        lcs.iter().for_each(|lc| {
            replaced = replaced.replace(&lc.0, &lc.1);
        });

        let mut time_format = self.time_format.clone();

        time_colors.iter().for_each(|color| {
            time_format = time_format.replace(color.0, color.2);
        });

        time_lcs.iter().for_each(|lc| {
            time_format = time_format.replace(&lc.0, &lc.1);
        });

        if self.with_colors {
            colors.iter().for_each(|color| {
                let new = replaced.split(color.2).collect::<Vec<&str>>();
                replaced = format!("{}{}{}", new[0], color.2.color(color.1 .0), new[1]);
            });

            time_colors.iter().for_each(|color| {
                let time_new = time_format.split(color.2).collect::<Vec<&str>>();
                time_format = format!(
                    "{}{}{}",
                    time_new[0],
                    color.2.color(color.1 .0),
                    time_new[1]
                );
            });
        }

        let message = Message {
            parsed: replaced,
            time_format,
            logger_colors: self.lc_color.clone(),
            with_colors: self.with_colors,
            lc: lcs,
            time_lc: time_lcs,
        };

        Ok(message)
    }
}

#[derive(Debug, Default)]
pub struct Message {
    parsed: String,
    time_format: String,
    logger_colors: Option<LoggerColors>,
    with_colors: bool,
    lc: Vec<(String, String)>,
    time_lc: Vec<(String, String)>,
}

impl Message {
    pub fn format_parsed(&self, record: &log::Record<'_>) -> String {
        let mut replaced = self.parsed.clone();

        let mut time_format = self.time_format.clone();
        if self.with_colors {
            if let Some(lc_color) = &self.logger_colors {
                let level = match record.metadata().level() {
                    log::Level::Error => &lc_color.error,
                    log::Level::Warn => &lc_color.warn,
                    log::Level::Info => &lc_color.info,
                    log::Level::Debug => &lc_color.debug,
                    log::Level::Trace => &lc_color.trace,
                };

                self.lc.iter().for_each(|lc| {
                    let new = replaced.split(&lc.1).collect::<Vec<&str>>();
                    replaced = format!("{}{}{}", new[0], lc.1.color(level.color.0), new[1]);
                });

                self.time_lc.iter().for_each(|lc| {
                    let time_new = time_format.split(&lc.1).collect::<Vec<&str>>();
                    time_format = format!(
                        "{}{}{}",
                        time_new[0],
                        lc.1.color(level.color.0),
                        time_new[1]
                    );
                });
            }
        }

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
                "{time}",
                &chrono::Local::now().format(&time_format).to_string(),
            )
        }

        replaced
    }
}
