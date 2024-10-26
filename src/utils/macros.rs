/// Sets forground color of text to specified color
#[macro_export]
macro_rules! fg_color {
    ($string:literal, $color:ident) => {
        format!(
            "{}{}{}",
            styling::Style::new().fg_color(Some(styling::Color::Ansi(styling::AnsiColor::$color))),
            $string,
            styling::Reset::default().render(),
        )
    };
}
