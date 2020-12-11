pub struct DID {
    scheme: String,
    method: String,
    specific_id: String,
}

impl DID {
    pub fn new(method: &str, specific_id: &str) -> DID {
        DID {
            scheme: "did".to_owned(),
            method: method.to_owned(),
            specific_id: specific_id.to_owned(),
        }
    }

    pub fn parse(&self) -> String {
        format!("{}:{}:{}", self.scheme.as_str(), self.method.as_str(), self.specific_id.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did() {
        let example = DID::new("example", "123456789asdfghj");
        assert_eq!("did:example:123456789asdfghj", example.parse());
    }
}