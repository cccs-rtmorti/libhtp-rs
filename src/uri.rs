use crate::{
    bstr::Bstr,
    config::{DecoderConfig, HtpUnwanted},
    parsers::{credentials, fragment, hostname, parse_hostport, path, port, query, scheme},
    util::{
        convert_port, decode_uri_path_inplace, urldecode_inplace, urldecode_uri_inplace,
        utf8_decode_and_validate_uri_path_inplace, Flags,
    },
};
use nom::{combinator::opt, sequence::tuple};

use std::io::Write;

/// URI structure. Each of the fields provides access to a single
/// URI element. Where an element is not present in a URI, the
/// corresponding field will be set to NULL or -1, depending on the
/// field type.
#[derive(Clone, Debug)]
pub struct Uri {
    /// Scheme, e.g., "http".
    pub scheme: Option<Bstr>,
    /// Username.
    pub username: Option<Bstr>,
    /// Password.
    pub password: Option<Bstr>,
    /// Hostname.
    pub hostname: Option<Bstr>,
    /// Port, as string.
    pub port: Option<Bstr>,
    /// Port, as number. This field will be None if there was
    /// no port information in the URI or the port information
    /// was invalid (e.g., it's not a number or it falls out of range.
    pub port_number: Option<u16>,
    /// The path part of this URI.
    pub path: Option<Bstr>,
    /// Query string.
    pub query: Option<Bstr>,
    /// Fragment identifier. This field will rarely be available in a server-side
    /// setting, but it's not impossible to see it.
    pub fragment: Option<Bstr>,
}

impl Uri {
    pub fn new(
        scheme: Option<Bstr>,
        username: Option<Bstr>,
        password: Option<Bstr>,
        hostname: Option<Bstr>,
        port: Option<Bstr>,
        port_number: Option<u16>,
        path: Option<Bstr>,
        query: Option<Bstr>,
        fragment: Option<Bstr>,
    ) -> Self {
        Self {
            scheme,
            username,
            password,
            hostname,
            port,
            port_number,
            path,
            query,
            fragment,
        }
    }

    pub fn default() -> Self {
        Self {
            scheme: None,
            username: None,
            password: None,
            hostname: None,
            port: None,
            port_number: None,
            path: None,
            query: None,
            fragment: None,
        }
    }

    pub fn normalized_scheme(&self) -> Option<Bstr> {
        if let Some(mut scheme) = self.scheme.clone() {
            scheme.make_ascii_lowercase();
            Some(scheme)
        } else {
            None
        }
    }

    pub fn normalized_username(
        &self,
        decoder_cfg: &DecoderConfig,
        flags: &mut Flags,
    ) -> Option<Bstr> {
        if let Some(mut username) = self.username.clone() {
            let _ = urldecode_uri_inplace(decoder_cfg, flags, &mut username);
            Some(username)
        } else {
            None
        }
    }

    pub fn normalized_password(
        &self,
        decoder_cfg: &DecoderConfig,
        flags: &mut Flags,
    ) -> Option<Bstr> {
        if let Some(mut password) = self.password.clone() {
            let _ = urldecode_uri_inplace(decoder_cfg, flags, &mut password);
            Some(password)
        } else {
            None
        }
    }

    pub fn normalized_hostname(
        &self,
        decoder_cfg: &DecoderConfig,
        flags: &mut Flags,
    ) -> Option<Bstr> {
        if let Some(mut hostname) = self.hostname.clone() {
            let _ = urldecode_uri_inplace(decoder_cfg, flags, &mut hostname);
            hostname.make_ascii_lowercase();
            // Remove dots from the end of the string.
            while hostname.last() == Some(&('.' as u8)) {
                hostname.pop();
            }
            Some(hostname)
        } else {
            None
        }
    }

    pub fn normalized_port(&self, flags: &mut Flags) -> Option<u16> {
        if let Some(port) = self.port.clone() {
            if let Some(port) = convert_port(&port.as_slice()) {
                Some(port)
            } else {
                // Failed to parse the port number.
                *flags |= Flags::HOSTU_INVALID;
                None
            }
        } else {
            None
        }
    }

