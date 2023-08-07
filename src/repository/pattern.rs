use owo_colors::OwoColorize;

use crate::prelude::*;
use std::str::FromStr;

use owo_colors::colors;

use super::color::LoggerColor;

pub struct MessagePattern {}

impl MessagePattern {
    pub fn parse(s: &str, with_colors: bool, lc_color: LoggerColor) -> Result<&str> {
        let colors_re = regex::Regex::new(r"(?P<color>[A-Z][a-z]*)<(?P<content>[^>]+)>")?;
        let lc_re = regex::Regex::new(r"lc<(?P<content>[^>]+)>")?;

        let mut colors = vec![];

        for (origin, [color, content]) in colors_re.captures_iter(s).map(|c| c.extract()) {
            colors.push((
                origin,
                color.parse::<LoggerColor>().unwrap_or_default(),
                content,
            ))
        }

        let mut lcs = vec![];

        for (origin, [content]) in lc_re.captures_iter(s).map(|c| c.extract()) {
            lcs.push((origin, content))
        }

        println!("{}", s);
        println!("{:#?}", colors);
        println!("{:#?}", lcs);

        s.contains("{message}");
        s.contains("{level}");
        s.contains("{time}");
        s.contains("{source}");

        let mut replaced = s.to_string();

        colors.iter().for_each(|color| {
            replaced = replaced.replace(color.0, color.2);
        });

        lcs.iter()
            .for_each(|lc| replaced = replaced.replace(lc.0, lc.1));

        if with_colors {
            colors.iter().for_each(|color| {
                let new = replaced.split(color.2).collect::<Vec<&str>>();
                replaced = format!("{}{}{}", new[0], color.2.color(color.1 .0), new[1]);
            });

            lcs.iter().for_each(|lc| {
                let new = replaced.split(lc.1).collect::<Vec<&str>>();
                replaced = format!("{}{}{}", new[0], lc.1.color(lc_color.0), new[1]);
            });
        }

        println!("{}", replaced);

        Ok("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_test() {
        let pattern = "Yellow<[{time}]>lc<[{level}]>Red<[{source}]>: {message}";

        MessagePattern::parse(pattern, true, Wrapper(owo_colors::AnsiColors::Blue)).unwrap();
    }
}
