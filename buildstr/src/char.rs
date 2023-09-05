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
                }
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
        let c = crate::__private::unescape::to_char(self.to_string())
            .unwrap()
            .to_build_string();
        format!("::std::primitive::char::escape_debug({c})")
    }
}

impl BuildStr for std::char::EscapeDefault {
    fn to_build_string(&self) -> String {
        let c = crate::__private::unescape::to_char(self.to_string())
            .unwrap()
            .to_build_string();
        format!("::std::primitive::char::escape_default({c})")
    }
}

impl BuildStr for std::char::EscapeUnicode {
    fn to_build_string(&self) -> String {
        let c = crate::__private::unescape::to_char(self.to_string())
            .unwrap()
            .to_build_string();
        format!("::std::primitive::char::escape_unicode({c})")
    }
}

impl BuildStr for std::char::ParseCharError {
    fn to_build_string(&self) -> String {
        let s = self.to_string();
        match s.as_str() {
            "cannot parse char from empty string" => {
                "<::std::primitive::char as ::std::str::FromStr>::from_str(\"\").unwrap_err()"
            }
            "too many characters in string" => {
                "<::std::primitive::char as ::std::str::FromStr>::from_str(\"aa\").unwrap_err()"
            }
            _ => unreachable!("::std::char::CharErrorKind doesn't have more variants. {s}"),
        }
        .into()
    }
}

impl BuildStr for std::char::ToLowercase {
    fn to_build_string(&self) -> String {
        let s = self.to_string();
        if let Ok(c) = s.parse::<char>() {
            return format!(
                "::std::primitive::char::to_lowercase({})",
                c.to_build_string()
            );
        }

        // Based on `core::unicode::unicode_data::conversions::LOWERCASE_TABLE_MULTI`
        match s.as_str() {
            "i\u{307}" => "::std::primitive::char::to_lowercase('İ')".into(),
            _ => panic!("{s:?} is not handled. Please, open an issue at https://github.com/lyonsyonii/buildstr.")
        }
    }
}

impl BuildStr for std::char::ToUppercase {
    fn to_build_string(&self) -> String {
        let s = self.to_string();
        if let Ok(c) = s.parse::<char>() {
            return format!(
                "::std::primitive::char::to_uppercase({})",
                c.to_build_string()
            );
        }

        let mut chars = self.clone().collect::<Vec<_>>();
        chars.resize(3, '\0');
        let i = UPPERCASE_TABLE_MULTI
            .iter()
            .position(|&c| chars == c)
            .unwrap_or_else(|| panic!("{chars:?} is not handled. Please, open an issue at https://github.com/lyonsyonii/buildstr."));

        let c = UPPERCASE_TABLE[i].to_build_string();
        format!("::std::primitive::char::to_uppercase({c})")
    }
}

static UPPERCASE_TABLE: &[char] = &[
    '\u{df}', '\u{149}', '\u{1f0}', '\u{390}', '\u{3b0}', '\u{587}', '\u{1e96}', '\u{1e97}',
    '\u{1e99}', '\u{1e9a}', '\u{1f52}', '\u{1f56}', '\u{1f80}', '\u{1f81}', '\u{1f83}', '\u{1f84}',
    '\u{1f85}', '\u{1f87}', '\u{1f88}', '\u{1f89}', '\u{1f8b}', '\u{1f8c}', '\u{1f8d}', '\u{1f8f}',
    '\u{1f90}', '\u{1f91}', '\u{1f93}', '\u{1f94}', '\u{1f95}', '\u{1f97}', '\u{1f98}', '\u{1f99}',
    '\u{1f9b}', '\u{1f9c}', '\u{1f9d}', '\u{1f9f}', '\u{1fa0}', '\u{1fa1}', '\u{1fa3}', '\u{1fa4}',
    '\u{1fa5}', '\u{1fa7}', '\u{1fa8}', '\u{1fa9}', '\u{1fab}', '\u{1fac}', '\u{1fad}', '\u{1faf}',
    '\u{1fb3}', '\u{1fb4}', '\u{1fb6}', '\u{1fbc}', '\u{1fc2}', '\u{1fc4}', '\u{1fc6}', '\u{1fc7}',
    '\u{1fd2}', '\u{1fd6}', '\u{1fd7}', '\u{1fe2}', '\u{1fe3}', '\u{1fe4}', '\u{1fe6}', '\u{1fe7}',
    '\u{1ff2}', '\u{1ff4}', '\u{1ff6}', '\u{1ff7}', '\u{fb00}', '\u{fb01}', '\u{fb02}', '\u{fb04}',
    '\u{fb05}',
];

