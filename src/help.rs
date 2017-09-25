trait FixStyle {
    fn fix_style(&self) -> String;
    fn fix_style_left(&self) -> String;
    fn fix_style_right(&self) -> String;
}
impl<'a, S: AsRef<str>> FixStyle for S {
    fn fix_style(&self) -> String {
        let msg = self.as_ref();
        if msg.trim().is_empty() {
            String::new()
        } else {
            format!("\n{}\n", msg.trim())
        }
    }
    fn fix_style_left(&self) -> String {
        let msg = self.as_ref();
        if msg.trim().is_empty() {
            String::new()
        } else {
            format!("\n{}", msg.trim())
        }
    }
    fn fix_style_right(&self) -> String {
        let msg = self.as_ref();
        if msg.trim().is_empty() {
            String::new()
        } else {
            format!("{}\n", msg.trim())
        }
    }
}

impl Helps {
    /// `-h/--help`
    pub fn version(&self) -> &str {
        self.version.as_str()
    }
    /// `Main`
    pub fn help(&self) -> String {
        self.help_cmd(&None)
    }
    /// `Cmd`
    pub fn help_cmd(&self, cmd_name: &Option<String>) -> String {
        dbln!(
            "{:?}\n{:?}\n\n{:?},\n\n{:?}\n\n{:?}",
            cmd_name,
            self.cmd_infos,
            self.cmd_usages,
            self.cmd_options,
            self.cmd_args
        );
        let info = &self.cmd_infos[cmd_name];
        let usages = &self.cmd_usages[cmd_name];
        let options = &self.cmd_options[cmd_name];
        let args = self.cmd_args.get(cmd_name).map(|s| s.as_str()).unwrap_or(
            "",
        );
        let main_sub_cmds = if cmd_name.is_some() {
            ""
        } else {
            &self.sub_cmds
        };
        format!(
            r#"{}{}{}{}{}{}{}"#,
            info.fix_style(),
            self.author.fix_style_right(),
            self.addrs.fix_style_right(),
            usages.fix_style(),
            options.fix_style(),
            args.fix_style(),
            main_sub_cmds.fix_style()
        )
    }
}

///**`Helps`**
#[derive(Debug, Default)]
pub struct Helps {
    /// `-v/--version`  "name version"
    pub version: String,
    /// `INFO`
    pub cmd_infos: Map<Option<String>, String>,
    /// `AUTHOR`
    pub author: String,
    /// `ADDRESS`
    pub addrs: String,
    /// `USAGE`
    pub cmd_usages: Map<Option<String>, String>,
    /// `OPTIONS`
    pub cmd_options: Map<Option<String>, String>,
    /// `ARGS`
    pub cmd_args: Map<Option<String>, String>,
    pub sub_cmds: String,
}

/// **`Helper`**
#[derive(Debug, Default)]
pub struct Helper {
    is_built: bool,
    // info
    name: String,
    version: String,
    authors: Vec<(String, String)>, // (name,email)
    addrs: Vec<(String, String)>, // (addr_name,addr)
    desc: String,
    // env_vars
    current_exe: Option<String>,
    current_dir: Option<String>,
    home_dir: Option<String>,
    temp_dir: String,
    //  current_cmd
    current_cmd: Option<String>, //main is None
    current_cmd_sort_key: Option<String>,
    // args_len
    args_len:usize,
    helps: Helps,
}

impl Helper {
    /// name
    pub fn name(&self) -> &String {
        &self.name
    }
    /// version
    pub fn version(&self) -> &String {
        &self.version
    }
    /// description
    pub fn desc(&self) -> &String {
        &self.desc
    }
    /// name, email
    pub fn authors(&self) -> &Vec<(String, String)> {
        &self.authors
    }
    /// url_name, url
    pub fn addrs(&self) -> &Vec<(String, String)> {
        &self.addrs
    }
}

