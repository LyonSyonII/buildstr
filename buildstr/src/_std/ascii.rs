use crate::BuildStr;

impl BuildStr for ::std::ascii::EscapeDefault {
    fn to_build_string(&self) -> String {
        let s = crate::__private::unescape::unescape(self.to_string()).unwrap();
        let c = s.parse::<char>().expect(&s) as u8;
        format!("::std::ascii::escape_default({c})")
    }
}