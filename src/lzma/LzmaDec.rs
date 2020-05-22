use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: SizeT) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: SizeT) -> *mut libc::c_void;
}
pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type Byte = u8;
pub type SRes = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type SizeT = size_t;
pub type BoolInt = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISzAlloc {
    pub Alloc: Option<unsafe fn(_: ISzAllocPtr, _: SizeT) -> *mut libc::c_void>,
    pub Free: Option<unsafe fn(_: ISzAllocPtr, _: *mut libc::c_void) -> ()>,
}
pub type ISzAllocPtr = *const ISzAlloc;
pub type CLzmaProb = UInt16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CLzmaProps {
    pub lc: Byte,
    pub lp: Byte,
    pub pb: Byte,
    pub _pad_: Byte,
    pub dicSize: UInt32,
}
pub type CLzmaProps = _CLzmaProps;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CLzmaDec {
    pub prop: CLzmaProps,
    pub probs: *mut CLzmaProb,
    pub probs_1664: *mut CLzmaProb,
    pub dic: *mut Byte,
    pub dicBufSize: SizeT,
    pub dicPos: SizeT,
    pub buf: *const Byte,
    pub range: UInt32,
    pub code: UInt32,
    pub processedPos: UInt32,
    pub checkDicSize: UInt32,
    pub reps: [UInt32; 4],
    pub state: UInt32,
    pub remainLen: UInt32,
    pub numProbs: UInt32,
    pub tempBufSize: libc::c_uint,
    pub tempBuf: [Byte; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ELzmaFinishMode {
    LZMA_FINISH_ANY,
    LZMA_FINISH_END,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum ELzmaStatus {
    LZMA_STATUS_NOT_SPECIFIED,
    LZMA_STATUS_FINISHED_WITH_MARK,
    LZMA_STATUS_NOT_FINISHED,
    LZMA_STATUS_NEEDS_MORE_INPUT,
    LZMA_STATUS_MAYBE_FINISHED_WITHOUT_MARK,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum ELzmaDummy {
    DUMMY_ERROR,
    DUMMY_LIT,
    DUMMY_MATCH,
    DUMMY_REP,
}

/*
p->remainLen : shows status of LZMA decoder:
    < kMatchSpecLenStart : normal remain
    = kMatchSpecLenStart : finished
    = kMatchSpecLenStart + 1 : need init range coder
    = kMatchSpecLenStart + 2 : need init range coder and state
*/
/* ---------- LZMA_DECODE_REAL ---------- */
/*
LzmaDec_DecodeReal_3() can be implemented in external ASM file.
3 - is the code compatibility version of that function for check at link time.
*/
/*
LZMA_DECODE_REAL()
In:
  RangeCoder is normalized
  if (p->dicPos == limit)
  {
    LzmaDec_TryDummy() was called before to exclude LITERAL and MATCH-REP cases.
    So first symbol can be only MATCH-NON-REP. And if that MATCH-NON-REP symbol
    is not END_OF_PAYALOAD_MARKER, then function returns error code.
  }

Processing:
  first LZMA symbol will be decoded in any case
  All checks for limits are at the end of main loop,
  It will decode new LZMA-symbols while (p->buf < bufLimit && dicPos < limit),
  RangeCoder is still without last normalization when (p->buf < bufLimit) is being checked.

Out:
  RangeCoder is normalized
  Result:
    SZ_OK - OK
    SZ_ERROR_DATA - Error
  p->remainLen:
    < kMatchSpecLenStart : normal remain
    = kMatchSpecLenStart : finished
*/
unsafe fn LzmaDec_DecodeReal_3(
    mut p: *mut CLzmaDec,
    limit: SizeT,
    bufLimit: *const Byte,
) -> libc::c_int {
    let probs: *mut CLzmaProb = (*p).probs_1664;
    let mut state: libc::c_uint = (*p).state;
    let mut rep0: UInt32 = (*p).reps[0 as libc::c_int as usize];
    let mut rep1: UInt32 = (*p).reps[1 as libc::c_int as usize];
    let mut rep2: UInt32 = (*p).reps[2 as libc::c_int as usize];
    let mut rep3: UInt32 = (*p).reps[3 as libc::c_int as usize];
    let pbMask: libc::c_uint = ((1 as libc::c_int as libc::c_uint) << (*p).prop.pb as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let lc: libc::c_uint = (*p).prop.lc as libc::c_uint;
    let lpMask: libc::c_uint = ((0x100 as libc::c_int as libc::c_uint)
        << (*p).prop.lp as libc::c_int)
        .wrapping_sub(0x100 as libc::c_int as libc::c_uint >> lc);
    let dic: *mut Byte = (*p).dic;
    let dicBufSize: SizeT = (*p).dicBufSize;
    let mut dicPos: SizeT = (*p).dicPos;
    let mut processedPos: UInt32 = (*p).processedPos;
    let checkDicSize: UInt32 = (*p).checkDicSize;
    let mut len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut buf: *const Byte = (*p).buf;
    let mut range: UInt32 = (*p).range;
    let mut code: UInt32 = (*p).code;
    let mut current_block_1101: u64;
    loop {
        let mut prob: *mut CLzmaProb = 0 as *mut CLzmaProb;
        let mut bound: UInt32 = 0;
        let mut ttt: libc::c_uint = 0;
        let posState: libc::c_uint = (processedPos & pbMask) << 4 as libc::c_int;
        prob = probs
            .offset(
                (-(1664 as libc::c_int)
                    + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                    + ((16 as libc::c_int) << 4 as libc::c_int)
                    + (0 as libc::c_int
                        + 2 as libc::c_int
                            * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                        + ((1 as libc::c_int) << 8 as libc::c_int))
                    + (0 as libc::c_int
                        + 2 as libc::c_int
                            * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                        + ((1 as libc::c_int) << 8 as libc::c_int))) as isize,
            )
            .offset(posState.wrapping_add(state) as isize);
        ttt = *prob as libc::c_uint;
        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
            range <<= 8 as libc::c_int;
            let fresh0 = buf;
            buf = buf.offset(1);
            code = code << 8 as libc::c_int | *fresh0 as libc::c_uint
        }
        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
        if code < bound {
            let mut symbol: libc::c_uint = 0;
            range = bound;
            *prob = ttt.wrapping_add(
                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint).wrapping_sub(ttt)
                    >> 5 as libc::c_int,
            ) as CLzmaProb;
            prob = probs.offset(
                (-(1664 as libc::c_int)
                    + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                    + ((16 as libc::c_int) << 4 as libc::c_int)
                    + (0 as libc::c_int
                        + 2 as libc::c_int
                            * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                        + ((1 as libc::c_int) << 8 as libc::c_int))
                    + (0 as libc::c_int
                        + 2 as libc::c_int
                            * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                        + ((1 as libc::c_int) << 8 as libc::c_int))
                    + ((16 as libc::c_int) << 4 as libc::c_int)
                    + ((1 as libc::c_int) << 4 as libc::c_int)
                    + 12 as libc::c_int
                    + 12 as libc::c_int
                    + 12 as libc::c_int
                    + 12 as libc::c_int
                    + ((4 as libc::c_int) << 6 as libc::c_int)) as isize,
            );
            if processedPos != 0 as libc::c_int as libc::c_uint
                || checkDicSize != 0 as libc::c_int as libc::c_uint
            {
                prob = prob.offset((3 as libc::c_int as UInt32).wrapping_mul(
                    ((processedPos << 8 as libc::c_int).wrapping_add(*dic.offset(
                        (if dicPos == 0 { dicBufSize } else { dicPos }).wrapping_sub(1) as isize,
                    )
                        as libc::c_uint)
                        & lpMask)
                        << lc,
                ) as isize)
            }
            processedPos = processedPos.wrapping_add(1);
            if state < 7 as libc::c_int as libc::c_uint {
                state = state.wrapping_sub(if state < 4 as libc::c_int as libc::c_uint {
                    state
                } else {
                    3 as libc::c_int as libc::c_uint
                });
                symbol = 1 as libc::c_int as libc::c_uint;
                ttt = *prob.offset(symbol as isize) as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh1 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh1 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *prob.offset(symbol as isize) = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol)
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *prob.offset(symbol as isize) =
                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                ttt = *prob.offset(symbol as isize) as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh2 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh2 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *prob.offset(symbol as isize) = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol)
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *prob.offset(symbol as isize) =
                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                ttt = *prob.offset(symbol as isize) as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh3 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh3 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *prob.offset(symbol as isize) = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol)
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *prob.offset(symbol as isize) =
                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                ttt = *prob.offset(symbol as isize) as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh4 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh4 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *prob.offset(symbol as isize) = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol)
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *prob.offset(symbol as isize) =
                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                ttt = *prob.offset(symbol as isize) as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh5 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh5 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *prob.offset(symbol as isize) = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol)
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *prob.offset(symbol as isize) =
                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                ttt = *prob.offset(symbol as isize) as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh6 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh6 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *prob.offset(symbol as isize) = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol)
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *prob.offset(symbol as isize) =
                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                ttt = *prob.offset(symbol as isize) as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh7 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh7 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *prob.offset(symbol as isize) = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol)
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *prob.offset(symbol as isize) =
                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                ttt = *prob.offset(symbol as isize) as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh8 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh8 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *prob.offset(symbol as isize) = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol)
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *prob.offset(symbol as isize) =
                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
            } else {
                let mut matchByte: libc::c_uint =
                    *dic.offset(dicPos.wrapping_sub(rep0 as SizeT).wrapping_add(
                        if dicPos < rep0 as SizeT {
                            dicBufSize
                        } else {
                            0
                        },
                    ) as isize) as libc::c_uint;
                let mut offs: libc::c_uint = 0x100 as libc::c_int as libc::c_uint;
                state = state.wrapping_sub(if state < 10 as libc::c_int as libc::c_uint {
                    3 as libc::c_int
                } else {
                    6 as libc::c_int
                } as libc::c_uint);
                symbol = 1 as libc::c_int as libc::c_uint;
                let mut bit: libc::c_uint = 0;
                let mut probLit: *mut CLzmaProb = 0 as *mut CLzmaProb;
                matchByte = matchByte.wrapping_add(matchByte);
                bit = offs;
                offs &= matchByte;
                probLit = prob.offset(offs.wrapping_add(bit).wrapping_add(symbol) as isize);
                ttt = *probLit as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh9 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh9 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *probLit = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol);
                    offs ^= bit
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *probLit = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                matchByte = matchByte.wrapping_add(matchByte);
                bit = offs;
                offs &= matchByte;
                probLit = prob.offset(offs.wrapping_add(bit).wrapping_add(symbol) as isize);
                ttt = *probLit as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh10 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh10 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *probLit = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol);
                    offs ^= bit
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *probLit = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                matchByte = matchByte.wrapping_add(matchByte);
                bit = offs;
                offs &= matchByte;
                probLit = prob.offset(offs.wrapping_add(bit).wrapping_add(symbol) as isize);
                ttt = *probLit as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh11 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh11 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *probLit = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol);
                    offs ^= bit
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *probLit = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                matchByte = matchByte.wrapping_add(matchByte);
                bit = offs;
                offs &= matchByte;
                probLit = prob.offset(offs.wrapping_add(bit).wrapping_add(symbol) as isize);
                ttt = *probLit as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh12 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh12 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *probLit = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol);
                    offs ^= bit
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *probLit = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                matchByte = matchByte.wrapping_add(matchByte);
                bit = offs;
                offs &= matchByte;
                probLit = prob.offset(offs.wrapping_add(bit).wrapping_add(symbol) as isize);
                ttt = *probLit as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh13 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh13 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *probLit = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol);
                    offs ^= bit
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *probLit = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                matchByte = matchByte.wrapping_add(matchByte);
                bit = offs;
                offs &= matchByte;
                probLit = prob.offset(offs.wrapping_add(bit).wrapping_add(symbol) as isize);
                ttt = *probLit as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh14 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh14 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *probLit = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol);
                    offs ^= bit
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *probLit = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                matchByte = matchByte.wrapping_add(matchByte);
                bit = offs;
                offs &= matchByte;
                probLit = prob.offset(offs.wrapping_add(bit).wrapping_add(symbol) as isize);
                ttt = *probLit as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh15 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh15 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *probLit = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol);
                    offs ^= bit
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *probLit = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                matchByte = matchByte.wrapping_add(matchByte);
                bit = offs;
                offs &= matchByte;
                probLit = prob.offset(offs.wrapping_add(bit).wrapping_add(symbol) as isize);
                ttt = *probLit as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh16 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh16 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *probLit = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    symbol = symbol.wrapping_add(symbol);
                    offs ^= bit
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *probLit = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
            }
            let fresh17 = dicPos;
            dicPos = dicPos.wrapping_add(1);
            *dic.offset(fresh17 as isize) = symbol as Byte
        } else {
            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
            *prob = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
            prob = probs
                .offset(
                    (-(1664 as libc::c_int)
                        + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                        + ((16 as libc::c_int) << 4 as libc::c_int)
                        + (0 as libc::c_int
                            + 2 as libc::c_int
                                * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                            + ((1 as libc::c_int) << 8 as libc::c_int))
                        + (0 as libc::c_int
                            + 2 as libc::c_int
                                * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                            + ((1 as libc::c_int) << 8 as libc::c_int))
                        + ((16 as libc::c_int) << 4 as libc::c_int)
                        + ((1 as libc::c_int) << 4 as libc::c_int)) as isize,
                )
                .offset(state as isize);
            ttt = *prob as libc::c_uint;
            if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                range <<= 8 as libc::c_int;
                let fresh18 = buf;
                buf = buf.offset(1);
                code = code << 8 as libc::c_int | *fresh18 as libc::c_uint
            }
            bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
            if code < bound {
                range = bound;
                *prob = ttt.wrapping_add(
                    (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint).wrapping_sub(ttt)
                        >> 5 as libc::c_int,
                ) as CLzmaProb;
                state = state.wrapping_add(12 as libc::c_int as libc::c_uint);
                prob = probs.offset(
                    (-(1664 as libc::c_int)
                        + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                        + ((16 as libc::c_int) << 4 as libc::c_int)
                        + (0 as libc::c_int
                            + 2 as libc::c_int
                                * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                            + ((1 as libc::c_int) << 8 as libc::c_int)))
                        as isize,
                );
                current_block_1101 = 4800884466390615302;
            } else {
                range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                *prob = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                /*
                // that case was checked before with kBadRepCode
                if (checkDicSize == 0 && processedPos == 0)
                  return SZ_ERROR_DATA;
                */
                prob = probs
                    .offset(
                        (-(1664 as libc::c_int)
                            + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                            + ((16 as libc::c_int) << 4 as libc::c_int)
                            + (0 as libc::c_int
                                + 2 as libc::c_int
                                    * (((1 as libc::c_int) << 4 as libc::c_int)
                                        << 3 as libc::c_int)
                                + ((1 as libc::c_int) << 8 as libc::c_int))
                            + (0 as libc::c_int
                                + 2 as libc::c_int
                                    * (((1 as libc::c_int) << 4 as libc::c_int)
                                        << 3 as libc::c_int)
                                + ((1 as libc::c_int) << 8 as libc::c_int))
                            + ((16 as libc::c_int) << 4 as libc::c_int)
                            + ((1 as libc::c_int) << 4 as libc::c_int)
                            + 12 as libc::c_int) as isize,
                    )
                    .offset(state as isize);
                ttt = *prob as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    range <<= 8 as libc::c_int;
                    let fresh19 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh19 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    *prob = ttt.wrapping_add(
                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(ttt)
                            >> 5 as libc::c_int,
                    ) as CLzmaProb;
                    prob = probs
                        .offset(
                            (-(1664 as libc::c_int)
                                + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int)))
                                as isize,
                        )
                        .offset(posState.wrapping_add(state) as isize);
                    ttt = *prob as libc::c_uint;
                    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                        range <<= 8 as libc::c_int;
                        let fresh20 = buf;
                        buf = buf.offset(1);
                        code = code << 8 as libc::c_int | *fresh20 as libc::c_uint
                    }
                    bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                    if code < bound {
                        range = bound;
                        *prob = ttt.wrapping_add(
                            (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                .wrapping_sub(ttt)
                                >> 5 as libc::c_int,
                        ) as CLzmaProb;
                        *dic.offset(dicPos as isize) =
                            *dic.offset(dicPos.wrapping_sub(rep0 as SizeT).wrapping_add(
                                if dicPos < rep0 as SizeT {
                                    dicBufSize
                                } else {
                                    0
                                },
                            ) as isize);
                        dicPos = dicPos.wrapping_add(1);
                        processedPos = processedPos.wrapping_add(1);
                        state = if state < 7 as libc::c_int as libc::c_uint {
                            9 as libc::c_int
                        } else {
                            11 as libc::c_int
                        } as libc::c_uint;
                        current_block_1101 = 13183875560443969876;
                    } else {
                        range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                        code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                        *prob = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                        current_block_1101 = 7237866862515803946;
                    }
                } else {
                    let mut distance: UInt32 = 0;
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    *prob = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                    prob = probs
                        .offset(
                            (-(1664 as libc::c_int)
                                + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                                + ((16 as libc::c_int) << 4 as libc::c_int)
                                + (0 as libc::c_int
                                    + 2 as libc::c_int
                                        * (((1 as libc::c_int) << 4 as libc::c_int)
                                            << 3 as libc::c_int)
                                    + ((1 as libc::c_int) << 8 as libc::c_int))
                                + (0 as libc::c_int
                                    + 2 as libc::c_int
                                        * (((1 as libc::c_int) << 4 as libc::c_int)
                                            << 3 as libc::c_int)
                                    + ((1 as libc::c_int) << 8 as libc::c_int))
                                + ((16 as libc::c_int) << 4 as libc::c_int)
                                + ((1 as libc::c_int) << 4 as libc::c_int)
                                + 12 as libc::c_int
                                + 12 as libc::c_int) as isize,
                        )
                        .offset(state as isize);
                    ttt = *prob as libc::c_uint;
                    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                        range <<= 8 as libc::c_int;
                        let fresh21 = buf;
                        buf = buf.offset(1);
                        code = code << 8 as libc::c_int | *fresh21 as libc::c_uint
                    }
                    bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                    if code < bound {
                        range = bound;
                        *prob = ttt.wrapping_add(
                            (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                .wrapping_sub(ttt)
                                >> 5 as libc::c_int,
                        ) as CLzmaProb;
                        distance = rep1
                    } else {
                        range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                        code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                        *prob = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                        prob = probs
                            .offset(
                                (-(1664 as libc::c_int)
                                    + ((1 as libc::c_int)
                                        << (14 as libc::c_int >> 1 as libc::c_int))
                                    + ((16 as libc::c_int) << 4 as libc::c_int)
                                    + (0 as libc::c_int
                                        + 2 as libc::c_int
                                            * (((1 as libc::c_int) << 4 as libc::c_int)
                                                << 3 as libc::c_int)
                                        + ((1 as libc::c_int) << 8 as libc::c_int))
                                    + (0 as libc::c_int
                                        + 2 as libc::c_int
                                            * (((1 as libc::c_int) << 4 as libc::c_int)
                                                << 3 as libc::c_int)
                                        + ((1 as libc::c_int) << 8 as libc::c_int))
                                    + ((16 as libc::c_int) << 4 as libc::c_int)
                                    + ((1 as libc::c_int) << 4 as libc::c_int)
                                    + 12 as libc::c_int
                                    + 12 as libc::c_int
                                    + 12 as libc::c_int) as isize,
                            )
                            .offset(state as isize);
                        ttt = *prob as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh22 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh22 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *prob = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            ) as CLzmaProb;
                            distance = rep2
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *prob = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            distance = rep3;
                            rep3 = rep2
                        }
                        rep2 = rep1
                    }
                    rep1 = rep0;
                    rep0 = distance;
                    current_block_1101 = 7237866862515803946;
                }
                match current_block_1101 {
                    13183875560443969876 => {}
                    _ => {
                        state = if state < 7 as libc::c_int as libc::c_uint {
                            8 as libc::c_int
                        } else {
                            11 as libc::c_int
                        } as libc::c_uint;
                        prob = probs.offset(
                            (-(1664 as libc::c_int)
                                + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                                + ((16 as libc::c_int) << 4 as libc::c_int))
                                as isize,
                        );
                        current_block_1101 = 4800884466390615302;
                    }
                }
            }
            match current_block_1101 {
                13183875560443969876 => {}
                _ => {
                    let mut probLen: *mut CLzmaProb = prob.offset(0 as libc::c_int as isize);
                    ttt = *probLen as libc::c_uint;
                    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                        range <<= 8 as libc::c_int;
                        let fresh23 = buf;
                        buf = buf.offset(1);
                        code = code << 8 as libc::c_int | *fresh23 as libc::c_uint
                    }
                    bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                    if code < bound {
                        range = bound;
                        *probLen = ttt.wrapping_add(
                            (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                .wrapping_sub(ttt)
                                >> 5 as libc::c_int,
                        ) as CLzmaProb;
                        probLen = prob
                            .offset(0 as libc::c_int as isize)
                            .offset(posState as isize);
                        len = 1 as libc::c_int as libc::c_uint;
                        ttt = *probLen.offset(len as isize) as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh24 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh24 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *probLen.offset(len as isize) = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            )
                                as CLzmaProb;
                            len = len.wrapping_add(len)
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *probLen.offset(len as isize) =
                                ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            len = len
                                .wrapping_add(len)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                        }
                        ttt = *probLen.offset(len as isize) as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh25 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh25 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *probLen.offset(len as isize) = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            )
                                as CLzmaProb;
                            len = len.wrapping_add(len)
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *probLen.offset(len as isize) =
                                ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            len = len
                                .wrapping_add(len)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                        }
                        ttt = *probLen.offset(len as isize) as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh26 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh26 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *probLen.offset(len as isize) = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            )
                                as CLzmaProb;
                            len = len.wrapping_add(len)
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *probLen.offset(len as isize) =
                                ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            len = len
                                .wrapping_add(len)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                        }
                        len = len.wrapping_sub(8 as libc::c_int as libc::c_uint)
                    } else {
                        range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                        code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                        *probLen = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                        probLen = prob.offset(
                            (0 as libc::c_int + ((1 as libc::c_int) << 3 as libc::c_int)) as isize,
                        );
                        ttt = *probLen as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh27 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh27 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *probLen = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            ) as CLzmaProb;
                            probLen = prob
                                .offset(0 as libc::c_int as isize)
                                .offset(posState as isize)
                                .offset(((1 as libc::c_int) << 3 as libc::c_int) as isize);
                            len = 1 as libc::c_int as libc::c_uint;
                            ttt = *probLen.offset(len as isize) as libc::c_uint;
                            if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                                range <<= 8 as libc::c_int;
                                let fresh28 = buf;
                                buf = buf.offset(1);
                                code = code << 8 as libc::c_int | *fresh28 as libc::c_uint
                            }
                            bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                            if code < bound {
                                range = bound;
                                *probLen.offset(len as isize) = ttt.wrapping_add(
                                    (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                        .wrapping_sub(ttt)
                                        >> 5 as libc::c_int,
                                )
                                    as CLzmaProb;
                                len = len.wrapping_add(len)
                            } else {
                                range =
                                    (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                                code =
                                    (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                                *probLen.offset(len as isize) =
                                    ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                                len = len
                                    .wrapping_add(len)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                            }
                            ttt = *probLen.offset(len as isize) as libc::c_uint;
                            if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                                range <<= 8 as libc::c_int;
                                let fresh29 = buf;
                                buf = buf.offset(1);
                                code = code << 8 as libc::c_int | *fresh29 as libc::c_uint
                            }
                            bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                            if code < bound {
                                range = bound;
                                *probLen.offset(len as isize) = ttt.wrapping_add(
                                    (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                        .wrapping_sub(ttt)
                                        >> 5 as libc::c_int,
                                )
                                    as CLzmaProb;
                                len = len.wrapping_add(len)
                            } else {
                                range =
                                    (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                                code =
                                    (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                                *probLen.offset(len as isize) =
                                    ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                                len = len
                                    .wrapping_add(len)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                            }
                            ttt = *probLen.offset(len as isize) as libc::c_uint;
                            if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                                range <<= 8 as libc::c_int;
                                let fresh30 = buf;
                                buf = buf.offset(1);
                                code = code << 8 as libc::c_int | *fresh30 as libc::c_uint
                            }
                            bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                            if code < bound {
                                range = bound;
                                *probLen.offset(len as isize) = ttt.wrapping_add(
                                    (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                        .wrapping_sub(ttt)
                                        >> 5 as libc::c_int,
                                )
                                    as CLzmaProb;
                                len = len.wrapping_add(len)
                            } else {
                                range =
                                    (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                                code =
                                    (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                                *probLen.offset(len as isize) =
                                    ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                                len = len
                                    .wrapping_add(len)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                            }
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *probLen = ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            probLen = prob.offset(
                                (0 as libc::c_int
                                    + 2 as libc::c_int
                                        * (((1 as libc::c_int) << 4 as libc::c_int)
                                            << 3 as libc::c_int))
                                    as isize,
                            );
                            len = 1 as libc::c_int as libc::c_uint;
                            loop {
                                ttt = *probLen.offset(len as isize) as libc::c_uint;
                                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                                    range <<= 8 as libc::c_int;
                                    let fresh31 = buf;
                                    buf = buf.offset(1);
                                    code = code << 8 as libc::c_int | *fresh31 as libc::c_uint
                                }
                                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                                if code < bound {
                                    range = bound;
                                    *probLen.offset(len as isize) = ttt.wrapping_add(
                                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                            .wrapping_sub(ttt)
                                            >> 5 as libc::c_int,
                                    )
                                        as CLzmaProb;
                                    len = len.wrapping_add(len)
                                } else {
                                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32
                                        as UInt32;
                                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32
                                        as UInt32;
                                    *probLen.offset(len as isize) =
                                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                                    len = len
                                        .wrapping_add(len)
                                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                                }
                                if !(len < ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint)
                                {
                                    break;
                                }
                            }
                            len = len.wrapping_sub(
                                ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint,
                            );
                            len = len.wrapping_add(
                                (((1 as libc::c_int) << 3 as libc::c_int) * 2 as libc::c_int)
                                    as libc::c_uint,
                            )
                        }
                    }
                    if state >= 12 as libc::c_int as libc::c_uint {
                        let mut distance_0: UInt32 = 0;
                        prob = probs
                            .offset(
                                (-(1664 as libc::c_int)
                                    + ((1 as libc::c_int)
                                        << (14 as libc::c_int >> 1 as libc::c_int))
                                    + ((16 as libc::c_int) << 4 as libc::c_int)
                                    + (0 as libc::c_int
                                        + 2 as libc::c_int
                                            * (((1 as libc::c_int) << 4 as libc::c_int)
                                                << 3 as libc::c_int)
                                        + ((1 as libc::c_int) << 8 as libc::c_int))
                                    + (0 as libc::c_int
                                        + 2 as libc::c_int
                                            * (((1 as libc::c_int) << 4 as libc::c_int)
                                                << 3 as libc::c_int)
                                        + ((1 as libc::c_int) << 8 as libc::c_int))
                                    + ((16 as libc::c_int) << 4 as libc::c_int)
                                    + ((1 as libc::c_int) << 4 as libc::c_int)
                                    + 12 as libc::c_int
                                    + 12 as libc::c_int
                                    + 12 as libc::c_int
                                    + 12 as libc::c_int) as isize,
                            )
                            .offset(
                                ((if len < 4 as libc::c_int as libc::c_uint {
                                    len
                                } else {
                                    (4 as libc::c_int - 1 as libc::c_int) as libc::c_uint
                                }) << 6 as libc::c_int) as isize,
                            );
                        distance_0 = 1 as libc::c_int as UInt32;
                        ttt = *prob.offset(distance_0 as isize) as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh32 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh32 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *prob.offset(distance_0 as isize) = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            )
                                as CLzmaProb;
                            distance_0 = distance_0.wrapping_add(distance_0)
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *prob.offset(distance_0 as isize) =
                                ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            distance_0 = distance_0
                                .wrapping_add(distance_0)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                        }
                        ttt = *prob.offset(distance_0 as isize) as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh33 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh33 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *prob.offset(distance_0 as isize) = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            )
                                as CLzmaProb;
                            distance_0 = distance_0.wrapping_add(distance_0)
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *prob.offset(distance_0 as isize) =
                                ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            distance_0 = distance_0
                                .wrapping_add(distance_0)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                        }
                        ttt = *prob.offset(distance_0 as isize) as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh34 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh34 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *prob.offset(distance_0 as isize) = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            )
                                as CLzmaProb;
                            distance_0 = distance_0.wrapping_add(distance_0)
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *prob.offset(distance_0 as isize) =
                                ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            distance_0 = distance_0
                                .wrapping_add(distance_0)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                        }
                        ttt = *prob.offset(distance_0 as isize) as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh35 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh35 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *prob.offset(distance_0 as isize) = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            )
                                as CLzmaProb;
                            distance_0 = distance_0.wrapping_add(distance_0)
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *prob.offset(distance_0 as isize) =
                                ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            distance_0 = distance_0
                                .wrapping_add(distance_0)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                        }
                        ttt = *prob.offset(distance_0 as isize) as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh36 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh36 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *prob.offset(distance_0 as isize) = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            )
                                as CLzmaProb;
                            distance_0 = distance_0.wrapping_add(distance_0)
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *prob.offset(distance_0 as isize) =
                                ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            distance_0 = distance_0
                                .wrapping_add(distance_0)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                        }
                        ttt = *prob.offset(distance_0 as isize) as libc::c_uint;
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            range <<= 8 as libc::c_int;
                            let fresh37 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh37 as libc::c_uint
                        }
                        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                        if code < bound {
                            range = bound;
                            *prob.offset(distance_0 as isize) = ttt.wrapping_add(
                                (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                    .wrapping_sub(ttt)
                                    >> 5 as libc::c_int,
                            )
                                as CLzmaProb;
                            distance_0 = distance_0.wrapping_add(distance_0)
                        } else {
                            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                            *prob.offset(distance_0 as isize) =
                                ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                            distance_0 = distance_0
                                .wrapping_add(distance_0)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                        }
                        distance_0 = (distance_0 as libc::c_uint)
                            .wrapping_sub(0x40 as libc::c_int as libc::c_uint)
                            as UInt32 as UInt32;
                        if distance_0 >= 4 as libc::c_int as libc::c_uint {
                            let posSlot: libc::c_uint = distance_0;
                            let mut numDirectBits: libc::c_uint = (distance_0 >> 1 as libc::c_int)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint);
                            distance_0 = 2 as libc::c_int as libc::c_uint
                                | distance_0 & 1 as libc::c_int as libc::c_uint;
                            if posSlot < 14 as libc::c_int as libc::c_uint {
                                distance_0 <<= numDirectBits;
                                prob = probs.offset(-(1664 as libc::c_int) as isize);
                                let mut m: UInt32 = 1 as libc::c_int as UInt32;
                                distance_0 = distance_0.wrapping_add(1);
                                loop {
                                    ttt = *prob.offset(distance_0 as isize) as libc::c_uint;
                                    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                                        range <<= 8 as libc::c_int;
                                        let fresh38 = buf;
                                        buf = buf.offset(1);
                                        code = code << 8 as libc::c_int | *fresh38 as libc::c_uint
                                    }
                                    bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                                    if code < bound {
                                        range = bound;
                                        *prob.offset(distance_0 as isize) = ttt.wrapping_add(
                                            (((1 as libc::c_int) << 11 as libc::c_int)
                                                as libc::c_uint)
                                                .wrapping_sub(ttt)
                                                >> 5 as libc::c_int,
                                        )
                                            as CLzmaProb;
                                        distance_0 = (distance_0 as libc::c_uint).wrapping_add(m)
                                            as UInt32
                                            as UInt32;
                                        m = (m as libc::c_uint).wrapping_add(m) as UInt32 as UInt32
                                    } else {
                                        range = (range as libc::c_uint).wrapping_sub(bound)
                                            as UInt32
                                            as UInt32;
                                        code = (code as libc::c_uint).wrapping_sub(bound) as UInt32
                                            as UInt32;
                                        *prob.offset(distance_0 as isize) =
                                            ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                                        m = (m as libc::c_uint).wrapping_add(m) as UInt32 as UInt32;
                                        distance_0 = (distance_0 as libc::c_uint).wrapping_add(m)
                                            as UInt32
                                            as UInt32
                                    }
                                    numDirectBits = numDirectBits.wrapping_sub(1);
                                    if !(numDirectBits != 0) {
                                        break;
                                    }
                                }
                                distance_0 =
                                    (distance_0 as libc::c_uint).wrapping_sub(m) as UInt32 as UInt32
                            } else {
                                numDirectBits =
                                    numDirectBits.wrapping_sub(4 as libc::c_int as libc::c_uint);
                                loop {
                                    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                                        range <<= 8 as libc::c_int;
                                        let fresh39 = buf;
                                        buf = buf.offset(1);
                                        code = code << 8 as libc::c_int | *fresh39 as libc::c_uint
                                    }
                                    range >>= 1 as libc::c_int;
                                    let mut t: UInt32 = 0;
                                    code = (code as libc::c_uint).wrapping_sub(range) as UInt32
                                        as UInt32;
                                    /*
                                    distance <<= 1;
                                    if (code >= range)
                                    {
                                      code -= range;
                                      distance |= 1;
                                    }
                                    */
                                    t = (0 as libc::c_int as libc::c_uint)
                                        .wrapping_sub(code >> 31 as libc::c_int); /* (UInt32)((Int32)code >> 31) */
                                    distance_0 = (distance_0 << 1 as libc::c_int).wrapping_add(
                                        t.wrapping_add(1 as libc::c_int as libc::c_uint),
                                    ); /* we use SizeT to avoid the BUG of VC14 for AMD64 */
                                    code = (code as libc::c_uint).wrapping_add(range & t) as UInt32
                                        as UInt32;
                                    numDirectBits = numDirectBits.wrapping_sub(1);
                                    if !(numDirectBits != 0) {
                                        break;
                                    }
                                }
                                prob = probs.offset(
                                    (-(1664 as libc::c_int)
                                        + ((1 as libc::c_int)
                                            << (14 as libc::c_int >> 1 as libc::c_int))
                                        + ((16 as libc::c_int) << 4 as libc::c_int)
                                        + (0 as libc::c_int
                                            + 2 as libc::c_int
                                                * (((1 as libc::c_int) << 4 as libc::c_int)
                                                    << 3 as libc::c_int)
                                            + ((1 as libc::c_int) << 8 as libc::c_int))
                                        + (0 as libc::c_int
                                            + 2 as libc::c_int
                                                * (((1 as libc::c_int) << 4 as libc::c_int)
                                                    << 3 as libc::c_int)
                                            + ((1 as libc::c_int) << 8 as libc::c_int))
                                        + ((16 as libc::c_int) << 4 as libc::c_int))
                                        as isize,
                                );
                                distance_0 <<= 4 as libc::c_int;
                                let mut i: libc::c_uint = 1 as libc::c_int as libc::c_uint;
                                ttt = *prob.offset(i as isize) as libc::c_uint;
                                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                                    range <<= 8 as libc::c_int;
                                    let fresh40 = buf;
                                    buf = buf.offset(1);
                                    code = code << 8 as libc::c_int | *fresh40 as libc::c_uint
                                }
                                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                                if code < bound {
                                    range = bound;
                                    *prob.offset(i as isize) = ttt.wrapping_add(
                                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                            .wrapping_sub(ttt)
                                            >> 5 as libc::c_int,
                                    )
                                        as CLzmaProb;
                                    i = i.wrapping_add(1 as libc::c_int as libc::c_uint)
                                } else {
                                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32
                                        as UInt32;
                                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32
                                        as UInt32;
                                    *prob.offset(i as isize) =
                                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                                    i = i.wrapping_add(
                                        (1 as libc::c_int * 2 as libc::c_int) as libc::c_uint,
                                    )
                                }
                                ttt = *prob.offset(i as isize) as libc::c_uint;
                                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                                    range <<= 8 as libc::c_int;
                                    let fresh41 = buf;
                                    buf = buf.offset(1);
                                    code = code << 8 as libc::c_int | *fresh41 as libc::c_uint
                                }
                                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                                if code < bound {
                                    range = bound;
                                    *prob.offset(i as isize) = ttt.wrapping_add(
                                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                            .wrapping_sub(ttt)
                                            >> 5 as libc::c_int,
                                    )
                                        as CLzmaProb;
                                    i = i.wrapping_add(2 as libc::c_int as libc::c_uint)
                                } else {
                                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32
                                        as UInt32;
                                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32
                                        as UInt32;
                                    *prob.offset(i as isize) =
                                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                                    i = i.wrapping_add(
                                        (2 as libc::c_int * 2 as libc::c_int) as libc::c_uint,
                                    )
                                }
                                ttt = *prob.offset(i as isize) as libc::c_uint;
                                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                                    range <<= 8 as libc::c_int;
                                    let fresh42 = buf;
                                    buf = buf.offset(1);
                                    code = code << 8 as libc::c_int | *fresh42 as libc::c_uint
                                }
                                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                                if code < bound {
                                    range = bound;
                                    *prob.offset(i as isize) = ttt.wrapping_add(
                                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                            .wrapping_sub(ttt)
                                            >> 5 as libc::c_int,
                                    )
                                        as CLzmaProb;
                                    i = i.wrapping_add(4 as libc::c_int as libc::c_uint)
                                } else {
                                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32
                                        as UInt32;
                                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32
                                        as UInt32;
                                    *prob.offset(i as isize) =
                                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb;
                                    i = i.wrapping_add(
                                        (4 as libc::c_int * 2 as libc::c_int) as libc::c_uint,
                                    )
                                }
                                ttt = *prob.offset(i as isize) as libc::c_uint;
                                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                                    range <<= 8 as libc::c_int;
                                    let fresh43 = buf;
                                    buf = buf.offset(1);
                                    code = code << 8 as libc::c_int | *fresh43 as libc::c_uint
                                }
                                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                                if code < bound {
                                    range = bound;
                                    *prob.offset(i as isize) = ttt.wrapping_add(
                                        (((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                                            .wrapping_sub(ttt)
                                            >> 5 as libc::c_int,
                                    )
                                        as CLzmaProb;
                                    i = i.wrapping_sub(8 as libc::c_int as libc::c_uint)
                                } else {
                                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32
                                        as UInt32;
                                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32
                                        as UInt32;
                                    *prob.offset(i as isize) =
                                        ttt.wrapping_sub(ttt >> 5 as libc::c_int) as CLzmaProb
                                }
                                distance_0 |= i;
                                if distance_0 == 0xffffffff as libc::c_uint {
                                    len = (2 as libc::c_int
                                        + ((1 as libc::c_int) << 3 as libc::c_int)
                                            * 2 as libc::c_int
                                        + ((1 as libc::c_int) << 8 as libc::c_int))
                                        as libc::c_uint;
                                    state = state.wrapping_sub(12 as libc::c_int as libc::c_uint);
                                    break;
                                }
                            }
                        }
                        rep3 = rep2;
                        rep2 = rep1;
                        rep1 = rep0;
                        rep0 = distance_0.wrapping_add(1 as libc::c_int as libc::c_uint);
                        state = if state < (12 as libc::c_int + 7 as libc::c_int) as libc::c_uint {
                            7 as libc::c_int
                        } else {
                            (7 as libc::c_int) + 3 as libc::c_int
                        } as libc::c_uint;
                        if distance_0
                            >= (if checkDicSize == 0 as libc::c_int as libc::c_uint {
                                processedPos
                            } else {
                                checkDicSize
                            })
                        {
                            (*p).dicPos = dicPos;
                            return 1 as libc::c_int;
                        }
                    }
                    len = len.wrapping_add(2 as libc::c_int as libc::c_uint);
                    let mut rem: SizeT = 0;
                    let mut curLen: libc::c_uint = 0;
                    let mut pos: SizeT = 0;
                    rem = limit.wrapping_sub(dicPos);
                    if rem == 0 {
                        (*p).dicPos = dicPos;
                        return 1;
                    }
                    curLen = if rem < len as SizeT {
                        rem as libc::c_uint
                    } else {
                        len
                    };
                    pos = dicPos.wrapping_sub(rep0 as SizeT).wrapping_add(
                        if dicPos < rep0 as SizeT {
                            dicBufSize
                        } else {
                            0
                        },
                    );
                    processedPos =
                        (processedPos as libc::c_uint).wrapping_add(curLen) as UInt32 as UInt32;
                    len = len.wrapping_sub(curLen);
                    if curLen as SizeT <= dicBufSize.wrapping_sub(pos) {
                        let mut dest: *mut Byte = dic.offset(dicPos as isize);
                        let src: ptrdiff_t = pos as ptrdiff_t - dicPos as ptrdiff_t;
                        let lim: *const Byte = dest.offset(curLen as isize);
                        dicPos = (dicPos).wrapping_add(curLen as SizeT);
                        loop {
                            *dest = *dest.offset(src as isize);
                            dest = dest.offset(1);
                            if !(dest != lim as *mut Byte) {
                                break;
                            }
                        }
                    } else {
                        loop {
                            let fresh44 = dicPos;
                            dicPos = dicPos.wrapping_add(1);
                            *dic.offset(fresh44 as isize) = *dic.offset(pos as isize);
                            pos = pos.wrapping_add(1);
                            if pos == dicBufSize {
                                pos = 0 as libc::c_int as SizeT
                            }
                            curLen = curLen.wrapping_sub(1);
                            if !(curLen != 0 as libc::c_int as libc::c_uint) {
                                break;
                            }
                        }
                    }
                }
            }
        }
        if !(dicPos < limit && buf < bufLimit) {
            break;
        }
    }
    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
        range <<= 8 as libc::c_int;
        let fresh45 = buf;
        buf = buf.offset(1);
        code = code << 8 as libc::c_int | *fresh45 as libc::c_uint
    }
    (*p).buf = buf;
    (*p).range = range;
    (*p).code = code;
    (*p).remainLen = len;
    (*p).dicPos = dicPos;
    (*p).processedPos = processedPos;
    (*p).reps[0 as libc::c_int as usize] = rep0;
    (*p).reps[1 as libc::c_int as usize] = rep1;
    (*p).reps[2 as libc::c_int as usize] = rep2;
    (*p).reps[3 as libc::c_int as usize] = rep3;
    (*p).state = state;
    return 0 as libc::c_int;
}
unsafe fn LzmaDec_WriteRem(mut p: *mut CLzmaDec, limit: SizeT) {
    if (*p).remainLen != 0 as libc::c_int as libc::c_uint
        && (*p).remainLen
            < (2 as libc::c_int
                + ((1 as libc::c_int) << 3 as libc::c_int) * 2 as libc::c_int
                + ((1 as libc::c_int) << 8 as libc::c_int)) as libc::c_uint
    {
        let dic: *mut Byte = (*p).dic;
        let mut dicPos: SizeT = (*p).dicPos;
        let dicBufSize: SizeT = (*p).dicBufSize;
        let mut len: libc::c_uint = (*p).remainLen;
        let rep0: SizeT = (*p).reps[0 as libc::c_int as usize] as SizeT;
        let rem: SizeT = limit.wrapping_sub(dicPos);
        if rem < len as SizeT {
            len = rem as libc::c_uint
        }
        if (*p).checkDicSize == 0 as libc::c_int as libc::c_uint
            && (*p).prop.dicSize.wrapping_sub((*p).processedPos) <= len
        {
            (*p).checkDicSize = (*p).prop.dicSize
        }
        (*p).processedPos =
            ((*p).processedPos as libc::c_uint).wrapping_add(len) as UInt32 as UInt32;
        (*p).remainLen = ((*p).remainLen as libc::c_uint).wrapping_sub(len) as UInt32 as UInt32;
        while len != 0 as libc::c_int as libc::c_uint {
            len = len.wrapping_sub(1);
            *dic.offset(dicPos as isize) =
                *dic.offset(dicPos.wrapping_sub(rep0).wrapping_add(if dicPos < rep0 {
                    dicBufSize
                } else {
                    0
                }) as isize);
            dicPos = dicPos.wrapping_add(1)
        }
        (*p).dicPos = dicPos
    };
}
unsafe fn LzmaDec_DecodeReal2(
    mut p: *mut CLzmaDec,
    limit: SizeT,
    bufLimit: *const Byte,
    memlimit: SizeT,
) -> libc::c_int {
    loop {
        let mut limit2: SizeT = limit;
        if (*p).checkDicSize == 0 as libc::c_int as libc::c_uint {
            let rem: UInt32 = (*p).prop.dicSize.wrapping_sub((*p).processedPos);
            if limit.wrapping_sub((*p).dicPos) > rem as SizeT {
                if (*p).dicBufSize < (*p).prop.dicSize as SizeT {
                    (*p).dicBufSize = (*p).prop.dicSize as SizeT
                }
                if (*p).dicBufSize > memlimit {
                    return 2 as libc::c_int;
                }
                let tmp: *mut Byte =
                    realloc((*p).dic as *mut libc::c_void, (*p).dicBufSize) as *mut Byte;
                if tmp.is_null() {
                    return 2 as libc::c_int;
                }
                (*p).dic = tmp;
                limit2 = (*p).dicPos.wrapping_add(rem as SizeT)
            }
            if (*p).processedPos == 0 as libc::c_int as libc::c_uint {
                if (*p).code
                    >= ((0xffffffff as libc::c_uint >> 11 as libc::c_int)
                        << 11 as libc::c_int - 1 as libc::c_int)
                        .wrapping_add(
                            ((0xffffffff as libc::c_uint).wrapping_sub(
                                (0xffffffff as libc::c_uint >> 11 as libc::c_int)
                                    << 11 as libc::c_int - 1 as libc::c_int,
                            ) >> 11 as libc::c_int)
                                << 11 as libc::c_int - 1 as libc::c_int,
                        )
                {
                    return 1 as libc::c_int;
                }
            }
        }
        let mut __result__: libc::c_int = LzmaDec_DecodeReal_3(p, limit2, bufLimit);
        if __result__ != 0 as libc::c_int {
            return __result__;
        }
        if (*p).checkDicSize == 0 as libc::c_int as libc::c_uint
            && (*p).processedPos >= (*p).prop.dicSize
        {
            (*p).checkDicSize = (*p).prop.dicSize
        }
        LzmaDec_WriteRem(p, limit);
        if !((*p).dicPos < limit
            && (*p).buf < bufLimit
            && (*p).remainLen
                < (2 as libc::c_int
                    + ((1 as libc::c_int) << 3 as libc::c_int) * 2 as libc::c_int
                    + ((1 as libc::c_int) << 8 as libc::c_int)) as libc::c_uint)
        {
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe fn LzmaDec_TryDummy(p: *const CLzmaDec, mut buf: *const Byte, inSize: SizeT) -> ELzmaDummy {
    let mut range: UInt32 = (*p).range;
    let mut code: UInt32 = (*p).code;
    let bufLimit: *const Byte = buf.offset(inSize as isize);
    let probs: *const CLzmaProb = (*p).probs_1664;
    let mut state: libc::c_uint = (*p).state;
    let mut res: ELzmaDummy = ELzmaDummy::DUMMY_ERROR;
    let mut prob: *const CLzmaProb = 0 as *const CLzmaProb;
    let mut bound: UInt32 = 0;
    let mut ttt: libc::c_uint = 0;
    let posState: libc::c_uint = ((*p).processedPos
        & (((1 as libc::c_int) << (*p).prop.pb as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << 4 as libc::c_int;
    prob = probs
        .offset(
            (-(1664 as libc::c_int)
                + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                + ((16 as libc::c_int) << 4 as libc::c_int)
                + (0 as libc::c_int
                    + 2 as libc::c_int
                        * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                    + ((1 as libc::c_int) << 8 as libc::c_int))
                + (0 as libc::c_int
                    + 2 as libc::c_int
                        * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                    + ((1 as libc::c_int) << 8 as libc::c_int))) as isize,
        )
        .offset(posState.wrapping_add(state) as isize);
    ttt = *prob as libc::c_uint;
    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
        if buf >= bufLimit {
            return ELzmaDummy::DUMMY_ERROR;
        }
        range <<= 8 as libc::c_int;
        let fresh46 = buf;
        buf = buf.offset(1);
        code = code << 8 as libc::c_int | *fresh46 as libc::c_uint
    }
    bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
    if code < bound {
        range = bound;
        /* if (bufLimit - buf >= 7) return DUMMY_LIT; */
        prob = probs.offset(
            (-(1664 as libc::c_int)
                + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                + ((16 as libc::c_int) << 4 as libc::c_int)
                + (0 as libc::c_int
                    + 2 as libc::c_int
                        * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                    + ((1 as libc::c_int) << 8 as libc::c_int))
                + (0 as libc::c_int
                    + 2 as libc::c_int
                        * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                    + ((1 as libc::c_int) << 8 as libc::c_int))
                + ((16 as libc::c_int) << 4 as libc::c_int)
                + ((1 as libc::c_int) << 4 as libc::c_int)
                + 12 as libc::c_int
                + 12 as libc::c_int
                + 12 as libc::c_int
                + 12 as libc::c_int
                + ((4 as libc::c_int) << 6 as libc::c_int)) as isize,
        );
        if (*p).checkDicSize != 0 as libc::c_int as libc::c_uint
            || (*p).processedPos != 0 as libc::c_int as libc::c_uint
        {
            prob = prob.offset(
                (0x300 as libc::c_int as UInt32).wrapping_mul(
                    (((*p).processedPos
                        & (((1 as libc::c_int) << (*p).prop.lp as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (*p).prop.lc as libc::c_int)
                        .wrapping_add(
                            (*(*p).dic.offset(
                                (if (*p).dicPos == 0 {
                                    (*p).dicBufSize
                                } else {
                                    (*p).dicPos
                                })
                                .wrapping_sub(1) as isize,
                            ) as libc::c_int
                                >> 8 as libc::c_int - (*p).prop.lc as libc::c_int)
                                as libc::c_uint,
                        ),
                ) as isize,
            )
        }
        if state < 7 as libc::c_int as libc::c_uint {
            let mut symbol: libc::c_uint = 1 as libc::c_int as libc::c_uint;
            loop {
                ttt = *prob.offset(symbol as isize) as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    if buf >= bufLimit {
                        return ELzmaDummy::DUMMY_ERROR;
                    }
                    range <<= 8 as libc::c_int;
                    let fresh47 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh47 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    symbol = symbol.wrapping_add(symbol)
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    symbol = symbol
                        .wrapping_add(symbol)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                if !(symbol < 0x100 as libc::c_int as libc::c_uint) {
                    break;
                }
            }
        } else {
            let mut matchByte: libc::c_uint = *(*p).dic.offset(
                (*p).dicPos
                    .wrapping_sub((*p).reps[0 as usize] as SizeT)
                    .wrapping_add(if (*p).dicPos < (*p).reps[0 as usize] as SizeT {
                        (*p).dicBufSize
                    } else {
                        0
                    }) as isize,
            ) as libc::c_uint;
            let mut offs: libc::c_uint = 0x100 as libc::c_int as libc::c_uint;
            let mut symbol_0: libc::c_uint = 1 as libc::c_int as libc::c_uint;
            loop {
                let mut bit: libc::c_uint = 0;
                let mut probLit: *const CLzmaProb = 0 as *const CLzmaProb;
                matchByte = matchByte.wrapping_add(matchByte);
                bit = offs;
                offs &= matchByte;
                probLit = prob.offset(offs.wrapping_add(bit).wrapping_add(symbol_0) as isize);
                ttt = *probLit as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    if buf >= bufLimit {
                        return ELzmaDummy::DUMMY_ERROR;
                    }
                    range <<= 8 as libc::c_int;
                    let fresh48 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh48 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    symbol_0 = symbol_0.wrapping_add(symbol_0);
                    offs ^= bit
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    symbol_0 = symbol_0
                        .wrapping_add(symbol_0)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                if !(symbol_0 < 0x100 as libc::c_int as libc::c_uint) {
                    break;
                }
            }
        }
        res = ELzmaDummy::DUMMY_LIT
    } else {
        let mut len: libc::c_uint = 0;
        range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
        code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
        prob = probs
            .offset(
                (-(1664 as libc::c_int)
                    + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                    + ((16 as libc::c_int) << 4 as libc::c_int)
                    + (0 as libc::c_int
                        + 2 as libc::c_int
                            * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                        + ((1 as libc::c_int) << 8 as libc::c_int))
                    + (0 as libc::c_int
                        + 2 as libc::c_int
                            * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                        + ((1 as libc::c_int) << 8 as libc::c_int))
                    + ((16 as libc::c_int) << 4 as libc::c_int)
                    + ((1 as libc::c_int) << 4 as libc::c_int)) as isize,
            )
            .offset(state as isize);
        ttt = *prob as libc::c_uint;
        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
            if buf >= bufLimit {
                return ELzmaDummy::DUMMY_ERROR;
            }
            range <<= 8 as libc::c_int;
            let fresh49 = buf;
            buf = buf.offset(1);
            code = code << 8 as libc::c_int | *fresh49 as libc::c_uint
        }
        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
        if code < bound {
            range = bound;
            state = 0 as libc::c_int as libc::c_uint;
            prob = probs.offset(
                (-(1664 as libc::c_int)
                    + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                    + ((16 as libc::c_int) << 4 as libc::c_int)
                    + (0 as libc::c_int
                        + 2 as libc::c_int
                            * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                        + ((1 as libc::c_int) << 8 as libc::c_int))) as isize,
            );
            res = ELzmaDummy::DUMMY_MATCH
        } else {
            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
            res = ELzmaDummy::DUMMY_REP;
            prob = probs
                .offset(
                    (-(1664 as libc::c_int)
                        + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                        + ((16 as libc::c_int) << 4 as libc::c_int)
                        + (0 as libc::c_int
                            + 2 as libc::c_int
                                * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                            + ((1 as libc::c_int) << 8 as libc::c_int))
                        + (0 as libc::c_int
                            + 2 as libc::c_int
                                * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                            + ((1 as libc::c_int) << 8 as libc::c_int))
                        + ((16 as libc::c_int) << 4 as libc::c_int)
                        + ((1 as libc::c_int) << 4 as libc::c_int)
                        + 12 as libc::c_int) as isize,
                )
                .offset(state as isize);
            ttt = *prob as libc::c_uint;
            if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                if buf >= bufLimit {
                    return ELzmaDummy::DUMMY_ERROR;
                }
                range <<= 8 as libc::c_int;
                let fresh50 = buf;
                buf = buf.offset(1);
                code = code << 8 as libc::c_int | *fresh50 as libc::c_uint
            }
            bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
            if code < bound {
                range = bound;
                prob = probs
                    .offset(
                        (-(1664 as libc::c_int)
                            + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int)))
                            as isize,
                    )
                    .offset(posState.wrapping_add(state) as isize);
                ttt = *prob as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    if buf >= bufLimit {
                        return ELzmaDummy::DUMMY_ERROR;
                    }
                    range <<= 8 as libc::c_int;
                    let fresh51 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh51 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                        if buf >= bufLimit {
                            return ELzmaDummy::DUMMY_ERROR;
                        }
                        range <<= 8 as libc::c_int;
                        let fresh52 = buf;
                        buf = buf.offset(1);
                        code = code << 8 as libc::c_int | *fresh52 as libc::c_uint
                    }
                    return ELzmaDummy::DUMMY_REP;
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32
                }
            } else {
                range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                prob = probs
                    .offset(
                        (-(1664 as libc::c_int)
                            + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                            + ((16 as libc::c_int) << 4 as libc::c_int)
                            + (0 as libc::c_int
                                + 2 as libc::c_int
                                    * (((1 as libc::c_int) << 4 as libc::c_int)
                                        << 3 as libc::c_int)
                                + ((1 as libc::c_int) << 8 as libc::c_int))
                            + (0 as libc::c_int
                                + 2 as libc::c_int
                                    * (((1 as libc::c_int) << 4 as libc::c_int)
                                        << 3 as libc::c_int)
                                + ((1 as libc::c_int) << 8 as libc::c_int))
                            + ((16 as libc::c_int) << 4 as libc::c_int)
                            + ((1 as libc::c_int) << 4 as libc::c_int)
                            + 12 as libc::c_int
                            + 12 as libc::c_int) as isize,
                    )
                    .offset(state as isize);
                ttt = *prob as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    if buf >= bufLimit {
                        return ELzmaDummy::DUMMY_ERROR;
                    }
                    range <<= 8 as libc::c_int;
                    let fresh53 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh53 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    prob = probs
                        .offset(
                            (-(1664 as libc::c_int)
                                + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                                + ((16 as libc::c_int) << 4 as libc::c_int)
                                + (0 as libc::c_int
                                    + 2 as libc::c_int
                                        * (((1 as libc::c_int) << 4 as libc::c_int)
                                            << 3 as libc::c_int)
                                    + ((1 as libc::c_int) << 8 as libc::c_int))
                                + (0 as libc::c_int
                                    + 2 as libc::c_int
                                        * (((1 as libc::c_int) << 4 as libc::c_int)
                                            << 3 as libc::c_int)
                                    + ((1 as libc::c_int) << 8 as libc::c_int))
                                + ((16 as libc::c_int) << 4 as libc::c_int)
                                + ((1 as libc::c_int) << 4 as libc::c_int)
                                + 12 as libc::c_int
                                + 12 as libc::c_int
                                + 12 as libc::c_int) as isize,
                        )
                        .offset(state as isize);
                    ttt = *prob as libc::c_uint;
                    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                        if buf >= bufLimit {
                            return ELzmaDummy::DUMMY_ERROR;
                        }
                        range <<= 8 as libc::c_int;
                        let fresh54 = buf;
                        buf = buf.offset(1);
                        code = code << 8 as libc::c_int | *fresh54 as libc::c_uint
                    }
                    bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                    if code < bound {
                        range = bound
                    } else {
                        range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                        code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32
                    }
                }
            }
            state = 12 as libc::c_int as libc::c_uint;
            prob = probs.offset(
                (-(1664 as libc::c_int)
                    + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                    + ((16 as libc::c_int) << 4 as libc::c_int)) as isize,
            )
        }
        let mut limit: libc::c_uint = 0;
        let mut offset: libc::c_uint = 0;
        let mut probLen: *const CLzmaProb = prob.offset(0 as libc::c_int as isize);
        ttt = *probLen as libc::c_uint;
        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
            if buf >= bufLimit {
                return ELzmaDummy::DUMMY_ERROR;
            }
            range <<= 8 as libc::c_int;
            let fresh55 = buf;
            buf = buf.offset(1);
            code = code << 8 as libc::c_int | *fresh55 as libc::c_uint
        }
        bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
        if code < bound {
            range = bound;
            probLen = prob
                .offset(0 as libc::c_int as isize)
                .offset(posState as isize);
            offset = 0 as libc::c_int as libc::c_uint;
            limit = ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
        } else {
            range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
            code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
            probLen =
                prob.offset((0 as libc::c_int + ((1 as libc::c_int) << 3 as libc::c_int)) as isize);
            ttt = *probLen as libc::c_uint;
            if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                if buf >= bufLimit {
                    return ELzmaDummy::DUMMY_ERROR;
                }
                range <<= 8 as libc::c_int;
                let fresh56 = buf;
                buf = buf.offset(1);
                code = code << 8 as libc::c_int | *fresh56 as libc::c_uint
            }
            bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
            if code < bound {
                range = bound;
                probLen = prob
                    .offset(0 as libc::c_int as isize)
                    .offset(posState as isize)
                    .offset(((1 as libc::c_int) << 3 as libc::c_int) as isize);
                offset = ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
                limit = ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
            } else {
                range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                probLen = prob.offset(
                    (0 as libc::c_int
                        + 2 as libc::c_int
                            * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int))
                        as isize,
                );
                offset =
                    (((1 as libc::c_int) << 3 as libc::c_int) * 2 as libc::c_int) as libc::c_uint;
                limit = ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
            }
        }
        len = 1 as libc::c_int as libc::c_uint;
        loop {
            ttt = *probLen.offset(len as isize) as libc::c_uint;
            if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                if buf >= bufLimit {
                    return ELzmaDummy::DUMMY_ERROR;
                }
                range <<= 8 as libc::c_int;
                let fresh57 = buf;
                buf = buf.offset(1);
                code = code << 8 as libc::c_int | *fresh57 as libc::c_uint
            }
            bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
            if code < bound {
                range = bound;
                len = len.wrapping_add(len)
            } else {
                range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                len = len
                    .wrapping_add(len)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
            }
            if !(len < limit) {
                break;
            }
        }
        len = len.wrapping_sub(limit);
        len = len.wrapping_add(offset);
        if state < 4 as libc::c_int as libc::c_uint {
            let mut posSlot: libc::c_uint = 0;
            prob = probs
                .offset(
                    (-(1664 as libc::c_int)
                        + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                        + ((16 as libc::c_int) << 4 as libc::c_int)
                        + (0 as libc::c_int
                            + 2 as libc::c_int
                                * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                            + ((1 as libc::c_int) << 8 as libc::c_int))
                        + (0 as libc::c_int
                            + 2 as libc::c_int
                                * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                            + ((1 as libc::c_int) << 8 as libc::c_int))
                        + ((16 as libc::c_int) << 4 as libc::c_int)
                        + ((1 as libc::c_int) << 4 as libc::c_int)
                        + 12 as libc::c_int
                        + 12 as libc::c_int
                        + 12 as libc::c_int
                        + 12 as libc::c_int) as isize,
                )
                .offset(
                    ((if len < (4 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
                        len
                    } else {
                        (4 as libc::c_int - 1 as libc::c_int) as libc::c_uint
                    }) << 6 as libc::c_int) as isize,
                );
            posSlot = 1 as libc::c_int as libc::c_uint;
            loop {
                ttt = *prob.offset(posSlot as isize) as libc::c_uint;
                if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                    if buf >= bufLimit {
                        return ELzmaDummy::DUMMY_ERROR;
                    }
                    range <<= 8 as libc::c_int;
                    let fresh58 = buf;
                    buf = buf.offset(1);
                    code = code << 8 as libc::c_int | *fresh58 as libc::c_uint
                }
                bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                if code < bound {
                    range = bound;
                    posSlot = posSlot.wrapping_add(posSlot)
                } else {
                    range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                    posSlot = posSlot
                        .wrapping_add(posSlot)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }
                if !(posSlot < ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint) {
                    break;
                }
            }
            posSlot =
                posSlot.wrapping_sub(((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint);
            if posSlot >= 4 as libc::c_int as libc::c_uint {
                let mut numDirectBits: libc::c_uint =
                    (posSlot >> 1 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint);
                /* if (bufLimit - buf >= 8) return DUMMY_MATCH; */
                if posSlot < 14 as libc::c_int as libc::c_uint {
                    prob = probs.offset(-(1664 as libc::c_int) as isize).offset(
                        ((2 as libc::c_int as libc::c_uint
                            | posSlot & 1 as libc::c_int as libc::c_uint)
                            << numDirectBits) as isize,
                    )
                } else {
                    numDirectBits = numDirectBits.wrapping_sub(4 as libc::c_int as libc::c_uint);
                    loop {
                        if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                            if buf >= bufLimit {
                                return ELzmaDummy::DUMMY_ERROR;
                            }
                            range <<= 8 as libc::c_int;
                            let fresh59 = buf;
                            buf = buf.offset(1);
                            code = code << 8 as libc::c_int | *fresh59 as libc::c_uint
                        }
                        range >>= 1 as libc::c_int;
                        code = (code as libc::c_uint).wrapping_sub(
                            range
                                & (code.wrapping_sub(range) >> 31 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        ) as UInt32 as UInt32;
                        numDirectBits = numDirectBits.wrapping_sub(1);
                        if !(numDirectBits != 0) {
                            break;
                        }
                        /* if (code >= range) code -= range; */
                    } /* some internal error */
                    prob = probs.offset(
                        (-(1664 as libc::c_int)
                            + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                            + ((16 as libc::c_int) << 4 as libc::c_int)
                            + (0 as libc::c_int
                                + 2 as libc::c_int
                                    * (((1 as libc::c_int) << 4 as libc::c_int)
                                        << 3 as libc::c_int)
                                + ((1 as libc::c_int) << 8 as libc::c_int))
                            + (0 as libc::c_int
                                + 2 as libc::c_int
                                    * (((1 as libc::c_int) << 4 as libc::c_int)
                                        << 3 as libc::c_int)
                                + ((1 as libc::c_int) << 8 as libc::c_int))
                            + ((16 as libc::c_int) << 4 as libc::c_int))
                            as isize,
                    ); /* some internal error */
                    numDirectBits = 4 as libc::c_int as libc::c_uint
                }
                let mut i: libc::c_uint = 1 as libc::c_int as libc::c_uint;
                let mut m: libc::c_uint = 1 as libc::c_int as libc::c_uint;
                loop {
                    ttt = *prob.offset(i as isize) as libc::c_uint;
                    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
                        if buf >= bufLimit {
                            return ELzmaDummy::DUMMY_ERROR;
                        }
                        range <<= 8 as libc::c_int;
                        let fresh60 = buf;
                        buf = buf.offset(1);
                        code = code << 8 as libc::c_int | *fresh60 as libc::c_uint
                    }
                    bound = (range >> 11 as libc::c_int).wrapping_mul(ttt);
                    if code < bound {
                        range = bound;
                        i = i.wrapping_add(m);
                        m = m.wrapping_add(m)
                    } else {
                        range = (range as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                        code = (code as libc::c_uint).wrapping_sub(bound) as UInt32 as UInt32;
                        m = m.wrapping_add(m);
                        i = i.wrapping_add(m)
                    }
                    numDirectBits = numDirectBits.wrapping_sub(1);
                    if !(numDirectBits != 0) {
                        break;
                    }
                }
            }
        }
    }
    if range < (1 as libc::c_int as UInt32) << 24 as libc::c_int {
        if buf >= bufLimit {
            return ELzmaDummy::DUMMY_ERROR;
        }
        range <<= 8 as libc::c_int;
        let fresh61 = buf;
        buf = buf.offset(1);
        code = code << 8 as libc::c_int | *fresh61 as libc::c_uint
    }
    return res;
}
unsafe fn LzmaDec_InitDicAndState(mut p: *mut CLzmaDec, initDic: BoolInt, initState: BoolInt) {
    (*p).remainLen = (2 as libc::c_int
        + ((1 as libc::c_int) << 3 as libc::c_int) * 2 as libc::c_int
        + ((1 as libc::c_int) << 8 as libc::c_int)
        + 1 as libc::c_int) as UInt32;
    (*p).tempBufSize = 0 as libc::c_int as libc::c_uint;
    if initDic != 0 {
        (*p).processedPos = 0 as libc::c_int as UInt32;
        (*p).checkDicSize = 0 as libc::c_int as UInt32;
        (*p).remainLen = (2 as libc::c_int
            + ((1 as libc::c_int) << 3 as libc::c_int) * 2 as libc::c_int
            + ((1 as libc::c_int) << 8 as libc::c_int)
            + 2 as libc::c_int) as UInt32
    }
    if initState != 0 {
        (*p).remainLen = (2 as libc::c_int
            + ((1 as libc::c_int) << 3 as libc::c_int) * 2 as libc::c_int
            + ((1 as libc::c_int) << 8 as libc::c_int)
            + 2 as libc::c_int) as UInt32
    };
}

#[no_mangle]
pub unsafe extern "C" fn LzmaDec_Init(mut p: *mut CLzmaDec) {
    (*p).dicPos = 0 as libc::c_int as SizeT;
    LzmaDec_InitDicAndState(p, 1 as libc::c_int, 1 as libc::c_int);
}

pub unsafe fn LzmaDec_DecodeToDic(
    mut p: *mut CLzmaDec,
    dicLimit: SizeT,
    mut src: *const Byte,
    srcLen: *mut SizeT,
    finishMode: ELzmaFinishMode,
    status: *mut ELzmaStatus,
    memlimit: SizeT,
) -> SRes {
    let mut inSize: SizeT = *srcLen;
    *srcLen = 0 as libc::c_int as SizeT;
    *status = ELzmaStatus::LZMA_STATUS_NOT_SPECIFIED;
    if (*p).remainLen
        > (2 as libc::c_int
            + ((1 as libc::c_int) << 3 as libc::c_int) * 2 as libc::c_int
            + ((1 as libc::c_int) << 8 as libc::c_int)) as libc::c_uint
    {
        while inSize > 0 && (*p).tempBufSize < 5 {
            let fresh62 = src;
            src = src.offset(1);
            let fresh63 = (*p).tempBufSize;
            (*p).tempBufSize = (*p).tempBufSize.wrapping_add(1);
            (*p).tempBuf[fresh63 as usize] = *fresh62;
            *srcLen = (*srcLen).wrapping_add(1);
            inSize = inSize.wrapping_sub(1)
        }
        if (*p).tempBufSize != 0 as libc::c_int as libc::c_uint
            && (*p).tempBuf[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        if (*p).tempBufSize < 5 as libc::c_int as libc::c_uint {
            *status = ELzmaStatus::LZMA_STATUS_NEEDS_MORE_INPUT;
            return 0 as libc::c_int;
        }
        (*p).code = ((*p).tempBuf[1 as libc::c_int as usize] as UInt32) << 24 as libc::c_int
            | ((*p).tempBuf[2 as libc::c_int as usize] as UInt32) << 16 as libc::c_int
            | ((*p).tempBuf[3 as libc::c_int as usize] as UInt32) << 8 as libc::c_int
            | (*p).tempBuf[4 as libc::c_int as usize] as UInt32;
        (*p).range = 0xffffffff as libc::c_uint;
        (*p).tempBufSize = 0 as libc::c_int as libc::c_uint;
        if (*p).remainLen
            > (2 as libc::c_int
                + ((1 as libc::c_int) << 3 as libc::c_int) * 2 as libc::c_int
                + ((1 as libc::c_int) << 8 as libc::c_int)
                + 1 as libc::c_int) as libc::c_uint
        {
            let numProbs: SizeT = ((-(1664 as libc::c_int)
                + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
                + ((16 as libc::c_int) << 4 as libc::c_int)
                + (0 as libc::c_int
                    + 2 as libc::c_int
                        * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                    + ((1 as libc::c_int) << 8 as libc::c_int))
                + (0 as libc::c_int
                    + 2 as libc::c_int
                        * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
                    + ((1 as libc::c_int) << 8 as libc::c_int))
                + ((16 as libc::c_int) << 4 as libc::c_int)
                + ((1 as libc::c_int) << 4 as libc::c_int)
                + 12 as libc::c_int
                + 12 as libc::c_int
                + 12 as libc::c_int
                + 12 as libc::c_int
                + ((4 as libc::c_int) << 6 as libc::c_int)
                + 1664 as libc::c_int) as libc::c_uint)
                .wrapping_add(
                    (0x300 as libc::c_int as UInt32)
                        << (*p).prop.lc as libc::c_int + (*p).prop.lp as libc::c_int,
                ) as SizeT;
            let mut i: SizeT = 0;
            let probs: *mut CLzmaProb = (*p).probs;
            i = 0 as libc::c_int as SizeT;
            while i < numProbs {
                *probs.offset(i as isize) =
                    ((1 as libc::c_int) << 11 as libc::c_int >> 1 as libc::c_int) as CLzmaProb;
                i = i.wrapping_add(1)
            }
            (*p).reps[3 as libc::c_int as usize] = 1 as libc::c_int as UInt32;
            (*p).reps[2 as libc::c_int as usize] = (*p).reps[3 as libc::c_int as usize];
            (*p).reps[1 as libc::c_int as usize] = (*p).reps[2 as libc::c_int as usize];
            (*p).reps[0 as libc::c_int as usize] = (*p).reps[1 as libc::c_int as usize];
            (*p).state = 0 as libc::c_int as UInt32
        }
        (*p).remainLen = 0 as libc::c_int as UInt32
    }
    LzmaDec_WriteRem(p, dicLimit);
    while (*p).remainLen
        != (2 as libc::c_int
            + ((1 as libc::c_int) << 3 as libc::c_int) * 2 as libc::c_int
            + ((1 as libc::c_int) << 8 as libc::c_int)) as libc::c_uint
    {
        let mut checkEndMarkNow: libc::c_int = 0 as libc::c_int;
        if (*p).dicPos >= dicLimit {
            if (*p).remainLen == 0 as libc::c_int as libc::c_uint
                && (*p).code == 0 as libc::c_int as libc::c_uint
            {
                *status = ELzmaStatus::LZMA_STATUS_MAYBE_FINISHED_WITHOUT_MARK;
                return 0 as libc::c_int;
            }
            if finishMode as libc::c_uint
                == ELzmaFinishMode::LZMA_FINISH_ANY as libc::c_int as libc::c_uint
            {
                *status = ELzmaStatus::LZMA_STATUS_NOT_FINISHED;
                return 0 as libc::c_int;
            }
            if (*p).remainLen != 0 as libc::c_int as libc::c_uint {
                *status = ELzmaStatus::LZMA_STATUS_NOT_FINISHED;
                return 1 as libc::c_int;
            }
            checkEndMarkNow = 1 as libc::c_int
        }
        if (*p).tempBufSize == 0 as libc::c_int as libc::c_uint {
            let mut processed: SizeT = 0;
            let mut bufLimit: *const Byte = 0 as *const Byte;
            if inSize < 20 || checkEndMarkNow != 0 {
                let dummyRes: libc::c_int = LzmaDec_TryDummy(p, src, inSize) as libc::c_int;
                if dummyRes == ELzmaDummy::DUMMY_ERROR as libc::c_int {
                    memcpy(
                        (*p).tempBuf.as_mut_ptr() as *mut libc::c_void,
                        src as *const libc::c_void,
                        inSize,
                    );
                    (*p).tempBufSize = inSize as libc::c_uint;
                    *srcLen = (*srcLen).wrapping_add(inSize);
                    *status = ELzmaStatus::LZMA_STATUS_NEEDS_MORE_INPUT;
                    return 0 as libc::c_int;
                }
                if checkEndMarkNow != 0 && dummyRes != ELzmaDummy::DUMMY_MATCH as libc::c_int {
                    *status = ELzmaStatus::LZMA_STATUS_NOT_FINISHED;
                    return 1 as libc::c_int;
                }
                bufLimit = src
            } else {
                bufLimit = src
                    .offset(inSize as isize)
                    .offset(-(20 as libc::c_int as isize))
            }
            (*p).buf = src;
            if LzmaDec_DecodeReal2(p, dicLimit, bufLimit, memlimit) != 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            processed = (*p).buf.wrapping_offset_from(src) as libc::c_long as SizeT;
            *srcLen = (*srcLen).wrapping_add(processed);
            src = src.offset(processed as isize);
            inSize = (inSize).wrapping_sub(processed)
        } else {
            let mut rem: libc::c_uint = (*p).tempBufSize;
            let mut lookAhead: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while rem < 20 && (lookAhead as SizeT) < inSize {
                let fresh64 = lookAhead;
                lookAhead = lookAhead.wrapping_add(1);
                let fresh65 = rem;
                rem = rem.wrapping_add(1);
                (*p).tempBuf[fresh65 as usize] = *src.offset(fresh64 as isize)
            }
            (*p).tempBufSize = rem;
            if rem < 20 as libc::c_int as libc::c_uint || checkEndMarkNow != 0 {
                let dummyRes_0: libc::c_int =
                    LzmaDec_TryDummy(p, (*p).tempBuf.as_mut_ptr(), rem as SizeT) as libc::c_int;
                if dummyRes_0 == ELzmaDummy::DUMMY_ERROR as libc::c_int {
                    *srcLen = (*srcLen).wrapping_add(lookAhead as SizeT);
                    *status = ELzmaStatus::LZMA_STATUS_NEEDS_MORE_INPUT;
                    return 0 as libc::c_int;
                }
                if checkEndMarkNow != 0 && dummyRes_0 != ELzmaDummy::DUMMY_MATCH as libc::c_int {
                    *status = ELzmaStatus::LZMA_STATUS_NOT_FINISHED;
                    return 1 as libc::c_int;
                }
            }
            (*p).buf = (*p).tempBuf.as_mut_ptr();
            if LzmaDec_DecodeReal2(p, dicLimit, (*p).buf, memlimit) != 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            let kkk: libc::c_uint = (*p).buf.wrapping_offset_from((*p).tempBuf.as_mut_ptr())
                as libc::c_long as libc::c_uint;
            if rem < kkk {
                return 11 as libc::c_int;
            }
            rem = rem.wrapping_sub(kkk);
            if lookAhead < rem {
                return 11 as libc::c_int;
            }
            lookAhead = lookAhead.wrapping_sub(rem);
            *srcLen = (*srcLen).wrapping_add(lookAhead as SizeT);
            src = src.offset(lookAhead as isize);
            inSize = inSize.wrapping_sub(lookAhead as SizeT);
            (*p).tempBufSize = 0 as libc::c_int as libc::c_uint
        }
    }
    if (*p).code != 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    *status = ELzmaStatus::LZMA_STATUS_FINISHED_WITH_MARK;
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn LzmaDec_DecodeToBuf(
    mut p: *mut CLzmaDec,
    mut dest: *mut Byte,
    destLen: *mut SizeT,
    mut src: *const Byte,
    srcLen: *mut SizeT,
    finishMode: ELzmaFinishMode,
    status: *mut ELzmaStatus,
    memlimit: SizeT,
) -> SRes {
    let mut outSize: SizeT = *destLen;
    let mut inSize: SizeT = *srcLen;
    *destLen = 0 as libc::c_int as SizeT;
    *srcLen = *destLen;
    loop {
        let mut inSizeCur: SizeT = inSize;
        let mut outSizeCur: SizeT = 0;
        let mut dicPos: SizeT = 0;
        let mut curFinishMode = ELzmaFinishMode::LZMA_FINISH_ANY;
        let mut res: SRes = 0;
        if (*p).dicPos == (*p).dicBufSize {
            if (*p).dicBufSize < (*p).prop.dicSize as SizeT {
                if (*p).dicBufSize < memlimit {
                    (*p).dicBufSize = (*p).dicBufSize << 2 as libc::c_int;
                    if (*p).dicBufSize > memlimit {
                        (*p).dicBufSize = memlimit
                    }
                    if (*p).dicBufSize > (*p).prop.dicSize as SizeT {
                        (*p).dicBufSize = (*p).prop.dicSize as SizeT
                    }
                    let tmp: *mut Byte =
                        realloc((*p).dic as *mut libc::c_void, (*p).dicBufSize) as *mut Byte;
                    if tmp.is_null() {
                        return 2 as libc::c_int;
                    }
                    (*p).dic = tmp
                } else {
                    return 2 as libc::c_int;
                }
            } else {
                (*p).dicPos = 0 as libc::c_int as SizeT
            }
        }
        dicPos = (*p).dicPos;
        if outSize > (*p).dicBufSize.wrapping_sub(dicPos) {
            outSizeCur = (*p).dicBufSize;
            curFinishMode = ELzmaFinishMode::LZMA_FINISH_ANY
        } else {
            outSizeCur = dicPos.wrapping_add(outSize);
            curFinishMode = finishMode
        }
        res = LzmaDec_DecodeToDic(
            p,
            outSizeCur,
            src,
            &mut inSizeCur,
            curFinishMode,
            status,
            memlimit,
        );
        src = src.offset(inSizeCur as isize);
        inSize = (inSize).wrapping_sub(inSizeCur);
        *srcLen = (*srcLen).wrapping_add(inSizeCur);
        outSizeCur = (*p).dicPos.wrapping_sub(dicPos);
        memcpy(
            dest as *mut libc::c_void,
            (*p).dic.offset(dicPos as isize) as *const libc::c_void,
            outSizeCur,
        );
        dest = dest.offset(outSizeCur as isize);
        outSize = (outSize).wrapping_sub(outSizeCur);
        *destLen = (*destLen).wrapping_add(outSizeCur);
        if res != 0 as libc::c_int {
            return res;
        }
        if outSizeCur == 0 || outSize == 0 {
            return 0;
        }
    }
}

pub unsafe fn LzmaDec_FreeProbs(mut p: *mut CLzmaDec, alloc: ISzAllocPtr) {
    (*alloc).Free.expect("non-null function pointer")(alloc, (*p).probs as *mut libc::c_void);
    (*p).probs = 0 as *mut CLzmaProb;
}
unsafe fn LzmaDec_FreeDict(mut p: *mut CLzmaDec, alloc: ISzAllocPtr) {
    (*alloc).Free.expect("non-null function pointer")(alloc, (*p).dic as *mut libc::c_void);
    (*p).dic = 0 as *mut Byte;
}

#[no_mangle]
pub unsafe extern "C" fn LzmaDec_Free(p: *mut CLzmaDec, alloc: ISzAllocPtr) {
    LzmaDec_FreeProbs(p, alloc);
    LzmaDec_FreeDict(p, alloc);
}

pub unsafe fn LzmaProps_Decode(
    mut p: *mut CLzmaProps,
    data: *const Byte,
    size: libc::c_uint,
) -> SRes {
    let mut dicSize: UInt32 = 0;
    let mut d: Byte = 0;
    if size < 5 as libc::c_int as libc::c_uint {
        return 4 as libc::c_int;
    } else {
        dicSize = *data.offset(1 as libc::c_int as isize) as libc::c_uint
            | (*data.offset(2 as libc::c_int as isize) as UInt32) << 8 as libc::c_int
            | (*data.offset(3 as libc::c_int as isize) as UInt32) << 16 as libc::c_int
            | (*data.offset(4 as libc::c_int as isize) as UInt32) << 24 as libc::c_int
    }
    if dicSize < ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint {
        dicSize = ((1 as libc::c_int) << 12 as libc::c_int) as UInt32
    }
    (*p).dicSize = dicSize;
    d = *data.offset(0 as libc::c_int as isize);
    if d as libc::c_int >= 9 as libc::c_int * 5 as libc::c_int * 5 as libc::c_int {
        return 4 as libc::c_int;
    }
    (*p).lc = (d as libc::c_int % 9 as libc::c_int) as Byte;
    d = (d as libc::c_int / 9 as libc::c_int) as Byte;
    (*p).pb = (d as libc::c_int / 5 as libc::c_int) as Byte;
    (*p).lp = (d as libc::c_int % 5 as libc::c_int) as Byte;
    (*p)._pad_ = 0 as libc::c_int as Byte;
    return 0 as libc::c_int;
}
unsafe fn LzmaDec_AllocateProbs2(
    mut p: *mut CLzmaDec,
    propNew: *const CLzmaProps,
    alloc: ISzAllocPtr,
) -> SRes {
    let numProbs: UInt32 = ((-(1664 as libc::c_int)
        + ((1 as libc::c_int) << (14 as libc::c_int >> 1 as libc::c_int))
        + ((16 as libc::c_int) << 4 as libc::c_int)
        + (0 as libc::c_int
            + 2 as libc::c_int * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
            + ((1 as libc::c_int) << 8 as libc::c_int))
        + (0 as libc::c_int
            + 2 as libc::c_int * (((1 as libc::c_int) << 4 as libc::c_int) << 3 as libc::c_int)
            + ((1 as libc::c_int) << 8 as libc::c_int))
        + ((16 as libc::c_int) << 4 as libc::c_int)
        + ((1 as libc::c_int) << 4 as libc::c_int)
        + 12 as libc::c_int
        + 12 as libc::c_int
        + 12 as libc::c_int
        + 12 as libc::c_int
        + ((4 as libc::c_int) << 6 as libc::c_int)
        + 1664 as libc::c_int) as libc::c_uint)
        .wrapping_add(
            (0x300 as libc::c_int as UInt32)
                << (*propNew).lc as libc::c_int + (*propNew).lp as libc::c_int,
        );
    if (*p).probs.is_null() || numProbs != (*p).numProbs {
        LzmaDec_FreeProbs(p, alloc);
        (*p).probs = (*alloc).Alloc.expect("non-null function pointer")(
            alloc,
            (numProbs as SizeT).wrapping_mul(::std::mem::size_of::<CLzmaProb>()),
        ) as *mut CLzmaProb;
        if (*p).probs.is_null() {
            return 2 as libc::c_int;
        }
        (*p).probs_1664 = (*p).probs.offset(1664 as libc::c_int as isize);
        (*p).numProbs = numProbs
    }
    return 0 as libc::c_int;
}

pub unsafe fn LzmaDec_AllocateProbs(
    mut p: *mut CLzmaDec,
    props: *const Byte,
    propsSize: libc::c_uint,
    alloc: ISzAllocPtr,
) -> SRes {
    let mut propNew: CLzmaProps = CLzmaProps {
        lc: 0,
        lp: 0,
        pb: 0,
        _pad_: 0,
        dicSize: 0,
    };
    let mut __result__: libc::c_int = LzmaProps_Decode(&mut propNew, props, propsSize);
    if __result__ != 0 as libc::c_int {
        return __result__;
    }
    let mut __result___0: libc::c_int = LzmaDec_AllocateProbs2(p, &mut propNew, alloc);
    if __result___0 != 0 as libc::c_int {
        return __result___0;
    }
    (*p).prop = propNew;
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn LzmaDec_Allocate(
    mut p: *mut CLzmaDec,
    props: *const Byte,
    propsSize: libc::c_uint,
    alloc: ISzAllocPtr,
) -> SRes {
    let mut propNew: CLzmaProps = CLzmaProps {
        lc: 0,
        lp: 0,
        pb: 0,
        _pad_: 0,
        dicSize: 0,
    };
    let mut dicBufSize: SizeT = 0;
    let mut __result__: libc::c_int = LzmaProps_Decode(&mut propNew, props, propsSize);
    if __result__ != 0 as libc::c_int {
        return __result__;
    }
    let mut __result___0: libc::c_int = LzmaDec_AllocateProbs2(p, &mut propNew, alloc);
    if __result___0 != 0 as libc::c_int {
        return __result___0;
    }
    let dictSize: UInt32 = propNew.dicSize;
    let mut mask: SizeT = ((1 as libc::c_int as UInt32) << 12 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as SizeT;
    if dictSize >= (1 as libc::c_int as UInt32) << 30 as libc::c_int {
        mask = ((1 as libc::c_int as UInt32) << 22 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as SizeT
    } else if dictSize >= (1 as libc::c_int as UInt32) << 22 as libc::c_int {
        mask = ((1 as libc::c_int as UInt32) << 20 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as SizeT
    }
    dicBufSize = (dictSize as SizeT).wrapping_add(mask) & !mask;
    if dicBufSize < dictSize as SizeT {
        dicBufSize = dictSize as SizeT
    }
    if dicBufSize > ((1) << 12) {
        dicBufSize = (1) << 12
    }
    if (*p).dic.is_null() || dicBufSize != (*p).dicBufSize {
        LzmaDec_FreeDict(p, alloc);
        (*p).dic = (*alloc).Alloc.expect("non-null function pointer")(alloc, dicBufSize as SizeT)
            as *mut Byte;
        if (*p).dic.is_null() {
            LzmaDec_FreeProbs(p, alloc);
            return 2 as libc::c_int;
        }
    }
    (*p).dicBufSize = dicBufSize;
    (*p).prop = propNew;
    return 0 as libc::c_int;
}
/* LzmaDec.h -- LZMA Decoder
2018-04-21 : Igor Pavlov : Public domain */
/* #define _LZMA_PROB32 */
/* _LZMA_PROB32 can increase the speed on some CPUs,
but memory usage for CLzmaDec::probs will be doubled in that case */
/* ---------- LZMA Properties ---------- */
/* LzmaProps_Decode - decodes properties
Returns:
  SZ_OK
  SZ_ERROR_UNSUPPORTED - Unsupported properties
*/
/* ---------- LZMA Decoder state ---------- */
/* LZMA_REQUIRED_INPUT_MAX = number of required input bytes for worst case.
Num bits = log2((2^11 / 31) ^ 22) + 26 < 134 + 26 = 160; */
/* Don't change this structure. ASM code can use it. */
/* There are two types of LZMA streams:
- Stream with end mark. That end mark adds about 6 bytes to compressed size.
- Stream without end mark. You must know exact uncompressed size to decompress such stream. */
/* finish at any point */
/* block must be finished at the end */
/* ELzmaFinishMode has meaning only if the decoding reaches output limit !!!

You must use LZMA_FINISH_END, when you know that current output buffer
covers last bytes of block. In other cases you must use LZMA_FINISH_ANY.

If LZMA decoder sees end marker before reaching output limit, it returns SZ_OK,
and output value of destLen will be less than output buffer size limit.
You can check status result also.

You can use multiple checks to test data integrity after full decompression:
  1) Check Result and "status" variable.
  2) Check that output(destLen) = uncompressedSize, if you know real uncompressedSize.
  3) Check that output(srcLen) = compressedSize, if you know real compressedSize.
     You must use correct finish mode in that case. */
/* use main error code instead */
/* stream was finished with end mark. */
/* stream was not finished */
/* you must provide more input bytes */
/* there is probability that stream was finished without end mark */
/* ELzmaStatus is used only as output value for function call */
/* ---------- Interfaces ---------- */
/* There are 3 levels of interfaces:
  1) Dictionary Interface
  2) Buffer Interface
  3) One Call Interface
You can select any of these interfaces, but don't mix functions from different
groups for same object. */
/* There are two variants to allocate state for Dictionary Interface:
     1) LzmaDec_Allocate / LzmaDec_Free
     2) LzmaDec_AllocateProbs / LzmaDec_FreeProbs
   You can use variant 2, if you set dictionary buffer manually.
   For Buffer Interface you must always use variant 1.

LzmaDec_Allocate* can return:
  SZ_OK
  SZ_ERROR_MEM         - Memory allocation error
  SZ_ERROR_UNSUPPORTED - Unsupported properties
*/
/* ---------- Dictionary Interface ---------- */
/* You can use it, if you want to eliminate the overhead for data copying from
   dictionary to some other external buffer.
   You must work with CLzmaDec variables directly in this interface.

   STEPS:
     LzmaDec_Construct()
     LzmaDec_Allocate()
     for (each new stream)
     {
       LzmaDec_Init()
       while (it needs more decompression)
       {
         LzmaDec_DecodeToDic()
         use data from CLzmaDec::dic and update CLzmaDec::dicPos
       }
     }
     LzmaDec_Free()
*/
/* LzmaDec_DecodeToDic

   The decoding to internal dictionary buffer (CLzmaDec::dic).
   You must manually update CLzmaDec::dicPos, if it reaches CLzmaDec::dicBufSize !!!

finishMode:
  It has meaning only if the decoding reaches output limit (dicLimit).
  LZMA_FINISH_ANY - Decode just dicLimit bytes.
  LZMA_FINISH_END - Stream must be finished after dicLimit.

Returns:
  SZ_OK
    status:
      LZMA_STATUS_FINISHED_WITH_MARK
      LZMA_STATUS_NOT_FINISHED
      LZMA_STATUS_NEEDS_MORE_INPUT
      LZMA_STATUS_MAYBE_FINISHED_WITHOUT_MARK
  SZ_ERROR_DATA - Data error
*/
/* ---------- Buffer Interface ---------- */
/* It's zlib-like interface.
   See LzmaDec_DecodeToDic description for information about STEPS and return results,
   but you must use LzmaDec_DecodeToBuf instead of LzmaDec_DecodeToDic and you don't need
   to work with CLzmaDec variables manually.

finishMode:
  It has meaning only if the decoding reaches output limit (*destLen).
  LZMA_FINISH_ANY - Decode just destLen bytes.
  LZMA_FINISH_END - Stream must be finished after (*destLen).
*/
/* ---------- One Call Interface ---------- */
/* LzmaDecode

finishMode:
  It has meaning only if the decoding reaches output limit (*destLen).
  LZMA_FINISH_ANY - Decode just destLen bytes.
  LZMA_FINISH_END - Stream must be finished after (*destLen).

Returns:
  SZ_OK
    status:
      LZMA_STATUS_FINISHED_WITH_MARK
      LZMA_STATUS_NOT_FINISHED
      LZMA_STATUS_MAYBE_FINISHED_WITHOUT_MARK
  SZ_ERROR_DATA - Data error
  SZ_ERROR_MEM  - Memory allocation error
  SZ_ERROR_UNSUPPORTED - Unsupported properties
  SZ_ERROR_INPUT_EOF - It needs more bytes in input buffer (src).
*/

pub unsafe fn LzmaDecode(
    dest: *mut Byte,
    destLen: *mut SizeT,
    src: *const Byte,
    srcLen: *mut SizeT,
    propData: *const Byte,
    propSize: libc::c_uint,
    finishMode: ELzmaFinishMode,
    status: *mut ELzmaStatus,
    alloc: ISzAllocPtr,
) -> SRes {
    let mut p: CLzmaDec = CLzmaDec {
        prop: CLzmaProps {
            lc: 0,
            lp: 0,
            pb: 0,
            _pad_: 0,
            dicSize: 0,
        },
        probs: 0 as *mut CLzmaProb,
        probs_1664: 0 as *mut CLzmaProb,
        dic: 0 as *mut Byte,
        dicBufSize: 0,
        dicPos: 0,
        buf: 0 as *const Byte,
        range: 0,
        code: 0,
        processedPos: 0,
        checkDicSize: 0,
        reps: [0; 4],
        state: 0,
        remainLen: 0,
        numProbs: 0,
        tempBufSize: 0,
        tempBuf: [0; 20],
    };
    let mut res: SRes = 0;
    let outSize: SizeT = *destLen;
    let inSize: SizeT = *srcLen;
    *srcLen = 0 as libc::c_int as SizeT;
    *destLen = *srcLen;
    *status = ELzmaStatus::LZMA_STATUS_NOT_SPECIFIED;
    if inSize < 5 {
        return 6;
    }
    p.dic = 0 as *mut Byte;
    p.probs = 0 as *mut CLzmaProb;
    let mut __result__: libc::c_int = LzmaDec_AllocateProbs(&mut p, propData, propSize, alloc);
    if __result__ != 0 as libc::c_int {
        return __result__;
    }
    p.dic = dest;
    p.dicBufSize = outSize;
    LzmaDec_Init(&mut p);
    *srcLen = inSize;
    res = LzmaDec_DecodeToDic(
        &mut p,
        outSize,
        src,
        srcLen,
        finishMode,
        status,
        18446744073709551615,
    );
    *destLen = p.dicPos;
    if res == 0 as libc::c_int
        && *status as libc::c_uint
            == ELzmaStatus::LZMA_STATUS_NEEDS_MORE_INPUT as libc::c_int as libc::c_uint
    {
        res = 6 as libc::c_int
    }
    LzmaDec_FreeProbs(&mut p, alloc);
    return res;
}