    pub fn normalized_fragment(
        &self,
        decoder_cfg: &DecoderConfig,
        flags: &mut Flags,
    ) -> Option<Bstr> {
        if let Some(mut fragment) = self.fragment.clone() {
            let _ = urldecode_uri_inplace(decoder_cfg, flags, &mut fragment);
            Some(fragment)
        } else {
            None
        }
    }

    pub fn normalized_path(
        &self,
        decoder_cfg: &DecoderConfig,
        flags: &mut Flags,
        status: &mut HtpUnwanted,
    ) -> Option<Bstr> {
        if let Some(mut path) = self.path.clone() {
            // Decode URL-encoded (and %u-encoded) characters, as well as lowercase,
            // compress separators and convert backslashes.
            // Ignore result.
            let _ = decode_uri_path_inplace(decoder_cfg, flags, status, &mut path);
            // Handle UTF-8 in the path. Validate it first, and only save it if cfg specifies it
            utf8_decode_and_validate_uri_path_inplace(&decoder_cfg, flags, status, &mut path);
            // RFC normalization.
            normalize_uri_path_inplace(&mut path);
            Some(path)
        } else {
            None
        }
    }

    /// Parses request URI, making no attempt to validate the contents.
    ///
    /// It attempts, but is not guaranteed to successfully parse out a scheme, username, password, hostname, port, query, and fragment.
    /// Note: only attempts to extract a username, password, and hostname and subsequently port if it successfully parsed a scheme.
    pub fn parse_uri(&mut self, input: &[u8]) {
        let res = tuple((
            opt(tuple((
                scheme(),
                opt(credentials()),
                opt(tuple((hostname(), opt(port())))),
            ))),
            opt(path()),
            opt(query()),
            opt(fragment()),
        ))(input);
        if let Ok((_, (scheme_authority, path, query, fragment))) = res {
            if let Some(path) = path {
                self.path = Some(Bstr::from(path));
            }
            if let Some(query) = query {
                self.query = Some(Bstr::from(query));
            }
            if let Some(fragment) = fragment {
                self.fragment = Some(Bstr::from(fragment));
            }
            if let Some((scheme, authority, hostname_port)) = scheme_authority {
                self.scheme = Some(Bstr::from(scheme));
                if let Some((username, password)) = authority {
                    self.username = Some(Bstr::from(username));
                    if let Some(password) = password {
                        self.password = Some(Bstr::from(password));
                    }
                }
                if let Some((hostname, port)) = hostname_port {
                    self.hostname = Some(Bstr::from(hostname));
                    if let Some(port) = port {
                        self.port = Some(Bstr::from(port));
                    }
                }
            }
        }
    }

    /// Parses hostport provided in the URI.
    pub fn parse_uri_hostport(&mut self, hostport: &Bstr, flags: &mut Flags) {
        if let Ok((_, (host, port_nmb, mut valid))) = parse_hostport(hostport) {
            let hostname = &host.to_ascii_lowercase();
            self.hostname = Some(Bstr::from(hostname.as_slice()));
            if let Some((port, port_nmb)) = port_nmb {
                self.port = Some(Bstr::from(port));
                if let Some(num) = port_nmb {
                    self.port_number = Some(num);
                } else {
                    valid = false;
                }
            }
            if !valid {
                *flags |= Flags::HOSTU_INVALID
            }
        }
    }

