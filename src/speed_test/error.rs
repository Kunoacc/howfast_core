#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Io(::std::io::Error),
    SystemTimeError(::std::time::SystemTimeError),
    ThreadPoolBuildError(rayon::ThreadPoolBuildError),
    ParseFloatError(::std::num::ParseFloatError),
    ParseIntError(::std::num::ParseIntError),
    ParseAddressError(::std::net::AddrParseError),
    ParseXmlTreeError(roxmltree::Error),
    ParseUrlError(url::ParseError),
    ParseSharedUrlError,
    ConfigParseError,
    ServerParseError,
    LatencyTestError,
    LatencyTestClosestError
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self { Error::Reqwest(error) }
}

impl From<::std::io::Error> for Error {
    fn from(error: ::std::io::Error) -> Self { Error::Io(error) }
}

impl From<::std::time::SystemTimeError> for Error {
    fn from(error: ::std::time::SystemTimeError) -> Self {
        Error::SystemTimeError(error)
    }
}

impl From<rayon::ThreadPoolBuildError> for Error {
    fn from(error: rayon::ThreadPoolBuildError) -> Self {
        Error::ThreadPoolBuildError(error)
    }
}

impl From<::std::num::ParseIntError> for Error {
    fn from(error: ::std::num::ParseIntError) -> Self { Error::ParseIntError(error) }
}

impl From<::std::num::ParseFloatError> for Error {
    fn from(error: ::std::num::ParseFloatError) -> Self {
        Error::ParseFloatError(error)
    }
}

impl From<::std::net::AddrParseError> for Error {
    fn from(error: ::std::net::AddrParseError) -> Self { Error::ParseAddressError(error) }
}

impl From<roxmltree::Error> for Error {
    fn from(error: roxmltree::Error) -> Self {
        Error::ParseXmlTreeError(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::ParseUrlError(error)
    }
}