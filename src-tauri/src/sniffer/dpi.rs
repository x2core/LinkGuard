use md5::{Md5, Digest};
use tls_parser::{parse_tls_plaintext, TlsMessage, TlsMessageHandshake, TlsExtension, TlsExtensionType};
use httparse::{Request, Response};
use dns_parser::{Packet as DnsPacket};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpMetadata {
    pub method: Option<String>,
    pub path: Option<String>,
    pub host: Option<String>,
    pub status_code: Option<u16>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DnsMetadata {
    pub domain: Option<String>,
    pub resolved_ips: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TlsMetadata {
    pub sni: Option<String>,
    pub ja3_fingerprint: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ApplicationLayer {
    Http(HttpMetadata),
    Dns(DnsMetadata),
    Tls(TlsMetadata),
    Unknown,
}

pub fn parse_http(payload: &[u8]) -> Option<HttpMetadata> {
    // Try Request
    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = Request::new(&mut headers);

    if let Ok(httparse::Status::Complete(_)) = req.parse(payload) {
        let method = req.method.map(|s| s.to_string());
        let path = req.path.map(|s| s.to_string());
        let host = req.headers.iter()
            .find(|h| h.name.eq_ignore_ascii_case("host"))
            .and_then(|h| std::str::from_utf8(h.value).ok().map(|s| s.to_string()));

        return Some(HttpMetadata {
            method,
            path,
            host,
            status_code: None,
        });
    }

    // Try Response
    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut res = Response::new(&mut headers);

    if let Ok(httparse::Status::Complete(_)) = res.parse(payload) {
        return Some(HttpMetadata {
            method: None,
            path: None,
            host: None,
            status_code: res.code,
        });
    }

    None
}

pub fn parse_dns(payload: &[u8]) -> Option<DnsMetadata> {
    if let Ok(packet) = DnsPacket::parse(payload) {
        let domain = packet.questions.first().map(|q| q.qname.to_string());
        let mut resolved_ips = Vec::new();

        for answer in packet.answers {
            match answer.data {
                dns_parser::RData::A(record) => resolved_ips.push(record.0.to_string()),
                dns_parser::RData::AAAA(record) => resolved_ips.push(record.0.to_string()),
                _ => {}
            }
        }

        return Some(DnsMetadata {
            domain,
            resolved_ips,
        });
    }
    None
}

pub fn parse_tls(payload: &[u8]) -> Option<TlsMetadata> {
    if let Ok((_, record)) = parse_tls_plaintext(payload) {
        let mut sni = None;
        let mut ja3_str = String::new();

        for msg in record.msg {
            if let TlsMessage::Handshake(TlsMessageHandshake::ClientHello(client_hello)) = msg {
                // JA3 part 1: SSLVersion
                ja3_str.push_str(&format!("{},", u16::from(client_hello.version)));

                // JA3 part 2: Cipher suites
                let ciphers: Vec<String> = client_hello.ciphers.iter().map(|c| u16::from(*c).to_string()).collect();
                ja3_str.push_str(&format!("{},", ciphers.join("-")));

                if let Some(ext_bytes) = client_hello.ext {
                    if let Ok((_, extensions)) = tls_parser::parse_tls_extensions(ext_bytes) {
                        let mut ext_types = Vec::new();
                        let mut ellip_curves = Vec::new();
                        let mut ec_point_formats = Vec::new();

                        for ext in extensions {
                            ext_types.push(u16::from(TlsExtensionType::from(&ext)).to_string());

                            match ext {
                                TlsExtension::SNI(sni_ext) => {
                                    if let Some(first_name) = sni_ext.first() {
                                        if let Ok(name_str) = std::str::from_utf8(first_name.1) {
                                            sni = Some(name_str.to_string());
                                        }
                                    }
                                }
                                TlsExtension::EllipticCurves(curves_ext) => {
                                    ellip_curves = curves_ext.iter().map(|c| c.0.to_string()).collect();
                                }
                                TlsExtension::EcPointFormats(formats_ext) => {
                                    ec_point_formats = formats_ext.iter().map(|f| f.to_string()).collect();
                                }
                                _ => {}
                            }
                        }

                        // JA3 part 3: Extensions
                        ja3_str.push_str(&format!("{},", ext_types.join("-")));
                        // JA3 part 4: Elliptic curves
                        ja3_str.push_str(&format!("{},", ellip_curves.join("-")));
                        // JA3 part 5: Elliptic curve point formats
                        ja3_str.push_str(&ec_point_formats.join("-"));
                    }
                } else {
                    ja3_str.push_str(",,"); // Empty extensions, curves, formats
                }

                // Compute MD5 hash of the JA3 string
                let mut hasher = Md5::new();
                hasher.update(ja3_str.as_bytes());
                let result = hasher.finalize();
                let ja3_fingerprint = hex::encode(result);

                return Some(TlsMetadata {
                    sni,
                    ja3_fingerprint: Some(ja3_fingerprint),
                });
            }
        }
    }
    None
}

pub fn inspect_payload(payload: &[u8]) -> ApplicationLayer {
    if let Some(http) = parse_http(payload) {
        return ApplicationLayer::Http(http);
    }

    if let Some(tls) = parse_tls(payload) {
        return ApplicationLayer::Tls(tls);
    }

    if let Some(dns) = parse_dns(payload) {
        return ApplicationLayer::Dns(dns);
    }

    ApplicationLayer::Unknown
}

#[cfg(test)]
#[path = "dpi_tests.rs"]
mod tests;
