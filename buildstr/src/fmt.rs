use crate::BuildStr;

impl BuildStr for core::fmt::Alignment {
    fn to_build_string(&self) -> String {
        match self {
            core::fmt::Alignment::Left => "core::fmt::Alignment::Left",
            core::fmt::Alignment::Right => "core::fmt::Alignment::Right",
            core::fmt::Alignment::Center => "core::fmt::Alignment::Center",
        }.to_string()
    }
}