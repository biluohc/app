impl<'app> App<'app> {
    fn _build_helper(&mut self) {
        if self.helper.is_built {
            return;
        } else {
            self.helper.is_built = true;
        }

        self.helper.helps.version = self._ver(1);
        self.helper.helps.author = self._help_author(3);
        self.helper.helps.addrs = self._help_address(3);
        //CAMMANDS:
        let sub_cmds = self._help_sub_cmds(3, 5);
        self.helper.helps.sub_cmds = if sub_cmds.is_empty() {
            sub_cmds
        } else {
            format!("CAMMANDS:\n{}", sub_cmds)
        };

        // CMDs
        for (k, v) in &self.cmds {
            // INFP
            let info = self._help_info(k, 1);
            self.helper.helps.cmd_infos.insert(k.clone(), info);
            // USAGE
            let usage = self._help_usage(k, 3);
            self.helper.helps.cmd_usages.insert(k.clone(), usage);
            // OPTIONS
            if !v.opts.is_empty() {
                self.helper
                    .helps
                    .cmd_options
                    .insert(k.clone(),
                            format!("OPTIONS:\n{}", v.to_opts_info().to_string(3, 5)));
            }
            // ARGS
            if !v.args.is_empty() {
                self.helper
                    .helps
                    .cmd_args
                    .insert(k.clone(),
                            format!("ARGS:\n{}", v.to_args_info().to_string(3, 5)));
            }
        }
    }
    // --version
    fn _ver(&self, blanks0: usize) -> String {
        format!("{}{}{}",
                self.helper.name.trim(),
                blanks_fix(blanks0),
                self.helper.version.trim())
    }
    // CMD_INFO
    fn _help_info(&self, cmd_name: &Option<String>, blanks0: usize) -> String {
        let version_or_subcmd = cmd_name.as_ref().unwrap_or(self.helper.version()).trim();
        format!("{}{}{}\n{}",
                self.helper.name.trim(),
                blanks_fix(blanks0),
                version_or_subcmd,
                self.cmds[cmd_name].desc.trim())
    }
    // AUTHOR
    fn _help_author(&self, blanks0: usize) -> String {
        let mut authors = String::new();
        if !self.helper.authors.is_empty() {
            authors.push_str("AUTHOR:\n");
            for &(ref author, ref email) in &self.helper.authors {
                authors.push_str(&format!("{}{} <{}>\n", blanks_fix(blanks0), author, email));
            }
        }
        authors
    }
    // ADDRESS
    fn _help_address(&self, blanks0: usize) -> String {
        let mut authors = String::new();
        if !self.helper.addrs.is_empty() {
            authors.push_str("ADDRESS:\n");
            for &(ref author, ref email) in &self.helper.addrs {
                authors.push_str(&&format!("{}{}: {}\n", blanks_fix(blanks0), author, email));
            }
        }
        authors
    }
    // CAMMANDS
    fn _help_sub_cmds(&self, blanks0: usize, blanks1: usize) -> String {
        let mut cammands = "".to_owned();
        let mut max_len = 0;
        self.cmds
            .values()
            .map(|cmd| {
                     cmd.name
                         .map(|s| if s.len() > max_len {
                                  max_len = s.len()
                              })
                 })
            .count();
        self.cmds
            .values()
            .map(|cmd| if cmd.name != None {
                     cammands.push_str(&format!("{}{}{}{}\n",
                                               blanks_fix(blanks0),
                                               cmd.name.as_ref().unwrap(),
                                               blanks_fix(blanks1 + max_len - cmd.name.as_ref().unwrap().len()),
                                               cmd.desc))
                 })
            .count();
        cammands
    }
    //CMD_USAGE
    fn _help_usage(&self, cmd_name: &Option<String>, blanks0: usize) -> String {
        let pkg = &self.helper.name;
        let none_or_cmdname = cmd_name
            .as_ref()
            .map(|s| format!(" {}", s))
            .unwrap_or("".to_owned());
        let cmd = &self.cmds[cmd_name];
        let mut usages = Vec::new();

        let mut option_optional = false;
        let mut argss = " ".to_owned();
        cmd.opts
            .values()
            .map(|opt| if opt.optional || opt.value.as_ref().default().is_some() {
                     option_optional = true;
                 })
            .count();
        cmd.args
            .as_slice()
            .iter()
            .map(|args| {
                let fmt_ = if args.optional || args.value.as_ref().default().is_some() {
                    if args.len == Some(1) {
                        format!("[{}] ", args.name)
                    } else if args.len == Some(2) {
                        format!("[{} {}] ", args.name, args.name)
                    } else if args.len == None {
                        format!("[{}...] ", args.name)
                    } else {
                        format!("[{}{{{}}}] ", args.name, args.len.as_ref().unwrap())
                    }
                } else {
                    if args.len == Some(1) {
                        format!("<{}> ", args.name)
                    } else if args.len == Some(2) {
                        format!("<{}> <{}> ", args.name, args.name)
                    } else if args.len == None {
                        format!("<{}...> ", args.name)
                    } else {
                        format!("<{}{{{}}}> ", args.name, args.len.as_ref().unwrap())
                    }
                };
                argss.push_str(&fmt_);
            })
            .count();
        if option_optional {
            usages.push(format!("{}{} [options] {}", pkg, none_or_cmdname, argss.trim()));
        } else {
            usages.push(format!("{}{} options {}", pkg, none_or_cmdname, argss.trim()));
        }
        if *cmd_name == None && self.cmds.len() > 1 {
            usages.push(format!("{} <command> [args]", pkg));
        }
        usages.as_mut_slice().sort_by(|a, b| a.len().cmp(&b.len()));
        let mut help = "USAGE:\n".to_owned();
        usages
            .as_slice()
            .iter()
            .map(|s| help.push_str(&format!("{}{}\n", blanks_fix(blanks0), s)))
            .count();
        help
    }
}

