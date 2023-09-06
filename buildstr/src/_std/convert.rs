use crate::BuildStr;

impl<T: ?Sized> BuildStr for &dyn std::convert::AsRef<T> where for<'a> &'a T: BuildStr {
    fn to_build_string(&self) -> String {
        let r = self.as_ref().to_build_string();
        let ty = std::any::type_name::<T>();
        format!("&{r} as &dyn ::std::convert::AsRef<{ty}>")
    }
}