use crate::BuildStr;

impl BuildStr for std::env::JoinPathsError {
    fn to_build_string(&self) -> String {
        r#"::std::env::join_paths([":\""]).unwrap_err()"#.into()
    }
}