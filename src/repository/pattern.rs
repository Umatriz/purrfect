use std::str::FromStr;

use super::color::LoggerColor;

pub struct MessagePattern {}

impl FromStr for MessagePattern {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.contains("{message}");
        s.contains("{level}");
        s.contains("{time}");
        s.contains("{source}");

        let colors_re = regex::Regex::new(r"(?P<color>[A-Z][a-z]*)<(?P<content>[^>]+)>")?;
        let lc_re = regex::Regex::new(r"lc<(?P<content>[^>]+)>")?;

        let mut colors = vec![];

        for (_, [color, content]) in colors_re.captures_iter(s).map(|c| c.extract()) {
            colors.push((color.parse::<LoggerColor>().unwrap_or_default(), content))
        }

        let lc = lc_re.captures(s);

        println!("{}", s);
        println!("{:#?}", colors);
        println!("{:#?}", lc);

        // let f = s.split()

        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_test() {
        let pattern = "Blue<[{time}]>lc<[{level}]>Red<[{source}]>: {message}";

        pattern.parse::<MessagePattern>().unwrap();
    }
}
