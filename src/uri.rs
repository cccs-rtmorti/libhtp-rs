use crate::{
    bstr::Bstr,
    config::{DecoderConfig, HtpUnwanted},
    log::Logger,
    parsers::{credentials, fragment, hostname, parse_hostport, path, port, query, scheme},
    util::{
        convert_port, decode_uri_path_inplace, urldecode_inplace, urldecode_uri_inplace,
        utf8_decode_and_validate_uri_path_inplace, FlagOperations, HtpFlags,
    },
};
use nom::{combinator::opt, sequence::tuple};

/// URI structure. Each of the fields provides access to a single
/// URI element. Where an element is not present in a URI, the
/// corresponding field will be set to NULL or -1, depending on the
/// field type.
#[derive(Clone)]
pub struct Uri {
    /// Decoder configuration
    pub cfg: DecoderConfig,
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

impl std::fmt::Debug for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Uri")
            .field("scheme", &self.scheme)
            .field("username", &self.username)
            .field("password", &self.password)
            .field("hostname", &self.hostname)
            .field("port", &self.port)
            .field("port_number", &self.port_number)
            .field("path", &self.path)
            .field("query", &self.query)
            .field("fragment", &self.fragment)
            .finish()
    }
}

impl Uri {
    /// Create an empty Uri struct but with the given DecoderCfg
    pub fn with_config(cfg: DecoderConfig) -> Self {
        Self {
            cfg,
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
    /// Create a new Uri struct from given values.
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
            cfg: DecoderConfig::default(),
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

    /// Create an empty Uri struct.
    pub fn default() -> Self {
        Self {
            cfg: DecoderConfig::default(),
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

    /// Normalize uri scheme.
    pub fn normalized_scheme(&self) -> Option<Bstr> {
        if let Some(mut scheme) = self.scheme.clone() {
            scheme.make_ascii_lowercase();
            Some(scheme)
        } else {
            None
        }
    }

    /// Normalize uri username.
    pub fn normalized_username(&self, flags: &mut u64) -> Option<Bstr> {
        if let Some(mut username) = self.username.clone() {
            let _ = urldecode_uri_inplace(&self.cfg, flags, &mut username);
            Some(username)
        } else {
            None
        }
    }

    /// Normalize uri password.
    pub fn normalized_password(&self, flags: &mut u64) -> Option<Bstr> {
        if let Some(mut password) = self.password.clone() {
            let _ = urldecode_uri_inplace(&self.cfg, flags, &mut password);
            Some(password)
        } else {
            None
        }
    }

    /// Normalize uri hostname.
    pub fn normalized_hostname(&self, flags: &mut u64) -> Option<Bstr> {
        if let Some(mut hostname) = self.hostname.clone() {
            let _ = urldecode_uri_inplace(&self.cfg, flags, &mut hostname);
            hostname.make_ascii_lowercase();
            // Remove dots from the end of the string.
            while hostname.last() == Some(&(b'.')) {
                hostname.pop();
            }
            Some(hostname)
        } else {
            None
        }
    }

    /// Normalize uri port.
    pub fn normalized_port(&self, flags: &mut u64) -> Option<u16> {
        if let Some(port) = self.port.clone() {
            if let Some(port) = convert_port(&port.as_slice()) {
                Some(port)
            } else {
                // Failed to parse the port number.
                flags.set(HtpFlags::HOSTU_INVALID);
                None
            }
        } else {
            None
        }
    }

    /// Normalize uri fragment.
    pub fn normalized_fragment(&self, flags: &mut u64) -> Option<Bstr> {
        if let Some(mut fragment) = self.fragment.clone() {
            let _ = urldecode_uri_inplace(&self.cfg, flags, &mut fragment);
            Some(fragment)
        } else {
            None
        }
    }

    /// Normalize uri path.
    pub fn normalized_path(&self, flags: &mut u64, status: &mut HtpUnwanted) -> Option<Bstr> {
        if let Some(mut path) = self.path.clone() {
            // Decode URL-encoded (and %u-encoded) characters, as well as lowercase,
            // compress separators and convert backslashes.
            // Ignore result.
            decode_uri_path_inplace(&self.cfg, flags, status, &mut path);
            // Handle UTF-8 in the path. Validate it first, and only save it if cfg specifies it
            utf8_decode_and_validate_uri_path_inplace(&self.cfg, flags, status, &mut path);
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
    pub fn parse_uri_hostport(&mut self, hostport: &Bstr, flags: &mut u64) {
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
                flags.set(HtpFlags::HOSTU_INVALID)
            }
        }
    }

    /// Generate a normalized uri string.
    pub fn generate_normalized_uri(
        &self,
        mut logger: Option<Logger>,
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
        if let Some(mut path) = self.path.clone() {
            // Path is already decoded when we parsed the uri in transaction, only decode once more
            if self.cfg.double_decode_normalized_path {
                let path_len = path.len();
                let _ = urldecode_inplace(&self.cfg, &mut path);
                if path_len > path.len() {
                    if let Some(logger) = logger.as_mut() {
                        htp_warn!(
                            logger,
                            HtpLogCode::DOUBLE_ENCODED_URI,
                            "URI path is double encoded"
                        );
                    }
                }
            }
            partial_normalized_uri.add(path.as_slice());
        }
        if let Some(mut query) = self.query.clone() {
            let _ = urldecode_inplace(&self.cfg, &mut query);
            if self.cfg.double_decode_normalized_query {
                let query_len = query.len();
                let _ = urldecode_inplace(&self.cfg, &mut query);
                if query_len > query.len() {
                    if let Some(logger) = logger.as_mut() {
                        htp_warn!(
                            logger,
                            HtpLogCode::DOUBLE_ENCODED_URI,
                            "URI query is double encoded"
                        );
                    }
                }
            }
            partial_normalized_uri.add("?");
            partial_normalized_uri.add(query.as_slice());
        }
        if let Some(fragment) = self.fragment.as_ref() {
            partial_normalized_uri.add("#");
            partial_normalized_uri.add(fragment.as_slice());
        }
        normalized_uri.add(partial_normalized_uri.as_slice());
        if !normalized_uri.is_empty() {
            if !partial_normalized_uri.is_empty() {
                (Some(partial_normalized_uri), Some(normalized_uri))
            } else {
                (None, Some(normalized_uri))
            }
        } else {
            (None, None)
        }
    }
}

/// Normalize URI path in place. This function implements the remove dot segments algorithm
/// specified in RFC 3986, section 5.2.4.
fn normalize_uri_path_inplace(s: &mut Bstr) {
    let mut out = Vec::<&[u8]>::with_capacity(10);
    s.as_slice()
        .split(|c| *c == b'/')
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
        assert_eq!(test.1.scheme, uri.scheme);
        assert_eq!(test.1.username, uri.username);
        assert_eq!(test.1.password, uri.password);
        assert_eq!(test.1.hostname, uri.hostname);
        assert_eq!(test.1.port, uri.port);
        assert_eq!(test.1.path, uri.path);
        assert_eq!(test.1.query, uri.query);
        assert_eq!(test.1.fragment, uri.fragment);
    }
}

#[test]
fn GenerateNormalizedUri1() {
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from("http"));
    uri.username = Some(Bstr::from("user"));
    uri.password = Some(Bstr::from("pass"));
    uri.hostname = Some(Bstr::from("www.example.com"));
    uri.port = Some(Bstr::from("1234"));
    uri.path = Some(Bstr::from("/path1/path2"));
    uri.query = Some(Bstr::from("a=b&c=d"));
    uri.fragment = Some(Bstr::from("frag"));

    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(None);
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
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from("http"));
    uri.hostname = Some(Bstr::from("host.com"));
    uri.path = Some(Bstr::from("/path"));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(None);
    assert_eq!(partial_normalized_uri, Some(Bstr::from("/path")));
    assert_eq!(normalized_uri, Some(Bstr::from("http://host.com/path")));
}

#[test]
fn GenerateNormalizedUri3() {
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from("http"));
    uri.hostname = Some(Bstr::from("host.com"));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(None);
    assert_eq!(partial_normalized_uri, None);
    assert_eq!(normalized_uri, Some(Bstr::from("http://host.com")));
}