    pub fn generate_normalized_uri(
        &mut self,
        decoder_cfg: &DecoderConfig,
    ) -> (Option<Bstr>, Option<Bstr>) {
        // On the first pass determine the length of the final bstrs
        let mut partial_len = 0usize;
        let mut complete_len = 0usize;
        complete_len += self
            .scheme
            .as_ref()
            .map(|scheme| scheme.len() + 3)
            .unwrap_or(0); // '://'
        complete_len += self
            .username
            .as_ref()
            .map(|username| username.len())
            .unwrap_or(0);
        complete_len += self
            .password
            .as_ref()
            .map(|password| password.len())
            .unwrap_or(0);
        if self.username.is_some() || self.password.is_some() {
            complete_len += 2; // ':' and '@'
        }
        complete_len += self
            .hostname
            .as_ref()
            .map(|hostname| hostname.len())
            .unwrap_or(0);
        complete_len += self.port.as_ref().map(|port| port.len()).unwrap_or(0); // ':'
        partial_len += self.path.as_ref().map(|path| path.len()).unwrap_or(0);
        partial_len += self
            .query
            .as_ref()
            .map(|query| query.len() + 1)
            .unwrap_or(0); // ?
        partial_len += self
            .fragment
            .as_ref()
            .map(|fragment| fragment.len() + 1)
            .unwrap_or(0); // #
        complete_len += partial_len;
        // On the second pass construct the string
        let mut normalized_uri = Bstr::with_capacity(complete_len);
        let mut partial_normalized_uri = Bstr::with_capacity(partial_len);

        if let Some(scheme) = self.scheme.as_ref() {
            normalized_uri.add(scheme.as_slice());
            normalized_uri.add("://");
        }
        if self.username.is_some() || self.password.is_some() {
            if let Some(username) = self.username.as_ref() {
                normalized_uri.add(username.as_slice());
            }
            normalized_uri.add(":");
            if let Some(password) = self.password.as_ref() {
                normalized_uri.add(password.as_slice());
            }
            normalized_uri.add("@");
        }
        if let Some(hostname) = self.hostname.as_ref() {
            normalized_uri.add(hostname.as_slice());
        }
        if let Some(port) = self.port.as_ref() {
            normalized_uri.add(":");
            normalized_uri.add(port.as_slice());
        }
        if let Some(path) = self.path.as_ref() {
            partial_normalized_uri.add(path.as_slice());
        }
        if let Some(mut query) = self.query.clone() {
            let mut flags = Flags::empty();
            let _ = urldecode_inplace(decoder_cfg, &mut query, &mut flags);
            partial_normalized_uri.add("?");
            partial_normalized_uri.add(query.as_slice());
        }
        if let Some(fragment) = self.fragment.as_ref() {
            partial_normalized_uri.add("#");
            partial_normalized_uri.add(fragment.as_slice());
        }
        normalized_uri.add(partial_normalized_uri.as_slice());
        if normalized_uri.len() > 0 {
            if partial_normalized_uri.len() > 0 {
                (Some(partial_normalized_uri), Some(normalized_uri))
            } else {
                (None, Some(normalized_uri))
            }
        } else {
            (None, None)
        }
    }
}

/// Normalize URL path in place. This function implements the remove dot segments algorithm
/// specified in RFC 3986, section 5.2.4.
fn normalize_uri_path_inplace(s: &mut Bstr) {
    let mut out = Vec::<&[u8]>::with_capacity(10);
    s.as_slice()
        .split(|c| *c == '/' as u8)
        .for_each(|segment| match segment {
            b"." => {}
            b".." => {
                if !(out.len() == 1 && out[0] == b"") {
                    out.pop();
                }
            }
            x => out.push(x),
        });
    let out = out.join(b"/" as &[u8]);
    s.clear();
    s.add(out.as_slice());
}

//Tests
#[allow(dead_code)]
fn UriIsExpected(expected: &Uri, actual: &Uri) -> Result<(), std::io::Error> {
    let mut msg: Vec<u8> = vec![];
    let mut equal: bool = true;

    if actual.scheme != expected.scheme {
        equal = false;
        append_message(
            &mut msg,
            b"scheme",
            expected.scheme.as_ref(),
            actual.scheme.as_ref(),
        )?;
    }

    if actual.username != expected.username {
        equal = false;
        append_message(
            &mut msg,
            b"username",
            expected.username.as_ref(),
            actual.username.as_ref(),
        )?;
    }

    if actual.password != expected.password {
        equal = false;
        append_message(
            &mut msg,
            b"password",
            expected.password.as_ref(),
            (*actual).password.as_ref(),
        )?;
    }

    if actual.hostname != expected.hostname {
        equal = false;
        append_message(
            &mut msg,
            b"hostname",
            expected.hostname.as_ref(),
            actual.hostname.as_ref(),
        )?;
    }

    if actual.port != expected.port {
        equal = false;
        append_message(
            &mut msg,
            b"port",
            expected.port.as_ref(),
            actual.port.as_ref(),
        )?;
    }

    if actual.path != expected.path {
        equal = false;
        append_message(
            &mut msg,
            b"path",
            expected.path.as_ref(),
            actual.path.as_ref(),
        )?;
    }

    if actual.query != expected.query {
        equal = false;
        append_message(
            &mut msg,
            b"query",
            expected.query.as_ref(),
            actual.query.as_ref(),
        )?;
    }

    if actual.fragment != expected.fragment {
        equal = false;
        append_message(
            &mut msg,
            b"fragment",
            expected.fragment.as_ref(),
            actual.fragment.as_ref(),
        )?;
    }

    if equal {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            std::str::from_utf8(&msg).unwrap(),
        ))
    }
}

