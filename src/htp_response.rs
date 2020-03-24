use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* *
 * Find the element at the given index.
 *
 * @param[in] l
 * @param[in] idx
 * @return the desired element, or NULL if the list is too small, or
 *         if the element at that position carries a NULL
 */
    #[no_mangle]
    fn htp_list_array_get(l: *const htp_list_array_t, idx: size_t)
     -> *mut libc::c_void;
    /* *
 * Append a memory region to destination, growing destination if necessary. If
 * the string is expanded, the pointer will change. You must replace the
 * original destination pointer with the returned one. Destination is not
 * changed on memory allocation failure.
 *
 * @param[in] b
 * @param[in] data
 * @param[in] len
 * @return Updated bstring, or NULL on memory allocation failure.
 */
    #[no_mangle]
    fn bstr_add_mem(b: *mut bstr, data: *const libc::c_void, len: size_t)
     -> *mut bstr;
    /* *
 * Adjust bstring length. You will need to use this method whenever
 * you work directly with the string contents, and end up changing
 * its length by direct structure manipulation.
 *
 * @param[in] b
 * @param[in] newlen
 */
    #[no_mangle]
    fn bstr_adjust_len(b: *mut bstr, newlen: size_t);
    /* *
 * Return the first position of the provided byte.
 *
 * @param[in] b
 * @param[in] c
 * @return The first position of the byte, or -1 if it could not be found
 */
    #[no_mangle]
    fn bstr_chr(b: *const bstr, c: libc::c_int) -> libc::c_int;
    /* *
 * Case-insensitive comparison of a bstring with a NUL-terminated string.
 *
 * @param[in] b
 * @param[in] cstr
 * @return Zero on string match, 1 if b is greater than cstr, and -1 if cstr is greater than b.
 */
    #[no_mangle]
    fn bstr_cmp_c_nocase(b: *const bstr, cstr: *const libc::c_char)
     -> libc::c_int;
    /* *
 * Create a new bstring by copying the provided NUL-terminated string.
 *
 * @param[in] cstr
 * @return New bstring, or NULL if memory allocation failed.
 */
    #[no_mangle]
    fn bstr_dup_c(cstr: *const libc::c_char) -> *mut bstr;
    /* *
 * Create a copy of the provided bstring, then convert it to lowercase.
 *
 * @param[in] b
 * @return New bstring, or NULL if memory allocation failed
 */
    #[no_mangle]
    fn bstr_dup_lower(b: *const bstr) -> *mut bstr;
    /* *
 * Create a new bstring by copying the provided memory region.
 *
 * @param[in] data
 * @param[in] len
 * @return New bstring, or NULL if memory allocation failed
 */
    #[no_mangle]
    fn bstr_dup_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
    /* *
 * Deallocate the supplied bstring instance and set it to NULL. Allows NULL on
 * input.
 *
 * @param[in] b
 */
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
    /* *
 * Find the needle in the haystack, with the needle being a NUL-terminated
 * string. Ignore case differences.
 *
 * @param[in] bhaystack
 * @param[in] cneedle
 * @return Position of the match, or -1 if the needle could not be found.
 */
    #[no_mangle]
    fn bstr_index_of_c_nocase(bhaystack: *const bstr,
                              cneedle: *const libc::c_char) -> libc::c_int;
    /* *
 * Find the needle in the haystack, with the needle being a NUL-terminated
 * string. Ignore case differences. Skip zeroes in haystack
 *
 * @param[in] bhaystack
 * @param[in] cneedle
 * @return Position of the match, or -1 if the needle could not be found.
 */
    #[no_mangle]
    fn bstr_index_of_c_nocasenorzero(bhaystack: *const bstr,
                                     cneedle: *const libc::c_char)
     -> libc::c_int;
    /* *
 * Frees all data contained in the uri, and then the uri itself.
 * 
 * @param[in] uri
 */
    /* *
 * Allocates and initializes a new htp_uri_t structure.
 *
 * @return New structure, or NULL on memory allocation failure.
 */
    /* *
 * Creates a new log entry and stores it with the connection. The file and line
 * parameters are typically auto-generated using the HTP_LOG_MARK macro.
*
 * @param[in] connp
 * @param[in] file
 * @param[in] line
 * @param[in] level
 * @param[in] code
 * @param[in] fmt
 * @param[in] ...
 */
    #[no_mangle]
    fn htp_log(connp: *mut htp_connp_t, file: *const libc::c_char,
               line: libc::c_int, level: htp_log_level_t, code: libc::c_int,
               fmt: *const libc::c_char, _: ...);
    /* *
 * Creates a copy of the provided hook. The hook is allowed to be NULL,
 * in which case this function simply returns a NULL.
 *
 * @param[in] hook
 * @return A copy of the hook, or NULL (if the provided hook was NULL
 *         or, if it wasn't, if there was a memory allocation problem while
 *         constructing a copy).
 */
    /* *
 * Creates a new hook.
 *
 * @return New htp_hook_t structure on success, NULL on failure.
 */
    /* *
 * Destroys an existing hook. It is all right to send a NULL
 * to this method because it will simply return straight away.
 *
 * @param[in] hook
 */
    /* *
 * Registers a new callback with the hook.
 *
 * @param[in] hook
 * @param[in] callback_fn
 * @return HTP_OK on success, HTP_ERROR on memory allocation error.
 */
    /* *
 * Runs all the callbacks associated with a given hook. Only stops if
 * one of the callbacks returns an error (HTP_ERROR) or stop (HTP_STOP).
 *
 * @param[in] hook
 * @param[in] user_data
 * @return HTP_OK if at least one hook ran successfully, HTP_STOP if there was
 *         no error but processing should stop, and HTP_ERROR or any other value
 *         less than zero on error.
 */
    #[no_mangle]
    fn htp_hook_run_all(hook: *mut htp_hook_t, user_data: *mut libc::c_void)
     -> htp_status_t;
    // Parsing functions
    // Private transaction functions
    // Utility functions
    #[no_mangle]
    fn htp_is_folding_char(c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn htp_connp_is_line_folded(data: *mut libc::c_uchar, len: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn htp_chomp(data: *mut libc::c_uchar, len: *mut size_t) -> libc::c_int;
    #[no_mangle]
    fn htp_tx_state_response_complete_ex(tx: *mut htp_tx_t,
                                         hybrid_mode: libc::c_int)
     -> htp_status_t;
    #[no_mangle]
    fn htp_tx_res_process_body_data_ex(tx: *mut htp_tx_t,
                                       data: *const libc::c_void, len: size_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_treat_response_line_as_body(data: *const uint8_t, len: size_t)
     -> libc::c_int;
    /* *
 * Creates a new transaction structure.
 *
 * @param[in] connp Connection parser pointer. Must not be NULL.
 * @return The newly created transaction, or NULL on memory allocation failure.
 */
    /* *
 * Destroys the supplied transaction.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 */
    /* *
 * Determines if the transaction used a shared configuration structure. See the
 * documentation for htp_tx_set_config() for more information why you might want
 * to know that.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_CFG_SHARED or HTP_CFG_PRIVATE.
 */
    /* *
 * Returns the user data associated with this transaction.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return A pointer to user data or NULL.
 */
    /* *
 * Registers a callback that will be invoked to process the transaction's request body data.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] callback_fn Callback function pointer. Must not be NULL.
 */
    /* *
 * Registers a callback that will be invoked to process the transaction's response body data.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] callback_fn Callback function pointer. Must not be NULL.
 */
    /* *
 * Adds one parameter to the request. THis function will take over the
 * responsibility for the provided htp_param_t structure.
 * 
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] param Parameter pointer. Must not be NULL.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Returns the first request parameter that matches the given name, using case-insensitive matching.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] name Name data pointer. Must not be NULL.
 * @param[in] name_len Name data length.
 * @return htp_param_t instance, or NULL if parameter not found.
 */
    /* *
 * Returns the first request parameter from the given source that matches the given name,
 * using case-insensitive matching.
 * 
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] source Parameter source (where in request the parameter was located).
 * @param[in] name Name data pointer. Must not be NULL.
 * @param[in] name_len Name data length.
 * @return htp_param_t instance, or NULL if parameter not found.
 */
    /* *
 * Determine if the request has a body.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return 1 if there is a body, 0 otherwise.
 */
    /* *
 * Process a chunk of request body data. This function assumes that
 * handling of chunked encoding is implemented by the container. When
 * you're done submitting body data, invoke a state change (to REQUEST)
 * to finalize any processing that might be pending. The supplied data is
 * fully consumed and there is no expectation that it will be available
 * afterwards. The protocol parsing code makes no copies of the data,
 * but some parsers might.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] data Data pointer. Must not be NULL.
 * @param[in] len Data length.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Set one request header. This function should be invoked once for
 * each available header, and in the order in which headers were
 * seen in the request.
 * 
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] name Name data pointer. Must not be NULL.
 * @param[in] name_len Name data length.
 * @param[in] value Value data pointer. Must not be NULL.
 * @param[in] value_len Value data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Removes all request headers associated with this transaction. This
 * function is needed because in some cases the container does not
 * differentiate between standard and trailing headers. In that case,
 * you set request headers once at the beginning of the transaction,
 * read the body (at this point the request headers should contain the
 * mix of regular and trailing headers), clear all headers, and then set
 * them all again.
 * 
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Set request line. When used, this function should always be called first,
 * with more specific functions following. Must not contain line terminators.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] line Line data pointer. Must not be NULL.
 * @param[in] line_len Line data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Set transaction request method. This function will enable you to keep
 * track of the text representation of the method.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] method Method data pointer. Must not be NULL.
 * @param[in] method_len Method data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Set transaction request method number. This function enables you to
 * keep track how a particular method string is interpreted. This function
 * is useful with web servers that ignore invalid methods; for example, some
 * web servers will treat them as a GET.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] method_number Method number.
 */
    /* *
 * Set parsed request URI. You don't need to use this function if you are already providing
 * the request line or request URI. But if your container already has this data available,
 * feeding it to LibHTP will minimize any potential data differences. This function assumes
 * management of the data provided in parsed_uri. This function will not change htp_tx_t::parsed_uri_raw
 * (which may have data in it from the parsing of the request URI).
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] parsed_uri URI pointer. Must not be NULL.
 */
    /* *
 * Forces HTTP/0.9 as the transaction protocol. This method exists to ensure
 * that both LibHTP and the container treat the transaction as HTTP/0.9, despite
 * potential differences in how the protocol version is determined.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] is_protocol_0_9 Zero if protocol is not HTTP/0.9, or 1 if it is.
 */
    /* *
 * Sets the request protocol string (e.g., "HTTP/1.0"). The information provided
 * is only stored, not parsed. Use htp_tx_req_set_protocol_number() to set the
 * actual protocol number, as interpreted by the container.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] protocol Protocol data pointer. Must not be NULL.
 * @param[in] protocol_len Protocol data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Set request protocol version number. Must be invoked after
 * htp_txh_set_req_protocol(), because it will overwrite the previously
 * extracted version number. Convert the protocol version number to an integer
 * by multiplying it with 100. For example, 1.1 becomes 110. Alternatively,
 * use the HTP_PROTOCOL_0_9, HTP_PROTOCOL_1_0, and HTP_PROTOCOL_1_1 constants.
 * Note: setting protocol to HTP_PROTOCOL_0_9 alone will _not_ get the library to
 * treat the transaction as HTTP/0.9. You need to also invoke htp_tx_req_set_protocol_0_9().
 * This is because HTTP 0.9 is used only when protocol information is absent from the
 * request line, and not when it is explicitly stated (as "HTTP/0.9"). This behavior is
 * consistent with that of Apache httpd.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] protocol_number Protocol number.
 */
    /* *
 * Set transaction request URI. The value provided here will be stored in htp_tx_t::request_uri
 * and subsequently parsed. If htp_tx_req_set_line() was previously used, the uri provided
 * when calling this function will overwrite any previously parsed value.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] uri URI data pointer. Must not be NULL.
 * @param[in] uri_len URI data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Process a chunk of response body data. This function assumes that
 * handling of chunked encoding is implemented by the container. When
 * you're done submitting body data, invoking a state change (to RESPONSE)
 * will finalize any processing that might be pending.
 *
 * The response body data will be decompressed if two conditions are met: one,
 * decompression is enabled in configuration and two, if the response headers
 * indicate compression. Alternatively, you can control decompression from
 * a RESPONSE_HEADERS callback, by setting tx->response_content_encoding either
 * to COMPRESSION_NONE (to disable compression), or to one of the supported
 * decompression algorithms.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] data Data pointer. Must not be NULL.
 * @param[in] len Data length.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Set one response header. This function should be invoked once for
 * each available header, and in the order in which headers were
 * seen in the response.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] name Name data pointer. Must not be NULL.
 * @param[in] name_len Name data length.
 * @param[in] value Value data pointer. Must not be NULL.
 * @param[in] value_len Value length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Removes all response headers associated with this transaction. This
 * function is needed because in some cases the container does not
 * differentiate between standard and trailing headers. In that case,
 * you set response headers once at the beginning of the transaction,
 * read the body, clear all headers, and then set them all again. After
 * the headers are set for the second time, they will potentially contain
 * a mixture of standard and trailing headers.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Set response protocol number. See htp_tx_res_set_protocol_number() for more information
 * about the correct format of the protocol_parameter parameter.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] protocol_number Protocol number.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Set response line. Use this function is you have a single buffer containing
 * the entire line. If you have individual request line pieces, use the other
 * available functions.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] line Line data pointer. Must not be NULL.
 * @param[in] line_len Line data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Set response status code.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] status_code Response status code.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Set response status message, which is the part of the response
 * line that comes after the status code.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] msg Message data pointer. Must not be NULL.
 * @param[in] msg_len Message data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Sets the configuration that is to be used for this transaction. If the
 * second parameter is set to HTP_CFG_PRIVATE, the transaction will adopt
 * the configuration structure and destroy it when appropriate. This function is
 * useful if you need to make changes to configuration on per-transaction basis.
 * Initially, all transactions will share the configuration with that of the
 * connection; if you were to make changes on it, they would affect all
 * current and future connections. To work around that, you make a copy of the
 * configuration object, call this function with the second parameter set to
 * HTP_CFG_PRIVATE, and modify configuration at will.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] cfg Configuration pointer. Must not be NULL.
 * @param[in] is_cfg_shared HTP_CFG_SHARED or HTP_CFG_PRIVATE
 */
    /* *
 * Associates user data with this transaction.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] user_data Opaque user data pointer.
 */
    /* *
 * Change transaction state to REQUEST and invoke registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
    /* *
 * Change transaction state to REQUEST_HEADERS and invoke all
 * registered callbacks.
 * 
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
    /* *
 * Change transaction state to REQUEST_LINE and invoke all
 * registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
    /* *
 * Initialize hybrid parsing mode, change state to TRANSACTION_START,
 * and invoke all registered callbacks.
 * 
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
    /* *
 * Change transaction state to RESPONSE and invoke registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
    /* *
 * Change transaction state to RESPONSE_HEADERS and invoke registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
    #[no_mangle]
    fn htp_tx_state_response_headers(tx: *mut htp_tx_t) -> htp_status_t;
    /* *
 * Add a new element to the table. The key will be copied, and the copy
 * managed by the table. The table keeps a pointer to the element. It is the
 * callers responsibility to ensure the pointer remains valid.
 *
 * @param[in] table
 * @param[in] key
 * @param[in] element
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Add a new element to the table. The key provided will be adopted and managed
 * by the table. You should not keep a copy of the pointer to the key unless you're
 * certain that the table will live longer that the copy. The table keeps a pointer
 * to the element. It is the callers responsibility to ensure the pointer remains
 * valid.
 *
 * @param[in] table
 * @param[in] key
 * @param[in] element
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Add a new element to the table. The key provided will be only referenced and the
 * caller remains responsible to keep it alive until after the table is destroyed. The
 * table keeps a pointer to the element. It is the callers responsibility to ensure
 * the pointer remains valid.
 *
 * @param[in] table
 * @param[in] key
 * @param[in] element
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    /* *
 * Remove all elements from the table. This function handles keys
 * according to the active allocation strategy. If the elements need freeing,
 * you need to free them before invoking this function.
 *
 * @param[in] table
 */
    /* *
 * Remove all elements from the table without freeing any of the keys, even
 * if the table is using an allocation strategy where keys belong to it. This
 * function is useful if all the keys have been adopted by some other structure.
 *
 * @param[in] table
 */
    /* *
 * Create a new table structure. The table will grow automatically as needed,
 * but you are required to provide a starting size.
 *
 * @param[in] size The starting size.
 * @return Newly created table instance, or NULL on failure.
 */
    /* *
 * Destroy a table. This function handles the keys according to the active
 * allocation strategy. If the elements need freeing, you need to free them
 * before invoking this function. After the table has been destroyed,
 * the pointer is set to NULL.
 *
 * @param[in]   table
 */
    /* *
 * Destroy the given table, but don't free the keys. even if they are managed by
 * the table. Use this method when the responsibility for the keys has been transferred
 * elsewhere. After the table has been destroyed, the pointer is set to NULL.
 *
 * @param[in] table
 */
    /* *
 * Retrieve the first element that matches the given bstr key.
 *
 * @param[in] table
 * @param[in] key
 * @return Matched element, or NULL if no elements match the key.
 */
    /* *
 * Retrieve the first element that matches the given NUL-terminated key.
 *
 * @param[in] table
 * @param[in] ckey
 * @return Matched element, or NULL if no elements match the key.
 */
    #[no_mangle]
    fn htp_table_get_c(table: *const htp_table_t, ckey: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn htp_parse_content_length(b: *mut bstr, connp: *mut htp_connp_t)
     -> int64_t;
    #[no_mangle]
    fn htp_parse_chunked_length(data: *mut libc::c_uchar, len: size_t)
     -> int64_t;
    #[no_mangle]
    fn htp_is_space(c: libc::c_int) -> libc::c_int;
    /* *
 * Change transaction state to HTP_RESPONSE_LINE and invoke registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
    #[no_mangle]
    fn htp_tx_state_response_line(tx: *mut htp_tx_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_is_line_ignorable(connp: *mut htp_connp_t,
                                   data: *mut libc::c_uchar, len: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn htp_table_clear(table: *mut htp_table_t);
    /* *
 * Retrieve key and element at the given index.
 *
 * @param[in] table
 * @param[in] idx
 * @param[in,out] key Pointer in which the key will be returned. Can be NULL.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
    #[no_mangle]
    fn htp_table_get_index(table: *const htp_table_t, idx: size_t,
                           key: *mut *mut bstr) -> *mut libc::c_void;
    /* *
 * Retrieve table key defined by the provided pointer and length.
 *
 * @param[in] table
 * @param[in] key
 * @param[in] key_len
 * @return Matched element, or NULL if no elements match the key.
 */
    /* *
 * Return the size of the table.
 *
 * @param[in] table
 * @return table size
 */
    #[no_mangle]
    fn htp_table_size(table: *const htp_table_t) -> size_t;
    #[no_mangle]
    fn htp_connp_is_line_terminator(connp: *mut htp_connp_t,
                                    data: *mut libc::c_uchar, len: size_t)
     -> libc::c_int;
    /* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
    /* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
    /* *
 * Creates a new connection structure.
 * 
 * @return A new connection structure on success, NULL on memory allocation failure.
 */
    /* *
 * Closes the connection.
 *
 * @param[in] conn
 * @param[in] timestamp
 */
    /* *
 * Destroys a connection, as well as all the transactions it contains. It is
 * not possible to destroy a connection structure yet leave any of its
 * transactions intact. This is because transactions need its connection and
 * connection structures hold little data anyway. The opposite is true, though
 * it is possible to delete a transaction but leave its connection alive.
 *
 * @param[in] conn
 */
    /* *
 * Opens a connection. This function will essentially only store the provided data
 * for future reference. The timestamp parameter is optional.
 * 
 * @param[in] conn
 * @param[in] remote_addr
 * @param[in] remote_port
 * @param[in] local_addr
 * @param[in] local_port
 * @param[in] timestamp
 * @return
 */
    /* *
 * Removes the given transaction structure, which makes it possible to
 * safely destroy it. It is safe to destroy transactions in this way
 * because the index of the transactions (in a connection) is preserved.
 *
 * @param[in] conn
 * @param[in] tx
 * @return HTP_OK if transaction was removed (replaced with NULL) or HTP_ERROR if it wasn't found.
 */
    /* *
 * Keeps track of inbound packets and data.
 *
 * @param[in] conn
 * @param[in] len
 * @param[in] timestamp
 */
    /* *
 * Keeps track of outbound packets and data.
 * 
 * @param[in] conn
 * @param[in] len
 * @param[in] timestamp
 */
    #[no_mangle]
    fn htp_conn_track_outbound_data(conn: *mut htp_conn_t, len: size_t,
                                    timestamp: *const htp_time_t);
    /* *
 * Change transaction state to RESPONSE_START and invoke registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
    #[no_mangle]
    fn htp_tx_state_response_start(tx: *mut htp_tx_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_FINALIZE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_uri_alloc() -> *mut htp_uri_t;
    /* *
 * Create a new transaction using the connection parser provided.
 *
 * @param[in] connp
 * @return Transaction instance on success, NULL on failure.
 */
    #[no_mangle]
    fn htp_connp_tx_create(connp: *mut htp_connp_t) -> *mut htp_tx_t;
    #[no_mangle]
    fn htp_tx_state_request_complete(tx: *mut htp_tx_t) -> htp_status_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
pub type htp_status_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_cfg_t {
    pub field_limit_hard: size_t,
    pub field_limit_soft: size_t,
    pub log_level: htp_log_level_t,
    pub tx_auto_destroy: libc::c_int,
    pub server_personality: htp_server_personality_t,
    pub parse_request_line: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                                       -> libc::c_int>,
    pub parse_response_line: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                                        -> libc::c_int>,
    pub process_request_header: Option<unsafe extern "C" fn(_:
                                                                *mut htp_connp_t,
                                                            _:
                                                                *mut libc::c_uchar,
                                                            _: size_t)
                                           -> libc::c_int>,
    pub process_response_header: Option<unsafe extern "C" fn(_:
                                                                 *mut htp_connp_t,
                                                             _:
                                                                 *mut libc::c_uchar,
                                                             _: size_t)
                                            -> libc::c_int>,
    pub parameter_processor: Option<unsafe extern "C" fn(_: *mut htp_param_t)
                                        -> libc::c_int>,
    pub decoder_cfgs: [htp_decoder_cfg_t; 3],
    pub generate_request_uri_normalized: libc::c_int,
    pub response_decompression_enabled: libc::c_int,
    pub request_encoding: *mut libc::c_char,
    pub internal_encoding: *mut libc::c_char,
    pub parse_request_cookies: libc::c_int,
    pub parse_request_auth: libc::c_int,
    pub extract_request_files: libc::c_int,
    pub extract_request_files_limit: libc::c_int,
    pub tmpdir: *mut libc::c_char,
    pub hook_request_start: *mut htp_hook_t,
    pub hook_request_line: *mut htp_hook_t,
    pub hook_request_uri_normalize: *mut htp_hook_t,
    pub hook_request_header_data: *mut htp_hook_t,
    pub hook_request_headers: *mut htp_hook_t,
    pub hook_request_body_data: *mut htp_hook_t,
    pub hook_request_file_data: *mut htp_hook_t,
    pub hook_request_trailer_data: *mut htp_hook_t,
    pub hook_request_trailer: *mut htp_hook_t,
    pub hook_request_complete: *mut htp_hook_t,
    pub hook_response_start: *mut htp_hook_t,
    pub hook_response_line: *mut htp_hook_t,
    pub hook_response_header_data: *mut htp_hook_t,
    pub hook_response_headers: *mut htp_hook_t,
    pub hook_response_body_data: *mut htp_hook_t,
    pub hook_response_trailer_data: *mut htp_hook_t,
    pub hook_response_trailer: *mut htp_hook_t,
    pub hook_response_complete: *mut htp_hook_t,
    pub hook_transaction_complete: *mut htp_hook_t,
    pub hook_log: *mut htp_hook_t,
    pub user_data: *mut libc::c_void,
    pub requestline_leading_whitespace_unwanted: htp_unwanted_t,
    pub response_decompression_layer_limit: libc::c_int,
    pub lzma_memlimit: size_t,
    pub compression_bomb_limit: int32_t,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* *
 * Decoder contexts.
 */
/* * Default settings. Settings applied to this context are propagated to all other contexts. */
/* * Urlencoded decoder settings. */
/* * URL path decoder settings. */
/* *
 * Enumerates the possible server personalities.
 */
/* *
     * Minimal personality that performs at little work as possible. All optional
     * features are disabled. This personality is a good starting point for customization.
     */
/* * A generic personality that aims to work reasonably well for all server types. */
/* * The IDS personality tries to perform as much decoding as possible. */
/* * Mimics the behavior of IIS 4.0, as shipped with Windows NT 4.0. */
/* * Mimics the behavior of IIS 5.0, as shipped with Windows 2000. */
/* * Mimics the behavior of IIS 5.1, as shipped with Windows XP Professional. */
/* * Mimics the behavior of IIS 6.0, as shipped with Windows 2003. */
/* * Mimics the behavior of IIS 7.0, as shipped with Windows 2008. */
/* Mimics the behavior of IIS 7.5, as shipped with Windows 7. */
/* Mimics the behavior of Apache 2.x. */
/* *
 * Enumerates the ways in which servers respond to malformed data.
 */
pub type htp_unwanted_t = libc::c_uint;
/* * Responds with HTTP 404 status code. */
pub const HTP_UNWANTED_404: htp_unwanted_t = 404;
/* * Responds with HTTP 400 status code. */
pub const HTP_UNWANTED_400: htp_unwanted_t = 400;
/* * Ignores problem. */
pub const HTP_UNWANTED_IGNORE: htp_unwanted_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_hook_t {
    pub callbacks: *mut htp_list_array_t,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_list_array_t {
    pub first: size_t,
    pub last: size_t,
    pub max_size: size_t,
    pub current_size: size_t,
    pub elements: *mut *mut libc::c_void,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_decoder_cfg_t {
    pub backslash_convert_slashes: libc::c_int,
    pub convert_lowercase: libc::c_int,
    pub path_separators_compress: libc::c_int,
    pub path_separators_decode: libc::c_int,
    pub plusspace_decode: libc::c_int,
    pub path_separators_encoded_unwanted: htp_unwanted_t,
    pub nul_raw_terminates: libc::c_int,
    pub nul_raw_unwanted: htp_unwanted_t,
    pub control_chars_unwanted: htp_unwanted_t,
    pub u_encoding_decode: libc::c_int,
    pub u_encoding_unwanted: htp_unwanted_t,
    pub url_encoding_invalid_handling: htp_url_encoding_handling_t,
    pub url_encoding_invalid_unwanted: htp_unwanted_t,
    pub nul_encoded_terminates: libc::c_int,
    pub nul_encoded_unwanted: htp_unwanted_t,
    pub utf8_invalid_unwanted: htp_unwanted_t,
    pub utf8_convert_bestfit: libc::c_int,
    pub bestfit_map: *mut libc::c_uchar,
    pub bestfit_replacement_byte: libc::c_uchar,
}
/* *
 * Enumerates the possible approaches to handling invalid URL-encodings.
 */
pub type htp_url_encoding_handling_t = libc::c_uint;
/* * Decode invalid URL encodings. */
pub const HTP_URL_DECODE_PROCESS_INVALID: htp_url_encoding_handling_t = 2;
/* * Ignore invalid URL encodings, but remove the % from the data. */
pub const HTP_URL_DECODE_REMOVE_PERCENT: htp_url_encoding_handling_t = 1;
/* * Ignore invalid URL encodings and leave the % in the data. */
pub const HTP_URL_DECODE_PRESERVE_PERCENT: htp_url_encoding_handling_t = 0;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 * 
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* *
 * Represents a single TCP connection.
 */
/* * Client IP address. */
/* * Client port. */
/* * Server IP address. */
/* * Server port. */
/* *
     * Transactions carried out on this connection. The list may contain
     * NULL elements when some of the transactions are deleted (and then
     * removed from a connection by calling htp_conn_remove_tx().
     */
/* * Log messages associated with this connection. */
/* * Parsing flags: HTP_CONN_PIPELINED. */
/* * When was this connection opened? Can be NULL. */
/* * When was this connection closed? Can be NULL. */
/* * Inbound data counter. */
/* * Outbound data counter. */
/* *
 * Used to represent files that are seen during the processing of HTTP traffic. Most
 * commonly this refers to files seen in multipart/form-data payloads. In addition, PUT
 * request bodies can be treated as files.
 */
/* * Where did this file come from? Possible values: HTP_FILE_MULTIPART and HTP_FILE_PUT. */
/* * File name, as provided (e.g., in the Content-Disposition multipart part header. */
/* * File length. */
/* * The unique filename in which this file is stored on the filesystem, when applicable.*/
/* * The file descriptor used for external storage, or -1 if unused. */
/* *
 * Represents a chunk of file data.
 */
/* * File information. */
/* * Pointer to the data buffer. */
/* * Buffer length. */
/* *
 * Represents a single log entry.
 */
/* * The connection parser associated with this log message. */
/* * The transaction associated with this log message, if any. */
/* * Log message. */
/* * Message level. */
/* * Message code. */
/* * File in which the code that emitted the message resides. */
/* * Line number on which the code that emitted the message resides. */
/* *
 * Represents a single request or response header.
 */
/* * Header name. */
/* * Header value. */
/* * Parsing flags; a combination of: HTP_FIELD_INVALID, HTP_FIELD_FOLDED, HTP_FIELD_REPEATED. */
/* *
 * Represents a single request parameter.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_param_t {
    pub name: *mut bstr,
    pub value: *mut bstr,
    pub source: htp_data_source_t,
    pub parser_id: htp_parser_id_t,
    pub parser_data: *mut libc::c_void,
}
// Below are all htp_status_t return codes used by LibHTP. Enum is not
// used here to allow applications to define their own codes.
/* *
 * The lowest htp_status_t value LibHTP will use internally.
 */
/* * General-purpose error code. */
/* *
 * No processing or work was done. This is typically used by callbacks
 * to indicate that they were not interested in doing any work in the
 * given context.
 */
/* * Returned by a function when its work was successfully completed. */
/* *
 * Returned when processing a connection stream, after consuming all
 * provided data. The caller should call again with more data.
 */
/* *
 * Returned when processing a connection stream, after encountering
 * a situation where processing needs to continue on the alternate
 * stream (e.g., the inbound parser needs to observe some outbound
 * data). The data provided was not completely consumed. On the next
 * invocation the caller should supply only the data that has not
 * been processed already. Use htp_connp_req_data_consumed() and
 * htp_connp_res_data_consumed() to determine how much of the most
 * recent data chunk was consumed.
 */
/* *
 * Used by callbacks to indicate that the processing should stop. For example,
 * returning HTP_STOP from a connection callback indicates that LibHTP should
 * stop following that particular connection.
 */
/* *
 * Same as HTP_DATA, but indicates that any non-consumed part of the
 * data chunk should be preserved (buffered) for later.
 */
/* *
 * The highest htp_status_t value LibHTP will use internally.
 */
/* *
 * Enumerates the possible values for authentication type.
 */
/* *
     * This is the default value that is used before
     * the presence of authentication is determined (e.g.,
     * before request headers are seen).
     */
/* * No authentication. */
/* * HTTP Basic authentication used. */
/* * HTTP Digest authentication used. */
/* * Unrecognized authentication method. */
/* *
     * This is the default value, which is used until the presence
     * of content encoding is determined (e.g., before request headers
     * are seen.
     */
/* * No compression. */
/* * Gzip compression. */
/* * Deflate compression. */
/* * LZMA compression. */
/* *
 * Enumerates the possible request and response body codings.
 */
/* * Body coding not determined yet. */
/* * No body. */
/* * Identity coding is used, which means that the body was sent as is. */
/* * Chunked encoding. */
/* * We could not recognize the encoding. */
// Various flag bits. Even though we have a flag field in several places
// (header, transaction, connection), these fields are all in the same namespace
// because we may want to set the same flag in several locations. For example, we
// may set HTP_FIELD_FOLDED on the actual folded header, but also on the transaction
// that contains the header. Both uses are useful.
// Connection flags are 8 bits wide.
// All other flags are 64 bits wide.
/* At least one valid UTF-8 character and no invalid ones. */
/* Range U+FF00 - U+FFEF detected. */
/* Host in the URI. */
/* Host in the Host header. */
/* Range U+FF00 - U+FFEF detected. */
// Logging-related constants.
/* *
 * Enumerates all log levels.
 */
/* *
 * HTTP methods.
 */
/* *
     * Used by default, until the method is determined (e.g., before
     * the request line is processed.
     */
// A collection of unique parser IDs.
pub type htp_parser_id_t = libc::c_uint;
/* * multipart/form-data parser. */
pub const HTP_PARSER_MULTIPART: htp_parser_id_t = 1;
/* * application/x-www-form-urlencoded parser. */
pub const HTP_PARSER_URLENCODED: htp_parser_id_t = 0;
// Protocol version constants; an enum cannot be
// used here because we allow any properly-formatted protocol
// version (e.g., 1.3), even those that do not actually exist.
// A collection of possible data sources.
pub type htp_data_source_t = libc::c_uint;
/* * Transported in the request body. */
pub const HTP_SOURCE_BODY: htp_data_source_t = 3;
/* * Cookies. */
pub const HTP_SOURCE_COOKIE: htp_data_source_t = 2;
/* * Transported in the query string. */
pub const HTP_SOURCE_QUERY_STRING: htp_data_source_t = 1;
/* * Embedded in the URL. */
pub const HTP_SOURCE_URL: htp_data_source_t = 0;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 * 
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
pub type bstr = bstr_t;
// Data structures
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bstr_t {
    pub len: size_t,
    pub size: size_t,
    pub realptr: *mut libc::c_uchar,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* *
 * Connection parser structure.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_connp_t {
    pub cfg: *mut htp_cfg_t,
    pub conn: *mut htp_conn_t,
    pub user_data: *const libc::c_void,
    pub last_error: *mut htp_log_t,
    pub in_status: htp_stream_state_t,
    pub out_status: htp_stream_state_t,
    pub out_data_other_at_tx_end: libc::c_uint,
    pub in_timestamp: htp_time_t,
    pub in_current_data: *mut libc::c_uchar,
    pub in_current_len: int64_t,
    pub in_current_read_offset: int64_t,
    pub in_current_consume_offset: int64_t,
    pub in_current_receiver_offset: int64_t,
    pub in_chunk_count: size_t,
    pub in_chunk_request_index: size_t,
    pub in_stream_offset: int64_t,
    pub in_next_byte: libc::c_int,
    pub in_buf: *mut libc::c_uchar,
    pub in_buf_size: size_t,
    pub in_header: *mut bstr,
    pub in_tx: *mut htp_tx_t,
    pub in_content_length: int64_t,
    pub in_body_data_left: int64_t,
    pub in_chunked_length: int64_t,
    pub in_state: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> libc::c_int>,
    pub in_state_previous: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                                      -> libc::c_int>,
    pub in_data_receiver_hook: *mut htp_hook_t,
    pub out_next_tx_index: size_t,
    pub out_timestamp: htp_time_t,
    pub out_current_data: *mut libc::c_uchar,
    pub out_current_len: int64_t,
    pub out_current_read_offset: int64_t,
    pub out_current_consume_offset: int64_t,
    pub out_current_receiver_offset: int64_t,
    pub out_stream_offset: int64_t,
    pub out_next_byte: libc::c_int,
    pub out_buf: *mut libc::c_uchar,
    pub out_buf_size: size_t,
    pub out_header: *mut bstr,
    pub out_tx: *mut htp_tx_t,
    pub out_content_length: int64_t,
    pub out_body_data_left: int64_t,
    pub out_chunked_length: int64_t,
    pub out_state: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                              -> libc::c_int>,
    pub out_state_previous: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                                       -> libc::c_int>,
    pub out_data_receiver_hook: *mut htp_hook_t,
    pub out_decompressor: *mut htp_decompressor_t,
    pub put_file: *mut htp_file_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_file_t {
    pub source: htp_file_source_t,
    pub filename: *mut bstr,
    pub len: int64_t,
    pub tmpname: *mut libc::c_char,
    pub fd: libc::c_int,
}
pub type htp_file_source_t = libc::c_uint;
pub const HTP_FILE_PUT: htp_file_source_t = 2;
pub const HTP_FILE_MULTIPART: htp_file_source_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_decompressor_t {
    pub decompress: Option<unsafe extern "C" fn(_: *mut htp_decompressor_t,
                                                _: *mut htp_tx_data_t)
                               -> htp_status_t>,
    pub callback: Option<unsafe extern "C" fn(_: *mut htp_tx_data_t)
                             -> htp_status_t>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut htp_decompressor_t)
                            -> ()>,
    pub next: *mut htp_decompressor_t,
}
/* *
 * Represents a single HTTP transaction, which is a combination of a request and a response.
 */
/* * The connection parser associated with this transaction. */
/* * The connection to which this transaction belongs. */
/* * The configuration structure associated with this transaction. */
/* *
     * Is the configuration structure shared with other transactions or connections? If
     * this field is set to HTP_CONFIG_PRIVATE, the transaction owns the configuration.
     */
/* * The user data associated with this transaction. */
// Request fields
/* * Contains a count of how many empty lines were skipped before the request line. */
/* * The first line of this request. */
/* * Request method. */
/* * Request method, as number. Available only if we were able to recognize the request method. */
/* *
     * Request URI, raw, as given to us on the request line. This field can take different forms,
     * for example authority for CONNECT methods, absolute URIs for proxy requests, and the query
     * string when one is provided. Use htp_tx_t::parsed_uri if you need to access to specific
     * URI elements. Can be NULL if the request line contains only a request method (which is
     * an extreme case of HTTP/0.9, but passes in practice.
     */
/* * Request protocol, as text. Can be NULL if no protocol was specified. */
/* *
     * Protocol version as a number. Multiply the high version number by 100, then add the low
     * version number. You should prefer to work the pre-defined HTP_PROTOCOL_* constants.
     */
/* *
     * Is this request using HTTP/0.9? We need a separate field for this purpose because
     * the protocol version alone is not sufficient to determine if HTTP/0.9 is used. For
     * example, if you submit "GET / HTTP/0.9" to Apache, it will not treat the request
     * as HTTP/0.9.
     */
/* *
     * This structure holds the individual components parsed out of the request URI, with
     * appropriate normalization and transformation applied, per configuration. No information
     * is added. In extreme cases when no URI is provided on the request line, all fields
     * will be NULL. (Well, except for port_number, which will be -1.) To inspect raw data, use
     * htp_tx_t::request_uri or htp_tx_t::parsed_uri_raw.
     */
/* *
     * This structure holds the individual components parsed out of the request URI, but
     * without any modification. The purpose of this field is to allow you to look at the data as it
     * was supplied on the request line. Fields can be NULL, depending on what data was supplied.
     * The port_number field is always -1.
     */
/* HTTP 1.1 RFC
     * 
     * 4.3 Message Body
     * 
     * The message-body (if any) of an HTTP message is used to carry the
     * entity-body associated with the request or response. The message-body
     * differs from the entity-body only when a transfer-coding has been
     * applied, as indicated by the Transfer-Encoding header field (section
     * 14.41).
     *
     *     message-body = entity-body
     *                  | <entity-body encoded as per Transfer-Encoding>
     */
/* *
     * The length of the request message-body. In most cases, this value
     * will be the same as request_entity_len. The values will be different
     * if request compression or chunking were applied. In that case,
     * request_message_len contains the length of the request body as it
     * has been seen over TCP; request_entity_len contains length after
     * de-chunking and decompression.
     */
/* *
     * The length of the request entity-body. In most cases, this value
     * will be the same as request_message_len. The values will be different
     * if request compression or chunking were applied. In that case,
     * request_message_len contains the length of the request body as it
     * has been seen over TCP; request_entity_len contains length after
     * de-chunking and decompression.
     */
/* * Parsed request headers. */
/* *
     * Request transfer coding. Can be one of HTP_CODING_UNKNOWN (body presence not
     * determined yet), HTP_CODING_IDENTITY, HTP_CODING_CHUNKED, HTP_CODING_NO_BODY,
     * and HTP_CODING_UNRECOGNIZED.
     */
/* * Request body compression. */
/* *
     * This field contain the request content type when that information is
     * available in request headers. The contents of the field will be converted
     * to lowercase and any parameters (e.g., character set information) removed.
     */
/* *
     * Contains the value specified in the Content-Length header. The value of this
     * field will be -1 from the beginning of the transaction and until request
     * headers are processed. It will stay -1 if the C-L header was not provided,
     * or if the value in it cannot be parsed.
     */
/* *
     * Transaction-specific REQUEST_BODY_DATA hook. Behaves as
     * the configuration hook with the same name.
     */
/* *
     * Transaction-specific RESPONSE_BODY_DATA hook. Behaves as
     * the configuration hook with the same name.
     */
/* *
     * Query string URLENCODED parser. Available only
     * when the query string is not NULL and not empty.
     */
/* *
     * Request body URLENCODED parser. Available only when the request body is in the
     * application/x-www-form-urlencoded format and the parser was configured to run.
     */
/* *
     * Request body MULTIPART parser. Available only when the body is in the
     * multipart/form-data format and the parser was configured to run.
     */
/* * Request parameters. */
/* * Request cookies */
/* * Authentication type used in the request. */
/* * Authentication username. */
/* * Authentication password. Available only when htp_tx_t::request_auth_type is HTP_AUTH_BASIC. */
/* *
     * Request hostname. Per the RFC, the hostname will be taken from the Host header
     * when available. If the host information is also available in the URI, it is used
     * instead of whatever might be in the Host header. Can be NULL. This field does
     * not contain port information.
     */
/* *
     * Request port number, if presented. The rules for htp_tx_t::request_host apply. Set to
     * -1 by default.
     */
// Response fields
/* * How many empty lines did we ignore before reaching the status line? */
/* * Response line. */
/* * Response protocol, as text. Can be NULL. */
/* *
     * Response protocol as number. Available only if we were able to parse the protocol version,
     * HTP_PROTOCOL_INVALID otherwise. HTP_PROTOCOL_UNKNOWN until parsing is attempted.
     */
/* *
     * Response status code, as text. Starts as NULL and can remain NULL on
     * an invalid response that does not specify status code.
     */
/* *
     * Response status code, available only if we were able to parse it, HTP_STATUS_INVALID
     * otherwise. HTP_STATUS_UNKNOWN until parsing is attempted.
     */
/* *
     * This field is set by the protocol decoder with it thinks that the
     * backend server will reject a request with a particular status code.
     */
/* * The message associated with the response status code. Can be NULL. */
/* * Have we seen the server respond with a 100 response? */
/* * Parsed response headers. Contains instances of htp_header_t. */
/* HTTP 1.1 RFC
     * 
     * 4.3 Message Body
     * 
     * The message-body (if any) of an HTTP message is used to carry the
     * entity-body associated with the request or response. The message-body
     * differs from the entity-body only when a transfer-coding has been
     * applied, as indicated by the Transfer-Encoding header field (section
     * 14.41).
     *
     *     message-body = entity-body
     *                  | <entity-body encoded as per Transfer-Encoding>
     */
/* *
     * The length of the response message-body. In most cases, this value
     * will be the same as response_entity_len. The values will be different
     * if response compression or chunking were applied. In that case,
     * response_message_len contains the length of the response body as it
     * has been seen over TCP; response_entity_len contains the length after
     * de-chunking and decompression.
     */
/* *
     * The length of the response entity-body. In most cases, this value
     * will be the same as response_message_len. The values will be different
     * if request compression or chunking were applied. In that case,
     * response_message_len contains the length of the response body as it
     * has been seen over TCP; response_entity_len contains length after
     * de-chunking and decompression.
     */
/* *
     * Contains the value specified in the Content-Length header. The value of this
     * field will be -1 from the beginning of the transaction and until response
     * headers are processed. It will stay -1 if the C-L header was not provided,
     * or if the value in it cannot be parsed.
     */
/* *
     * Response transfer coding, which indicates if there is a response body,
     * and how it is transported (e.g., as-is, or chunked).
     */
/* *
     * Response body compression, which indicates if compression is used
     * for the response body. This field is an interpretation of the information
     * available in response headers.
     */
/* *
     * Response body compression processing information, which is related to how
     * the library is going to process (or has processed) a response body. Changing
     * this field mid-processing can influence library actions. For example, setting
     * this field to HTP_COMPRESSION_NONE in a RESPONSE_HEADERS callback will prevent
     * decompression.
     */
/* *
     * This field will contain the response content type when that information
     * is available in response headers. The contents of the field will be converted
     * to lowercase and any parameters (e.g., character set information) removed.
     */
// Common fields
/* *
     * Parsing flags; a combination of: HTP_REQUEST_INVALID_T_E, HTP_INVALID_FOLDING,
     * HTP_REQUEST_SMUGGLING, HTP_MULTI_PACKET_HEAD, and HTP_FIELD_UNPARSEABLE.
     */
/* * Request progress. */
/* * Response progress. */
/* * Transaction index on the connection. */
/* * Total repetitions for headers in request. */
/* * Total repetitions for headers in response. */
/* *
 * This structure is used to pass transaction data (for example
 * request and response body buffers) to callbacks.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_tx_data_t {
    pub tx: *mut htp_tx_t,
    pub data: *const libc::c_uchar,
    pub len: size_t,
    pub is_last: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_tx_t {
    pub connp: *mut htp_connp_t,
    pub conn: *mut htp_conn_t,
    pub cfg: *mut htp_cfg_t,
    pub is_config_shared: libc::c_int,
    pub user_data: *mut libc::c_void,
    pub request_ignored_lines: libc::c_uint,
    pub request_line: *mut bstr,
    pub request_method: *mut bstr,
    pub request_method_number: htp_method_t,
    pub request_uri: *mut bstr,
    pub request_protocol: *mut bstr,
    pub request_protocol_number: libc::c_int,
    pub is_protocol_0_9: libc::c_int,
    pub parsed_uri: *mut htp_uri_t,
    pub parsed_uri_raw: *mut htp_uri_t,
    pub request_message_len: int64_t,
    pub request_entity_len: int64_t,
    pub request_headers: *mut htp_table_t,
    pub request_transfer_coding: htp_transfer_coding_t,
    pub request_content_encoding: htp_content_encoding_t,
    pub request_content_type: *mut bstr,
    pub request_content_length: int64_t,
    pub hook_request_body_data: *mut htp_hook_t,
    pub hook_response_body_data: *mut htp_hook_t,
    pub request_urlenp_query: *mut htp_urlenp_t,
    pub request_urlenp_body: *mut htp_urlenp_t,
    pub request_mpartp: *mut htp_mpartp_t,
    pub request_params: *mut htp_table_t,
    pub request_cookies: *mut htp_table_t,
    pub request_auth_type: htp_auth_type_t,
    pub request_auth_username: *mut bstr,
    pub request_auth_password: *mut bstr,
    pub request_hostname: *mut bstr,
    pub request_port_number: libc::c_int,
    pub response_ignored_lines: libc::c_uint,
    pub response_line: *mut bstr,
    pub response_protocol: *mut bstr,
    pub response_protocol_number: libc::c_int,
    pub response_status: *mut bstr,
    pub response_status_number: libc::c_int,
    pub response_status_expected_number: libc::c_int,
    pub response_message: *mut bstr,
    pub seen_100continue: libc::c_int,
    pub response_headers: *mut htp_table_t,
    pub response_message_len: int64_t,
    pub response_entity_len: int64_t,
    pub response_content_length: int64_t,
    pub response_transfer_coding: htp_transfer_coding_t,
    pub response_content_encoding: htp_content_encoding_t,
    pub response_content_encoding_processing: htp_content_encoding_t,
    pub response_content_type: *mut bstr,
    pub flags: uint64_t,
    pub request_progress: htp_tx_req_progress_t,
    pub response_progress: htp_tx_res_progress_t,
    pub index: size_t,
    pub req_header_repetitions: uint16_t,
    pub res_header_repetitions: uint16_t,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 * 
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* 
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* *
 * Enumerate possible data handling strategies in hybrid parsing
 * mode. The two possibilities are to make copies of all data and
 * use bstr instances to wrap already available data.
 */
/* *
     * Make copies of all data. This strategy should be used when
     * the supplied buffers are transient and will go away after
     * the invoked function returns.
     */
/* *
     * Reuse buffers, without a change of ownership. We assume the
     * buffers will continue to be available until the transaction
     * is deleted by the container.
     */
/* *
 * Possible states of a progressing transaction. Internally, progress will change
 * to the next state when the processing activities associated with that state
 * begin. For example, when we start to process request line bytes, the request
 * state will change from HTP_REQUEST_NOT_STARTED to HTP_REQUEST_LINE.*
 */
pub type htp_tx_res_progress_t = libc::c_uint;
pub const HTP_RESPONSE_COMPLETE: htp_tx_res_progress_t = 5;
pub const HTP_RESPONSE_TRAILER: htp_tx_res_progress_t = 4;
pub const HTP_RESPONSE_BODY: htp_tx_res_progress_t = 3;
pub const HTP_RESPONSE_HEADERS: htp_tx_res_progress_t = 2;
pub const HTP_RESPONSE_LINE: htp_tx_res_progress_t = 1;
pub const HTP_RESPONSE_NOT_STARTED: htp_tx_res_progress_t = 0;
pub type htp_tx_req_progress_t = libc::c_uint;
pub const HTP_REQUEST_COMPLETE: htp_tx_req_progress_t = 5;
pub const HTP_REQUEST_TRAILER: htp_tx_req_progress_t = 4;
pub const HTP_REQUEST_BODY: htp_tx_req_progress_t = 3;
pub const HTP_REQUEST_HEADERS: htp_tx_req_progress_t = 2;
pub const HTP_REQUEST_LINE: htp_tx_req_progress_t = 1;
pub const HTP_REQUEST_NOT_STARTED: htp_tx_req_progress_t = 0;
pub type htp_content_encoding_t = libc::c_uint;
pub const HTP_COMPRESSION_LZMA: htp_content_encoding_t = 4;
pub const HTP_COMPRESSION_DEFLATE: htp_content_encoding_t = 3;
pub const HTP_COMPRESSION_GZIP: htp_content_encoding_t = 2;
pub const HTP_COMPRESSION_NONE: htp_content_encoding_t = 1;
pub const HTP_COMPRESSION_UNKNOWN: htp_content_encoding_t = 0;
pub type htp_transfer_coding_t = libc::c_uint;
pub const HTP_CODING_INVALID: htp_transfer_coding_t = 4;
pub const HTP_CODING_CHUNKED: htp_transfer_coding_t = 3;
pub const HTP_CODING_IDENTITY: htp_transfer_coding_t = 2;
pub const HTP_CODING_NO_BODY: htp_transfer_coding_t = 1;
pub const HTP_CODING_UNKNOWN: htp_transfer_coding_t = 0;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* * This is the default value, used only until the first element is added. */
/* * Keys are copied.*/
/* * Keys are adopted and freed when the table is destroyed. */
/* * Keys are only referenced; the caller is still responsible for freeing them after the table is destroyed. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_table_t {
    pub list: htp_list_array_t,
    pub alloc_type: htp_table_alloc_t,
}
pub type htp_table_alloc_t = libc::c_uint;
pub const HTP_TABLE_KEYS_REFERENCED: htp_table_alloc_t = 3;
pub const HTP_TABLE_KEYS_ADOPTED: htp_table_alloc_t = 2;
pub const HTP_TABLE_KEYS_COPIED: htp_table_alloc_t = 1;
pub const HTP_TABLE_KEYS_ALLOC_UKNOWN: htp_table_alloc_t = 0;
pub type htp_auth_type_t = libc::c_uint;
pub const HTP_AUTH_UNRECOGNIZED: htp_auth_type_t = 9;
pub const HTP_AUTH_DIGEST: htp_auth_type_t = 3;
pub const HTP_AUTH_BASIC: htp_auth_type_t = 2;
pub const HTP_AUTH_NONE: htp_auth_type_t = 1;
pub const HTP_AUTH_UNKNOWN: htp_auth_type_t = 0;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* * When in line mode, the parser is handling part headers. */
/* * When in data mode, the parser is consuming part data. */
/* * Initial state, after the parser has been created but before the boundary initialized. */
/* * Processing data, waiting for a new line (which might indicate a new boundary). */
/* * Testing a potential boundary. */
/* * Checking the first byte after a boundary. */
/* * Checking the second byte after a boundary. */
/* * Consuming linear whitespace after a boundary. */
/* * Used after a CR byte is detected in STATE_BOUNDARY_EAT_LWS. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_mpartp_t {
    pub multipart: htp_multipart_t,
    pub cfg: *mut htp_cfg_t,
    pub extract_files: libc::c_int,
    pub extract_limit: libc::c_int,
    pub extract_dir: *mut libc::c_char,
    pub file_count: libc::c_int,
    pub handle_data: Option<unsafe extern "C" fn(_: *mut htp_mpartp_t,
                                                 _: *const libc::c_uchar,
                                                 _: size_t, _: libc::c_int)
                                -> libc::c_int>,
    pub handle_boundary: Option<unsafe extern "C" fn(_: *mut htp_mpartp_t)
                                    -> libc::c_int>,
    pub parser_state: htp_multipart_state_t,
    pub boundary_match_pos: size_t,
    pub current_part: *mut htp_multipart_part_t,
    pub current_part_mode: htp_part_mode_t,
    pub boundary_pieces: *mut bstr_builder_t,
    pub part_header_pieces: *mut bstr_builder_t,
    pub pending_header_line: *mut bstr,
    pub part_data_pieces: *mut bstr_builder_t,
    pub boundary_candidate_pos: size_t,
    pub cr_aside: libc::c_int,
    pub gave_up_data: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bstr_builder_t {
    pub pieces: *mut htp_list_array_t,
}
pub type htp_part_mode_t = libc::c_uint;
pub const MODE_DATA: htp_part_mode_t = 1;
pub const MODE_LINE: htp_part_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_multipart_part_t {
    pub parser: *mut htp_mpartp_t,
    pub type_0: htp_multipart_type_t,
    pub len: size_t,
    pub name: *mut bstr,
    pub value: *mut bstr,
    pub content_type: *mut bstr,
    pub headers: *mut htp_table_t,
    pub file: *mut htp_file_t,
}
pub type htp_multipart_type_t = libc::c_uint;
pub const MULTIPART_PART_EPILOGUE: htp_multipart_type_t = 4;
pub const MULTIPART_PART_PREAMBLE: htp_multipart_type_t = 3;
pub const MULTIPART_PART_FILE: htp_multipart_type_t = 2;
pub const MULTIPART_PART_TEXT: htp_multipart_type_t = 1;
pub const MULTIPART_PART_UNKNOWN: htp_multipart_type_t = 0;
pub type htp_multipart_state_t = libc::c_uint;
pub const STATE_BOUNDARY_EAT_LWS_CR: htp_multipart_state_t = 6;
pub const STATE_BOUNDARY_EAT_LWS: htp_multipart_state_t = 5;
pub const STATE_BOUNDARY_IS_LAST2: htp_multipart_state_t = 4;
pub const STATE_BOUNDARY_IS_LAST1: htp_multipart_state_t = 3;
pub const STATE_BOUNDARY: htp_multipart_state_t = 2;
pub const STATE_DATA: htp_multipart_state_t = 1;
pub const STATE_INIT: htp_multipart_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_multipart_t {
    pub boundary: *mut libc::c_char,
    pub boundary_len: size_t,
    pub boundary_count: libc::c_int,
    pub parts: *mut htp_list_array_t,
    pub flags: uint64_t,
}
// The MIME type that triggers the parser. Must be lowercase.
/* *
 * This is the main URLENCODED parser structure. It is used to store
 * parser configuration, temporary parsing data, as well as the parameters.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_urlenp_t {
    pub tx: *mut htp_tx_t,
    pub argument_separator: libc::c_uchar,
    pub decode_url_encoding: libc::c_int,
    pub params: *mut htp_table_t,
    pub _state: libc::c_int,
    pub _complete: libc::c_int,
    pub _name: *mut bstr,
    pub _bb: *mut bstr_builder_t,
}
/* *
 * URI structure. Each of the fields provides access to a single
 * URI element. Where an element is not present in a URI, the
 * corresponding field will be set to NULL or -1, depending on the
 * field type.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_uri_t {
    pub scheme: *mut bstr,
    pub username: *mut bstr,
    pub password: *mut bstr,
    pub hostname: *mut bstr,
    pub port: *mut bstr,
    pub port_number: libc::c_int,
    pub path: *mut bstr,
    pub query: *mut bstr,
    pub fragment: *mut bstr,
}
pub type htp_method_t = libc::c_uint;
pub const HTP_M_INVALID: htp_method_t = 28;
pub const HTP_M_MERGE: htp_method_t = 27;
pub const HTP_M_BASELINE_CONTROL: htp_method_t = 26;
pub const HTP_M_MKACTIVITY: htp_method_t = 25;
pub const HTP_M_MKWORKSPACE: htp_method_t = 24;
pub const HTP_M_REPORT: htp_method_t = 23;
pub const HTP_M_LABEL: htp_method_t = 22;
pub const HTP_M_UPDATE: htp_method_t = 21;
pub const HTP_M_CHECKIN: htp_method_t = 20;
pub const HTP_M_UNCHECKOUT: htp_method_t = 19;
pub const HTP_M_CHECKOUT: htp_method_t = 18;
pub const HTP_M_VERSION_CONTROL: htp_method_t = 17;
pub const HTP_M_UNLOCK: htp_method_t = 16;
pub const HTP_M_LOCK: htp_method_t = 15;
pub const HTP_M_MOVE: htp_method_t = 14;
pub const HTP_M_COPY: htp_method_t = 13;
pub const HTP_M_MKCOL: htp_method_t = 12;
pub const HTP_M_PROPPATCH: htp_method_t = 11;
pub const HTP_M_PROPFIND: htp_method_t = 10;
pub const HTP_M_PATCH: htp_method_t = 9;
pub const HTP_M_TRACE: htp_method_t = 8;
pub const HTP_M_OPTIONS: htp_method_t = 7;
pub const HTP_M_CONNECT: htp_method_t = 6;
pub const HTP_M_DELETE: htp_method_t = 5;
pub const HTP_M_POST: htp_method_t = 4;
pub const HTP_M_PUT: htp_method_t = 3;
pub const HTP_M_GET: htp_method_t = 2;
pub const HTP_M_HEAD: htp_method_t = 1;
pub const HTP_M_UNKNOWN: htp_method_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_conn_t {
    pub client_addr: *mut libc::c_char,
    pub client_port: libc::c_int,
    pub server_addr: *mut libc::c_char,
    pub server_port: libc::c_int,
    pub transactions: *mut htp_list_array_t,
    pub messages: *mut htp_list_array_t,
    pub flags: uint8_t,
    pub open_timestamp: htp_time_t,
    pub close_timestamp: htp_time_t,
    pub in_data_counter: int64_t,
    pub out_data_counter: int64_t,
}
pub type htp_time_t = timeval;
/* *
 * Enumerates all stream states. Each connection has two streams, one
 * inbound and one outbound. Their states are tracked separately.
 */
pub type htp_stream_state_t = libc::c_uint;
pub const HTP_STREAM_DATA: htp_stream_state_t = 9;
pub const HTP_STREAM_STOP: htp_stream_state_t = 6;
pub const HTP_STREAM_DATA_OTHER: htp_stream_state_t = 5;
pub const HTP_STREAM_TUNNEL: htp_stream_state_t = 4;
pub const HTP_STREAM_ERROR: htp_stream_state_t = 3;
pub const HTP_STREAM_CLOSED: htp_stream_state_t = 2;
pub const HTP_STREAM_OPEN: htp_stream_state_t = 1;
pub const HTP_STREAM_NEW: htp_stream_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_log_t {
    pub connp: *mut htp_connp_t,
    pub tx: *mut htp_tx_t,
    pub msg: *const libc::c_char,
    pub level: htp_log_level_t,
    pub code: libc::c_int,
    pub file: *const libc::c_char,
    pub line: libc::c_uint,
}
pub type htp_log_level_t = libc::c_uint;
pub const HTP_LOG_DEBUG2: htp_log_level_t = 6;
pub const HTP_LOG_DEBUG: htp_log_level_t = 5;
pub const HTP_LOG_INFO: htp_log_level_t = 4;
pub const HTP_LOG_NOTICE: htp_log_level_t = 3;
pub const HTP_LOG_WARNING: htp_log_level_t = 2;
pub const HTP_LOG_ERROR: htp_log_level_t = 1;
pub const HTP_LOG_NONE: htp_log_level_t = 0;
pub type htp_server_personality_t = libc::c_uint;
pub const HTP_SERVER_APACHE_2: htp_server_personality_t = 9;
pub const HTP_SERVER_IIS_7_5: htp_server_personality_t = 8;
pub const HTP_SERVER_IIS_7_0: htp_server_personality_t = 7;
pub const HTP_SERVER_IIS_6_0: htp_server_personality_t = 6;
pub const HTP_SERVER_IIS_5_1: htp_server_personality_t = 5;
pub const HTP_SERVER_IIS_5_0: htp_server_personality_t = 4;
pub const HTP_SERVER_IIS_4_0: htp_server_personality_t = 3;
pub const HTP_SERVER_IDS: htp_server_personality_t = 2;
pub const HTP_SERVER_GENERIC: htp_server_personality_t = 1;
pub const HTP_SERVER_MINIMAL: htp_server_personality_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_header_t {
    pub name: *mut bstr,
    pub value: *mut bstr,
    pub flags: uint64_t,
}
/* *
 * Sends outstanding connection data to the currently active data receiver hook.
 *
 * @param[in] connp
 * @param[in] is_last
 * @return HTP_OK, or a value returned from a callback.
 */
unsafe extern "C" fn htp_connp_res_receiver_send_data(mut connp:
                                                          *mut htp_connp_t,
                                                      mut is_last:
                                                          libc::c_int)
 -> htp_status_t {
    if (*connp).out_data_receiver_hook.is_null() { return 1 as libc::c_int }
    let mut d: htp_tx_data_t =
        htp_tx_data_t{tx: 0 as *mut htp_tx_t,
                      data: 0 as *const libc::c_uchar,
                      len: 0,
                      is_last: 0,};
    d.tx = (*connp).out_tx;
    d.data =
        (*connp).out_current_data.offset((*connp).out_current_receiver_offset
                                             as isize);
    d.len =
        ((*connp).out_current_read_offset -
             (*connp).out_current_receiver_offset) as size_t;
    d.is_last = is_last;
    let mut rc: htp_status_t =
        htp_hook_run_all((*connp).out_data_receiver_hook,
                         &mut d as *mut htp_tx_data_t as *mut libc::c_void);
    if rc != 1 as libc::c_int { return rc }
    (*connp).out_current_receiver_offset = (*connp).out_current_read_offset;
    return 1 as libc::c_int;
}
/* *
 * Finalizes an existing data receiver hook by sending any outstanding data to it. The
 * hook is then removed so that it receives no more data.
 *
 * @param[in] connp
 * @return HTP_OK, or a value returned from a callback.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_res_receiver_finalize_clear(mut connp:
                                                                   *mut htp_connp_t)
 -> htp_status_t {
    if (*connp).out_data_receiver_hook.is_null() { return 1 as libc::c_int }
    let mut rc: htp_status_t =
        htp_connp_res_receiver_send_data(connp, 1 as libc::c_int);
    (*connp).out_data_receiver_hook = 0 as *mut htp_hook_t;
    return rc;
}
/* *
 * Configures the data receiver hook. If there is a previous hook, it will be finalized and cleared.
 *
 * @param[in] connp
 * @param[in] data_receiver_hook
 * @return HTP_OK, or a value returned from a callback.
 */
unsafe extern "C" fn htp_connp_res_receiver_set(mut connp: *mut htp_connp_t,
                                                mut data_receiver_hook:
                                                    *mut htp_hook_t)
 -> htp_status_t {
    htp_connp_res_receiver_finalize_clear(connp);
    (*connp).out_data_receiver_hook = data_receiver_hook;
    (*connp).out_current_receiver_offset = (*connp).out_current_read_offset;
    return 1 as libc::c_int;
}
/* *
 * Handles request parser state changes. At the moment, this function is used only
 * to configure data receivers, which are sent raw connection data.
 *
 * @param[in] connp
 * @return HTP_OK, or a value returned from a callback.
 */
unsafe extern "C" fn htp_res_handle_state_change(mut connp: *mut htp_connp_t)
 -> htp_status_t {
    if (*connp).out_state_previous == (*connp).out_state {
        return 1 as libc::c_int
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_HEADERS as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        let mut rc: htp_status_t = 1 as libc::c_int;
        match (*(*connp).out_tx).response_progress as libc::c_uint {
            2 => {
                rc =
                    htp_connp_res_receiver_set(connp,
                                               (*(*(*connp).out_tx).cfg).hook_response_header_data)
            }
            4 => {
                rc =
                    htp_connp_res_receiver_set(connp,
                                               (*(*(*connp).out_tx).cfg).hook_response_trailer_data)
            }
            _ => { }
        }
        if rc != 1 as libc::c_int { return rc }
    }
    // Same comment as in htp_req_handle_state_change(). Below is a copy.
    // Initially, I had the finalization of raw data sending here, but that
    // caused the last REQUEST_HEADER_DATA hook to be invoked after the
    // REQUEST_HEADERS hook -- which I thought made no sense. For that reason,
    // the finalization is now initiated from the request header processing code,
    // which is less elegant but provides a better user experience. Having some
    // (or all) hooks to be invoked on state change might work better.
    (*connp).out_state_previous = (*connp).out_state;
    return 1 as libc::c_int;
}
/* *
 * If there is any data left in the outbound data chunk, this function will preserve
 * it for later consumption. The maximum amount accepted for buffering is controlled
 * by htp_config_t::field_limit_hard.
 *
 * @param[in] connp
 * @return HTP_OK, or HTP_ERROR on fatal failure.
 */
unsafe extern "C" fn htp_connp_res_buffer(mut connp: *mut htp_connp_t)
 -> htp_status_t {
    if (*connp).out_current_data.is_null() { return 1 as libc::c_int }
    let mut data: *mut libc::c_uchar =
        (*connp).out_current_data.offset((*connp).out_current_consume_offset
                                             as isize);
    let mut len: size_t =
        ((*connp).out_current_read_offset -
             (*connp).out_current_consume_offset) as size_t;
    // Check the hard (buffering) limit.
    let mut newlen: size_t = (*connp).out_buf_size.wrapping_add(len);
    // When calculating the size of the buffer, take into account the
    // space we're using for the response header buffer.
    if !(*connp).out_header.is_null() {
        newlen =
            (newlen as libc::c_ulong).wrapping_add((*(*connp).out_header).len)
                as size_t as size_t
    }
    if newlen > (*(*(*connp).out_tx).cfg).field_limit_hard {
        htp_log(connp,
                b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                212 as libc::c_int, HTP_LOG_ERROR, 0 as libc::c_int,
                b"Response the buffer limit: size %zd limit %zd.\x00" as
                    *const u8 as *const libc::c_char, newlen,
                (*(*(*connp).out_tx).cfg).field_limit_hard);
        return -(1 as libc::c_int)
    }
    // Copy the data remaining in the buffer.
    if (*connp).out_buf.is_null() {
        (*connp).out_buf = malloc(len) as *mut libc::c_uchar;
        if (*connp).out_buf.is_null() { return -(1 as libc::c_int) }
        memcpy((*connp).out_buf as *mut libc::c_void,
               data as *const libc::c_void, len);
        (*connp).out_buf_size = len
    } else {
        let mut newsize: size_t = (*connp).out_buf_size.wrapping_add(len);
        let mut newbuf: *mut libc::c_uchar =
            realloc((*connp).out_buf as *mut libc::c_void, newsize) as
                *mut libc::c_uchar;
        if newbuf.is_null() { return -(1 as libc::c_int) }
        (*connp).out_buf = newbuf;
        memcpy((*connp).out_buf.offset((*connp).out_buf_size as isize) as
                   *mut libc::c_void, data as *const libc::c_void, len);
        (*connp).out_buf_size = newsize
    }
    // Reset the consumer position.
    (*connp).out_current_consume_offset = (*connp).out_current_read_offset;
    return 1 as libc::c_int;
}
/* *
 * Returns to the caller the memory region that should be processed next. This function
 * hides away the buffering process from the rest of the code, allowing it to work with
 * non-buffered data that's in the outbound chunk, or buffered data that's in our structures.
 *
 * @param[in] connp
 * @param[out] data
 * @param[out] len
 * @return HTP_OK
 */
unsafe extern "C" fn htp_connp_res_consolidate_data(mut connp:
                                                        *mut htp_connp_t,
                                                    mut data:
                                                        *mut *mut libc::c_uchar,
                                                    mut len: *mut size_t)
 -> htp_status_t {
    if (*connp).out_buf.is_null() {
        // We do not have any data buffered; point to the current data chunk.
        *data =
            (*connp).out_current_data.offset((*connp).out_current_consume_offset
                                                 as isize);
        *len =
            ((*connp).out_current_read_offset -
                 (*connp).out_current_consume_offset) as size_t
    } else {
        // We do have data in the buffer. Add data from the current
        // chunk, and point to the consolidated buffer.
        if htp_connp_res_buffer(connp) != 1 as libc::c_int {
            return -(1 as libc::c_int)
        }
        *data = (*connp).out_buf;
        *len = (*connp).out_buf_size
    }
    return 1 as libc::c_int;
}
/* *
 * Clears buffered outbound data and resets the consumer position to the reader position.
 *
 * @param[in] connp
 */
unsafe extern "C" fn htp_connp_res_clear_buffer(mut connp: *mut htp_connp_t) {
    (*connp).out_current_consume_offset = (*connp).out_current_read_offset;
    if !(*connp).out_buf.is_null() {
        free((*connp).out_buf as *mut libc::c_void);
        (*connp).out_buf = 0 as *mut libc::c_uchar;
        (*connp).out_buf_size = 0 as libc::c_int as size_t
    };
}
/* *
 * Consumes bytes until the end of the current line.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_RES_BODY_CHUNKED_DATA_END(mut connp:
                                                                 *mut htp_connp_t)
 -> htp_status_t {
    loop 
         // TODO We shouldn't really see anything apart from CR and LF,
    //      so we should warn about anything else.
         {
        if (*connp).out_current_read_offset < (*connp).out_current_len {
            (*connp).out_next_byte =
                *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                      as isize) as
                    libc::c_int;
            (*connp).out_current_read_offset += 1;
            (*connp).out_current_consume_offset += 1;
            (*connp).out_stream_offset += 1
        } else { return 2 as libc::c_int }
        (*(*connp).out_tx).response_message_len += 1;
        if (*connp).out_next_byte == '\n' as i32 {
            (*connp).out_state =
                Some(htp_connp_RES_BODY_CHUNKED_LENGTH as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            return 1 as libc::c_int
        }
    };
}
/* *
 * Processes a chunk of data.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_RES_BODY_CHUNKED_DATA(mut connp:
                                                             *mut htp_connp_t)
 -> htp_status_t {
    let mut bytes_to_consume: size_t = 0;
    // Determine how many bytes we can consume.
    if (*connp).out_current_len - (*connp).out_current_read_offset >=
           (*connp).out_chunked_length {
        bytes_to_consume = (*connp).out_chunked_length as size_t
    } else {
        bytes_to_consume =
            ((*connp).out_current_len - (*connp).out_current_read_offset) as
                size_t
    }
    if bytes_to_consume == 0 as libc::c_int as libc::c_ulong {
        return 2 as libc::c_int
    }
    // Consume the data.
    let mut rc: htp_status_t =
        htp_tx_res_process_body_data_ex((*connp).out_tx,
                                        (*connp).out_current_data.offset((*connp).out_current_read_offset
                                                                             as
                                                                             isize)
                                            as *const libc::c_void,
                                        bytes_to_consume);
    if rc != 1 as libc::c_int { return rc }
    // Adjust the counters.
    (*connp).out_current_read_offset =
        ((*connp).out_current_read_offset as
             libc::c_ulong).wrapping_add(bytes_to_consume) as int64_t as
            int64_t;
    (*connp).out_current_consume_offset =
        ((*connp).out_current_consume_offset as
             libc::c_ulong).wrapping_add(bytes_to_consume) as int64_t as
            int64_t;
    (*connp).out_stream_offset =
        ((*connp).out_stream_offset as
             libc::c_ulong).wrapping_add(bytes_to_consume) as int64_t as
            int64_t;
    (*connp).out_chunked_length =
        ((*connp).out_chunked_length as
             libc::c_ulong).wrapping_sub(bytes_to_consume) as int64_t as
            int64_t;
    // Have we seen the entire chunk?
    if (*connp).out_chunked_length == 0 as libc::c_int as libc::c_long {
        (*connp).out_state =
            Some(htp_connp_RES_BODY_CHUNKED_DATA_END as
                     unsafe extern "C" fn(_: *mut htp_connp_t)
                         -> htp_status_t);
        return 1 as libc::c_int
    }
    return 2 as libc::c_int;
}
/* *
 * Peeks ahead into the data to try to see if it starts with a valid Chunked
 * length field.
 *
 * @returns 1 if it looks valid, 0 if it looks invalid
 */
#[inline]
unsafe extern "C" fn data_probe_chunk_length(mut connp: *mut htp_connp_t)
 -> libc::c_int {
    if (*connp).out_current_read_offset - (*connp).out_current_consume_offset
           < 8 as libc::c_int as libc::c_long {
        // not enough data so far, consider valid still
        return 1 as libc::c_int
    }
    let mut data: *mut libc::c_uchar =
        (*connp).out_current_data.offset((*connp).out_current_consume_offset
                                             as isize);
    let mut len: size_t =
        ((*connp).out_current_read_offset -
             (*connp).out_current_consume_offset) as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        let mut c: libc::c_uchar = *data.offset(i as isize);
        if c as libc::c_int == 0xd as libc::c_int ||
               c as libc::c_int == 0xa as libc::c_int ||
               c as libc::c_int == 0x20 as libc::c_int ||
               c as libc::c_int == 0x9 as libc::c_int ||
               c as libc::c_int == 0xb as libc::c_int ||
               c as libc::c_int == 0xc as libc::c_int {
        } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
                      libc::c_int &
                      _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 ||
                      c as libc::c_int >= 'a' as i32 &&
                          c as libc::c_int <= 'f' as i32 ||
                      c as libc::c_int >= 'A' as i32 &&
                          c as libc::c_int <= 'F' as i32 {
            // real chunklen char
            return 1 as libc::c_int
        } else {
            // leading junk, bad
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Extracts chunk length.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_RES_BODY_CHUNKED_LENGTH(mut connp:
                                                               *mut htp_connp_t)
 -> htp_status_t {
    loop  {
        if (*connp).out_current_read_offset < (*connp).out_current_len {
            (*connp).out_next_byte =
                *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                      as isize) as
                    libc::c_int;
            (*connp).out_current_read_offset += 1;
            (*connp).out_stream_offset += 1
        } else { return 5 as libc::c_int }
        // Have we reached the end of the line? Or is this not chunked after all?
        if !((*connp).out_next_byte == '\n' as i32 ||
                 data_probe_chunk_length(connp) == 0) {
            continue ;
        }
        let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut len: size_t = 0;
        if htp_connp_res_consolidate_data(connp, &mut data, &mut len) !=
               1 as libc::c_int {
            return -(1 as libc::c_int)
        }
        (*(*connp).out_tx).response_message_len =
            ((*(*connp).out_tx).response_message_len as
                 libc::c_ulong).wrapping_add(len) as int64_t as int64_t;
        (*connp).out_chunked_length = htp_parse_chunked_length(data, len);
        // empty chunk length line, lets try to continue
        if (*connp).out_chunked_length ==
               -(1004 as libc::c_int) as libc::c_long {
            continue ;
        }
        if (*connp).out_chunked_length < 0 as libc::c_int as libc::c_long {
            // reset out_current_read_offset so htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE
                // doesn't miss the first bytes
            if len > (*connp).out_current_read_offset as size_t {
                (*connp).out_current_read_offset = 0 as libc::c_int as int64_t
            } else {
                (*connp).out_current_read_offset =
                    ((*connp).out_current_read_offset as
                         libc::c_ulong).wrapping_sub(len) as int64_t as
                        int64_t
            }
            (*connp).out_state =
                Some(htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*(*connp).out_tx).response_transfer_coding = HTP_CODING_IDENTITY;
            htp_log(connp,
                    b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                    421 as libc::c_int, HTP_LOG_ERROR, 0 as libc::c_int,
                    b"Response chunk encoding: Invalid chunk length: %ld\x00"
                        as *const u8 as *const libc::c_char,
                    (*connp).out_chunked_length);
            return 1 as libc::c_int
        }
        htp_connp_res_clear_buffer(connp);
        // Handle chunk length
        if (*connp).out_chunked_length > 0 as libc::c_int as libc::c_long {
            // More data available
            (*connp).out_state =
                Some(htp_connp_RES_BODY_CHUNKED_DATA as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t)
        } else if (*connp).out_chunked_length ==
                      0 as libc::c_int as libc::c_long {
            // End of data
            (*connp).out_state =
                Some(htp_connp_RES_HEADERS as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*(*connp).out_tx).response_progress = HTP_RESPONSE_TRAILER
        }
        return 1 as libc::c_int
    };
}
/* *
 * Processes an identity response body of known length.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_RES_BODY_IDENTITY_CL_KNOWN(mut connp:
                                                                  *mut htp_connp_t)
 -> htp_status_t {
    let mut bytes_to_consume: size_t = 0;
    // Determine how many bytes we can consume.
    if (*connp).out_current_len - (*connp).out_current_read_offset >=
           (*connp).out_body_data_left {
        bytes_to_consume = (*connp).out_body_data_left as size_t
    } else {
        bytes_to_consume =
            ((*connp).out_current_len - (*connp).out_current_read_offset) as
                size_t
    }
    if (*connp).out_status as libc::c_uint ==
           HTP_STREAM_CLOSED as libc::c_int as libc::c_uint {
        (*connp).out_state =
            Some(htp_connp_RES_FINALIZE as
                     unsafe extern "C" fn(_: *mut htp_connp_t)
                         -> htp_status_t);
        // Sends close signal to decompressors
        let mut rc: htp_status_t =
            htp_tx_res_process_body_data_ex((*connp).out_tx,
                                            0 as *const libc::c_void,
                                            0 as libc::c_int as size_t);
        return rc
    }
    if bytes_to_consume == 0 as libc::c_int as libc::c_ulong {
        return 2 as libc::c_int
    }
    // Consume the data.
    let mut rc_0: htp_status_t =
        htp_tx_res_process_body_data_ex((*connp).out_tx,
                                        (*connp).out_current_data.offset((*connp).out_current_read_offset
                                                                             as
                                                                             isize)
                                            as *const libc::c_void,
                                        bytes_to_consume);
    if rc_0 != 1 as libc::c_int { return rc_0 }
    // Adjust the counters.
    (*connp).out_current_read_offset =
        ((*connp).out_current_read_offset as
             libc::c_ulong).wrapping_add(bytes_to_consume) as int64_t as
            int64_t;
    (*connp).out_current_consume_offset =
        ((*connp).out_current_consume_offset as
             libc::c_ulong).wrapping_add(bytes_to_consume) as int64_t as
            int64_t;
    (*connp).out_stream_offset =
        ((*connp).out_stream_offset as
             libc::c_ulong).wrapping_add(bytes_to_consume) as int64_t as
            int64_t;
    (*connp).out_body_data_left =
        ((*connp).out_body_data_left as
             libc::c_ulong).wrapping_sub(bytes_to_consume) as int64_t as
            int64_t;
    // Have we seen the entire response body?
    if (*connp).out_body_data_left == 0 as libc::c_int as libc::c_long {
        (*connp).out_state =
            Some(htp_connp_RES_FINALIZE as
                     unsafe extern "C" fn(_: *mut htp_connp_t)
                         -> htp_status_t);
        // Tells decompressors to output partially decompressed data
        rc_0 =
            htp_tx_res_process_body_data_ex((*connp).out_tx,
                                            0 as *const libc::c_void,
                                            0 as libc::c_int as size_t);
        return rc_0
    }
    return 2 as libc::c_int;
}
/* *
 * Processes identity response body of unknown length. In this case, we assume the
 * response body consumes all data until the end of the stream.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE(mut connp:
                                                                      *mut htp_connp_t)
 -> htp_status_t {
    // Consume all data from the input buffer.
    let mut bytes_to_consume: size_t =
        ((*connp).out_current_len - (*connp).out_current_read_offset) as
            size_t;
    if bytes_to_consume != 0 as libc::c_int as libc::c_ulong {
        let mut rc: htp_status_t =
            htp_tx_res_process_body_data_ex((*connp).out_tx,
                                            (*connp).out_current_data.offset((*connp).out_current_read_offset
                                                                                 as
                                                                                 isize)
                                                as *const libc::c_void,
                                            bytes_to_consume);
        if rc != 1 as libc::c_int { return rc }
        // Adjust the counters.
        (*connp).out_current_read_offset =
            ((*connp).out_current_read_offset as
                 libc::c_ulong).wrapping_add(bytes_to_consume) as int64_t as
                int64_t;
        (*connp).out_current_consume_offset =
            ((*connp).out_current_consume_offset as
                 libc::c_ulong).wrapping_add(bytes_to_consume) as int64_t as
                int64_t;
        (*connp).out_stream_offset =
            ((*connp).out_stream_offset as
                 libc::c_ulong).wrapping_add(bytes_to_consume) as int64_t as
                int64_t
    }
    // Have we seen the entire response body?
    if (*connp).out_status as libc::c_uint ==
           HTP_STREAM_CLOSED as libc::c_int as libc::c_uint {
        (*connp).out_state =
            Some(htp_connp_RES_FINALIZE as
                     unsafe extern "C" fn(_: *mut htp_connp_t)
                         -> htp_status_t);
        return 1 as libc::c_int
    }
    return 2 as libc::c_int;
}
/* *
 * Determines presence (and encoding) of a response body.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_RES_BODY_DETERMINE(mut connp:
                                                          *mut htp_connp_t)
 -> htp_status_t {
    // If the request uses the CONNECT method, then not only are we
    // to assume there's no body, but we need to ignore all
    // subsequent data in the stream.
    if (*(*connp).out_tx).request_method_number as libc::c_uint ==
           HTP_M_CONNECT as libc::c_int as libc::c_uint {
        if (*(*connp).out_tx).response_status_number >= 200 as libc::c_int &&
               (*(*connp).out_tx).response_status_number <= 299 as libc::c_int
           {
            // This is a successful CONNECT stream, which means
            // we need to switch into tunneling mode: on the
            // request side we'll now probe the tunnel data to see
            // if we need to parse or ignore it. So on the response
            // side we wrap up the tx and wait.
            (*connp).out_state =
                Some(htp_connp_RES_FINALIZE as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            // we may have response headers
            let mut rc: htp_status_t =
                htp_tx_state_response_headers((*connp).out_tx);
            return rc
        } else {
            if (*(*connp).out_tx).response_status_number == 407 as libc::c_int
               {
                // proxy telling us to auth
                (*connp).in_status = HTP_STREAM_DATA
            } else {
                // This is a failed CONNECT stream, which means that
            // we can unblock request parsing
                (*connp).in_status = HTP_STREAM_DATA;
                // We are going to continue processing this transaction,
            // adding a note for ourselves to stop at the end (because
            // we don't want to see the beginning of a new transaction).
                (*connp).out_data_other_at_tx_end =
                    1 as libc::c_int as libc::c_uint
            }
        }
    }
    let mut cl: *mut htp_header_t =
        htp_table_get_c((*(*connp).out_tx).response_headers,
                        b"content-length\x00" as *const u8 as
                            *const libc::c_char) as *mut htp_header_t;
    let mut te: *mut htp_header_t =
        htp_table_get_c((*(*connp).out_tx).response_headers,
                        b"transfer-encoding\x00" as *const u8 as
                            *const libc::c_char) as *mut htp_header_t;
    // Check for "101 Switching Protocol" response.
    // If it's seen, it means that traffic after empty line following headers
    // is no longer HTTP. We can treat it similarly to CONNECT.
    // Unlike CONNECT, however, upgrades from HTTP to HTTP seem
    // rather unlikely, so don't try to probe tunnel for nested HTTP,
    // and switch to tunnel mode right away.
    if (*(*connp).out_tx).response_status_number == 101 as libc::c_int {
        if te.is_null() && cl.is_null() {
            (*connp).out_state =
                Some(htp_connp_RES_FINALIZE as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*connp).in_status = HTP_STREAM_TUNNEL;
            (*connp).out_status = HTP_STREAM_TUNNEL;
            // we may have response headers
            let mut rc_0: htp_status_t =
                htp_tx_state_response_headers((*connp).out_tx);
            return rc_0
        } else {
            htp_log(connp,
                    b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                    581 as libc::c_int, HTP_LOG_WARNING, 0 as libc::c_int,
                    b"Switching Protocol with Content-Length\x00" as *const u8
                        as *const libc::c_char);
        }
    }
    // Check for an interim "100 Continue" response. Ignore it if found, and revert back to RES_LINE.
    if (*(*connp).out_tx).response_status_number == 100 as libc::c_int &&
           te.is_null() && cl.is_null() {
        if (*(*connp).out_tx).seen_100continue != 0 as libc::c_int {
            htp_log(connp,
                    b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                    588 as libc::c_int, HTP_LOG_ERROR, 0 as libc::c_int,
                    b"Already seen 100-Continue.\x00" as *const u8 as
                        *const libc::c_char);
            return -(1 as libc::c_int)
        }
        // Ignore any response headers seen so far.
        let mut h: *mut htp_header_t = 0 as *mut htp_header_t;
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t =
            htp_table_size((*(*connp).out_tx).response_headers);
        while i < n {
            h =
                htp_table_get_index((*(*connp).out_tx).response_headers, i,
                                    0 as *mut *mut bstr) as *mut htp_header_t;
            bstr_free((*h).name);
            bstr_free((*h).value);
            free(h as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        htp_table_clear((*(*connp).out_tx).response_headers);
        // Expecting to see another response line next.
        (*connp).out_state =
            Some(htp_connp_RES_LINE as
                     unsafe extern "C" fn(_: *mut htp_connp_t)
                         -> htp_status_t);
        (*(*connp).out_tx).response_progress = HTP_RESPONSE_LINE;
        (*(*connp).out_tx).seen_100continue += 1;
        return 1 as libc::c_int
    }
    // 1. Any response message which MUST NOT include a message-body
    //  (such as the 1xx, 204, and 304 responses and any response to a HEAD
    //  request) is always terminated by the first empty line after the
    //  header fields, regardless of the entity-header fields present in the
    //  message.
    if (*(*connp).out_tx).request_method_number as libc::c_uint ==
           HTP_M_HEAD as libc::c_int as libc::c_uint {
        // There's no response body whatsoever
        (*(*connp).out_tx).response_transfer_coding = HTP_CODING_NO_BODY;
        (*connp).out_state =
            Some(htp_connp_RES_FINALIZE as
                     unsafe extern "C" fn(_: *mut htp_connp_t)
                         -> htp_status_t)
    } else if (*(*connp).out_tx).response_status_number >= 100 as libc::c_int
                  &&
                  (*(*connp).out_tx).response_status_number <=
                      199 as libc::c_int ||
                  (*(*connp).out_tx).response_status_number ==
                      204 as libc::c_int ||
                  (*(*connp).out_tx).response_status_number ==
                      304 as libc::c_int {
        // There should be no response body
        // but browsers interpret content sent by the server as such
        if te.is_null() && cl.is_null() {
            (*(*connp).out_tx).response_transfer_coding = HTP_CODING_NO_BODY;
            (*connp).out_state =
                Some(htp_connp_RES_FINALIZE as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t)
        } else {
            htp_log(connp,
                    b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                    629 as libc::c_int, HTP_LOG_WARNING, 0 as libc::c_int,
                    b"Unexpected Response body\x00" as *const u8 as
                        *const libc::c_char);
        }
    }
    // Hack condition to check that we do not assume "no body"
    if (*connp).out_state !=
           Some(htp_connp_RES_FINALIZE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        // We have a response body
        let mut ct: *mut htp_header_t =
            htp_table_get_c((*(*connp).out_tx).response_headers,
                            b"content-type\x00" as *const u8 as
                                *const libc::c_char) as *mut htp_header_t;
        if !ct.is_null() {
            (*(*connp).out_tx).response_content_type =
                bstr_dup_lower((*ct).value);
            if (*(*connp).out_tx).response_content_type.is_null() {
                return -(1 as libc::c_int)
            }
            // Ignore parameters
            let mut data: *mut libc::c_uchar =
                if (*(*(*connp).out_tx).response_content_type).realptr.is_null()
                   {
                    ((*(*connp).out_tx).response_content_type as
                         *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>()
                                                        as libc::c_ulong as
                                                        isize)
                } else {
                    (*(*(*connp).out_tx).response_content_type).realptr
                };
            let mut len: size_t = (*(*ct).value).len;
            let mut newlen: size_t = 0 as libc::c_int as size_t;
            while newlen < len {
                // TODO Some platforms may do things differently here.
                if htp_is_space(*data.offset(newlen as isize) as libc::c_int)
                       != 0 ||
                       *data.offset(newlen as isize) as libc::c_int ==
                           ';' as i32 {
                    bstr_adjust_len((*(*connp).out_tx).response_content_type,
                                    newlen);
                    break ;
                } else { newlen = newlen.wrapping_add(1) }
            }
        }
        // 2. If a Transfer-Encoding header field (section 14.40) is present and
        //   indicates that the "chunked" transfer coding has been applied, then
        //   the length is defined by the chunked encoding (section 3.6).
        if !te.is_null() &&
               bstr_index_of_c_nocasenorzero((*te).value,
                                             b"chunked\x00" as *const u8 as
                                                 *const libc::c_char) !=
                   -(1 as libc::c_int) {
            if bstr_cmp_c_nocase((*te).value,
                                 b"chunked\x00" as *const u8 as
                                     *const libc::c_char) != 0 as libc::c_int
               {
                htp_log(connp,
                        b"htp_response.c\x00" as *const u8 as
                            *const libc::c_char, 660 as libc::c_int,
                        HTP_LOG_WARNING, 0 as libc::c_int,
                        b"Transfer-encoding has abnormal chunked value\x00" as
                            *const u8 as
                            *const libc::c_char); // 3. If a Content-Length header field (section 14.14) is present, its
            }
            // spec says chunked is HTTP/1.1 only, but some browsers accept it
            // with 1.0 as well
            if (*(*connp).out_tx).response_protocol_number <
                   101 as libc::c_int {
                htp_log(connp,
                        b"htp_response.c\x00" as *const u8 as
                            *const libc::c_char, 667 as libc::c_int,
                        HTP_LOG_WARNING, 0 as libc::c_int,
                        b"Chunked transfer-encoding on HTTP/0.9 or HTTP/1.0\x00"
                            as *const u8 as *const libc::c_char);
            }
            // If the T-E header is present we are going to use it.
            (*(*connp).out_tx).response_transfer_coding = HTP_CODING_CHUNKED;
            // We are still going to check for the presence of C-L
            if !cl.is_null() {
                // This is a violation of the RFC
                (*(*connp).out_tx).flags =
                    ((*(*connp).out_tx).flags as libc::c_ulonglong |
                         0x100 as libc::c_ulonglong) as uint64_t
            }
            (*connp).out_state =
                Some(htp_connp_RES_BODY_CHUNKED_LENGTH as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*(*connp).out_tx).response_progress = HTP_RESPONSE_BODY
        } else if !cl.is_null() {
            //   value in bytes represents the length of the message-body.
            // We know the exact length
            (*(*connp).out_tx).response_transfer_coding = HTP_CODING_IDENTITY;
            // Check for multiple C-L headers
            if (*cl).flags as libc::c_ulonglong & 0x20 as libc::c_ulonglong !=
                   0 {
                (*(*connp).out_tx).flags =
                    ((*(*connp).out_tx).flags as libc::c_ulonglong |
                         0x100 as libc::c_ulonglong) as uint64_t
            }
            // Get body length
            (*(*connp).out_tx).response_content_length =
                htp_parse_content_length((*cl).value, connp);
            if (*(*connp).out_tx).response_content_length <
                   0 as libc::c_int as libc::c_long {
                htp_log(connp,
                        b"htp_response.c\x00" as *const u8 as
                            *const libc::c_char, 696 as libc::c_int,
                        HTP_LOG_ERROR, 0 as libc::c_int,
                        b"Invalid C-L field in response: %ld\x00" as *const u8
                            as *const libc::c_char,
                        (*(*connp).out_tx).response_content_length);
                return -(1 as libc::c_int)
            } else {
                (*connp).out_content_length =
                    (*(*connp).out_tx).response_content_length;
                (*connp).out_body_data_left = (*connp).out_content_length;
                if (*connp).out_content_length !=
                       0 as libc::c_int as libc::c_long {
                    (*connp).out_state =
                        Some(htp_connp_RES_BODY_IDENTITY_CL_KNOWN as
                                 unsafe extern "C" fn(_: *mut htp_connp_t)
                                     -> htp_status_t);
                    (*(*connp).out_tx).response_progress = HTP_RESPONSE_BODY
                } else {
                    (*connp).out_state =
                        Some(htp_connp_RES_FINALIZE as
                                 unsafe extern "C" fn(_: *mut htp_connp_t)
                                     -> htp_status_t)
                }
            }
        } else {
            // 4. If the message uses the media type "multipart/byteranges", which is
            //   self-delimiting, then that defines the length. This media type MUST
            //   NOT be used unless the sender knows that the recipient can parse it;
            //   the presence in a request of a Range header with multiple byte-range
            //   specifiers implies that the client can parse multipart/byteranges
            //   responses.
            if !ct.is_null() {
                // TODO Handle multipart/byteranges
                if bstr_index_of_c_nocase((*ct).value,
                                          b"multipart/byteranges\x00" as
                                              *const u8 as
                                              *const libc::c_char) !=
                       -(1 as libc::c_int) {
                    htp_log(connp,
                            b"htp_response.c\x00" as *const u8 as
                                *const libc::c_char, 720 as libc::c_int,
                            HTP_LOG_ERROR, 0 as libc::c_int,
                            b"C-T multipart/byteranges in responses not supported\x00"
                                as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
            }
            // 5. By the server closing the connection. (Closing the connection
            //   cannot be used to indicate the end of a request body, since that
            //   would leave no possibility for the server to send back a response.)
            (*connp).out_state =
                Some(htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*(*connp).out_tx).response_transfer_coding = HTP_CODING_IDENTITY;
            (*(*connp).out_tx).response_progress = HTP_RESPONSE_BODY;
            (*connp).out_body_data_left = -(1 as libc::c_int) as int64_t
        }
    }
    // NOTE We do not need to check for short-style HTTP/0.9 requests here because
    //      that is done earlier, before response line parsing begins
    let mut rc_1: htp_status_t =
        htp_tx_state_response_headers((*connp).out_tx);
    if rc_1 != 1 as libc::c_int { return rc_1 }
    return 1 as libc::c_int;
}
/* *
 * Parses response headers.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_RES_HEADERS(mut connp: *mut htp_connp_t)
 -> htp_status_t {
    let mut endwithcr: libc::c_int = 0;
    let mut lfcrending: libc::c_int = 0 as libc::c_int;
    loop  {
        if (*connp).out_status as libc::c_uint ==
               HTP_STREAM_CLOSED as libc::c_int as libc::c_uint {
            // Finalize sending raw trailer data.
            let mut rc: htp_status_t =
                htp_connp_res_receiver_finalize_clear(connp);
            if rc != 1 as libc::c_int { return rc }
            // Run hook response_TRAILER.
            rc =
                htp_hook_run_all((*(*connp).cfg).hook_response_trailer,
                                 (*connp).out_tx as *mut libc::c_void);
            if rc != 1 as libc::c_int { return rc }
            (*connp).out_state =
                Some(htp_connp_RES_FINALIZE as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            return 1 as libc::c_int
        }
        if (*connp).out_current_read_offset < (*connp).out_current_len {
            (*connp).out_next_byte =
                *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                      as isize) as
                    libc::c_int;
            (*connp).out_current_read_offset += 1;
            (*connp).out_stream_offset += 1
        } else { return 5 as libc::c_int }
        // Have we reached the end of the line?
        if (*connp).out_next_byte != '\n' as i32 &&
               (*connp).out_next_byte != '\r' as i32 {
            lfcrending = 0 as libc::c_int
        } else {
            endwithcr = 0 as libc::c_int;
            if (*connp).out_next_byte == '\r' as i32 {
                if (*connp).out_current_read_offset >=
                       (*connp).out_current_len {
                    (*connp).out_next_byte = -(1 as libc::c_int)
                } else {
                    (*connp).out_next_byte =
                        *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                              as isize) as
                            libc::c_int
                }
                if (*connp).out_next_byte == -(1 as libc::c_int) {
                    return 5 as libc::c_int
                } else {
                    if (*connp).out_next_byte == '\n' as i32 {
                        if (*connp).out_current_read_offset <
                               (*connp).out_current_len {
                            (*connp).out_next_byte =
                                *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                                      as
                                                                      isize)
                                    as libc::c_int;
                            (*connp).out_current_read_offset += 1;
                            (*connp).out_stream_offset += 1
                        } else { return 5 as libc::c_int }
                        if lfcrending != 0 {
                            // Handling LFCRCRLFCRLF
                        // These 6 characters mean only 2 end of lines
                            if (*connp).out_current_read_offset >=
                                   (*connp).out_current_len {
                                (*connp).out_next_byte = -(1 as libc::c_int)
                            } else {
                                (*connp).out_next_byte =
                                    *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                                          as
                                                                          isize)
                                        as libc::c_int
                            }
                            if (*connp).out_next_byte == '\r' as i32 {
                                if (*connp).out_current_read_offset <
                                       (*connp).out_current_len {
                                    (*connp).out_next_byte =
                                        *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                                              as
                                                                              isize)
                                            as libc::c_int;
                                    (*connp).out_current_read_offset += 1;
                                    (*connp).out_stream_offset += 1
                                } else { return 5 as libc::c_int }
                                (*connp).out_current_consume_offset += 1;
                                if (*connp).out_current_read_offset >=
                                       (*connp).out_current_len {
                                    (*connp).out_next_byte =
                                        -(1 as libc::c_int)
                                } else {
                                    (*connp).out_next_byte =
                                        *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                                              as
                                                                              isize)
                                            as libc::c_int
                                }
                                if (*connp).out_next_byte == '\n' as i32 {
                                    if (*connp).out_current_read_offset <
                                           (*connp).out_current_len {
                                        (*connp).out_next_byte =
                                            *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                                                  as
                                                                                  isize)
                                                as libc::c_int;
                                        (*connp).out_current_read_offset += 1;
                                        (*connp).out_stream_offset += 1
                                    } else { return 5 as libc::c_int }
                                    (*connp).out_current_consume_offset += 1;
                                    htp_log(connp,
                                            b"htp_response.c\x00" as *const u8
                                                as *const libc::c_char,
                                            792 as libc::c_int,
                                            HTP_LOG_WARNING, 0 as libc::c_int,
                                            b"Weird response end of lines mix\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                        }
                    } else if (*connp).out_next_byte == '\r' as i32 {
                        continue ;
                    }
                    lfcrending = 0 as libc::c_int;
                    endwithcr = 1 as libc::c_int
                }
            } else {
                // connp->out_next_byte == LF
                if (*connp).out_current_read_offset >=
                       (*connp).out_current_len {
                    (*connp).out_next_byte = -(1 as libc::c_int)
                } else {
                    (*connp).out_next_byte =
                        *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                              as isize) as
                            libc::c_int
                }
                lfcrending = 0 as libc::c_int;
                if (*connp).out_next_byte == '\r' as i32 {
                    // hanldes LF-CR sequence as end of line
                    if (*connp).out_current_read_offset <
                           (*connp).out_current_len {
                        (*connp).out_next_byte =
                            *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                                  as isize) as
                                libc::c_int;
                        (*connp).out_current_read_offset += 1;
                        (*connp).out_stream_offset += 1
                    } else { return 5 as libc::c_int }
                    lfcrending = 1 as libc::c_int
                }
            }
            let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut len: size_t = 0;
            if htp_connp_res_consolidate_data(connp, &mut data, &mut len) !=
                   1 as libc::c_int {
                return -(1 as libc::c_int)
            }
            // CRCRLF is not an empty line
            if endwithcr != 0 && len < 2 as libc::c_int as libc::c_ulong {
                continue ;
            }
            // Should we terminate headers?
            if htp_connp_is_line_terminator(connp, data, len) != 0 {
                // Parse previous header, if any.
                if !(*connp).out_header.is_null() {
                    if (*(*connp).cfg).process_response_header.expect("non-null function pointer")(connp,
                                                                                                   (if (*(*connp).out_header).realptr.is_null()
                                                                                                       {
                                                                                                        ((*connp).out_header
                                                                                                             as
                                                                                                             *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>()
                                                                                                                                            as
                                                                                                                                            libc::c_ulong
                                                                                                                                            as
                                                                                                                                            isize)
                                                                                                    } else {
                                                                                                        (*(*connp).out_header).realptr
                                                                                                    }),
                                                                                                   (*(*connp).out_header).len)
                           != 1 as libc::c_int {
                        return -(1 as libc::c_int)
                    }
                    bstr_free((*connp).out_header);
                    (*connp).out_header = 0 as *mut bstr
                }
                htp_connp_res_clear_buffer(connp);
                // We've seen all response headers.
                if (*(*connp).out_tx).response_progress as libc::c_uint ==
                       HTP_RESPONSE_HEADERS as libc::c_int as libc::c_uint {
                    // Response headers.
                    // The next step is to determine if this response has a body.
                    (*connp).out_state =
                        Some(htp_connp_RES_BODY_DETERMINE as
                                 unsafe extern "C" fn(_: *mut htp_connp_t)
                                     -> htp_status_t)
                } else {
                    // Response trailer.
                    // Finalize sending raw trailer data.
                    let mut rc_0: htp_status_t =
                        htp_connp_res_receiver_finalize_clear(connp);
                    if rc_0 != 1 as libc::c_int { return rc_0 }
                    // Run hook response_TRAILER.
                    rc_0 =
                        htp_hook_run_all((*(*connp).cfg).hook_response_trailer,
                                         (*connp).out_tx as
                                             *mut libc::c_void);
                    if rc_0 != 1 as libc::c_int { return rc_0 }
                    // The next step is to finalize this response.
                    (*connp).out_state =
                        Some(htp_connp_RES_FINALIZE as
                                 unsafe extern "C" fn(_: *mut htp_connp_t)
                                     -> htp_status_t)
                }
                return 1 as libc::c_int
            }
            htp_chomp(data, &mut len);
            // Check for header folding.
            if htp_connp_is_line_folded(data, len) == 0 as libc::c_int {
                // New header line.
                // Parse previous header, if any.
                if !(*connp).out_header.is_null() {
                    if (*(*connp).cfg).process_response_header.expect("non-null function pointer")(connp,
                                                                                                   (if (*(*connp).out_header).realptr.is_null()
                                                                                                       {
                                                                                                        ((*connp).out_header
                                                                                                             as
                                                                                                             *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>()
                                                                                                                                            as
                                                                                                                                            libc::c_ulong
                                                                                                                                            as
                                                                                                                                            isize)
                                                                                                    } else {
                                                                                                        (*(*connp).out_header).realptr
                                                                                                    }),
                                                                                                   (*(*connp).out_header).len)
                           != 1 as libc::c_int {
                        return -(1 as libc::c_int)
                    }
                    bstr_free((*connp).out_header);
                    (*connp).out_header = 0 as *mut bstr
                }
                if (*connp).out_current_read_offset >=
                       (*connp).out_current_len {
                    (*connp).out_next_byte = -(1 as libc::c_int)
                } else {
                    (*connp).out_next_byte =
                        *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                              as isize) as
                            libc::c_int
                }
                if htp_is_folding_char((*connp).out_next_byte) ==
                       0 as libc::c_int {
                    // Because we know this header is not folded, we can process the buffer straight away.
                    if (*(*connp).cfg).process_response_header.expect("non-null function pointer")(connp,
                                                                                                   data,
                                                                                                   len)
                           != 1 as libc::c_int {
                        return -(1 as libc::c_int)
                    }
                } else {
                    // Keep the partial header data for parsing later.
                    (*connp).out_header =
                        bstr_dup_mem(data as *const libc::c_void, len);
                    if (*connp).out_header.is_null() {
                        return -(1 as libc::c_int)
                    }
                }
            } else if (*connp).out_header.is_null() {
                // Folding; check that there's a previous header line to add to.
                // Invalid folding.
                // Warn only once per transaction.
                if (*(*connp).out_tx).flags as libc::c_ulonglong &
                       0x200 as libc::c_ulonglong == 0 {
                    (*(*connp).out_tx).flags =
                        ((*(*connp).out_tx).flags as libc::c_ulonglong |
                             0x200 as libc::c_ulonglong) as uint64_t;
                    htp_log(connp,
                            b"htp_response.c\x00" as *const u8 as
                                *const libc::c_char, 899 as libc::c_int,
                            HTP_LOG_WARNING, 0 as libc::c_int,
                            b"Invalid response field folding\x00" as *const u8
                                as *const libc::c_char);
                }
                // Keep the header data for parsing later.
                (*connp).out_header =
                    bstr_dup_mem(data as *const libc::c_void, len);
                if (*connp).out_header.is_null() {
                    return -(1 as libc::c_int)
                }
            } else {
                let mut colon_pos: size_t = 0 as libc::c_int as size_t;
                while colon_pos < len &&
                          *data.offset(colon_pos as isize) as libc::c_int !=
                              ':' as i32 {
                    colon_pos = colon_pos.wrapping_add(1)
                }
                if colon_pos < len &&
                       bstr_chr((*connp).out_header, ':' as i32) >=
                           0 as libc::c_int &&
                       (*(*connp).out_tx).response_protocol_number ==
                           101 as libc::c_int {
                    // Warn only once per transaction.
                    if (*(*connp).out_tx).flags as libc::c_ulonglong &
                           0x200 as libc::c_ulonglong == 0 {
                        (*(*connp).out_tx).flags =
                            ((*(*connp).out_tx).flags as libc::c_ulonglong |
                                 0x200 as libc::c_ulonglong) as uint64_t;
                        htp_log(connp,
                                b"htp_response.c\x00" as *const u8 as
                                    *const libc::c_char, 915 as libc::c_int,
                                HTP_LOG_WARNING, 0 as libc::c_int,
                                b"Invalid response field folding\x00" as
                                    *const u8 as *const libc::c_char);
                    }
                    if (*(*connp).cfg).process_response_header.expect("non-null function pointer")(connp,
                                                                                                   (if (*(*connp).out_header).realptr.is_null()
                                                                                                       {
                                                                                                        ((*connp).out_header
                                                                                                             as
                                                                                                             *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>()
                                                                                                                                            as
                                                                                                                                            libc::c_ulong
                                                                                                                                            as
                                                                                                                                            isize)
                                                                                                    } else {
                                                                                                        (*(*connp).out_header).realptr
                                                                                                    }),
                                                                                                   (*(*connp).out_header).len)
                           != 1 as libc::c_int {
                        return -(1 as libc::c_int)
                    }
                    bstr_free((*connp).out_header);
                    (*connp).out_header =
                        bstr_dup_mem(data.offset(1 as libc::c_int as isize) as
                                         *const libc::c_void,
                                     len.wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong));
                    if (*connp).out_header.is_null() {
                        return -(1 as libc::c_int)
                    }
                } else {
                    // Add to the existing header.
                    let mut new_out_header: *mut bstr =
                        bstr_add_mem((*connp).out_header,
                                     data as *const libc::c_void, len);
                    if new_out_header.is_null() { return -(1 as libc::c_int) }
                    (*connp).out_header = new_out_header
                }
            }
            htp_connp_res_clear_buffer(connp);
        }
    };
}
/* *
 * Parses response line.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_RES_LINE(mut connp: *mut htp_connp_t)
 -> htp_status_t {
    loop  {
        // Don't try to get more data if the stream is closed. If we do, we'll return, asking for more data.
        if (*connp).out_status as libc::c_uint !=
               HTP_STREAM_CLOSED as libc::c_int as libc::c_uint {
            // Get one byte
            if (*connp).out_current_read_offset < (*connp).out_current_len {
                (*connp).out_next_byte =
                    *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                          as isize) as
                        libc::c_int;
                (*connp).out_current_read_offset += 1;
                (*connp).out_stream_offset += 1
            } else { return 5 as libc::c_int }
        }
        // Have we reached the end of the line? We treat stream closure as end of line in
        // order to handle the case when the first line of the response is actually response body
        // (and we wish it processed as such).
        if (*connp).out_next_byte == '\r' as i32 {
            if (*connp).out_current_read_offset >= (*connp).out_current_len {
                (*connp).out_next_byte = -(1 as libc::c_int)
            } else {
                (*connp).out_next_byte =
                    *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                          as isize) as
                        libc::c_int
            }
            if (*connp).out_next_byte == -(1 as libc::c_int) {
                return 5 as libc::c_int
            } else {
                if (*connp).out_next_byte == '\n' as i32 { continue ; }
                (*connp).out_next_byte = '\n' as i32
            }
        }
        if (*connp).out_next_byte == '\n' as i32 ||
               (*connp).out_status as libc::c_uint ==
                   HTP_STREAM_CLOSED as libc::c_int as libc::c_uint {
            let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut len: size_t = 0;
            if htp_connp_res_consolidate_data(connp, &mut data, &mut len) !=
                   1 as libc::c_int {
                return -(1 as libc::c_int)
            }
            // Is this a line that should be ignored?
            if htp_connp_is_line_ignorable(connp, data, len) != 0 {
                if (*connp).out_status as libc::c_uint ==
                       HTP_STREAM_CLOSED as libc::c_int as libc::c_uint {
                    (*connp).out_state =
                        Some(htp_connp_RES_FINALIZE as
                                 unsafe extern "C" fn(_: *mut htp_connp_t)
                                     -> htp_status_t)
                }
                // We have an empty/whitespace line, which we'll note, ignore and move on
                (*(*connp).out_tx).response_ignored_lines =
                    (*(*connp).out_tx).response_ignored_lines.wrapping_add(1);
                // TODO How many lines are we willing to accept?
                // Start again
                htp_connp_res_clear_buffer(connp);
                return 1 as libc::c_int
            }
            // Deallocate previous response line allocations, which we would have on a 100 response.
            if !(*(*connp).out_tx).response_line.is_null() {
                bstr_free((*(*connp).out_tx).response_line);
                (*(*connp).out_tx).response_line = 0 as *mut bstr
            }
            if !(*(*connp).out_tx).response_protocol.is_null() {
                bstr_free((*(*connp).out_tx).response_protocol);
                (*(*connp).out_tx).response_protocol = 0 as *mut bstr
            }
            if !(*(*connp).out_tx).response_status.is_null() {
                bstr_free((*(*connp).out_tx).response_status);
                (*(*connp).out_tx).response_status = 0 as *mut bstr
            }
            if !(*(*connp).out_tx).response_message.is_null() {
                bstr_free((*(*connp).out_tx).response_message);
                (*(*connp).out_tx).response_message = 0 as *mut bstr
            }
            // Process response line.           
            let mut chomp_result: libc::c_int = htp_chomp(data, &mut len);
            // If the response line is invalid, determine if it _looks_ like
            // a response line. If it does not look like a line, process the
            // data as a response body because that is what browsers do.
            if htp_treat_response_line_as_body(data, len) != 0 {
                (*(*connp).out_tx).response_content_encoding_processing =
                    HTP_COMPRESSION_NONE;
                (*connp).out_current_consume_offset =
                    (*connp).out_current_read_offset;
                let mut rc: htp_status_t =
                    htp_tx_res_process_body_data_ex((*connp).out_tx,
                                                    data as
                                                        *const libc::c_void,
                                                    len.wrapping_add(chomp_result
                                                                         as
                                                                         libc::c_ulong));
                if rc != 1 as libc::c_int { return rc }
                // Continue to process response body. Because we don't have
                // any headers to parse, we assume the body continues until
                // the end of the stream.
                // Have we seen the entire response body?
                if (*connp).out_current_len <=
                       (*connp).out_current_read_offset {
                    (*(*connp).out_tx).response_transfer_coding =
                        HTP_CODING_IDENTITY;
                    (*(*connp).out_tx).response_progress = HTP_RESPONSE_BODY;
                    (*connp).out_body_data_left =
                        -(1 as libc::c_int) as int64_t;
                    (*connp).out_state =
                        Some(htp_connp_RES_FINALIZE as
                                 unsafe extern "C" fn(_: *mut htp_connp_t)
                                     -> htp_status_t)
                }
                return 1 as libc::c_int
            }
            (*(*connp).out_tx).response_line =
                bstr_dup_mem(data as *const libc::c_void, len);
            if (*(*connp).out_tx).response_line.is_null() {
                return -(1 as libc::c_int)
            }
            if (*(*connp).cfg).parse_response_line.expect("non-null function pointer")(connp)
                   != 1 as libc::c_int {
                return -(1 as libc::c_int)
            }
            let mut rc_0: htp_status_t =
                htp_tx_state_response_line((*connp).out_tx);
            if rc_0 != 1 as libc::c_int { return rc_0 }
            htp_connp_res_clear_buffer(connp);
            // Move on to the next phase.
            (*connp).out_state =
                Some(htp_connp_RES_HEADERS as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*(*connp).out_tx).response_progress = HTP_RESPONSE_HEADERS;
            return 1 as libc::c_int
        }
    };
}
/* *
 * Returns the number of bytes consumed from the most recent outbound data chunk. Normally, an invocation
 * of htp_connp_res_data() will consume all data from the supplied buffer, but there are circumstances
 * where only partial consumption is possible. In such cases HTP_STREAM_DATA_OTHER will be returned.
 * Consumed bytes are no longer necessary, but the remainder of the buffer will be need to be saved
 * for later.
 *
 * @param[in] connp
 * @return The number of bytes consumed from the last data chunk sent for outbound processing.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_res_data_consumed(mut connp:
                                                         *mut htp_connp_t)
 -> size_t {
    return (*connp).out_current_read_offset as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn htp_connp_RES_FINALIZE(mut connp: *mut htp_connp_t)
 -> htp_status_t {
    if (*connp).out_status as libc::c_uint !=
           HTP_STREAM_CLOSED as libc::c_int as libc::c_uint {
        if (*connp).out_current_read_offset >= (*connp).out_current_len {
            (*connp).out_next_byte = -(1 as libc::c_int)
        } else {
            (*connp).out_next_byte =
                *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                      as isize) as libc::c_int
        }
        if (*connp).out_next_byte == -(1 as libc::c_int) {
            return htp_tx_state_response_complete_ex((*connp).out_tx,
                                                     0 as libc::c_int)
        }
        if (*connp).out_next_byte != '\n' as i32 ||
               (*connp).out_current_consume_offset >=
                   (*connp).out_current_read_offset {
            loop  {
                //;i < max_read; i++) {
                if (*connp).out_current_read_offset < (*connp).out_current_len
                   {
                    (*connp).out_next_byte =
                        *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                              as isize) as
                            libc::c_int;
                    (*connp).out_current_read_offset += 1;
                    (*connp).out_stream_offset += 1
                } else { return 5 as libc::c_int }
                // Have we reached the end of the line? For some reason
                // we can't test after IN_COPY_BYTE_OR_RETURN */
                if (*connp).out_next_byte == '\n' as i32 { break ; }
            }
        }
    }
    let mut bytes_left: size_t = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if htp_connp_res_consolidate_data(connp, &mut data, &mut bytes_left) !=
           1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if bytes_left == 0 as libc::c_int as libc::c_ulong {
        //closing
        return htp_tx_state_response_complete_ex((*connp).out_tx,
                                                 0 as libc::c_int)
    }
    if htp_treat_response_line_as_body(data, bytes_left) != 0 {
        // Interpret remaining bytes as body data
        htp_log(connp,
                b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                1104 as libc::c_int, HTP_LOG_WARNING, 0 as libc::c_int,
                b"Unexpected response body\x00" as *const u8 as
                    *const libc::c_char);
        let mut rc: htp_status_t =
            htp_tx_res_process_body_data_ex((*connp).out_tx,
                                            data as *const libc::c_void,
                                            bytes_left);
        htp_connp_res_clear_buffer(connp);
        return rc
    }
    //unread last end of line so that RES_LINE works
    if (*connp).out_current_read_offset < bytes_left as int64_t {
        (*connp).out_current_read_offset = 0 as libc::c_int as int64_t
    } else {
        (*connp).out_current_read_offset =
            ((*connp).out_current_read_offset as
                 libc::c_ulong).wrapping_sub(bytes_left) as int64_t as int64_t
    }
    if (*connp).out_current_read_offset < (*connp).out_current_consume_offset
       {
        (*connp).out_current_consume_offset = (*connp).out_current_read_offset
    }
    return htp_tx_state_response_complete_ex((*connp).out_tx,
                                             0 as libc::c_int);
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
// 1048576 is 1 Mbyte
//deflate max ratio is about 1000
// Parser states, in the order in which they are
// used as a single transaction is processed.
/* *
 * The response idle state will initialize response processing, as well as
 * finalize each transactions after we are done with it.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_RES_IDLE(mut connp: *mut htp_connp_t)
 -> htp_status_t {
    // We want to start parsing the next response (and change
    // the state from IDLE) only if there's at least one
    // byte of data available. Otherwise we could be creating
    // new structures even if there's no more data on the
    // connection.
    if (*connp).out_current_read_offset >= (*connp).out_current_len {
        return 2 as libc::c_int
    }
    // Parsing a new response
    // Find the next outgoing transaction
    // If there is none, we just create one so that responses without
    // request can still be processed.
    (*connp).out_tx =
        htp_list_array_get((*(*connp).conn).transactions,
                           (*connp).out_next_tx_index) as *mut htp_tx_t;
    if (*connp).out_tx.is_null() {
        htp_log(connp,
                b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                1145 as libc::c_int, HTP_LOG_ERROR, 0 as libc::c_int,
                b"Unable to match response to request\x00" as *const u8 as
                    *const libc::c_char);
        // finalize dangling request waiting for next request or body
        if (*connp).in_state ==
               Some(htp_connp_REQ_FINALIZE as
                        unsafe extern "C" fn(_: *mut htp_connp_t)
                            -> htp_status_t) {
            htp_tx_state_request_complete((*connp).in_tx);
        }
        (*connp).out_tx = htp_connp_tx_create(connp);
        if (*connp).out_tx.is_null() { return -(1 as libc::c_int) }
        (*(*connp).out_tx).parsed_uri = htp_uri_alloc();
        if (*(*connp).out_tx).parsed_uri.is_null() {
            return -(1 as libc::c_int)
        }
        (*(*(*connp).out_tx).parsed_uri).path =
            bstr_dup_c(b"/libhtp::request_uri_not_seen\x00" as *const u8 as
                           *const libc::c_char);
        if (*(*(*connp).out_tx).parsed_uri).path.is_null() {
            return -(1 as libc::c_int)
        }
        (*(*connp).out_tx).request_uri =
            bstr_dup_c(b"/libhtp::request_uri_not_seen\x00" as *const u8 as
                           *const libc::c_char);
        if (*(*connp).out_tx).request_uri.is_null() {
            return -(1 as libc::c_int)
        }
        (*connp).in_state =
            Some(htp_connp_REQ_FINALIZE as
                     unsafe extern "C" fn(_: *mut htp_connp_t)
                         -> htp_status_t);
        // We've used one transaction
        (*connp).out_next_tx_index =
            (*connp).out_next_tx_index.wrapping_add(1)
    } else {
        // We've used one transaction
        (*connp).out_next_tx_index =
            (*connp).out_next_tx_index.wrapping_add(1);
        // TODO Detect state mismatch
        (*connp).out_content_length = -(1 as libc::c_int) as int64_t;
        (*connp).out_body_data_left = -(1 as libc::c_int) as int64_t
    }
    let mut rc: htp_status_t = htp_tx_state_response_start((*connp).out_tx);
    if rc != 1 as libc::c_int { return rc }
    return 1 as libc::c_int;
}
/* *
 * Process a chunk of outbound (server or response) data.
 *
 * @param[in] connp
 * @param[in] timestamp Optional.
 * @param[in] data
 * @param[in] len
 * @return HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_res_data(mut connp: *mut htp_connp_t,
                                            mut timestamp: *const htp_time_t,
                                            mut data: *const libc::c_void,
                                            mut len: size_t) -> libc::c_int {
    // Return if the connection is in stop state
    if (*connp).out_status as libc::c_uint ==
           HTP_STREAM_STOP as libc::c_int as libc::c_uint {
        htp_log(connp,
                b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                1197 as libc::c_int, HTP_LOG_INFO, 0 as libc::c_int,
                b"Outbound parser is in HTP_STREAM_STOP\x00" as *const u8 as
                    *const libc::c_char);
        return HTP_STREAM_STOP as libc::c_int
    }
    // Return if the connection has had a fatal error
    if (*connp).out_status as libc::c_uint ==
           HTP_STREAM_ERROR as libc::c_int as libc::c_uint {
        htp_log(connp,
                b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                1204 as libc::c_int, HTP_LOG_ERROR, 0 as libc::c_int,
                b"Outbound parser is in HTP_STREAM_ERROR\x00" as *const u8 as
                    *const libc::c_char);
        return HTP_STREAM_ERROR as libc::c_int
    }
    // Sanity check: we must have a transaction pointer if the state is not IDLE (no outbound transaction)
    if (*connp).out_tx.is_null() &&
           (*connp).out_state !=
               Some(htp_connp_RES_IDLE as
                        unsafe extern "C" fn(_: *mut htp_connp_t)
                            -> htp_status_t) {
        (*connp).out_status = HTP_STREAM_ERROR;
        htp_log(connp,
                b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                1217 as libc::c_int, HTP_LOG_ERROR, 0 as libc::c_int,
                b"Missing outbound transaction data\x00" as *const u8 as
                    *const libc::c_char);
        return HTP_STREAM_ERROR as libc::c_int
    }
    // If the length of the supplied data chunk is zero, proceed
    // only if the stream has been closed. We do not allow zero-sized
    // chunks in the API, but we use it internally to force the parsers
    // to finalize parsing.
    if (data == 0 as *mut libc::c_void ||
            len == 0 as libc::c_int as libc::c_ulong) &&
           (*connp).out_status as libc::c_uint !=
               HTP_STREAM_CLOSED as libc::c_int as libc::c_uint {
        htp_log(connp,
                b"htp_response.c\x00" as *const u8 as *const libc::c_char,
                1227 as libc::c_int, HTP_LOG_ERROR, 0 as libc::c_int,
                b"Zero-length data chunks are not allowed\x00" as *const u8 as
                    *const libc::c_char);
        return HTP_STREAM_CLOSED as libc::c_int
    }
    // Remember the timestamp of the current response data chunk
    if !timestamp.is_null() {
        memcpy(&mut (*connp).out_timestamp as *mut htp_time_t as
                   *mut libc::c_void, timestamp as *const libc::c_void,
               ::std::mem::size_of::<htp_time_t>() as libc::c_ulong);
    }
    // Store the current chunk information
    (*connp).out_current_data = data as *mut libc::c_uchar;
    (*connp).out_current_len = len as int64_t;
    (*connp).out_current_read_offset = 0 as libc::c_int as int64_t;
    (*connp).out_current_consume_offset = 0 as libc::c_int as int64_t;
    (*connp).out_current_receiver_offset = 0 as libc::c_int as int64_t;
    htp_conn_track_outbound_data((*connp).conn, len, timestamp);
    // Return without processing any data if the stream is in tunneling
    // mode (which it would be after an initial CONNECT transaction.
    if (*connp).out_status as libc::c_uint ==
           HTP_STREAM_TUNNEL as libc::c_int as libc::c_uint {
        return HTP_STREAM_TUNNEL as libc::c_int
    }
    loop 
         // Invoke a processor, in a loop, until an error
    // occurs or until we run out of data. Many processors
    // will process a request, each pointing to the next
    // processor that needs to run.
         // Return if there's been an error
        // or if we've run out of data. We are relying
        // on processors to add error messages, so we'll
        // keep quiet here.
         {
        let mut rc: htp_status_t =
            (*connp).out_state.expect("non-null function pointer")(connp);
        if rc == 1 as libc::c_int {
            if (*connp).out_status as libc::c_uint ==
                   HTP_STREAM_TUNNEL as libc::c_int as libc::c_uint {
                return HTP_STREAM_TUNNEL as libc::c_int
            }
            rc = htp_res_handle_state_change(connp)
        }
        if rc != 1 as libc::c_int {
            // Do we need more data?
            if rc == 2 as libc::c_int || rc == 5 as libc::c_int {
                htp_connp_res_receiver_send_data(connp, 0 as libc::c_int);
                if rc == 5 as libc::c_int {
                    if htp_connp_res_buffer(connp) != 1 as libc::c_int {
                        (*connp).out_status = HTP_STREAM_ERROR;
                        return HTP_STREAM_ERROR as libc::c_int
                    }
                }
                (*connp).out_status = HTP_STREAM_DATA;
                return HTP_STREAM_DATA as libc::c_int
            }
            // Check for stop
            if rc == 4 as libc::c_int {
                (*connp).out_status = HTP_STREAM_STOP;
                return HTP_STREAM_STOP as libc::c_int
            }
            // Check for suspended parsing
            if rc == 3 as libc::c_int {
                // We might have actually consumed the entire data chunk?
                if (*connp).out_current_read_offset >=
                       (*connp).out_current_len {
                    (*connp).out_status = HTP_STREAM_DATA;
                    // Do not send STREAM_DATE_DATA_OTHER if we've
                    // consumed the entire chunk
                    return HTP_STREAM_DATA as libc::c_int
                } else {
                    (*connp).out_status = HTP_STREAM_DATA_OTHER;
                    // Partial chunk consumption
                    return HTP_STREAM_DATA_OTHER as libc::c_int
                }
            }
            // Permanent stream error.
            (*connp).out_status = HTP_STREAM_ERROR;
            return HTP_STREAM_ERROR as libc::c_int
        }
    };
}
