use rand::Rng;

const TYPE_A: u16 = 1;
const CLASS_IN: u16 = 1;

#[derive(Debug)]
struct DNSHeader {
    id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

impl DNSHeader {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12); // 6 fields * 2 bytes each
        bytes.extend(&self.id.to_be_bytes());
        bytes.extend(&self.flags.to_be_bytes());
        bytes.extend(&self.num_questions.to_be_bytes());
        bytes.extend(&self.num_answers.to_be_bytes());
        bytes.extend(&self.num_authorities.to_be_bytes());
        bytes.extend(&self.num_additionals.to_be_bytes());
        bytes
    }
}

#[derive(Debug)]
struct DNSQuestion {
    name: Vec<u8>,
    type_: u16,
    class_: u16,
}

impl DNSQuestion {
    fn new(domain_name: &str) -> Self {
        let encoded = DNSQuestion::encode_domain_name(domain_name);
        DNSQuestion {
            name: encoded,
            type_: TYPE_A,
            class_: CLASS_IN,
        }
    }

    fn encode_domain_name(domain_name: &str) -> Vec<u8> {
        let mut encoded = Vec::new();
        for part in domain_name.split('.') {
            encoded.push(part.len() as u8);
            encoded.extend_from_slice(part.as_bytes());
        }
        encoded.push(0); // Add the null byte at the end
        encoded
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.name.clone();
        bytes.extend(&self.type_.to_be_bytes());
        bytes.extend(&self.class_.to_be_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct DNSQuery {
    _header: DNSHeader,
    _question: DNSQuestion,
}

impl DNSQuery {
    pub fn build_query(domain_name: &str) -> Vec<u8> {
        let id = rand::thread_rng().gen_range(0..=65535);
        let recursion_desired = 1 << 8;
        let header = DNSHeader {
            id,
            flags: recursion_desired,
            num_questions: 1,
            num_answers: 0,
            num_authorities: 0,
            num_additionals: 0,
        };
        let question = DNSQuestion::new(domain_name);
        let mut query = header.to_bytes();
        query.extend(question.to_bytes());
        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_dns_name_with_new() {
        let question = DNSQuestion::new("www.example.com");
        let expected_name = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0,
        ];
        assert_eq!(question.name, expected_name);
    }

    #[test]
    fn test_encode_dns_name() {
        let encoded = DNSQuestion::encode_domain_name("www.example.com");
        let expected_name = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0,
        ];
        assert_eq!(encoded, expected_name);
    }

    #[test]
    fn test_header_to_bytes() {
        let header = DNSHeader {
            id: 0x1234,
            flags: 0x0100,
            num_questions: 1,
            num_answers: 0,
            num_authorities: 0,
            num_additionals: 0,
        };
        let expected_bytes = vec![
            0x12, 0x34, // id
            0x01, 0x00, // flags
            0x00, 0x01, // num_questions
            0x00, 0x00, // num_answers
            0x00, 0x00, // num_authorities
            0x00, 0x00, // num_additionals
        ];
        assert_eq!(header.to_bytes(), expected_bytes);
    }

    #[test]
    fn test_question_to_bytes() {
        let question = DNSQuestion::new("www.example.com");
        let mut expected_bytes = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0,
        ];
        expected_bytes.extend(&TYPE_A.to_be_bytes());
        expected_bytes.extend(&CLASS_IN.to_be_bytes());
        assert_eq!(question.to_bytes(), expected_bytes);
    }

    #[test]
    fn test_build_query() {
        let domain_name = "www.example.com";
        let query = DNSQuery::build_query(domain_name);
        // Check if the length of the query is correct
        let encoded_name_len = DNSQuestion::encode_domain_name(domain_name).len();
        let expected_query_len = 12 + encoded_name_len + 4;
        assert_eq!(query.len(), expected_query_len);

        let id = u16::from_be_bytes([query[0], query[1]]);
        let expected_header = [
            id.to_be_bytes()[0],
            id.to_be_bytes()[1], // id
            0x01,
            0x00, // flags
            0x00,
            0x01, // num_questions
            0x00,
            0x00, // num_answers
            0x00,
            0x00, // num_authorities
            0x00,
            0x00, // num_additionals
        ];
        assert_eq!(&query[..12], &expected_header[..]);

        // Check the question part
        let mut expected_question = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0,
        ];
        expected_question.extend(&TYPE_A.to_be_bytes());
        expected_question.extend(&CLASS_IN.to_be_bytes());
        assert_eq!(&query[12..], &expected_question[..]);
    }
}
