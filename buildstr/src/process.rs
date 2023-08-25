use buildstr::BuildStr;

impl BuildStr for std::process::Command {
    fn to_build_string(&self) -> String {
        let program = self.get_program().to_build_string();
        let args = self.get_args().collect::<Vec<_>>().to_build_string();
        let mut envs = self.get_envs().collect::<Vec<_>>();
        let mut removed = Vec::new();
        for i in 0..envs.len() {
            if envs[i].1.is_none() {
                removed.push(envs.remove(i).0)
            }
        }
        let envs = envs.into_iter().map(|(k, v)| (k, v.unwrap())).collect::<Vec<_>>().to_build_string();
        let dir = self.get_current_dir();
        let mut s = format!("{{std::process::Command::new({program})");
        s = format!("{s}.args({args}).envs({envs})");
        if let Some(dir) = dir {
            s = format!("{s}.current_dir({})", dir.to_build_string());
        }
        for k in removed {
            s = format!("{s}.env_remove({})", k.to_build_string());
        }
        s.push('}');
        s
        // let command = Self::new(program).args(args).envs(envs).current_dir(dir);
    }
}