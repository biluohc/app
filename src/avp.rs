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
/**
 **You can use custom `ArgsValue` by `impl` it**

 1. `into(self)` convert it(`&mut T`)  to `ArgsValue`.


 2. `default(&self)` is `Arguments`'s default value's str for help message print


 3. `parse(&mut self, args_name: &str, msg: &str, count: &mut usize, len: &mut Option<usize>)` maintains the value, and return message by `Result<(),String>`.

   `args_name` is the name of `Args`, `msg` is a arg need to pasre, `count` is the count for arg, `len` is the length setting fot `Args`, default is `None(Vec<T>)` or `self.len()(&mut [T])`

 4. `check(&self, args_name: &str, optional: &bool, count: &usize, len: Option<&usize>)` check value and return message by `Result<(),String>`, `optional` is the optional setting for `Args`

    If the `Opt` is not optional, and this `Opt` hasn't occurs, and `self.default().is_none()`, app will exit because of `ARGS` Missing.

    if value is a `&mut Vec<T>`, the setting of length(`len`) is None default,

    if value is a `&mut [T]`, the setting of length(`len`) is `value.len()` default,

    If `len.is_some()` and the `Args` occured, app will compare it with the times `Args` occurs('count`)(If not equal, app will exit)
    
* If the name of executable file is `ap` , has a `Port` `Args`(inner value is empty `Vec<u16>`)

```bash
ap "80" 
# vec[80]

# all types being impl `ArgsValue` will call `trim()` before call `parse::<T>()` except `char`,`String`, `PathBuf` and their `Option`, `Vec`, `Slice`.
# So have some fault-tolerant.

ap " 80  "       
# vec[80]

ap 80 "8000 " " 8080"
# Vec[8080,8000,80]
```
 */
pub trait ArgsValueParse<'app>: Debug {
    fn into(self) -> ArgsValue<'app>;
    fn default(&self) -> Option<String>;
    fn parse(&mut self, args_name: &str, msg: &str, count: &mut usize, len: &mut Option<usize>) -> Result<(), String>;
    fn check(&self, args_name: &str, optional: &bool, count: &usize, len: Option<&usize>) -> Result<(), String>;
}

impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut Vec<String> {
    fn into(self) -> ArgsValue<'app> {
        ArgsValue::new(Box::from(self))
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}", self))
        }
    }
    fn parse(&mut self, args_name: &str, msg: &str, count: &mut usize, len: &mut Option<usize>) -> Result<(), String> {
        if *count == 1 {
            self.clear(); //clear the dafault value
        }
        if let Some(len) = len.as_ref() {
            if count as &usize > len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
                    count,
                    msg
                ))?;
            }
        }
        self.push(msg.to_string());
        Ok(())
    }
    fn check(&self, args_name: &str, optional: &bool, count: &usize, len: Option<&usize>) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("ARGS(<{}>) missing", args_name))?;
        }
        if let Some(len) = len {
            if *count != 0 && count != len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
                    count,
                    self
                ))?;
            }
        }
        Ok(())
    }
}
impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut Vec<PathBuf> {
    fn into(self) -> ArgsValue<'app> {
        ArgsValue::new(Box::from(self))
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}", self))
        }
    }
    fn parse(&mut self, args_name: &str, msg: &str, count: &mut usize, len: &mut Option<usize>) -> Result<(), String> {
        if *count == 1 {
            self.clear();
        }
        if let Some(len) = len.as_ref() {
            if count as &usize > len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
                    count,
                    msg
                ))?;
            }
        }
        self.push(PathBuf::from(msg));
        Ok(())
    }
    fn check(&self, args_name: &str, optional: &bool, count: &usize, len: Option<&usize>) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("ARGS(<{}>) missing", args_name))?;
        }
        if let Some(len) = len {
            if *count != 0 && count != len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
                    count,
                    self
                ))?;
            }
        }
        Ok(())
    }
}

impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut Vec<char> {
    fn into(self) -> ArgsValue<'app> {
        ArgsValue::new(Box::from(self))
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}", self))
        }
    }
    fn parse(&mut self, args_name: &str, msg: &str, count: &mut usize, len: &mut Option<usize>) -> Result<(), String> {
        if *count == 1 {
            self.clear();
        }
        if let Some(len) = len.as_ref() {
            for (idx, c) in msg.chars().enumerate() {
                if idx != 0 {
                    *count += 1; // count_add_one() alrendy add one.
                }
                if count as &usize > len {
                    Err(format!(
                        "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                        args_name,
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
    fn check(&self, args_name: &str, optional: &bool, count: &usize, len: Option<&usize>) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("ARGS(<{}>) missing", args_name))?;
        }
        if let Some(len) = len {
            if *count != 0 && count != len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
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
        impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut Vec<$t> {
        fn into(self) -> ArgsValue<'app> {
        ArgsValue::new(Box::from(self))
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}",self))
        }
    }
    fn parse(&mut self, args_name: &str, msg: &str, count: &mut usize, len: &mut Option<usize>) -> Result<(), String> {
        if *count == 1 {
            self.clear();
        }
        if let Some(len) = len.as_ref()  {
            if count as &usize > len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,count,
                    msg
                ))?;
            }
        }
        self.push(msg.trim().parse::<$t>()
            .map_err(|_| {
                        format!("Args(<{}>) parse<{}> fails: \"{}\"",
                                args_name,
                                stringify!($t),
                                msg)
                    })?);
                Ok(())
    }
    fn check(&self, args_name: &str, optional: &bool, count: &usize, len: Option<&usize>) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("ARGS(<{}>) missing", args_name))?;
        }
        if let Some(len) = len {
            if *count != 0 && count != len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
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

impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut [String] {
    fn into(self) -> ArgsValue<'app> {
        ArgsValue::new(Box::from(self))
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}", self))
        }
    }
    fn parse(&mut self, args_name: &str, msg: &str, count: &mut usize, len: &mut Option<usize>) -> Result<(), String> {
        if len.is_none() {
            *len = Some(self.len());
        }
        if let Some(len) = len.as_ref() {
            if count as &usize > len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
                    count,
                    msg
                ))?;
            }
        }
        self[*count - 1] = msg.to_string();
        Ok(())
    }
    fn check(&self, args_name: &str, optional: &bool, count: &usize, len: Option<&usize>) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("ARGS(<{}>) missing", args_name))?;
        }
        if let Some(len) = len {
            if *count != 0 && count != len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
                    count,
                    self
                ))?;
            }
        }
        Ok(())
    }
}

impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut [PathBuf] {
    fn into(self) -> ArgsValue<'app> {
        ArgsValue::new(Box::from(self))
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}", self))
        }
    }
    fn parse(&mut self, args_name: &str, msg: &str, count: &mut usize, len: &mut Option<usize>) -> Result<(), String> {
        if len.is_none() {
            *len = Some(self.len());
        }
        if let Some(len) = len.as_ref() {
            if count as &usize > len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
                    count,
                    msg
                ))?;
            }
        }
        self[*count - 1] = PathBuf::from(msg);
        Ok(())
    }
    fn check(&self, args_name: &str, optional: &bool, count: &usize, len: Option<&usize>) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("ARGS(<{}>) missing", args_name))?;
        }
        if let Some(len) = len {
            if *count != 0 && count != len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
                    count,
                    self
                ))?;
            }
        }
        Ok(())
    }
}

impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut [char] {
    fn into(self) -> ArgsValue<'app> {
        ArgsValue::new(Box::from(self))
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}", self))
        }
    }
    fn parse(&mut self, args_name: &str, msg: &str, count: &mut usize, len: &mut Option<usize>) -> Result<(), String> {
        if len.is_none() {
            *len = Some(self.len());
        }
        for (idx, c) in msg.chars().enumerate() {
            if idx != 0 {
                *count += 1; // count_add_one() alrendy add one.
            }
            if let Some(len) = len.as_ref() {
                if count as &usize > len {
                    Err(format!(
                        "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                        args_name,
                        len,
                        count,
                        msg
                    ))?;
                }
                self[*count - 1] = c;
            }
        }
        Ok(())
    }
    fn check(&self, args_name: &str, optional: &bool, count: &usize, len: Option<&usize>) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("ARGS(<{}>) missing", args_name))?;
        }
        if let Some(len) = len {
            if *count != 0 && count != len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
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
        impl<'app, 's: 'app> ArgsValueParse<'app> for &'s mut [$t] {
        fn into(self) -> ArgsValue<'app> {
        ArgsValue::new(Box::from(self))
    }
    fn default(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(format!("{:?}",self))
        }
    }
    fn parse(&mut self, args_name: &str, msg: &str, count: &mut usize, len: &mut Option<usize>) -> Result<(), String> {
        if len.is_none() {
            *len = Some(self.len());
        }
        if let Some(len) = len.as_ref() {
            if count as &usize > len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,count,
                    msg
                ))?;
            }
        }
        self[*count-1] = msg.trim().parse::<$t>()
            .map_err(|_| {
                        format!("Args(<{}>) parse<{}> fails: \"{}\"",
                                args_name,
                                stringify!($t),
                                msg)
                    })?;
                Ok(())
    }
    fn check(&self, args_name: &str, optional: &bool, count: &usize, len: Option<&usize>) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            Err(format!("ARGS(<{}>) missing", args_name))?;
        }
        if let Some(len) = len {
            if *count != 0 && count != len {
                Err(format!(
                    "ARGS(<{}>) only needs {}, but the count {} beyond: {:?}",
                    args_name,
                    len,
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

#[cfg(test)]
mod tests {
    use Args as Opt;
    #[test]
    fn chars() {
        opt_()
    }
    fn opt_() {
        let mut cs: Vec<char> = Vec::new();
       "None".chars().map(|c|cs.push(c)).count();
    {
        let mut opt = Opt::new("char", &mut cs);
        assert!(opt.parse(&["charse".to_string()]).is_ok());
        assert_eq!(*opt.count_get(), "charse".len());
    }
    assert_eq!(cs, vec!['c', 'h', 'a', 'r', 's', 'e']);

    {
        let mut opt = Opt::new("char", &mut cs[..]);
        assert!(opt.parse(&["abcshx".to_string()]).is_ok());
        assert_eq!(*opt.count_get(), "abcshx".len());
    }
    assert_eq!(cs, vec!['a', 'b', 'c', 's', 'h', 'x']);

    {
        let mut opt = Opt::new("char", &mut cs[..]);
        assert!(opt.parse(&["a fg".to_string()]).is_ok());
        assert_eq!(*opt.count_get(), 4);
        assert!(opt.parse(&["78".to_string()]).is_ok());
        assert_eq!(*opt.count_get(), 6);
    }
    assert_eq!(cs, vec!['a', ' ', 'f', 'g', '7', '8']);

    let mut cs = [' '; 5];
    {
        let mut opt = Opt::new("char", &mut cs[..]);
        assert!(opt.parse(&["a fg".to_string()]).is_ok());
        assert_eq!(*opt.count_get(), 4);
        assert!(opt.parse(&["7".to_string()]).is_ok());
        assert_eq!(*opt.count_get(), 5);
    }
    assert_eq!(cs, ['a', ' ', 'f', 'g', '7']);

    let mut cs = [' '; 5];
    {
        let mut opt = Opt::new("char", &mut cs[..]).len(3u8);
        assert!(opt.parse(&["af".to_string()]).is_ok());
        assert_eq!(*opt.count_get(), 2);
        assert!(opt.parse(&["6".to_string()]).is_ok());
        assert_eq!(*opt.count_get(), 3);
        assert!(opt.parse(&["6".to_string()]).is_err());
        assert_eq!(*opt.count_get(), 4);
        assert!(opt.parse(&["xcmh".to_string()]).is_err());
        assert_eq!(*opt.count_get(), 5);
    }
    }
}