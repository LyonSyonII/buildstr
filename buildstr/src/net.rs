use crate::BuildStr;

impl BuildStr for ::std::net::IpAddr {
    fn to_build_string(&self) -> String {
        match self {
            ::std::net::IpAddr::V4(v) => format!("::std::net::IpAddr::V4({})", v.to_build_string()),
            ::std::net::IpAddr::V6(v) => format!("::std::net::IpAddr::V6({})", v.to_build_string()),
        }
    }
}
impl BuildStr for ::std::net::Ipv4Addr {
    fn to_build_string(&self) -> String {
        format!("::std::net::Ipv4Addr::from({})", self.octets().to_build_string())
    }
}
impl BuildStr for ::std::net::Ipv6Addr {
    fn to_build_string(&self) -> String {
        format!("::std::net::Ipv6Addr::from({})", self.octets().to_build_string())
    }
}
impl BuildStr for ::std::net::Shutdown {
    fn to_build_string(&self) -> String {
        match self {
            ::std::net::Shutdown::Read => "::std::net::Shutdown::Read",
            ::std::net::Shutdown::Write => "::std::net::Shutdown::Write",
            ::std::net::Shutdown::Both => "::std::net::Shutdown::Both",
        }.into()
    }
}
impl BuildStr for ::std::net::SocketAddr {
    fn to_build_string(&self) -> String {
        match self {
            ::std::net::SocketAddr::V4(v) => format!("::std::net::SocketAddr::V4({})", v.to_build_string()),
            ::std::net::SocketAddr::V6(v) => format!("::std::net::SocketAddr::V6({})", v.to_build_string())
        }
    }
}
impl BuildStr for ::std::net::SocketAddrV4 {
    fn to_build_string(&self) -> String {
        format!("::std::net::SocketAddrV4::new({}, {})", self.ip().to_build_string(), self.port())
    }
}
impl BuildStr for ::std::net::SocketAddrV6 {
    fn to_build_string(&self) -> String {
        let ip = self.ip().to_build_string();
        let port = self.port();
        let flowinfo = self.flowinfo();
        let scope_id = self.scope_id();
        format!("::std::net::SocketAddrV6::new({ip}, {port}, {flowinfo}, {scope_id})")
    }
}