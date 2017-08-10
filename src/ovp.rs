use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::fmt::Debug;

/// **`OptValue`**
#[derive(Debug)]
pub struct OptValue<'app> {
    inner: Box<OptValueParse<'app> + 'app>,
}

impl<'app> OptValue<'app> {
    pub fn new(value: Box<OptValueParse<'app> + 'app>) -> Self {
        OptValue { inner: value }
    }
}

impl<'app> AsRef<Box<OptValueParse<'app> + 'app>> for OptValue<'app> {
    fn as_ref(&self) -> &Box<OptValueParse<'app> + 'app> {
        &self.inner
    }
}
impl<'app> AsMut<Box<OptValueParse<'app> + 'app>> for OptValue<'app> {
    fn as_mut(&mut self) -> &mut Box<OptValueParse<'app> + 'app> {
        &mut self.inner
    }
}
/// **You can use custom `OptValue` by `impl` it**
///
/// ### **Explain**
///
/// * `into(self)` convert it(`&mut T`)  to `OptValue`.
///
///
/// * `is_bool(&self)` like `--help/-h`,they not have value follows it.
///
///    so you should return `false` except value's type is `&mut bool`(but it already defined).
///
///
/// * `default(&self)` is `Opt`'s default value's str for help message print
///
///
/// * `parse(&mut self, opt_name: String, msg: &str)` maintains the value, and return message by `Result<(),String>`.
///
///   `opt_name` is current `Opt`'s name, `msg` is the `&str` need to pasre.
///
/// * `check(&self, opt_name: &str)` check value and return message by `Result<(),String>`.
///
/// ### **Suggestion**
///
/// * `T` is suitable for options with default values.
///
///     You can initialize it using the default value.
///
/// * `Option<T>` is suitable for necessary options.
///
///     `app` will `check` them, is `value.is_none()`, `app` will `exit(1)` after print error and help message.
///
/// * `Vec<T>` is suitable a grout of comma-separated values of the same type.
///
///     `app` will `check` them, is `value.is_empty()`, `app` will `exit(1)` after print error and help message.
///
///     You can initialize it using the default value.
///
/// ```fuckrs
/// "80" -> vec[80]
/// ",80," -> vec[80]
/// ",80,," -> vec[80]
/// "8080,8000,80," -> Vec[8080,8000,80]
/// ```
pub trait OptValueParse<'app>: Debug {
    fn into(self) -> OptValue<'app>;
    fn is_bool(&self) -> bool;
    fn default(&self) -> Option<String>;
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String>;
    fn check(&self, opt_name: &str) -> Result<(), String>;
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut bool {
    fn into(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        true
    }
    fn default(&self) -> Option<String> {
        Some(format!("{}", self))
    }
    fn parse(&mut self, _: String, _: &str) -> Result<(), String> {
        **self = true;
        Ok(())
    }
    fn check(&self, _: &str) -> Result<(), String> {
        Ok(())
    }
}
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut String {
    fn into(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some((**self).to_string())
        }
    }
    fn parse(&mut self, _: String, msg: &str) -> Result<(), String> {
        **self = msg.to_string();
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_empty() {
            Err(format!("OPTION(<{}>) missing", opt_name))
        } else {
            Ok(())
        }
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut char {
    fn into(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        Some(self.to_string())
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        if msg.len() == 1 {
            **self = msg.chars().next().unwrap();
        } else {
            return Err(format!("OPTION(<{}>) parse<char> fails: \"{}\"", opt_name, msg));
        }
        Ok(())
    }
    fn check(&self, _: &str) -> Result<(), String> {
        Ok(())
    }
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl<'app, 's: 'app> OptValueParse<'app> for &'s mut $t {
        fn into(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        Some(format!("{}", self))
    }
    fn parse(&mut self, opt_name : String, msg: &str) -> Result<(), String> {
        **self = msg.parse::<$t>()
                .map_err(|_| format!("OPTION(<{}>) parse<{}> fails: \"{}\"", opt_name, stringify!($t),msg))?;
        Ok(())
    }
    fn check(&self, _ :  &str) -> Result<(), String> {
        Ok(())
    }
        }
    )*)
}

add_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<char> {
    fn into(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        self.map(|s| s.to_string())
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        if msg.len() == 1 {
            **self = Some(msg.chars().next().unwrap());
        } else {
            return Err(format!("OPTION(<{}>) parse<char> fails: \"{}\"", opt_name, msg));
        }
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))
        } else {
            Ok(())
        }
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<String> {
    fn into(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        (**self).clone()
    }
    fn parse(&mut self, _: String, msg: &str) -> Result<(), String> {
        **self = Some(msg.to_string());
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))
        } else {
            Ok(())
        }
    }
}

macro_rules! add_option_impl {
    ($($t:ty)*) => ($(
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<$t> {
    fn into(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
       self.as_ref().map(|ref s|format!("{}",s))
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        **self = Some(msg.parse::<$t>()
                          .map_err(|_| {
                                       format!("OPTION(<{}>) parse<{}> fails: \"{}\"",
                                               opt_name,
                                               stringify!($t),
                                               msg)
                                   })?);
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))
        } else {
            Ok(())
        }
    }
}
    )*)
}

add_option_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_option_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Vec<char> {
    fn into(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            let mut str = String::new();
            self.as_slice().iter().map(|s| str.push(*s)).count();
            Some(str)
        }
    }
    fn parse(&mut self, _: String, msg: &str) -> Result<(), String> {
        self.clear();
        for c in msg.chars() {
            self.push(c);
        }
        Ok(())
    }
    fn check(&self, _: &str) -> Result<(), String> {
        Ok(())
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Vec<String> {
    fn into(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            let mut str = String::new();
            self.as_slice()
                .iter()
                .map(|s| {
                         str.push_str(s);
                         str.push(',');
                     })
                .count();
            str.pop();
            Some(str)
        }
    }
    fn parse(&mut self, _: String, msg: &str) -> Result<(), String> {
        self.clear(); // What due to ?
        let _ = msg.split(',')
            .filter(|s| !s.is_empty())
            .map(|ss| self.push(ss.to_string()));
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_empty() {
            Err(format!("OPTION(<{}>) missing", opt_name))
        } else {
            Ok(())
        }
    }
}

macro_rules! add_vec_impl {
    ($($t:ty)*) => ($(
        impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Vec<$t> {
        fn into(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
        let mut str = String::new();
        self.as_slice()
            .iter()
            .map(|s| str.push_str(&format!("{},",s))).count();
        str.pop();
            Some(str)
        }
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
                self.clear();
                let vs: Vec<&str> = msg.split(',').filter(|s| !s.is_empty()).collect();
                for str in &vs {
                    self.push(str.parse::<$t>()
                               .map_err(|_| {
                                            format!("OPTION(<{}>) parse<Vec<{}>> fails: \"{}/{}\"",
                                                    opt_name,
                                                    stringify!($t),
                                                    str,
                                                    msg)
                                        })?)
                }
                Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_empty() {
          Err(format!("OPTION(<{}>) missing", opt_name))
        } else {
            Ok(())
        }
    }
        }
    )*)
}
add_vec_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_vec_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }
