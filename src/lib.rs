pub mod base64;


#[cfg(test)]
mod tests {

    use crate::base64::{base64_encode, base64_decode};

    #[test]
    fn encode_works() {
        let result = base64_encode(&String::from("abcd"));
        assert_eq!(result, String::from("YWJjZA=="));
    }

    #[test]
    fn decode_works() {
        let result = base64_decode(&String::from("YWJjZA=="));
        assert_eq!(result, String::from("abcd"));
    }
}
