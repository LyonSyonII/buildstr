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
        let i = UPPERCASE_TABLE.binary_search_by(|(v, _)| v.as_slice().cmp(chars.as_slice()))
            .unwrap_or_else(|_| panic!("{chars:?} is not handled. Please, open an issue at https://github.com/lyonsyonii/buildstr."));

        let c = UPPERCASE_TABLE[i].1.to_build_string();
        format!("::std::primitive::char::to_uppercase({c})")
    }
}

static UPPERCASE_TABLE: &[([char; 3], char)] = &[
    (['A', '\u{2be}', '\0'], 'ẚ'),
    (['F', 'F', '\0'], 'ﬀ'),
    (['F', 'F', 'I'], 'ﬃ'),
    (['F', 'F', 'L'], 'ﬄ'),
    (['F', 'I', '\0'], 'ﬁ'),
    (['F', 'L', '\0'], 'ﬂ'),
    (['H', '\u{331}', '\0'], 'ẖ'),
    (['J', '\u{30c}', '\0'], 'ǰ'),
    (['S', 'S', '\0'], 'ß'),
    (['S', 'T', '\0'], 'ﬅ'),
    (['S', 'T', '\0'], 'ﬆ'),
    (['T', '\u{308}', '\0'], 'ẗ'),
    (['W', '\u{30a}', '\0'], 'ẘ'),
    (['Y', '\u{30a}', '\0'], 'ẙ'),
    (['\u{2bc}', 'N', '\0'], 'ŉ'),
    (['Ά', '\u{399}', '\0'], 'ᾴ'),
    (['Ή', '\u{399}', '\0'], 'ῄ'),
    (['Ώ', '\u{399}', '\0'], 'ῴ'),
    (['\u{391}', '\u{342}', '\0'], 'ᾶ'),
    (['\u{391}', '\u{342}', '\u{399}'], 'ᾷ'),
    (['\u{391}', '\u{399}', '\0'], 'ᾳ'),
    (['\u{391}', '\u{399}', '\0'], 'ᾼ'),
    (['\u{397}', '\u{342}', '\0'], 'ῆ'),
    (['\u{397}', '\u{342}', '\u{399}'], 'ῇ'),
    (['\u{397}', '\u{399}', '\0'], 'ῃ'),
    (['\u{397}', '\u{399}', '\0'], 'ῌ'),
    (['\u{399}', '\u{308}', '\u{300}'], 'ῒ'),
    (['\u{399}', '\u{308}', '\u{301}'], 'ΐ'),
    (['\u{399}', '\u{308}', '\u{301}'], 'ΐ'),
    (['\u{399}', '\u{308}', '\u{342}'], 'ῗ'),
    (['\u{399}', '\u{342}', '\0'], 'ῖ'),
    (['\u{3a1}', '\u{313}', '\0'], 'ῤ'),
    (['\u{3a5}', '\u{308}', '\u{300}'], 'ῢ'),
    (['\u{3a5}', '\u{308}', '\u{301}'], 'ΰ'),
    (['\u{3a5}', '\u{308}', '\u{301}'], 'ΰ'),
    (['\u{3a5}', '\u{308}', '\u{342}'], 'ῧ'),
    (['\u{3a5}', '\u{313}', '\0'], 'ὐ'),
    (['\u{3a5}', '\u{313}', '\u{300}'], 'ὒ'),
    (['\u{3a5}', '\u{313}', '\u{301}'], 'ὔ'),
    (['\u{3a5}', '\u{313}', '\u{342}'], 'ὖ'),
    (['\u{3a5}', '\u{342}', '\0'], 'ῦ'),
    (['Ω', '\u{342}', '\0'], 'ῶ'),
    (['Ω', '\u{342}', '\u{399}'], 'ῷ'),
    (['Ω', '\u{399}', '\0'], 'ῳ'),
    (['Ω', '\u{399}', '\0'], 'ῼ'),
    (['Ե', 'Ւ', '\0'], 'և'),
    (['Մ', 'Ե', '\0'], 'ﬔ'),
    (['Մ', 'Ի', '\0'], 'ﬕ'),
    (['Մ', 'Խ', '\0'], 'ﬗ'),
    (['Մ', 'Ն', '\0'], 'ﬓ'),
    (['Վ', 'Ն', '\0'], 'ﬖ'),
    (['Ἀ', '\u{399}', '\0'], 'ᾀ'),
    (['Ἀ', '\u{399}', '\0'], 'ᾈ'),
    (['Ἁ', '\u{399}', '\0'], 'ᾁ'),
    (['Ἁ', '\u{399}', '\0'], 'ᾉ'),
    (['Ἂ', '\u{399}', '\0'], 'ᾂ'),
    (['Ἂ', '\u{399}', '\0'], 'ᾊ'),
    (['Ἃ', '\u{399}', '\0'], 'ᾃ'),
    (['Ἃ', '\u{399}', '\0'], 'ᾋ'),
    (['Ἄ', '\u{399}', '\0'], 'ᾄ'),
    (['Ἄ', '\u{399}', '\0'], 'ᾌ'),
    (['Ἅ', '\u{399}', '\0'], 'ᾅ'),
    (['Ἅ', '\u{399}', '\0'], 'ᾍ'),
    (['Ἆ', '\u{399}', '\0'], 'ᾆ'),
    (['Ἆ', '\u{399}', '\0'], 'ᾎ'),
    (['Ἇ', '\u{399}', '\0'], 'ᾇ'),
    (['Ἇ', '\u{399}', '\0'], 'ᾏ'),
    (['Ἠ', '\u{399}', '\0'], 'ᾐ'),
    (['Ἠ', '\u{399}', '\0'], 'ᾘ'),
    (['Ἡ', '\u{399}', '\0'], 'ᾑ'),
    (['Ἡ', '\u{399}', '\0'], 'ᾙ'),
    (['Ἢ', '\u{399}', '\0'], 'ᾒ'),
    (['Ἢ', '\u{399}', '\0'], 'ᾚ'),
    (['Ἣ', '\u{399}', '\0'], 'ᾓ'),
    (['Ἣ', '\u{399}', '\0'], 'ᾛ'),
    (['Ἤ', '\u{399}', '\0'], 'ᾔ'),
    (['Ἤ', '\u{399}', '\0'], 'ᾜ'),
    (['Ἥ', '\u{399}', '\0'], 'ᾕ'),
    (['Ἥ', '\u{399}', '\0'], 'ᾝ'),
    (['Ἦ', '\u{399}', '\0'], 'ᾖ'),
    (['Ἦ', '\u{399}', '\0'], 'ᾞ'),
    (['Ἧ', '\u{399}', '\0'], 'ᾗ'),
    (['Ἧ', '\u{399}', '\0'], 'ᾟ'),
    (['Ὠ', '\u{399}', '\0'], 'ᾠ'),
    (['Ὠ', '\u{399}', '\0'], 'ᾨ'),
    (['Ὡ', '\u{399}', '\0'], 'ᾡ'),
    (['Ὡ', '\u{399}', '\0'], 'ᾩ'),
    (['Ὢ', '\u{399}', '\0'], 'ᾢ'),
    (['Ὢ', '\u{399}', '\0'], 'ᾪ'),
    (['Ὣ', '\u{399}', '\0'], 'ᾣ'),
    (['Ὣ', '\u{399}', '\0'], 'ᾫ'),
    (['Ὤ', '\u{399}', '\0'], 'ᾤ'),
    (['Ὤ', '\u{399}', '\0'], 'ᾬ'),
    (['Ὥ', '\u{399}', '\0'], 'ᾥ'),
    (['Ὥ', '\u{399}', '\0'], 'ᾭ'),
    (['Ὦ', '\u{399}', '\0'], 'ᾦ'),
    (['Ὦ', '\u{399}', '\0'], 'ᾮ'),
    (['Ὧ', '\u{399}', '\0'], 'ᾧ'),
    (['Ὧ', '\u{399}', '\0'], 'ᾯ'),
    (['Ὰ', '\u{399}', '\0'], 'ᾲ'),
    (['Ὴ', '\u{399}', '\0'], 'ῂ'),
    (['Ὼ', '\u{399}', '\0'], 'ῲ'),
];
