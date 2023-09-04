use crate::BuildStr;

impl BuildStr for std::char::CharTryFromError {
    fn to_build_string(&self) -> String {
        "::std::primitive::char::try_from(::std::primitive::u32::MAX).unwrap_err()".into()
    }
}

impl<I: std::iter::Iterator<Item = u16> + Clone> BuildStr for std::char::DecodeUtf16<I> {
    fn to_build_string(&self) -> String {
        let mut chars = Vec::new();
        for c in self.clone() {
            match c {
                Ok(ok) => {
                    let mut v = vec![0; ok.len_utf16()];
                    ok.encode_utf16(&mut v);
                    chars.extend(v);
                },
                Err(err) => chars.push(err.unpaired_surrogate()),
            }
        }
        let chars = buildstr::array_to_build_string!(chars);
        format!("::std::primitive::char::decode_utf16([{chars}])")
    }
}

impl BuildStr for std::char::DecodeUtf16Error {
    fn to_build_string(&self) -> String {
        let code = self.unpaired_surrogate();
        format!("::std::primitive::char::decode_utf16([{code}u16]).next().unwrap().unwrap_err()")
    }
}

impl BuildStr for std::char::EscapeDebug {
    fn to_build_string(&self) -> String {
        let c = crate::__private::unescape::to_char(self.to_string()).unwrap().to_build_string();
        format!("::std::primitive::char::escape_debug({c})")
    }
}

impl BuildStr for std::char::EscapeDefault {
    fn to_build_string(&self) -> String {
        let c = crate::__private::unescape::to_char(self.to_string()).unwrap().to_build_string();
        format!("::std::primitive::char::escape_default({c})")
    }
}

impl BuildStr for std::char::EscapeUnicode {
    fn to_build_string(&self) -> String {
        let c = crate::__private::unescape::to_char(self.to_string()).unwrap().to_build_string();
        format!("::std::primitive::char::escape_unicode({c})")
    }
}

impl BuildStr for std::char::ParseCharError {
    fn to_build_string(&self) -> String {
        let s = self.to_string();
        match s.as_str() {
            "cannot parse char from empty string" => "<::std::primitive::char as ::std::str::FromStr>::from_str(\"\").unwrap_err()",
            "too many characters in string" => "<::std::primitive::char as ::std::str::FromStr>::from_str(\"aa\").unwrap_err()",
            _ => unreachable!("::std::char::CharErrorKind doesn't have more variants. {s}")
        }.into()
    }
}

impl BuildStr for std::char::ToLowercase {
    fn to_build_string(&self) -> String {
        let c = self.to_string().parse::<char>();
        match c {
            Ok(ok) => format!("::std::primitive::char::to_lowercase({})", ok.to_build_string()),
            Err(err) => todo!("[{self}] {err}"),
        }
    }
}

impl BuildStr for std::char::ToUppercase {
    fn to_build_string(&self) -> String {
        let c = self.to_string().parse::<char>();
        match c {
            Ok(ok) => format!("::std::primitive::char::to_lowercase({})", ok.to_build_string()),
            Err(err) => todo!("[{self}] {err}"),
        }
    }
}