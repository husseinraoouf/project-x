use std::fmt;

pub struct Empty {}

/// A type that represents a simple error.
///
/// This type uses a `String` to store the error string, and it implements `std::error::Error`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleError {
    err: String,
}

impl SimpleError {
    /// Creates a new simple error.
    ///
    /// This function can take any type that implements `Into<String>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use self::simple_error::SimpleError;
    ///
    /// // errors can be created from `str`
    /// let err = SimpleError::new("an error from str");
    ///
    /// // errors can also be created from `String`
    /// let err = SimpleError::new(String::from("an error from String"));
    /// ```
    #[inline]
    pub fn new<T: Into<String>>(t: T) -> SimpleError {
        SimpleError { err: t.into() }
    }

    /// Creates a new simple error from another error.
    ///
    /// This function can take any type that implements `std::error::Error`.
    /// The error string will be the `Display` of the `std::error::Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// use self::simple_error::SimpleError;
    /// use std::io;
    ///
    /// // errors can be created from `io::Error`
    /// let err = SimpleError::from(io::Error::new(io::ErrorKind::Other, "oh no"));
    /// assert_eq!("oh no", format!("{}", err));
    /// ```
    #[inline]
    pub fn from<T: std::error::Error>(t: T) -> SimpleError {
        SimpleError {
            err: format!("{}", t),
        }
    }

    /// Creates a new simple error from a string with another error.
    ///
    /// This function takes a string as error and a type that implements `std::error::Error` as
    /// reason.
    /// The error string will be the `Display` of the `std::error::Error` prefixed with the string
    /// and ", ".
    ///
    /// # Examples
    ///
    /// ```
    /// use self::simple_error::SimpleError;
    ///
    /// let err = SimpleError::with("cannot turn on tv", SimpleError::new("remote not found"));
    /// assert_eq!("cannot turn on tv, remote not found", format!("{}", err));
    /// ```
    #[inline]
    pub fn with<T: std::error::Error>(s: &str, t: T) -> SimpleError {
        SimpleError {
            err: format!("{}, {}", s, t),
        }
    }

    /// Extracts a string slice describing the error.
    ///
    /// # Examples
    ///
    /// ```
    /// use self::simple_error::SimpleError;
    ///
    /// let s = SimpleError::new("critical error");
    /// assert_eq!("critical error", s.as_str());
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.err
    }
}

// TODO: implement From<T> where T: std::error::Error when specialization lands, and remove
// inherent from function.

impl<'a> From<&'a str> for SimpleError {
    #[inline]
    fn from(s: &str) -> SimpleError {
        SimpleError { err: s.into() }
    }
}

impl fmt::Display for SimpleError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.err.fmt(f)
    }
}

impl std::error::Error for SimpleError {
    #[inline]
    fn description(&self) -> &str {
        &self.err
    }
}