#[test]
fn GenerateNormalizedUri4() {
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from("http"));
    uri.path = Some(Bstr::from("//"));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(None);
    assert_eq!(partial_normalized_uri, Some(Bstr::from("//")));
    assert_eq!(normalized_uri, Some(Bstr::from("http:////")));
}

#[test]
fn GenerateNormalizedUri5() {
    let mut uri = Uri::default();
    uri.path = Some(Bstr::from("/path"));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(None);
    assert_eq!(partial_normalized_uri, Some(Bstr::from("/path")));
    assert_eq!(normalized_uri, Some(Bstr::from("/path")));
}

#[test]
fn GenerateNormalizedUri6() {
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from(""));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(None);
    assert_eq!(partial_normalized_uri, None);
    assert_eq!(normalized_uri, Some(Bstr::from("://")));
}

#[test]
fn GenerateNormalizedUri7() {
    let uri = Uri::default();
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(None);
    assert_eq!(partial_normalized_uri, None);
    assert_eq!(normalized_uri, None);
}

#[test]
fn GenerateNormalizedUri8() {
    let mut uri = Uri::default();
    uri.scheme = Some(Bstr::from("http"));
    uri.username = Some(Bstr::from("user"));
    uri.hostname = Some(Bstr::from("host.com"));
    let (partial_normalized_uri, normalized_uri) = uri.generate_normalized_uri(None);
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
