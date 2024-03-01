use std::str;

static DELIMITER: &[u8] = b"\r\n";

pub enum RESP {
    SimpleString(String),
    SimpleError(String),
    Integer(i64),
    BulkString(String),
    // Array,
    // Null,
    // Boolean,
    // Double,
    // BigNumber,
    // BulkError,
    // VerbatimString,
    // Map,
    // Set,
    // Push,
}

// TODO: error handling with results
impl RESP {
    fn parse_to_str(content: &[u8]) -> &str {
        str::from_utf8(content).expect("Content should be utf8")
    }

    fn parse_to_string(content: &[u8]) -> String {
        Self::parse_to_str(content).to_string()
    }

    fn parse_to_int(content: &[u8]) -> i64 {
        Self::parse_to_str(content)
            .parse::<i64>()
            .expect("Content should be a valid i64 number")
    }

    fn find_next_delimiter_index(content: &[u8]) -> Option<usize> {
        for index in 0..content.len() {
            let mut found = false;

            for (delimiter_index, delimiter_char) in DELIMITER.iter().enumerate() {
                if delimiter_char != &content[index + delimiter_index] {
                    found = false;
                    break;
                }

                found = true;
            }

            if found {
                return Some(index);
            }
        }

        None
    }

    fn parse_bulk_string(content: &[u8]) -> Self {
        let length_delimiter =
            Self::find_next_delimiter_index(content).expect("There should be a length parameter");
        let length = &content[..length_delimiter];
        let content = &content[length.len() + DELIMITER.len()..];

        Self::BulkString(Self::parse_to_string(content))
    }

    pub fn try_parse_one(content: &[u8]) -> Result<Self, &str> {
        let initiator = content[0] as char;

        for (idx, separator) in DELIMITER.iter().rev().enumerate() {
            if &content[content.len() - (idx + 1)] != separator {
                return Err("The content does not have the right termination characters");
            }
        }

        let content = &content[1..content.len() - DELIMITER.len()];

        match initiator {
            '+' => Ok(Self::SimpleString(Self::parse_to_string(content))),
            '-' => Ok(Self::SimpleError(Self::parse_to_string(content))),
            ':' => Ok(Self::Integer(Self::parse_to_int(content))),
            '$' => Ok(Self::parse_bulk_string(content)),
            // '*' => RESP::Array,
            // '_' => RESP::Null,
            // '#' => RESP::Boolean,
            // ',' => RESP::Double,
            // '(' => RESP::BigNumber,
            // '!' => RESP::BulkError,
            // '=' => RESP::VerbatimString,
            // '%' => RESP::Map,
            // '~' => RESP::Set,
            // '>' => RESP::Push,
            _ => Err("Invalid first byte"),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn test_can_parse_simple_string() {
        let original = b"+OK\r\n";
        let expected = "OK";

        let parsed = RESP::try_parse_one(original);

        match parsed {
            Ok(RESP::SimpleString(value)) => assert!(expected == value),
            _ => panic!("Should be a simple string."),
        }
    }

    #[test]
    fn test_can_parse_simple_error() {
        let original = b"-Error message\r\n";
        let expected = "Error message";

        let parsed = RESP::try_parse_one(original);

        match parsed {
            Ok(RESP::SimpleError(value)) => assert!(expected == value),
            _ => panic!("Should be a simple error."),
        }
    }

    #[test]
    fn test_can_parse_a_positive_integer() {
        let original = b":10\r\n";
        let expected = 10i64;

        let parsed = RESP::try_parse_one(original);

        match parsed {
            Ok(RESP::Integer(value)) => assert!(expected == value),
            _ => panic!("Should be a integer"),
        }
    }

    #[test]
    fn test_can_parse_a_positive_integer_with_sign() {
        let original = b":+10\r\n";
        let expected = 10i64;

        let parsed = RESP::try_parse_one(original);

        match parsed {
            Ok(RESP::Integer(value)) => assert!(expected == value),
            _ => panic!("Should be a integer"),
        }
    }

    #[test]
    fn test_can_parse_a_negative_integer() {
        let original = b":-10\r\n";
        let expected = -10i64;

        let parsed = RESP::try_parse_one(original);

        match parsed {
            Ok(RESP::Integer(value)) => assert!(expected == value),
            _ => panic!("Should be a integer"),
        }
    }

    #[test]
    fn test_can_find_next_delimiter() {
        let original = b"content\r\n";
        let expected = 7;

        let result = RESP::find_next_delimiter_index(original);

        match result {
            Some(index) => assert!(index == expected),
            None => panic!("There should be an index"),
        }
    }

    #[test]
    fn test_cant_find_next_delimiter() {
        let original = b"content";

        let result = RESP::find_next_delimiter_index(original);

        if result.is_some() {
            panic!("There should be no index");
        }
    }

    #[test]
    fn test_can_parse_bulk_string() {
        let original = b"$5\r\nhello\r\n";
        let expected = "hello";

        let parsed = RESP::try_parse_one(original);

        match parsed {
            Ok(RESP::BulkString(value)) => assert!(expected == value),
            _ => panic!("Should be a bulk string."),
        }
    }
}
