use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::fmt::Debug;
use OptTypo;

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
/**
**You can use custom `OptValue` by `impl` it**

### **Explain**

1. `into(self)` convert it(`&mut T`)  to `OptValue`.


2. `is_bool(&self)` , `Opt` is flag, like `--help/-h`,it have not value follows it.

   so you should return `false` except value's type is `&mut bool`(but `&mut bool` already being impl).

3. `default(&self)` is `Opt`'s default value's str for help message print

4. `parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo)` maintains the value, and return message by `Result<(),String>`.

  `opt_name` is current `Opt`'s name, `msg` is the arg need to pasre, `count` is the count for the times `Opt` occurs, `typo` is the type setting for the `Opt`

5. `check(&self, opt_name: &str, optional: &bool, count: &usize, typo: &OptTypo)` check value and return message by `Result<(),String>`.

    `optional` is the optional setting for `Opt`.

    If the `Opt` is not optional, and this `Opt` hasn't occurs, and `self.default().is_none()`, app will exit because of `OPTION` Missing.

    if value is a `&mut Vec<T>`, the setting of length(`OptTypo::Multiple().get()`) is None default,

    if value is a `&mut [T]`, the setting of length(`OptTypo::Multiple().get()`) is `value.len()` default,

    If `OptTypo::Multiple().get().is_some()` and the `Opt` occured, app will compare it with the times `Opt` occurs('count`)(If not equal, app will exit)
    

* If the name of executable file is `ap` , has a `-p` `Opt`(inner value is empty `Vec<u16>`)

```sh
ap -p "80" 
# vec[80]

# all types being impl `OptValue` will call `trim()` before call `parse::<T>()` except `char`,`String`, `PathBuf` and their `Option`, `Vec`, `Slice`.
# So have some fault-tolerant.
ap -p " 80  "       
# vec[80]


ap -p 80 "8000 " " 8080"
# Vec[8080,8000,80]
```
*/
pub trait OptValueParse<'app>: Debug {
    fn into(self) -> OptValue<'app>;
    fn is_bool(&self) -> bool;
    fn default(&self) -> Option<String>;
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String>;
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, typo: &OptTypo) -> Result<(), String>;
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut bool {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        true
    }
    fn default(&self) -> Option<String> {
        None
    }
    fn parse(&mut self, _: &str, _: &str, _: &mut usize, _: &mut OptTypo) -> Result<(), String> {
        **self = ! **self;
        Ok(())
    }
    fn check(&self, _: &str, _: &bool, _: &usize, _: &OptTypo) -> Result<(), String> {
        Ok(())
    }
}
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut String {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
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
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if *count == 0 || typo.is_covered() || typo.is_multiple() {
            **self = msg.to_string();
        } else if typo.is_single() {
            Err(format!(
                "OPTION(<{}>) can only occurs once, but second: {:?}",
                opt_name,
                msg
            ))?;
        }
        Ok(())
    }
    /// env::arg could is `""`
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, _: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        Ok(())
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut char {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        Some(self.to_string())
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        for (idx, c) in msg.chars().enumerate() {
            if idx != 0 {
                *count += 1;
            }
            match *typo {
                OptTypo::Single => {
                    if *count > 1 {
                        Err(format!(
                            "OPTION(<{}>) can only occurs once, but second: {:?}",
                            opt_name,
                            msg
                        ))?;
                    }
                }
                OptTypo::Ignored => {
                    if *count == 1 {
                        **self = c;
                    }
                }
                OptTypo::Covered |
                OptTypo::Multiple(..) => {
                    **self = c;
                }     
            }
        }
        Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, _: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        Ok(())
    }
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl<'app, 's: 'app> OptValueParse<'app> for &'s mut $t {
        fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        Some(format!("{}", self))
    }
    fn parse(&mut self, opt_name : &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if *count == 0 || typo.is_covered() || typo.is_multiple() {
              **self = msg.trim().parse::<$t>()
                .map_err(|_| format!("OPTION(<{}>) parse<{}> fails: \"{}\"", opt_name, stringify!($t),msg))?;
        } else if typo.is_single() {
            Err(format!(
                "OPTION(<{}>) can only occurs once, but second: {:?}",
                opt_name,
                msg
            ))?;
        }
        Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, _ : &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        Ok(())
    }
        }
    )*)
}

add_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<char> {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        self.map(|s| s.to_string())
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        for (idx, c) in msg.chars().enumerate() {
            if idx != 0 {
                *count += 1;
            }
            match *typo {
                OptTypo::Single => {
                    if *count > 1 {
                        Err(format!(
                            "OPTION(<{}>) can only occurs once, but second: {:?}",
                            opt_name,
                            msg
                        ))?;
                    }
                }
                OptTypo::Ignored => {
                    if *count == 1 {
                        **self = Some(c);
                    }
                }
                OptTypo::Covered |
                OptTypo::Multiple(..) => {
                    **self = Some(c);
                }     
            }
        }
        Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, _: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        Ok(())
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<String> {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        (**self).clone()
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if *count == 0 || typo.is_covered() || typo.is_multiple() {
            **self = Some(msg.to_string());
        } else if typo.is_single() {
            Err(format!(
                "OPTION(<{}>) can only occurs once, but second: {:?}",
                opt_name,
                msg
            ))?;
        }
        Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, _: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        Ok(())
    }
}

macro_rules! add_option_impl {
    ($($t:ty)*) => ($(
impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Option<$t> {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
       self.as_ref().map(|ref s|format!("{}",s))
    }
    fn parse(&mut self, opt_name:&str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if *count == 0 || typo.is_covered() || typo.is_multiple() {
           **self = Some(msg.trim().parse::<$t>()
                          .map_err(|_| {
                                       format!("OPTION(<{}>) parse<{}> fails: \"{}\"",
                                               opt_name,
                                               stringify!($t),
                                               msg)
                                   })?);
        } else if typo.is_single() {
            Err(format!(
                "OPTION(<{}>) can only occurs once, but second: {:?}",
                opt_name,
                msg
            ))?;
        }
        Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, _ : &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        Ok(())
    }
}
    )*)
}

add_option_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_option_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Vec<char> {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        self.as_slice().joins("")
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if *count == 1 {
            self.clear();
        }
        if !typo.is_multiple() {
            typo.set_multiple(None);
        }
        if let Some(len) = typo.multiple_get() {
            for (idx, c) in msg.chars().enumerate() {
                if idx != 0 {
                    *count += 1; // count_add_one() alrendy add one.
                }
                if count as &usize > len {
                    Err(format!(
                        "OPTION(<{}>) can only occurs {} times, but the count {} beyond: {:?}",
                        opt_name,
                        len,
                        count,
                        msg
                    ))?;
                }
                self.push(c);
            }
        } else {
            *count = msg.chars().map(|c| self.push(c)).count() + *count - 1;
        }
        Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, typo: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        if let Some(len) = typo.multiple_get() {
            if *count != 0 && count != len {
                Err(format!(
                    "OPTION(<{}>) can only occurs {} times, but it occurs {} times: {:?}",
                    len,
                    opt_name,
                    count,
                    self
                ))?;
            }
        }
        Ok(())
    }
}

