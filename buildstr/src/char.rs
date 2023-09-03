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
                    if ok.len_utf16() == 1 {
                        let mut dst = [0; 1];
                        ok.encode_utf16(&mut dst);
                        chars.push(dst[0]);
                    } else {
                        let mut dst = [0; 2];
                        ok.encode_utf16(&mut dst);
                        chars.extend_from_slice(&dst);
                    }
                },
                Err(err) => chars.push(err.unpaired_surrogate()),
            }
        }
        let chars = buildstr::array_to_build_string!(chars);
        format!("::std::primitive::char::decode_utf16([{chars}])")
    }
}