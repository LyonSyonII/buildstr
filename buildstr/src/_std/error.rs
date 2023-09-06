use crate::BuildStr;

impl BuildStr for &dyn std::error::Error {
    fn to_build_string(&self) -> String {
        let s = self.to_string();
        format!(r#"<&::std::primitive::str as ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error>>>::into("{s}").as_ref()"#)
    }
}