use crate::BuildStr;

impl BuildStr for ::core::marker::PhantomPinned {
    fn to_build_string(&self) -> String {
        "::core::marker::PhantomPinned".into()
    }
}