#[allow(dead_code)]
fn append_message<W: Write>(
    o: &mut W,
    label: &[u8],
    expected: Option<&Bstr>,
    actual: Option<&Bstr>,
) -> Result<(), std::io::Error> {
    o.write(label)?;
    o.write(b" missmatch: ")?;
    if let Some(expected) = expected {
        o.write(b"'")?;
        o.write(expected.as_slice())?;

        o.write(b"'")?;
    } else {
        o.write(b"<NULL>")?;
    }
    o.write(b" != ")?;
    if let Some(actual) = actual {
        o.write(b"'")?;
        o.write(actual.as_slice())?;
        o.write(b"'")?;
    } else {
        o.write(b"<NULL>")?;
    }
    o.write(b"\n")?;
    Ok(())
}

#[test]
fn ParseUri() {
    let tests = [
        (
            Some(Bstr::from(
                "http://user:pass@www.example.com:1234/path1/path2?a=b&c=d#frag",
            )),
            Uri::new(
                Some(Bstr::from("http")),
                Some(Bstr::from("user")),
                Some(Bstr::from("pass")),
                Some(Bstr::from("www.example.com")),
                Some(Bstr::from("1234")),
                None,
                Some(Bstr::from("/path1/path2")),
                Some(Bstr::from("a=b&c=d")),
                Some(Bstr::from("frag")),
            ),
        ),
        (
            Some(Bstr::from("http://host.com/path")),
            Uri::new(
                Some(Bstr::from("http")),
                None,
                None,
                Some(Bstr::from("host.com")),
                None,
                None,
                Some(Bstr::from("/path")),
                None,
                None,
            ),
        ),
        (
            Some(Bstr::from("http://host.com")),
            Uri::new(
                Some(Bstr::from("http")),
                None,
                None,
                Some(Bstr::from("host.com")),
                None,
                None,
                None,
                None,
                None,
            ),
        ),
        (
            Some(Bstr::from("http://")),
            Uri::new(
                Some(Bstr::from("http")),
                None,
                None,
                None,
                None,
                None,
                Some(Bstr::from("//")),
                None,
                None,
            ),
        ),
        (
            Some(Bstr::from("/path")),
            Uri::new(
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Bstr::from("/path")),
                None,
                None,
            ),
        ),
        (
            Some(Bstr::from("://")),
            Uri::new(
                Some(Bstr::from("")),
                None,
                None,
                None,
                None,
                None,
                Some(Bstr::from("//")),
                None,
                None,
            ),
        ),
        (
            Some(Bstr::from("")),
            Uri::new(None, None, None, None, None, None, None, None, None),
        ),
        (
            Some(Bstr::from("http://user@host.com")),
            Uri::new(
                Some(Bstr::from("http")),
                Some(Bstr::from("user")),
                None,
                Some(Bstr::from("host.com")),
                None,
                None,
                None,
                None,
                None,
            ),
        ),
        (
            None,
            Uri::new(None, None, None, None, None, None, None, None, None),
        ),
    ]
    .to_vec();
    for test in tests {
        let mut uri = Uri::default();
        if test.0.is_some() {
            uri.parse_uri(test.0.as_ref().unwrap().as_slice())
        };
        if let Err(x) = UriIsExpected(&test.1, &uri) {
            println!("{}", x);
            println!("Failed URI = {:?}", test.0.unwrap());
            assert!(false);
        }
    }
}

