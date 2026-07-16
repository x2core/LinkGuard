#[cfg(test)]
mod tests {
    use crate::sniffer::dpi::{parse_http, parse_dns, parse_tls, inspect_payload, ApplicationLayer};

    #[test]
    fn test_parse_http_request() {
        let payload = b"GET /index.html HTTP/1.1\r\nHost: www.example.com\r\nConnection: close\r\n\r\n";
        let result = parse_http(payload);

        assert!(result.is_some());
        let meta = result.unwrap();
        assert_eq!(meta.method, Some("GET".to_string()));
        assert_eq!(meta.path, Some("/index.html".to_string()));
        assert_eq!(meta.host, Some("www.example.com".to_string()));
        assert_eq!(meta.status_code, None);
    }

    #[test]
    fn test_parse_http_response() {
        let payload = b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
        let result = parse_http(payload);

        assert!(result.is_some());
        let meta = result.unwrap();
        assert_eq!(meta.method, None);
        assert_eq!(meta.path, None);
        assert_eq!(meta.host, None);
        assert_eq!(meta.status_code, Some(200));
    }

    #[test]
    fn test_parse_dns_query() {
        // Raw DNS Query packet for google.com (A record)
        let payload: [u8; 28] = [
            0x12, 0x34, // Transaction ID
            0x01, 0x00, // Flags (Standard query)
            0x00, 0x01, // Questions: 1
            0x00, 0x00, // Answer RRs: 0
            0x00, 0x00, // Authority RRs: 0
            0x00, 0x00, // Additional RRs: 0
            // Queries
            0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, // "google"
            0x03, 0x63, 0x6f, 0x6d,                   // "com"
            0x00,                                     // Null terminator
            0x00, 0x01, // Type A
            0x00, 0x01, // Class IN
        ];

        let result = parse_dns(&payload);

        assert!(result.is_some());
        let meta = result.unwrap();
        assert_eq!(meta.domain, Some("google.com".to_string()));
        assert!(meta.resolved_ips.is_empty());
    }

    #[test]
    fn test_parse_dns_response() {
        // Raw DNS Response packet for google.com (A record -> 142.250.190.46)
        let payload: [u8; 44] = [
            0x12, 0x34, // Transaction ID
            0x81, 0x80, // Flags (Standard query response, No error)
            0x00, 0x01, // Questions: 1
            0x00, 0x01, // Answer RRs: 1
            0x00, 0x00, // Authority RRs: 0
            0x00, 0x00, // Additional RRs: 0
            // Queries
            0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, // "google"
            0x03, 0x63, 0x6f, 0x6d,                   // "com"
            0x00,                                     // Null terminator
            0x00, 0x01, // Type A
            0x00, 0x01, // Class IN
            // Answers
            0xc0, 0x0c, // Name pointer to google.com
            0x00, 0x01, // Type A
            0x00, 0x01, // Class IN
            0x00, 0x00, 0x01, 0x2b, // TTL
            0x00, 0x04, // Data length: 4
            0x8e, 0xfa, 0xbe, 0x2e, // IP: 142.250.190.46
        ];

        let result = parse_dns(&payload);

        assert!(result.is_some());
        let meta = result.unwrap();
        assert_eq!(meta.domain, Some("google.com".to_string()));
        assert_eq!(meta.resolved_ips.len(), 1);
        assert_eq!(meta.resolved_ips[0], "142.250.190.46");
    }

    #[test]
    fn test_parse_tls_client_hello() {
        let payload: [u8; 70] = [
            0x16, 0x03, 0x01, 0x00, 0x41, 0x01, 0x00, 0x00, 0x3d, 0x03, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xc0, 0x2b, 0x01, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x0c, 0x00, 0x00, 0x09, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x68, 0x6f, 0x73, 0x74
        ];

        let result = parse_tls(&payload);

        assert!(result.is_some());
        let meta = result.unwrap();
        assert_eq!(meta.sni, Some("localhost".to_string()));
        assert!(meta.ja3_fingerprint.is_some());
    }

    #[test]
    fn test_inspect_payload_routing() {
        let http_payload = b"GET / HTTP/1.1\r\n\r\n";
        match inspect_payload(http_payload) {
            ApplicationLayer::Http(_) => {},
            _ => panic!("Should route to HTTP"),
        }

        let random_payload = b"some random garbage data that is not http or tls or dns";
        match inspect_payload(random_payload) {
            ApplicationLayer::Unknown => {},
            _ => panic!("Should route to Unknown"),
        }
    }
}