use std::fmt::Display;
trait JoinSlice {
    #[inline]
    fn joins(&self, sep: &str) -> Option<String>;
}
impl<'a, T: Display> JoinSlice for &'a [T] {
    fn joins(&self, sep: &str) -> Option<String> {
        if self.is_empty() {
            return None;
        }
        let mut str = String::new();
        for idx in 0..self.len() {
            str.push_str(&self[idx].to_string());
            if idx + 1 != self.len() {
                str.push_str(sep);
            }
        }
        Some(str)
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Vec<String> {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        self.as_slice().joins(",")
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if *count == 1 {
            self.clear(); // clear default's value
        }
        if !typo.is_multiple() {
            typo.set_multiple(None);
        }
        let len = typo.multiple_get();
        if let Some(len) = len {
            if count as &usize > len {
                Err(format!(
                    "OPTION(<{}>) can only occurs {} times, but the count {} beyond: {:?}",
                    opt_name,
                    len,
                    count,
                    msg
                ))?;
            }
        }
        self.push(msg.to_owned());
        Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, typo: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        if let Some(len) = typo.multiple_get() {
            if *count != 0 && count != len {
                Err(format!(
                    "OPTION(<{}>) can only occurs {} times, but it occurs {} times: {:?}",
                    len,
                    opt_name,
                    count,
                    self
                ))?;
            }
        }
        Ok(())
    }
}

macro_rules! add_vec_impl {
    ($($t:ty)*) => ($(
        impl<'app, 's: 'app> OptValueParse<'app> for &'s mut Vec<$t> {
        fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
         self.as_slice().joins(",")
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if *count == 1 {
            self.clear(); // clear default's value
        }       
        if !typo.is_multiple() {
            typo.set_multiple(None);
        }
        let len = typo.multiple_get();
        if let Some(len) = len {
            if count as &usize > len {
                Err(format!(
                    "OPTION(<{}>) can only occurs {} times, but the count {} beyond: {:?}",
                    opt_name,
                    len,count,
                    msg
                ))?;
            }
        }
        self.push(msg.trim().parse::<$t>()
                    .map_err(|_| {
                                format!("OPTION(<{}>) parse<{}> fails: \"{}\"",
                                        opt_name,
                                        stringify!($t),
                                        msg)
                            })?);
                Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, typo: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        if let Some(len) = typo.multiple_get() {
            if *count != 0 && count != len {
                Err(format!(
                    "OPTION(<{}>) can only occurs {} times, but it occurs {} times: {:?}",
                    len,
                    opt_name,
                    count,
                    self
                ))?;
            }
        }
        Ok(())
    }
        }
    )*)
}
add_vec_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_vec_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut [char] {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        (self as &[_]).joins("")
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if !typo.is_multiple() {
            typo.set_multiple(Some(self.len()));
        }
        let len = typo.multiple_get().expect(
            &format!("OPTION(<{}>)'s value: can't set as None of the length of slice \"{:?}\"",
            opt_name,
            self,
        ),
        );
        assert!(*count >= 1);
        for (idx, c) in msg.chars().enumerate() {
            if idx != 0 {
                *count += 1; // count_add_one() alrendy add one.
            }
            if count as &usize > len {
                Err(format!(
                    "OPTION(<{}>) can only occurs {} times, but the count {} beyond: {:?}",
                    opt_name,
                    len,
                    count,
                    msg
                ))?;
            }
            self[*count - 1] = c;
        }
        Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, typo: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        if let Some(len) = typo.multiple_get() {
            if *count != 0 && count != len {
                Err(format!(
                    "OPTION(<{}>) can only occurs {} times, but it occurs {} times: {:?}",
                    len,
                    opt_name,
                    count,
                    self
                ))?;
            }
        }
        Ok(())
    }
}

impl<'app, 's: 'app> OptValueParse<'app> for &'s mut [String] {
    fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        (self as &[_]).joins(",")
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if !typo.is_multiple() {
            typo.set_multiple(Some(self.len()));
        }
        let len = typo.multiple_get().expect(
            &format!("OPTION(<{}>)'s value: can't set as None of the length of slice \"{:?}\"",
            opt_name,
            self,
        ),
        );
        assert!(*count >= 1);
        if count as &usize > len {
            Err(format!(
                "OPTION(<{}>) can only occurs {} times, but the count {} beyond: {:?}",
                opt_name,
                len,
                count,
                msg
            ))?;
        }
        self[*count - 1] = msg.to_owned();
        Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, typo: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        if let Some(len) = typo.multiple_get() {
            if *count != 0 && count != len {
                Err(format!(
                    "OPTION(<{}>) can only occurs {} times, but it occurs {} times: {:?}",
                    len,
                    opt_name,
                    count,
                    self
                ))?;
            }
        }
        Ok(())
    }
}