impl Helper {
    pub fn current_cmd(&self) -> Option<&String> {
        self.current_cmd.as_ref()
    }
    pub fn current_cmd_str(&self) -> Option<&str> {
        self.current_cmd.as_ref().map(|s| s.as_str())
    }
    pub fn current_cmd_ref(&self) -> &Option<String> {
        &self.current_cmd
    }
    pub fn current_exe(&self) -> Option<&String> {
        self.current_exe.as_ref()
    }
    pub fn current_dir(&self) -> Option<&String> {
        self.current_dir.as_ref()
    }
    pub fn home_dir(&self) -> Option<&String> {
        self.home_dir.as_ref()
    }
    pub fn temp_dir(&self) -> &String {
        &self.temp_dir
    }
    pub fn args_len(&self)->&usize {
        &self.args_len
    }
    pub fn as_helps(&self) -> &Helps {
        &self.helps
    }
    pub fn as_mut_helps(&mut self) -> &mut Helps {
        &mut self.helps
    }
}

impl Helper {
    /// exit with the `status`
    pub fn exit(&self, status: i32) {
        exit(status);
    }
    /// `format!("{}  {}", self.name(), self.version())`
    pub fn ver(&self) -> &String {
        &self.helps.version
    }
    /// print ver(`self.ver`) message and exit with the `status`
    pub fn ver_exit(&self, status: i32) {
        println!("{}", self.ver().trim());
        exit(status);
    }
    /// `format!("ERROR:\n  {}\n\n", error)`
    pub fn err<E>(&self, error: E) -> String
    where
        E: AsRef<str> + Display,
    {
        format!("ERROR:\n   {}\n\n", error)
    }
    /// print error(`self.err(error)`) message to `stderr` and exit with the `status`
    pub fn err_exit<E>(&self, error: E, status: i32)
    where
        E: AsRef<str> + Display,
    {
        self.err_line_print(&self.err(error), statics::error_line_color_get());
        exit(status);
    }
    /// print error message line(2) with Red color(fg)
    #[inline]
    pub fn err_line_print(&self, msg: &str, line_color: u16) {
        for (i, line) in msg.trim().lines().enumerate() {
            if i == 1 {
                let mut t = term::stderr().unwrap();
                t.fg(line_color).unwrap();
                write!(t, "{}", line).unwrap();
                t.reset().unwrap();
            } else {
                errln!("{}", line);
            }
        }
    }
    /// main's help mesage
    pub fn help(&self) -> String {
        self.helps.help()
    }
    /// print main's help message and exit with the `status`
    pub fn help_exit(&self, status: i32) {
        println!("{}", self.help().trim());
        exit(status);
    }
    /// `self.err(error) + self.help()`
    pub fn help_err<E>(&self, error: E) -> String
    where
        E: AsRef<str> + Display,
    {
        self.err(error) + &self.help()
    }
    /// print error and help message(`self.help_err(error)`) to `stderr` and exit with the `status`
    pub fn help_err_exit<E>(&self, error: E, status: i32)
    where
        E: AsRef<str> + Display,
    {
        self.err_line_print(&self.help_err(error), statics::error_line_color_get());
        exit(status);
    }
    /// get sub_command's help message
    pub fn help_cmd(&self, cmd_name: &Option<String>) -> String {
        self.helps.help_cmd(cmd_name)
    }
    /// print sub_command's help message and exit with the `status`
    pub fn help_cmd_exit(&self, cmd_name: &Option<String>, status: i32) {
        println!("{}", self.helps.help_cmd(cmd_name).trim());
        exit(status);
    }
    /// `self.err(error) + self.help_cmd(cmd_name)`
    pub fn help_cmd_err<E>(&self, cmd_name: &Option<String>, error: E) -> String
    where
        E: AsRef<str> + Display,
    {
        self.err(error) + &self.helps.help_cmd(cmd_name)
    }
    /// print error and sub_command's help message to `stderr`,s exit with the `status`
    pub fn help_cmd_err_exit<E>(&self, cmd_name: &Option<String>, error: E, status: i32)
    where
        E: AsRef<str> + Display,
    {
        self.err_line_print(
            &self.help_cmd_err(cmd_name, error),
            statics::error_line_color_get(),
        );
        exit(status);
    }
}