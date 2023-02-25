use core::fmt;

#[derive(Debug, Clone)]
pub enum KbinXMLError {
    InvalidXML,
    InvalidOption,
    InvalidEncodingType,
    ToXml,
    ToBinary,
    ResultConversion,
    Utf8Error,
}

impl std::error::Error for KbinXMLError {}
impl fmt::Display for KbinXMLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KbinXMLError::InvalidXML => write!(f, "invalid xml"),
            KbinXMLError::InvalidOption => write!(f, "invalid option"),
            KbinXMLError::InvalidEncodingType => write!(f, "invalid encoding type"),
            KbinXMLError::ToBinary => write!(f, "to_binary error"),
            KbinXMLError::ToXml => write!(f, "to_xml error"),
            KbinXMLError::ResultConversion => write!(f, "result data conversion error"),
            KbinXMLError::Utf8Error => write!(f, "from_utf8 error"),
        }
    }
}
