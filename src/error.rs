quick_error! {
    /// **AppError**
    #[derive(Debug,PartialEq)]
     pub enum AppError {
        Parse(err:String) {
            description("Parse Error")
            from()
        }
        Help(err: Option<String>) {
            description("-h, --help")
        }
        Version {
            description("-V, --version")
        }
    }
}

trait ToAppRest {
    fn to_app_rest(self) -> Result<(), AppError>;
}

impl<'a> ToAppRest for &'a Option<String> {
    fn to_app_rest(self) -> Result<(), AppError> {
        Err(AppError::Help((*self).clone()))
    }
}
impl<'a> ToAppRest for Option<String> {
    fn to_app_rest(self) -> Result<(), AppError> {
        Err(AppError::Help(self))
    }
}
