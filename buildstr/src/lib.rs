#[cfg(feature = "derive")]
pub use buildstr_derive::BuildStr;

pub use buildstr_derive::impl_buildstr;

#[cfg(feature = "prelude")]
mod primitives;

#[cfg(feature = "prelude")]
mod string;

extern crate self as buildstr;

/// Transforms an iterable of a single value to an array-like sequence without the enclosing brackets.
/// 
/// # Examples
/// ```
/// use buildstr::BuildStr;
/// use buildstr::array_to_build_string;
/// 
/// let array = array_to_build_string!(&[1, 2, 3]);
/// assert_eq!(array, "1i32,2i32,3i32,");
/// ```
#[macro_export]
macro_rules! array_to_build_string {
    ($array:expr) => {
        {
            // TODO: Test fails, why?
            let mut s = String::new();
            let array = $array;
            let array = array.iter().map(|x| x.to_build_string());
            for a in array {
                s.push_str(&a);
                s.push(',');
            }
            s
        }
    };
}

/// Transforms an iterable of a tuple of size two to an array-like sequence without the enclosing brackets.
/// 
/// # Examples
/// ```
/// use buildstr::map_to_build_string;
/// use buildstr::BuildStr;
/// 
/// let map = [("one", 1), ("two", 2), ("three", 3)];
/// assert_eq!(map_to_build_string!(map), "(\"one\",1i32),(\"two\",2i32),(\"three\",3i32),");
/// ```
#[macro_export]
macro_rules! map_to_build_string {
    ($map:ident) => {{
        let mut s = String::new();
        let map = $map
            .iter()
            .map(|(k, v)| format!("({},{})", k.to_build_string(), v.to_build_string()));
        for m in map {
            s.push_str(&m);
            s.push(',');
        }
        s
    }};
}

#[cfg(feature = "pretty")]
#[doc(hidden)]
pub fn __pretty(code: String) -> String {
    let expr = syn::parse_str(&code).unwrap();
    prettier_please::unparse_expr(&expr)
}

impl_buildstr!(BuildStr);

fn net() {
    impl BuildStr for std::net::IpAddr {
        fn to_build_string(&self) -> String {
            match self {
                std::net::IpAddr::V4(v) => format!("std::net::IpAddr::V4({})", v.to_build_string()),
                std::net::IpAddr::V6(v) => format!("std::net::IpAddr::V6({})", v.to_build_string()),
            }
        }
    }
    impl BuildStr for std::net::Ipv4Addr {
        fn to_build_string(&self) -> String {
            format!("std::net::Ipv4Addr::from({})", self.octets().to_build_string())
        }
    }
    impl BuildStr for std::net::Ipv6Addr {
        fn to_build_string(&self) -> String {
            format!("std::net::Ipv6Addr::from({})", self.octets().to_build_string())
        }
    }
    impl BuildStr for std::net::Shutdown {
        fn to_build_string(&self) -> String {
            match self {
                std::net::Shutdown::Read => "std::net::Shutdown::Read",
                std::net::Shutdown::Write => "std::net::Shutdown::Write",
                std::net::Shutdown::Both => "std::net::Shutdown::Both",
            }.into()
        }
    }
    impl BuildStr for std::net::SocketAddr {
        fn to_build_string(&self) -> String {
            match self {
                std::net::SocketAddr::V4(v) => format!("std::net::SocketAddr::V4({})", v.to_build_string()),
                std::net::SocketAddr::V6(v) => format!("std::net::SocketAddr::V6({})", v.to_build_string())
            }
        }
    }
    impl BuildStr for std::net::SocketAddrV4 {
        fn to_build_string(&self) -> String {
            format!("std::net::SocketAddrV4::new({}, {})", self.ip().to_build_string(), self.port())
        }
    }
    impl BuildStr for std::net::SocketAddrV6 {
        fn to_build_string(&self) -> String {
            let ip = self.ip().to_build_string();
            let port = self.port();
            let flowinfo = self.flowinfo();
            let scope_id = self.scope_id();
            format!("std::net::SocketAddrV6::new({ip}, {port}, {flowinfo}, {scope_id})")
        }
    }
    // impl BuildStr for std::net::TcpListener {
    //     fn to_build_string(&self) -> String {
    //         format!("std::net::TcpListener::bind({})", self.local_addr().to_build_string())
    //     }
    // }
}