struct OptInfo(String, String);
impl<'app> Opt<'app> {
    fn to_info(&self) -> OptInfo {
        let optional_or_dafault = if self.is_optional() {
            OPTIONAL.to_owned()
        } else {
            self.value
                .as_ref()
                .default()
                .map(|s| format!("[{}]", s))
                .unwrap_or_else(String::new)
        };
        let s = self.short_get().unwrap_or_else(String::new);
        let long = self.long_get().unwrap_or_else(String::new);
        let tmp_ = if self.is_bool() {
            if s != "" && long != "" {
                format!("{}, {}  ", s, long)
            } else {
                format!("{}{}  ", long, s)
            }
        } else if s != "" && long != "" {
            format!("{}, {} <{}>{}  ", s, long, self.name, optional_or_dafault)
        } else {
            format!("{}{} <{}>{}  ", s, long, self.name, optional_or_dafault)
        };
        OptInfo(tmp_, self.help.to_string())
    }
}

struct OptsInfo(Vec<OptInfo>);
impl<'app> Cmd<'app> {
    fn to_opts_info(&self) -> OptsInfo {
        let mut vs = Vec::new();
        for v in self.opts.values() {
            vs.push(v.to_info());
        }
        OptsInfo(vs)
    }
}
impl OptsInfo {
    //  -c, --config <config>(optional)    Sets a custom config file
    fn to_string(&self, blanks0: usize, mut blanks1: usize) -> String {
        let mut max_len = 0;
        for val in &self.0 {
            if val.0.len() > max_len {
                max_len = val.0.len();
            }
        }
        blanks1 += max_len;
        let blanks0 = blanks_fix(blanks0);
        let mut s_tmp = String::new();
        for val in &self.0 {
            s_tmp.push_str(&format!("{}{}{}{}\n",
                                   blanks0,
                                   val.0,
                                   blanks_fix(blanks1 - val.0.len()),
                                   val.1));
        }
        s_tmp
    }
}

fn blanks_fix(len: usize) -> String {
    let mut s_tmp = String::new();
    for _ in 0..len {
        s_tmp.push(' ');
    }
    s_tmp
}

struct ArgsInfo(String, String);
struct ArgssInfo(Vec<ArgsInfo>);
impl<'app> Cmd<'app> {
    fn to_args_info(&self) -> ArgssInfo {
        let mut vs = Vec::new();
        for v in &self.args {
            let optional_or_dafault = if v.is_optional() {
                OPTIONAL.to_owned()
            } else {
                v.value
                    .as_ref()
                    .default()
                    .clone()
                    // vec![] -> []
                    .unwrap_or_else(String::new)
            };
            vs.push(ArgsInfo(format!("<{}>{}", v.name, optional_or_dafault),
                             v.help.to_string()));
        }
        ArgssInfo(vs)
    }
}
impl ArgssInfo {
    //  <PATHS>(optional)    Sets the paths to share(default is "./")
    fn to_string(&self, blanks0: usize, mut blanks1: usize) -> String {
        let mut max_len = 0;
        for val in &self.0 {
            if val.0.len() > max_len {
                max_len = val.0.len();
            }
        }
        blanks1 += max_len;
        let blanks0 = blanks_fix(blanks0);
        let mut s_tmp = String::new();
        for val in &self.0 {
            s_tmp.push_str(&format!("{}{}{}{}\n",
                                   blanks0,
                                   val.0,
                                   blanks_fix(blanks1 - val.0.len()),
                                   val.1));
        }
        s_tmp
    }
}
