use buildstr::BuildStr;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[test]
fn cstr() {
    assert_eq!(
        std::ffi::CStr::from_bytes_with_nul(b"hello\0").unwrap().to_build_string(),
        "::std::ffi::CStr::from_bytes_with_nul(&[104u8,101u8,108u8,108u8,111u8,0u8,]).unwrap()"
    );
    assert_eq!(
        ::std::ffi::CStr::from_bytes_with_nul(&[104u8,101u8,108u8,108u8,111u8,0u8,]).unwrap().to_build_string(),
        "::std::ffi::CStr::from_bytes_with_nul(&[104u8,101u8,108u8,108u8,111u8,0u8,]).unwrap()"
    );
}

#[test]
fn cstring() {
    assert_eq!(
        std::ffi::CString::new(*b"hello").unwrap().to_build_string(),
        "::std::ffi::CString::new([104u8,101u8,108u8,108u8,111u8,]).unwrap()"
    );
    assert_eq!(
        ::std::ffi::CString::new([104u8,101u8,108u8,108u8,111u8,]).unwrap().to_build_string(),
        "::std::ffi::CString::new([104u8,101u8,108u8,108u8,111u8,]).unwrap()"
    )
}

#[test]
fn osstr() {
    assert_eq!(
        ::std::ffi::OsStr::new("hello").to_build_string(),
        "::std::ffi::OsStr::new(\"hello\")"
    );
}

#[test]
fn osstring() {
    assert_eq!(
        ::std::ffi::OsString::from("hello").to_build_string(),
        "::std::ffi::OsString::from(\"hello\")"
    );
}

#[test]
fn from_bytes_with_nul_error() {
    assert_eq!(
        ::std::ffi::CStr::from_bytes_with_nul(b"data provided contains \0an interior nul byte at byte pos 23").unwrap_err().to_build_string(),
        "::std::ffi::CStr::from_bytes_with_nul(\"aaaaaaaaaaaaaaaaaaaaaaa\0a\").unwrap_err()"
    );
    assert_eq!(
        ::std::ffi::CStr::from_bytes_with_nul(b"data provided is not nul terminated").unwrap_err().to_build_string(),
        "::std::ffi::CStr::from_bytes_with_nul(\"\").unwrap_err()"
    );
}

#[test]
fn from_vec_with_nul_error() {
    assert_eq!(
        ::std::ffi::CString::from_vec_with_nul(b"\0a".to_vec()).unwrap_err().to_build_string(),
        "::std::ffi::CString::from_vec_with_nul(::std::vec::Vec::from_iter([0u8,97u8,])).unwrap_err()"
    );
    assert_eq!(
        ::std::ffi::CString::from_vec_with_nul(::std::vec::Vec::from_iter([0u8,97u8,])).unwrap_err().to_build_string(),
        "::std::ffi::CString::from_vec_with_nul(::std::vec::Vec::from_iter([0u8,97u8,])).unwrap_err()"
    );
    assert_eq!(
        ::std::ffi::CString::from_vec_with_nul(b"a".to_vec()).unwrap_err().to_build_string(),
        "::std::ffi::CString::from_vec_with_nul(::std::vec::Vec::from_iter([97u8,])).unwrap_err()"
    );
    assert_eq!(
        ::std::ffi::CString::from_vec_with_nul(::std::vec::Vec::from_iter([97u8,])).unwrap_err().to_build_string(),
        "::std::ffi::CString::from_vec_with_nul(::std::vec::Vec::from_iter([97u8,])).unwrap_err()"
    )
}

#[test]
fn into_string_error() {
    assert_eq!(
        std::ffi::CString::new(vec![b'f', 0xff, b'o', b'o']).unwrap().into_string().unwrap_err().to_build_string(),
        "::std::ffi::CString::new([102u8,255u8,111u8,111u8,]).unwrap().into_string().unwrap_err()"
    );
    assert_eq!(
        ::std::ffi::CString::new([102u8,255u8,111u8,111u8,]).unwrap().into_string().unwrap_err().to_build_string(),
        "::std::ffi::CString::new([102u8,255u8,111u8,111u8,]).unwrap().into_string().unwrap_err()"
    );
}


#[test]
fn nul_error() {
    assert_eq!(
        std::ffi::CString::new("f\0oo").unwrap_err().to_build_string(),
        "::std::ffi::CString::new([102u8,0u8,111u8,111u8,]).unwrap_err()"
    );
    assert_eq!(
        ::std::ffi::CString::new([102u8,0u8,111u8,111u8,]).unwrap_err().to_build_string(),
        "::std::ffi::CString::new([102u8,0u8,111u8,111u8,]).unwrap_err()"
    );
}

#[test]
fn c_void() {
    let void = unsafe { std::mem::transmute::<u8, std::ffi::c_void>(0u8) };
    assert_eq!(
        void.to_build_string(),
        "unsafe{std::mem::transmute::<u8,::std::ffi::c_void>(0)}"
    )
}