#[test]
fn GenerateNormalizedUri1() {
    let cfg = DecoderConfig::default();
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from("http"));
    uri.username = Some(Bstr::from("user"));
    uri.password = Some(Bstr::from("pass"));
    uri.hostname = Some(Bstr::from("www.example.com"));
    uri.port = Some(Bstr::from("1234"));
    uri.path = Some(Bstr::from("/path1/path2"));
    uri.query = Some(Bstr::from("a=b&c=d"));
    uri.fragment = Some(Bstr::from("frag"));

    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(&cfg);
    assert_eq!(
        partial_normalized_uri,
        Some(Bstr::from("/path1/path2?a=b&c=d#frag"))
    );
    assert_eq!(
        normalized_uri,
        Some(Bstr::from(
            "http://user:pass@www.example.com:1234/path1/path2?a=b&c=d#frag"
        ))
    );
}

#[test]
fn GenerateNormalizedUri2() {
    let cfg = DecoderConfig::default();
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from("http"));
    uri.hostname = Some(Bstr::from("host.com"));
    uri.path = Some(Bstr::from("/path"));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(&cfg);
    assert_eq!(partial_normalized_uri, Some(Bstr::from("/path")));
    assert_eq!(normalized_uri, Some(Bstr::from("http://host.com/path")));
}

#[test]
fn GenerateNormalizedUri3() {
    let cfg = DecoderConfig::default();
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from("http"));
    uri.hostname = Some(Bstr::from("host.com"));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(&cfg);
    assert_eq!(partial_normalized_uri, None);
    assert_eq!(normalized_uri, Some(Bstr::from("http://host.com")));
}

#[test]
fn GenerateNormalizedUri4() {
    let cfg = DecoderConfig::default();
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from("http"));
    uri.path = Some(Bstr::from("//"));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(&cfg);
    assert_eq!(partial_normalized_uri, Some(Bstr::from("//")));
    assert_eq!(normalized_uri, Some(Bstr::from("http:////")));
}

#[test]
fn GenerateNormalizedUri5() {
    let cfg = DecoderConfig::default();
    let mut uri = Uri::default();
    uri.path = Some(Bstr::from("/path"));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(&cfg);
    assert_eq!(partial_normalized_uri, Some(Bstr::from("/path")));
    assert_eq!(normalized_uri, Some(Bstr::from("/path")));
}

#[test]
fn GenerateNormalizedUri6() {
    let cfg = DecoderConfig::default();
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from(""));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(&cfg);
    assert_eq!(partial_normalized_uri, None);
    assert_eq!(normalized_uri, Some(Bstr::from("://")));
}

#[test]
fn GenerateNormalizedUri7() {
    let cfg = DecoderConfig::default();
    let mut uri = Uri::default();
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(&cfg);
    assert_eq!(partial_normalized_uri, None);
    assert_eq!(normalized_uri, None);
}

#[test]
fn GenerateNormalizedUri8() {
    let cfg = DecoderConfig::default();

    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from("http"));
    uri.username = Some(Bstr::from("user"));
    uri.hostname = Some(Bstr::from("host.com"));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(&cfg);
    assert_eq!(partial_normalized_uri, None);
    assert_eq!(normalized_uri, Some(Bstr::from("http://user:@host.com")));
}

#[test]
fn NormalizeUriPath() {
    let mut s = Bstr::from("/a/b/c/./../../g");
    normalize_uri_path_inplace(&mut s);
    assert!(s.eq("/a/g"));

    let mut s = Bstr::from("mid/content=5/../6");
    normalize_uri_path_inplace(&mut s);
    assert!(s.eq("mid/6"));

    let mut s = Bstr::from("./one");
    normalize_uri_path_inplace(&mut s);
    assert!(s.eq("one"));

    let mut s = Bstr::from("../one");
    normalize_uri_path_inplace(&mut s);
    assert!(s.eq("one"));

    let mut s = Bstr::from(".");
    normalize_uri_path_inplace(&mut s);
    assert!(s.eq(""));

    let mut s = Bstr::from("..");
    normalize_uri_path_inplace(&mut s);
    assert!(s.eq(""));

    let mut s = Bstr::from("one/.");
    normalize_uri_path_inplace(&mut s);
    assert!(s.eq("one"));

    let mut s = Bstr::from("one/..");
    normalize_uri_path_inplace(&mut s);
    assert!(s.eq(""));

    let mut s = Bstr::from("one/../");
    normalize_uri_path_inplace(&mut s);
    assert!(s.eq(""));

    let mut s = Bstr::from("/../../../images.gif");
    normalize_uri_path_inplace(&mut s);
    assert!(s.eq("/images.gif"));
}
