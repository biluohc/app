use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::path::PathBuf;
use std::fmt::Debug;

/// **`ArgsValue`**
#[derive(Debug)]
pub struct ArgsValue<'app> {
    inner: Box<ArgsValueParse<'app> + 'app>,
}

impl<'app> ArgsValue<'app> {
    pub fn new(value: Box<ArgsValueParse<'app> + 'app>) -> Self {
        ArgsValue { inner: value }
    }
}

impl<'app> AsRef<Box<ArgsValueParse<'app> + 'app>> for ArgsValue<'app> {
    fn as_ref(&self) -> &Box<ArgsValueParse<'app> + 'app> {
        &self.inner
    }
}
impl<'app> AsMut<Box<ArgsValueParse<'app> + 'app>> for ArgsValue<'app> {
    fn as_mut(&mut self) -> &mut Box<ArgsValueParse<'app> + 'app> {
        &mut self.inner
    }
}
/// **You can use custom `ArgsValue` by `impl` it**
///
/// ### **Explain**
///
/// * `into(self)` convert it(`&mut T`)  to `ArgsValue`.
///
///
/// * `default(&self)` is `Arguments`'s default value's str for help message print
///
///
/// * `parse(&mut self, args_name: String, msg: &[String])` maintains the value, and return message by `Result<(),String>`.
///
///   `args_name` is current `cmd`'s `args_name`, `msg` is the `&[String]` need to pasre.
///
/// * `check(&self, opt_name: &str)` check value and return message by `Result<(),String>`.
pub trait ArgsValueParse<'app>: Debug {
    fn into(self) -> ArgsValue<'app>;
    fn default(&self) -> Option<String>;
    fn parse(&mut self, args_name: &str, msg: &[String]) -> Result<(), String>;
    fn check(&self, args_name: &str) -> Result<(), String>;
}

impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut Vec<String> {
    fn into(self) -> ArgsValue<'app> {
        ArgsValue { inner: Box::from(self) }
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}", self))
        }
    }
    fn parse(&mut self, _: &str, msg: &[String]) -> Result<(), String> {
        if !msg.is_empty() {
            self.clear();
        }
        msg.iter()
            .filter(|s| !s.is_empty())
            .map(|s| self.push(s.to_string()))
            .count();
        Ok(())
    }
    fn check(&self, args_name: &str) -> Result<(), String> {
        if self.is_empty() {
            Err(format!("Args(<{}>) missing", args_name))
        } else {
            Ok(())
        }
    }
}
impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut Vec<PathBuf> {
    fn into(self) -> ArgsValue<'app> {
        ArgsValue { inner: Box::from(self) }
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}", self))
        }
    }
    fn parse(&mut self, _: &str, msg: &[String]) -> Result<(), String> {
        if !msg.is_empty() {
            self.clear();
        }
        msg.iter()
            .filter(|s| !s.is_empty())
            .map(|s| self.push(PathBuf::from(s)))
            .count();
        Ok(())
    }
    fn check(&self, args_name: &str) -> Result<(), String> {
        if self.is_empty() {
            Err(format!("Args(<{}>) missing", args_name))
        } else {
            Ok(())
        }
    }
}

impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut Vec<char> {
    fn into(self) -> ArgsValue<'app> {
        ArgsValue { inner: Box::from(self) }
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}", self))
        }
    }
    fn parse(&mut self, args_name: &str, msg: &[String]) -> Result<(), String> {
        if !msg.is_empty() {
            self.clear();
        }
        for str in msg {
            let chars: Vec<char> = str.chars().collect();
            if chars.len() != 1 {
                return Err(format!("Args(<{}>): {:?} is invalid", args_name, str));
            } else {
                self.push(chars[0]);
            }
        }
        Ok(())
    }
    fn check(&self, args_name: &str) -> Result<(), String> {
        if self.is_empty() {
            Err(format!("Args(<{}>) missing", args_name))
        } else {
            Ok(())
        }
    }
}

macro_rules! add_vec_impl {
    ($($t:ty)*) => ($(
        impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut Vec<$t> {
        fn into(self) -> ArgsValue<'app> {
        ArgsValue { inner: Box::from(self) }
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}",self))
        }
    }
    fn parse(&mut self, args_name: &str, msg: &[String]) -> Result<(), String> {
        if !msg.is_empty(){ self.clear(); }
                let vs: Vec<&String> = msg.iter().filter(|s| !s.trim().is_empty()).collect();
                for str in &vs {
                    self.push(str.parse::<$t>()
                               .map_err(|_| {
                                            format!("Args(<{}>) parse<{}> fails: \"{}\"",
                                                    args_name,
                                                    stringify!($t),
                                                    str)
                                        })?)
                }
                Ok(())
    }
    fn check(&self, args_name: &str) -> Result<(), String> {
        if self.is_empty() {
            Err(format!("Args(<{}>) missing", args_name))
        } else {
            Ok(())
        }
    }
        }
    )*)
}
add_vec_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_vec_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }
