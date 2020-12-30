/* Needed to suppress type issues in the exposed
 * decompression state. Will go away when decompression
 * is cleaned up.
 */
typedef struct internal_state internal_state;

/* Declare timeval as struct timeval, so it can be
 * re-exported as htp_time_t. Also will be cleaned up
 * when we sort the timeval business.
 */
typedef struct timeval timeval;