macro_rules! add_slice_impl {
    ($($t:ty)*) => ($(
        impl<'app, 's: 'app> OptValueParse<'app> for &'s mut [$t] {
        fn into(self) -> OptValue<'app> {
        OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        (self as &[_]).joins(",")
    }
    fn parse(&mut self, opt_name: &str, msg: &str, count: &mut usize, typo: &mut OptTypo) -> Result<(), String> {
        if !typo.is_multiple() {
            typo.set_multiple(Some(self.len()));
        }
        let len = typo.multiple_get().expect(
            &format!("OPTION(<{}>)'s value: can't set as None of the length of slice \"{:?}\"",
            opt_name,
            self,
        ));
        assert!(*count >= 1);        
            if count as &usize > len {
                Err(format!(
                    "OPTION(<{}>) can only occurs {} times, but the count {} beyond: {:?}",
                    opt_name,
                    len,count,
                    msg
                ))?;
            }
        self[*count-1]= msg.trim().parse::<$t>()
                    .map_err(|_| {
                                format!("OPTION(<{}>) parse<{}> fails: \"{}\"",
                                        opt_name,
                                        stringify!($t),
                                        msg)
                            })?;
                Ok(())
    }
    fn check(&self, opt_name: &str, optional: &bool, count: &usize, typo: &OptTypo) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("OPTION(<{}>) missing", opt_name))?;
        }
        if let Some(len) = typo.multiple_get() {
            if *count != 0 && count != len {
                Err(format!(
                    "OPTION(<{}>) can only occurs {} times, but it occurs {} times: {:?}",
                    len,
                    opt_name,
                    count,
                    self
                ))?;
            }
        }
        Ok(())
    }
        }
    )*)
}
add_slice_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
add_slice_impl! { IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6 }



// cargo t -- --nocapture  ovp::tests::chars
#[cfg(test)]
mod tests {
    use {Opt, OptTypo};
    #[test]
    fn chars() {
        opt_()
    }
    fn opt_() {
        let mut cs: Vec<char> = Vec::new();
       "None".chars().map(|c|cs.push(c)).count();
    {
        let mut opt = Opt::new("char", &mut cs);
        assert!(opt.parse("charse").is_ok());
        assert_eq!(*opt.count_get(), "charse".len());
    }
    assert_eq!(cs, vec!['c', 'h', 'a', 'r', 's', 'e']);

    {
        let mut opt = Opt::new("char", &mut cs[..]);
        assert!(opt.parse("abcshx").is_ok());
        assert_eq!(*opt.count_get(), "abcshx".len());
    }
    assert_eq!(cs, vec!['a', 'b', 'c', 's', 'h', 'x']);

    {
        let mut opt = Opt::new("char", &mut cs[..]);
        assert!(opt.parse("a fg").is_ok());
        assert_eq!(*opt.count_get(), 4);
        assert!(opt.parse("78").is_ok());
        assert_eq!(*opt.count_get(), 6);
    }
    assert_eq!(cs, vec!['a', ' ', 'f', 'g', '7', '8']);

    let mut cs = [' '; 5];
    {
        let mut opt = Opt::new("char", &mut cs[..]);
        assert!(opt.parse("a fg").is_ok());
        assert_eq!(*opt.count_get(), 4);
        assert!(opt.parse("7").is_ok());
        assert_eq!(*opt.count_get(), 5);
    }
    assert_eq!(cs, ['a', ' ', 'f', 'g', '7']);

    let mut cs = [' '; 5];
    {
        let mut opt = Opt::new("char", &mut cs[..]).typo(OptTypo::Multiple(Some(3)));
        assert!(opt.parse("af").is_ok());
        assert_eq!(*opt.count_get(), 2);
        assert!(opt.parse("6").is_ok());
        assert_eq!(*opt.count_get(), 3);
        assert!(opt.parse("6").is_err());
        assert_eq!(*opt.count_get(), 4);
        assert!(opt.parse("xcmh").is_err());
        assert_eq!(*opt.count_get(), 5);
    }
    }
}