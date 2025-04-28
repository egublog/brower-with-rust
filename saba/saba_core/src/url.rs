use alloc::string::String;
use alloc::string::ToString;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    searchpart: String,
}

impl Url {
    fn is_http(&mut self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        return false;
    }

    pub fn parse(&mut self) -> Result<Self,String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported".to_string());
        }
    }
}