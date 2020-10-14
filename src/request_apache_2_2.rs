use crate::connection_parser;
use crate::error::Result;

impl connection_parser::htp_connp_t {
    /// Extract one request header. A header can span multiple lines, in
    /// which case they will be folded into one before parsing is attempted.
    ///
    /// Returns HTP_OK or HTP_ERROR
    pub unsafe fn process_request_header_apache_2_2(&mut self, data: &[u8]) -> Result<()> {
        self.process_request_header_generic(data)
    }
    /// Parse request line as Apache 2.2 does.
    ///
    /// Returns HTP_OK or HTP_ERROR
    pub unsafe fn parse_request_line_apache_2_2(&mut self) -> Result<()> {
        self.parse_request_line_generic_ex(true)
    }
}
