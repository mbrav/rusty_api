use super::method::Method;
pub struct Request {
    path: String,
    query: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, String> {
        let string = String::from("asd");
        // string.encrypt();
        unimplemented!();
    }
}