static UPPERCASE_TABLE_MULTI: &[[char; 3]] = &[
    ['S', 'S', '\u{0}'],
    ['\u{2bc}', 'N', '\u{0}'],
    ['J', '\u{30c}', '\u{0}'],
    ['\u{399}', '\u{308}', '\u{301}'],
    ['\u{3a5}', '\u{308}', '\u{301}'],
    ['\u{535}', '\u{552}', '\u{0}'],
    ['H', '\u{331}', '\u{0}'],
    ['T', '\u{308}', '\u{0}'],
    ['W', '\u{30a}', '\u{0}'],
    ['Y', '\u{30a}', '\u{0}'],
    ['A', '\u{2be}', '\u{0}'],
    ['\u{3a5}', '\u{313}', '\u{0}'],
    ['\u{3a5}', '\u{313}', '\u{300}'],
    ['\u{3a5}', '\u{313}', '\u{301}'],
    ['\u{3a5}', '\u{313}', '\u{342}'],
    ['\u{1f08}', '\u{399}', '\u{0}'],
    ['\u{1f09}', '\u{399}', '\u{0}'],
    ['\u{1f0a}', '\u{399}', '\u{0}'],
    ['\u{1f0b}', '\u{399}', '\u{0}'],
    ['\u{1f0c}', '\u{399}', '\u{0}'],
    ['\u{1f0d}', '\u{399}', '\u{0}'],
    ['\u{1f0e}', '\u{399}', '\u{0}'],
    ['\u{1f0f}', '\u{399}', '\u{0}'],
    ['\u{1f08}', '\u{399}', '\u{0}'],
    ['\u{1f09}', '\u{399}', '\u{0}'],
    ['\u{1f0a}', '\u{399}', '\u{0}'],
    ['\u{1f0b}', '\u{399}', '\u{0}'],
    ['\u{1f0c}', '\u{399}', '\u{0}'],
    ['\u{1f0d}', '\u{399}', '\u{0}'],
    ['\u{1f0e}', '\u{399}', '\u{0}'],
    ['\u{1f0f}', '\u{399}', '\u{0}'],
    ['\u{1f28}', '\u{399}', '\u{0}'],
    ['\u{1f29}', '\u{399}', '\u{0}'],
    ['\u{1f2a}', '\u{399}', '\u{0}'],
    ['\u{1f2b}', '\u{399}', '\u{0}'],
    ['\u{1f2c}', '\u{399}', '\u{0}'],
    ['\u{1f2d}', '\u{399}', '\u{0}'],
    ['\u{1f2e}', '\u{399}', '\u{0}'],
    ['\u{1f2f}', '\u{399}', '\u{0}'],
    ['\u{1f28}', '\u{399}', '\u{0}'],
    ['\u{1f29}', '\u{399}', '\u{0}'],
    ['\u{1f2a}', '\u{399}', '\u{0}'],
    ['\u{1f2b}', '\u{399}', '\u{0}'],
    ['\u{1f2c}', '\u{399}', '\u{0}'],
    ['\u{1f2d}', '\u{399}', '\u{0}'],
    ['\u{1f2e}', '\u{399}', '\u{0}'],
    ['\u{1f2f}', '\u{399}', '\u{0}'],
    ['\u{1f68}', '\u{399}', '\u{0}'],
    ['\u{1f69}', '\u{399}', '\u{0}'],
    ['\u{1f6a}', '\u{399}', '\u{0}'],
    ['\u{1f6b}', '\u{399}', '\u{0}'],
    ['\u{1f6c}', '\u{399}', '\u{0}'],
    ['\u{1f6d}', '\u{399}', '\u{0}'],
    ['\u{1f6e}', '\u{399}', '\u{0}'],
    ['\u{1f6f}', '\u{399}', '\u{0}'],
    ['\u{1f68}', '\u{399}', '\u{0}'],
    ['\u{1f69}', '\u{399}', '\u{0}'],
    ['\u{1f6a}', '\u{399}', '\u{0}'],
    ['\u{1f6b}', '\u{399}', '\u{0}'],
    ['\u{1f6c}', '\u{399}', '\u{0}'],
    ['\u{1f6d}', '\u{399}', '\u{0}'],
    ['\u{1f6e}', '\u{399}', '\u{0}'],
    ['\u{1f6f}', '\u{399}', '\u{0}'],
    ['\u{1fba}', '\u{399}', '\u{0}'],
    ['\u{391}', '\u{399}', '\u{0}'],
    ['\u{386}', '\u{399}', '\u{0}'],
    ['\u{391}', '\u{342}', '\u{0}'],
    ['\u{391}', '\u{342}', '\u{399}'],
    ['\u{391}', '\u{399}', '\u{0}'],
    ['\u{1fca}', '\u{399}', '\u{0}'],
    ['\u{397}', '\u{399}', '\u{0}'],
    ['\u{389}', '\u{399}', '\u{0}'],
    ['\u{397}', '\u{342}', '\u{0}'],
    ['\u{397}', '\u{342}', '\u{399}'],
    ['\u{397}', '\u{399}', '\u{0}'],
    ['\u{399}', '\u{308}', '\u{300}'],
    ['\u{399}', '\u{308}', '\u{301}'],
    ['\u{399}', '\u{342}', '\u{0}'],
    ['\u{399}', '\u{308}', '\u{342}'],
    ['\u{3a5}', '\u{308}', '\u{300}'],
    ['\u{3a5}', '\u{308}', '\u{301}'],
    ['\u{3a1}', '\u{313}', '\u{0}'],
    ['\u{3a5}', '\u{342}', '\u{0}'],
    ['\u{3a5}', '\u{308}', '\u{342}'],
    ['\u{1ffa}', '\u{399}', '\u{0}'],
    ['\u{3a9}', '\u{399}', '\u{0}'],
    ['\u{38f}', '\u{399}', '\u{0}'],
    ['\u{3a9}', '\u{342}', '\u{0}'],
    ['\u{3a9}', '\u{342}', '\u{399}'],
    ['\u{3a9}', '\u{399}', '\u{0}'],
    ['F', 'F', '\u{0}'],
    ['F', 'I', '\u{0}'],
    ['F', 'L', '\u{0}'],
    ['F', 'F', 'I'],
    ['F', 'F', 'L'],
    ['S', 'T', '\u{0}'],
    ['S', 'T', '\u{0}'],
    ['\u{544}', '\u{546}', '\u{0}'],
    ['\u{544}', '\u{535}', '\u{0}'],
    ['\u{544}', '\u{53b}', '\u{0}'],
    ['\u{54e}', '\u{546}', '\u{0}'],
    ['\u{544}', '\u{53d}', '\u{0}'],
];
