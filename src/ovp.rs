use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::fmt::Debug;

/// **`OptValue`**
#[derive(Debug)]
pub struct OptValue<'app> {
    pub inner: Box<OptValueParse<'app> + 'app>,
}

/// **You can use custom `OptValue` by `impl` it**
///
/// ### **Explain**
///
/// * `into_opt_value(self)` convert it(`&mut T`)  to `OptValue`.
///
///
/// * `is_bool(&self)` like `--help/-h`,they not have value follows it.
///
///    so you should return `false` except value's type is `&mut bool`(it already defined).
///
///
/// * `is_must(&self)` if it's true, `app` will add a `must` tag for it's help.
///
///     `String.is_empty()`, `Option<T>.is_none()`, `Vec<T>.is_empty()` in default `impl`
///
///     ```fuckrs
///         --user user,-u user(Must)       sets user information
///     ```
///
///
/// * `parse(&mut self, opt_name: String, msg: &str)` maintains the value, and return message by `Result<(),String>`.
///
///   `opt_name` is current `Opt`'s name, `msg` is the `&str` need to pasre.
///
/// * `check(&self, opt_name: &str)` check value  and return message by `Result<(),String>`.
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
    fn into_opt_value(self) -> OptValue<'app>;
    fn is_bool(&self) -> bool;
    fn is_must(&self) -> bool;
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String>;
    fn check(&self, opt_name: &str) -> Result<(), String>;
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut bool {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        true
    }
    fn is_must(&self) -> bool {
        false
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
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn is_must(&self) -> bool {
        self.is_empty()
    }
    fn parse(&mut self, _: String, msg: &str) -> Result<(), String> {
        **self = msg.to_string();
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_empty() {
            Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut char {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn is_must(&self) -> bool {
        false
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        if msg.len() == 1 {
            **self = msg.chars().next().unwrap();
        } else {
            return Err(format!("OPTION({}) parse<char> fails: \"{}\"", opt_name, msg));
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
        fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn is_must(&self) -> bool {
        false
    }   
    fn parse(&mut self, opt_name : String, msg: &str) -> Result<(), String> {
        **self = msg.parse::<$t>()
                .map_err(|_| format!("OPTION({}) parse<{}> fails: \"{}\"", opt_name, stringify!($t),msg))?;
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
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn is_must(&self) -> bool {
        self.is_none()
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        if msg.len() == 1 {
            **self = Some(msg.chars().next().unwrap());
        } else {
            return Err(format!("OPTION({}) parse<char> fails: \"{}\"", opt_name, msg));
        }
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_none() {
            Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<String> {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn is_must(&self) -> bool {
        self.is_none()
    }
    fn parse(&mut self, _: String, msg: &str) -> Result<(), String> {
        **self = Some(msg.to_string());
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_none() {
            Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
}

macro_rules! add_option_impl {
    ($($t:ty)*) => ($(
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<$t> {
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn is_must(&self) -> bool {
        self.is_none()
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
        **self = Some(msg.parse::<$t>()
                          .map_err(|_| {
                                       format!("OPTION({}) parse<{}> fails: \"{}\"",
                                               opt_name,
                                               stringify!($t),
                                               msg)
                                   })?);
        Ok(())
    }
    fn check(&self, opt_name: &str) -> Result<(), String> {
        if self.is_none() {
            Err(format!("OPTION({})'s value missing", opt_name))
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
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn is_must(&self) -> bool {
        self.is_empty()
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
    fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn is_must(&self) -> bool {
        self.is_empty()
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
            Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
}

macro_rules! add_vec_impl {
    ($($t:ty)*) => ($(
        impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Vec<$t> {
        fn into_opt_value(self) -> OptValue<'app> {
        OptValue { inner: Box::new(self) }
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn is_must(&self) -> bool {
        self.is_empty()
    }
    fn parse(&mut self, opt_name: String, msg: &str) -> Result<(), String> {
                self.clear();
                let vs: Vec<&str> = msg.split(',').filter(|s| !s.is_empty()).collect();
                for str in &vs {
                    self.push(str.parse::<$t>()
                               .map_err(|_| {
                                            format!("OPTION({}) parse<Vec<{}>> fails: \"{}/{}\"",
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
          Err(format!("OPTION({})'s value missing", opt_name))
        } else {
            Ok(())
        }
    }
        }
    )*)
}
add_vec_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_vec_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }
