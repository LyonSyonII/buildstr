use crate::BuildStr;

impl BuildStr for ::core::fmt::Arguments<'_> {
    fn to_build_string(&self) -> String {
        format!("::core::format_args!(\"{self}\")")
    }
}

impl BuildStr for ::std::fmt::Error {
    fn to_build_string(&self) -> String {
        "::std::fmt::Error".into()
    }
}

impl BuildStr for ::core::fmt::Alignment {
    fn to_build_string(&self) -> String {
        match self {
            ::core::fmt::Alignment::Left => "::core::fmt::Alignment::Left",
            ::core::fmt::Alignment::Right => "::core::fmt::Alignment::Right",
            ::core::fmt::Alignment::Center => "::core::fmt::Alignment::Center",
        }
        .to_string()
    }
}