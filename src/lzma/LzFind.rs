use ::libc;
extern "C" {
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type Byte = libc::c_uchar;
pub type SRes = libc::c_int;
pub type UInt32 = libc::c_uint;
pub type Int64 = libc::c_longlong;
pub type UInt64 = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISeqInStream {
    pub Read: Option<unsafe extern "C" fn(_: *const ISeqInStream,
                                          _: *mut libc::c_void,
                                          _: *mut size_t) -> SRes>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISzAlloc {
    pub Alloc: Option<unsafe extern "C" fn(_: ISzAllocPtr, _: size_t)
                          -> *mut libc::c_void>,
    pub Free: Option<unsafe extern "C" fn(_: ISzAllocPtr,
                                          _: *mut libc::c_void) -> ()>,
}
pub type ISzAllocPtr = *const ISzAlloc;
pub type CLzRef = UInt32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CMatchFinder {
    pub buffer: *mut Byte,
    pub pos: UInt32,
    pub posLimit: UInt32,
    pub streamPos: UInt32,
    pub lenLimit: UInt32,
    pub cyclicBufferPos: UInt32,
    pub cyclicBufferSize: UInt32,
    pub streamEndWasReached: Byte,
    pub btMode: Byte,
    pub bigHash: Byte,
    pub directInput: Byte,
    pub matchMaxLen: UInt32,
    pub hash: *mut CLzRef,
    pub son: *mut CLzRef,
    pub hashMask: UInt32,
    pub cutValue: UInt32,
    pub bufferBase: *mut Byte,
    pub stream: *mut ISeqInStream,
    pub blockSize: UInt32,
    pub keepSizeBefore: UInt32,
    pub keepSizeAfter: UInt32,
    pub numHashBytes: UInt32,
    pub directInputRem: size_t,
    pub historySize: UInt32,
    pub fixedHashSize: UInt32,
    pub hashSizeSum: UInt32,
    pub result: SRes,
    pub crc: [UInt32; 256],
    pub numRefs: size_t,
    pub expectedDataSize: UInt64,
}
pub type CMatchFinder = _CMatchFinder;
pub type Mf_Init_Func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type Mf_GetNumAvailableBytes_Func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> UInt32>;
pub type Mf_GetPointerToCurrentPos_Func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *const Byte>;
pub type Mf_GetMatches_Func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut UInt32)
               -> UInt32>;
pub type Mf_Skip_Func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IMatchFinder {
    pub Init: Mf_Init_Func,
    pub GetNumAvailableBytes: Mf_GetNumAvailableBytes_Func,
    pub GetPointerToCurrentPos: Mf_GetPointerToCurrentPos_Func,
    pub GetMatches: Mf_GetMatches_Func,
    pub Skip: Mf_Skip_Func,
}
pub type IMatchFinder = _IMatchFinder;
unsafe extern "C" fn LzInWindow_Free(mut p: *mut CMatchFinder,
                                     mut alloc: ISzAllocPtr) {
    if (*p).directInput == 0 {
        (*alloc).Free.expect("non-null function pointer")(alloc,
                                                          (*p).bufferBase as
                                                              *mut libc::c_void);
        (*p).bufferBase = 0 as *mut Byte
    };
}
/* keepSizeBefore + keepSizeAfter + keepSizeReserv must be < 4G) */
unsafe extern "C" fn LzInWindow_Create(mut p: *mut CMatchFinder,
                                       mut keepSizeReserv: UInt32,
                                       mut alloc: ISzAllocPtr)
 -> libc::c_int {
    let mut blockSize: UInt32 =
        (*p).keepSizeBefore.wrapping_add((*p).keepSizeAfter).wrapping_add(keepSizeReserv);
    if (*p).directInput != 0 {
        (*p).blockSize = blockSize;
        return 1 as libc::c_int
    }
    if (*p).bufferBase.is_null() || (*p).blockSize != blockSize {
        LzInWindow_Free(p, alloc);
        (*p).blockSize = blockSize;
        (*p).bufferBase =
            (*alloc).Alloc.expect("non-null function pointer")(alloc,
                                                               blockSize as
                                                                   size_t) as
                *mut Byte
    }
    return ((*p).bufferBase != 0 as *mut libc::c_void as *mut Byte) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_GetPointerToCurrentPos(mut p:
                                                                *mut CMatchFinder)
 -> *mut Byte {
    return (*p).buffer;
}
unsafe extern "C" fn MatchFinder_GetNumAvailableBytes(mut p:
                                                          *mut CMatchFinder)
 -> UInt32 {
    return (*p).streamPos.wrapping_sub((*p).pos);
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_ReduceOffsets(mut p: *mut CMatchFinder,
                                                   mut subValue: UInt32) {
    (*p).posLimit =
        ((*p).posLimit as libc::c_uint).wrapping_sub(subValue) as UInt32 as
            UInt32;
    (*p).pos =
        ((*p).pos as libc::c_uint).wrapping_sub(subValue) as UInt32 as UInt32;
    (*p).streamPos =
        ((*p).streamPos as libc::c_uint).wrapping_sub(subValue) as UInt32 as
            UInt32;
}
unsafe extern "C" fn MatchFinder_ReadBlock(mut p: *mut CMatchFinder) {
    if (*p).streamEndWasReached as libc::c_int != 0 ||
           (*p).result != 0 as libc::c_int {
        return
    }
    /* We use (p->streamPos - p->pos) value. (p->streamPos < p->pos) is allowed. */
    if (*p).directInput != 0 {
        let mut curSize: UInt32 =
            (0xffffffff as
                 libc::c_uint).wrapping_sub((*p).streamPos.wrapping_sub((*p).pos));
        if curSize as libc::c_ulong > (*p).directInputRem {
            curSize = (*p).directInputRem as UInt32
        }
        (*p).directInputRem =
            ((*p).directInputRem as
                 libc::c_ulong).wrapping_sub(curSize as libc::c_ulong) as
                size_t as size_t;
        (*p).streamPos =
            ((*p).streamPos as libc::c_uint).wrapping_add(curSize) as UInt32
                as UInt32;
        if (*p).directInputRem == 0 as libc::c_int as libc::c_ulong {
            (*p).streamEndWasReached = 1 as libc::c_int as Byte
        }
        return
    }
    loop  {
        let mut dest: *mut Byte =
            (*p).buffer.offset((*p).streamPos.wrapping_sub((*p).pos) as
                                   isize);
        let mut size: size_t =
            (*p).bufferBase.offset((*p).blockSize as
                                       isize).wrapping_offset_from(dest) as
                libc::c_long as size_t;
        if size == 0 as libc::c_int as libc::c_ulong { return }
        (*p).result =
            (*(*p).stream).Read.expect("non-null function pointer")((*p).stream,
                                                                    dest as
                                                                        *mut libc::c_void,
                                                                    &mut size);
        if (*p).result != 0 as libc::c_int { return }
        if size == 0 as libc::c_int as libc::c_ulong {
            (*p).streamEndWasReached = 1 as libc::c_int as Byte;
            return
        }
        (*p).streamPos =
            ((*p).streamPos as libc::c_uint).wrapping_add(size as UInt32) as
                UInt32 as UInt32;
        if (*p).streamPos.wrapping_sub((*p).pos) > (*p).keepSizeAfter {
            return
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_MoveBlock(mut p: *mut CMatchFinder) {
    memmove((*p).bufferBase as *mut libc::c_void,
            (*p).buffer.offset(-((*p).keepSizeBefore as isize)) as
                *const libc::c_void,
            ((*p).streamPos.wrapping_sub((*p).pos) as
                 size_t).wrapping_add((*p).keepSizeBefore as libc::c_ulong));
    (*p).buffer = (*p).bufferBase.offset((*p).keepSizeBefore as isize);
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_NeedMove(mut p: *mut CMatchFinder)
 -> libc::c_int {
    if (*p).directInput != 0 { return 0 as libc::c_int }
    /* if (p->streamEndWasReached) return 0; */
    return ((*p).bufferBase.offset((*p).blockSize as
                                       isize).wrapping_offset_from((*p).buffer)
                as libc::c_long as size_t <=
                (*p).keepSizeAfter as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_ReadIfRequired(mut p:
                                                        *mut CMatchFinder) {
    if (*p).streamEndWasReached != 0 { return }
    if (*p).keepSizeAfter >= (*p).streamPos.wrapping_sub((*p).pos) {
        MatchFinder_ReadBlock(p);
    };
}
unsafe extern "C" fn MatchFinder_CheckAndMoveAndRead(mut p:
                                                         *mut CMatchFinder) {
    if MatchFinder_NeedMove(p) != 0 { MatchFinder_MoveBlock(p); }
    MatchFinder_ReadBlock(p);
}
unsafe extern "C" fn MatchFinder_SetDefaultSettings(mut p:
                                                        *mut CMatchFinder) {
    (*p).cutValue = 32 as libc::c_int as UInt32;
    (*p).btMode = 1 as libc::c_int as Byte;
    (*p).numHashBytes = 4 as libc::c_int as UInt32;
    (*p).bigHash = 0 as libc::c_int as Byte;
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_Construct(mut p: *mut CMatchFinder) {
    let mut i: libc::c_uint = 0;
    (*p).bufferBase = 0 as *mut Byte;
    (*p).directInput = 0 as libc::c_int as Byte;
    (*p).hash = 0 as *mut CLzRef;
    (*p).expectedDataSize = -(1 as libc::c_int) as Int64 as UInt64;
    MatchFinder_SetDefaultSettings(p);
    i = 0 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        let mut r: UInt32 = i;
        let mut j: libc::c_uint = 0;
        j = 0 as libc::c_int as libc::c_uint;
        while j < 8 as libc::c_int as libc::c_uint {
            r =
                r >> 1 as libc::c_int ^
                    0xedb88320 as libc::c_uint &
                        (0 as libc::c_int as
                             UInt32).wrapping_sub(r &
                                                      1 as libc::c_int as
                                                          libc::c_uint);
            j = j.wrapping_add(1)
        }
        (*p).crc[i as usize] = r;
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn MatchFinder_FreeThisClassMemory(mut p: *mut CMatchFinder,
                                                     mut alloc: ISzAllocPtr) {
    (*alloc).Free.expect("non-null function pointer")(alloc,
                                                      (*p).hash as
                                                          *mut libc::c_void);
    (*p).hash = 0 as *mut CLzRef;
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_Free(mut p: *mut CMatchFinder,
                                          mut alloc: ISzAllocPtr) {
    MatchFinder_FreeThisClassMemory(p, alloc);
    LzInWindow_Free(p, alloc);
}
unsafe extern "C" fn AllocRefs(mut num: size_t, mut alloc: ISzAllocPtr)
 -> *mut CLzRef {
    let mut sizeInBytes: size_t =
        num.wrapping_mul(::std::mem::size_of::<CLzRef>() as libc::c_ulong);
    if sizeInBytes.wrapping_div(::std::mem::size_of::<CLzRef>() as
                                    libc::c_ulong) != num {
        return 0 as *mut CLzRef
    }
    return (*alloc).Alloc.expect("non-null function pointer")(alloc,
                                                              sizeInBytes) as
               *mut CLzRef;
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_Create(mut p: *mut CMatchFinder,
                                            mut historySize: UInt32,
                                            mut keepAddBufferBefore: UInt32,
                                            mut matchMaxLen: UInt32,
                                            mut keepAddBufferAfter: UInt32,
                                            mut alloc: ISzAllocPtr)
 -> libc::c_int {
    let mut sizeReserv: UInt32 = 0;
    if historySize > (7 as libc::c_int as UInt32) << 29 as libc::c_int {
        MatchFinder_Free(p, alloc);
        return 0 as libc::c_int
    }
    sizeReserv = historySize >> 1 as libc::c_int;
    if historySize >= (3 as libc::c_int as UInt32) << 30 as libc::c_int {
        sizeReserv = historySize >> 3 as libc::c_int
    } else if historySize >= (2 as libc::c_int as UInt32) << 30 as libc::c_int
     {
        sizeReserv = historySize >> 2 as libc::c_int
    }
    sizeReserv =
        (sizeReserv as
             libc::c_uint).wrapping_add(keepAddBufferBefore.wrapping_add(matchMaxLen).wrapping_add(keepAddBufferAfter).wrapping_div(2
                                                                                                                                        as
                                                                                                                                        libc::c_int
                                                                                                                                        as
                                                                                                                                        libc::c_uint).wrapping_add(((1
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_int)
                                                                                                                                                                        <<
                                                                                                                                                                        19
                                                                                                                                                                            as
                                                                                                                                                                            libc::c_int)
                                                                                                                                                                       as
                                                                                                                                                                       libc::c_uint))
            as UInt32 as UInt32;
    (*p).keepSizeBefore =
        historySize.wrapping_add(keepAddBufferBefore).wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint);
    (*p).keepSizeAfter = matchMaxLen.wrapping_add(keepAddBufferAfter);
    /* we need one additional byte, since we use MoveBlock after pos++ and before dictionary using */
    if LzInWindow_Create(p, sizeReserv, alloc) != 0 {
        let mut newCyclicBufferSize: UInt32 =
            historySize.wrapping_add(1 as libc::c_int as
                                         libc::c_uint); /* don't change it! It's required for Deflate */
        let mut hs: UInt32 = 0;
        (*p).matchMaxLen = matchMaxLen;
        (*p).fixedHashSize = 0 as libc::c_int as UInt32;
        if (*p).numHashBytes == 2 as libc::c_int as libc::c_uint {
            hs =
                (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                    as UInt32
        } else {
            hs = historySize;
            if hs as libc::c_ulonglong > (*p).expectedDataSize {
                hs = (*p).expectedDataSize as UInt32
            }
            if hs != 0 as libc::c_int as libc::c_uint {
                hs = hs.wrapping_sub(1)
            }
            hs |= hs >> 1 as libc::c_int;
            hs |= hs >> 2 as libc::c_int;
            hs |= hs >> 4 as libc::c_int;
            hs |= hs >> 8 as libc::c_int;
            hs >>= 1 as libc::c_int;
            hs |= 0xffff as libc::c_int as libc::c_uint;
            if hs > ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint
               {
                if (*p).numHashBytes == 3 as libc::c_int as libc::c_uint {
                    hs =
                        (((1 as libc::c_int) << 24 as libc::c_int) -
                             1 as libc::c_int) as UInt32
                } else { hs >>= 1 as libc::c_int }
                /* if (bigHash) mode, GetHeads4b() in LzFindMt.c needs (hs >= ((1 << 24) - 1))) */
            }
        }
        (*p).hashMask = hs;
        hs = hs.wrapping_add(1);
        if (*p).numHashBytes > 2 as libc::c_int as libc::c_uint {
            (*p).fixedHashSize =
                ((*p).fixedHashSize as
                     libc::c_uint).wrapping_add(((1 as libc::c_int) <<
                                                     10 as libc::c_int) as
                                                    libc::c_uint) as UInt32 as
                    UInt32
        }
        if (*p).numHashBytes > 3 as libc::c_int as libc::c_uint {
            (*p).fixedHashSize =
                ((*p).fixedHashSize as
                     libc::c_uint).wrapping_add(((1 as libc::c_int) <<
                                                     16 as libc::c_int) as
                                                    libc::c_uint) as UInt32 as
                    UInt32
        }
        if (*p).numHashBytes > 4 as libc::c_int as libc::c_uint {
            (*p).fixedHashSize =
                ((*p).fixedHashSize as
                     libc::c_uint).wrapping_add(((1 as libc::c_int) <<
                                                     20 as libc::c_int) as
                                                    libc::c_uint) as UInt32 as
                    UInt32
        }
        hs =
            (hs as libc::c_uint).wrapping_add((*p).fixedHashSize) as UInt32 as
                UInt32;
        let mut newSize: size_t = 0;
        let mut numSons: size_t = 0;
        (*p).historySize = historySize;
        (*p).hashSizeSum = hs;
        (*p).cyclicBufferSize = newCyclicBufferSize;
        numSons = newCyclicBufferSize as size_t;
        if (*p).btMode != 0 { numSons <<= 1 as libc::c_int }
        newSize = (hs as libc::c_ulong).wrapping_add(numSons);
        if !(*p).hash.is_null() && (*p).numRefs == newSize {
            return 1 as libc::c_int
        }
        MatchFinder_FreeThisClassMemory(p, alloc);
        (*p).numRefs = newSize;
        (*p).hash = AllocRefs(newSize, alloc);
        if !(*p).hash.is_null() {
            (*p).son = (*p).hash.offset((*p).hashSizeSum as isize);
            return 1 as libc::c_int
        }
    }
    MatchFinder_Free(p, alloc);
    return 0 as libc::c_int;
}
unsafe extern "C" fn MatchFinder_SetLimits(mut p: *mut CMatchFinder) {
    let mut limit: UInt32 =
        (0xffffffff as libc::c_uint).wrapping_sub((*p).pos);
    let mut limit2: UInt32 =
        (*p).cyclicBufferSize.wrapping_sub((*p).cyclicBufferPos);
    if limit2 < limit { limit = limit2 }
    limit2 = (*p).streamPos.wrapping_sub((*p).pos);
    if limit2 <= (*p).keepSizeAfter {
        if limit2 > 0 as libc::c_int as libc::c_uint {
            limit2 = 1 as libc::c_int as UInt32
        }
    } else {
        limit2 =
            (limit2 as libc::c_uint).wrapping_sub((*p).keepSizeAfter) as
                UInt32 as UInt32
    }
    if limit2 < limit { limit = limit2 }
    let mut lenLimit: UInt32 = (*p).streamPos.wrapping_sub((*p).pos);
    if lenLimit > (*p).matchMaxLen { lenLimit = (*p).matchMaxLen }
    (*p).lenLimit = lenLimit;
    (*p).posLimit = (*p).pos.wrapping_add(limit);
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_Init_LowHash(mut p: *mut CMatchFinder) {
    let mut i: size_t = 0;
    let mut items: *mut CLzRef = (*p).hash;
    let mut numItems: size_t = (*p).fixedHashSize as size_t;
    i = 0 as libc::c_int as size_t;
    while i < numItems {
        *items.offset(i as isize) = 0 as libc::c_int as CLzRef;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_Init_HighHash(mut p: *mut CMatchFinder) {
    let mut i: size_t = 0;
    let mut items: *mut CLzRef =
        (*p).hash.offset((*p).fixedHashSize as isize);
    let mut numItems: size_t =
        ((*p).hashMask as
             size_t).wrapping_add(1 as libc::c_int as libc::c_ulong);
    i = 0 as libc::c_int as size_t;
    while i < numItems {
        *items.offset(i as isize) = 0 as libc::c_int as CLzRef;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_Init_3(mut p: *mut CMatchFinder,
                                            mut readData: libc::c_int) {
    (*p).cyclicBufferPos = 0 as libc::c_int as UInt32;
    (*p).buffer = (*p).bufferBase;
    (*p).streamPos = (*p).cyclicBufferSize;
    (*p).pos = (*p).streamPos;
    (*p).result = 0 as libc::c_int;
    (*p).streamEndWasReached = 0 as libc::c_int as Byte;
    if readData != 0 { MatchFinder_ReadBlock(p); }
    MatchFinder_SetLimits(p);
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_Init(mut p: *mut CMatchFinder) {
    MatchFinder_Init_HighHash(p);
    MatchFinder_Init_LowHash(p);
    MatchFinder_Init_3(p, 1 as libc::c_int);
}
unsafe extern "C" fn MatchFinder_GetSubValue(mut p: *mut CMatchFinder)
 -> UInt32 {
    return (*p).pos.wrapping_sub((*p).historySize).wrapping_sub(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
               &
               !((((1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as UInt32);
}
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_Normalize3(mut subValue: UInt32,
                                                mut items: *mut CLzRef,
                                                mut numItems: size_t) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < numItems {
        let mut value: UInt32 = *items.offset(i as isize);
        if value <= subValue {
            value = 0 as libc::c_int as UInt32
        } else {
            value =
                (value as libc::c_uint).wrapping_sub(subValue) as UInt32 as
                    UInt32
        }
        *items.offset(i as isize) = value;
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn MatchFinder_Normalize(mut p: *mut CMatchFinder) {
    let mut subValue: UInt32 = MatchFinder_GetSubValue(p);
    MatchFinder_Normalize3(subValue, (*p).hash, (*p).numRefs);
    MatchFinder_ReduceOffsets(p, subValue);
}
unsafe extern "C" fn MatchFinder_CheckLimits(mut p: *mut CMatchFinder) {
    if (*p).pos == 0xffffffff as libc::c_uint { MatchFinder_Normalize(p); }
    if (*p).streamEndWasReached == 0 &&
           (*p).keepSizeAfter == (*p).streamPos.wrapping_sub((*p).pos) {
        MatchFinder_CheckAndMoveAndRead(p);
    }
    if (*p).cyclicBufferPos == (*p).cyclicBufferSize {
        (*p).cyclicBufferPos = 0 as libc::c_int as UInt32
    }
    MatchFinder_SetLimits(p);
}
/*
  (lenLimit > maxLen)
*/
unsafe extern "C" fn Hc_GetMatchesSpec(mut lenLimit: libc::c_uint,
                                       mut curMatch: UInt32, mut pos: UInt32,
                                       mut cur: *const Byte,
                                       mut son: *mut CLzRef,
                                       mut _cyclicBufferPos: UInt32,
                                       mut _cyclicBufferSize: UInt32,
                                       mut cutValue: UInt32,
                                       mut distances: *mut UInt32,
                                       mut maxLen: libc::c_uint)
 -> *mut UInt32 {
    /*
  son[_cyclicBufferPos] = curMatch;
  for (;;)
  {
    UInt32 delta = pos - curMatch;
    if (cutValue-- == 0 || delta >= _cyclicBufferSize)
      return distances;
    {
      const Byte *pb = cur - delta;
      curMatch = son[_cyclicBufferPos - delta + ((delta > _cyclicBufferPos) ? _cyclicBufferSize : 0)];
      if (pb[maxLen] == cur[maxLen] && *pb == *cur)
      {
        UInt32 len = 0;
        while (++len != lenLimit)
          if (pb[len] != cur[len])
            break;
        if (maxLen < len)
        {
          maxLen = len;
          *distances++ = len;
          *distances++ = delta - 1;
          if (len == lenLimit)
            return distances;
        }
      }
    }
  }
  */
    let mut lim: *const Byte = cur.offset(lenLimit as isize);
    *son.offset(_cyclicBufferPos as isize) = curMatch;
    loop  {
        let mut delta: UInt32 = pos.wrapping_sub(curMatch);
        if delta >= _cyclicBufferSize { break ; }
        let mut diff: ptrdiff_t = 0;
        curMatch =
            *son.offset(_cyclicBufferPos.wrapping_sub(delta).wrapping_add((if delta
                                                                                  >
                                                                                  _cyclicBufferPos
                                                                              {
                                                                               _cyclicBufferSize
                                                                           } else {
                                                                               0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint
                                                                           }))
                            as isize);
        diff = 0 as libc::c_int as ptrdiff_t - delta as libc::c_long;
        if *cur.offset(maxLen as isize) as libc::c_int ==
               *cur.offset((maxLen as libc::c_long + diff) as isize) as
                   libc::c_int {
            let mut c: *const Byte = cur;
            while *c as libc::c_int == *c.offset(diff as isize) as libc::c_int
                  {
                c = c.offset(1);
                if c == lim {
                    *distances.offset(0 as libc::c_int as isize) =
                        lim.wrapping_offset_from(cur) as libc::c_long as
                            UInt32;
                    *distances.offset(1 as libc::c_int as isize) =
                        delta.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    return distances.offset(2 as libc::c_int as isize)
                }
            }
            let mut len: libc::c_uint =
                c.wrapping_offset_from(cur) as libc::c_long as libc::c_uint;
            if maxLen < len {
                maxLen = len;
                *distances.offset(0 as libc::c_int as isize) = len;
                *distances.offset(1 as libc::c_int as isize) =
                    delta.wrapping_sub(1 as libc::c_int as libc::c_uint);
                distances = distances.offset(2 as libc::c_int as isize)
            }
        }
        cutValue = cutValue.wrapping_sub(1);
        if !(cutValue != 0) { break ; }
    }
    return distances;
}
#[no_mangle]
pub unsafe extern "C" fn GetMatchesSpec1(mut lenLimit: UInt32,
                                         mut curMatch: UInt32,
                                         mut pos: UInt32,
                                         mut cur: *const Byte,
                                         mut son: *mut CLzRef,
                                         mut _cyclicBufferPos: UInt32,
                                         mut _cyclicBufferSize: UInt32,
                                         mut cutValue: UInt32,
                                         mut distances: *mut UInt32,
                                         mut maxLen: UInt32) -> *mut UInt32 {
    let mut ptr0: *mut CLzRef =
        son.offset(((_cyclicBufferPos as size_t) << 1 as libc::c_int) as
                       isize).offset(1 as libc::c_int as isize);
    let mut ptr1: *mut CLzRef =
        son.offset(((_cyclicBufferPos as size_t) << 1 as libc::c_int) as
                       isize);
    let mut len0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut len1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop  {
        let mut delta: UInt32 = pos.wrapping_sub(curMatch);
        let fresh0 = cutValue;
        cutValue = cutValue.wrapping_sub(1);
        if fresh0 == 0 as libc::c_int as libc::c_uint ||
               delta >= _cyclicBufferSize {
            *ptr1 = 0 as libc::c_int as CLzRef;
            *ptr0 = *ptr1;
            return distances
        }
        let mut pair: *mut CLzRef =
            son.offset(((_cyclicBufferPos.wrapping_sub(delta).wrapping_add((if delta
                                                                                   >
                                                                                   _cyclicBufferPos
                                                                               {
                                                                                _cyclicBufferSize
                                                                            } else {
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint
                                                                            }))
                             as size_t) << 1 as libc::c_int) as isize);
        let mut pb: *const Byte = cur.offset(-(delta as isize));
        let mut len: libc::c_uint = if len0 < len1 { len0 } else { len1 };
        let mut pair0: UInt32 = *pair.offset(0 as libc::c_int as isize);
        if *pb.offset(len as isize) as libc::c_int ==
               *cur.offset(len as isize) as libc::c_int {
            len = len.wrapping_add(1);
            if len != lenLimit &&
                   *pb.offset(len as isize) as libc::c_int ==
                       *cur.offset(len as isize) as libc::c_int {
                loop  {
                    len = len.wrapping_add(1);
                    if !(len != lenLimit) { break ; }
                    if *pb.offset(len as isize) as libc::c_int !=
                           *cur.offset(len as isize) as libc::c_int {
                        break ;
                    }
                }
            }
            if maxLen < len {
                maxLen = len;
                let fresh1 = distances;
                distances = distances.offset(1);
                *fresh1 = len;
                let fresh2 = distances;
                distances = distances.offset(1);
                *fresh2 =
                    delta.wrapping_sub(1 as libc::c_int as libc::c_uint);
                if len == lenLimit {
                    *ptr1 = pair0;
                    *ptr0 = *pair.offset(1 as libc::c_int as isize);
                    return distances
                }
            }
        }
        if (*pb.offset(len as isize) as libc::c_int) <
               *cur.offset(len as isize) as libc::c_int {
            *ptr1 = curMatch;
            ptr1 = pair.offset(1 as libc::c_int as isize);
            curMatch = *ptr1;
            len1 = len
        } else { *ptr0 = curMatch; ptr0 = pair; curMatch = *ptr0; len0 = len }
    };
}
unsafe extern "C" fn SkipMatchesSpec(mut lenLimit: UInt32,
                                     mut curMatch: UInt32, mut pos: UInt32,
                                     mut cur: *const Byte,
                                     mut son: *mut CLzRef,
                                     mut _cyclicBufferPos: UInt32,
                                     mut _cyclicBufferSize: UInt32,
                                     mut cutValue: UInt32) {
    let mut ptr0: *mut CLzRef =
        son.offset(((_cyclicBufferPos as size_t) << 1 as libc::c_int) as
                       isize).offset(1 as libc::c_int as isize);
    let mut ptr1: *mut CLzRef =
        son.offset(((_cyclicBufferPos as size_t) << 1 as libc::c_int) as
                       isize);
    let mut len0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut len1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop  {
        let mut delta: UInt32 = pos.wrapping_sub(curMatch);
        let fresh3 = cutValue;
        cutValue = cutValue.wrapping_sub(1);
        if fresh3 == 0 as libc::c_int as libc::c_uint ||
               delta >= _cyclicBufferSize {
            *ptr1 = 0 as libc::c_int as CLzRef;
            *ptr0 = *ptr1;
            return
        }
        let mut pair: *mut CLzRef =
            son.offset(((_cyclicBufferPos.wrapping_sub(delta).wrapping_add((if delta
                                                                                   >
                                                                                   _cyclicBufferPos
                                                                               {
                                                                                _cyclicBufferSize
                                                                            } else {
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint
                                                                            }))
                             as size_t) << 1 as libc::c_int) as isize);
        let mut pb: *const Byte = cur.offset(-(delta as isize));
        let mut len: libc::c_uint = if len0 < len1 { len0 } else { len1 };
        if *pb.offset(len as isize) as libc::c_int ==
               *cur.offset(len as isize) as libc::c_int {
            loop  {
                len = len.wrapping_add(1);
                if !(len != lenLimit) { break ; }
                if *pb.offset(len as isize) as libc::c_int !=
                       *cur.offset(len as isize) as libc::c_int {
                    break ;
                }
            }
            if len == lenLimit {
                *ptr1 = *pair.offset(0 as libc::c_int as isize);
                *ptr0 = *pair.offset(1 as libc::c_int as isize);
                return
            }
        }
        if (*pb.offset(len as isize) as libc::c_int) <
               *cur.offset(len as isize) as libc::c_int {
            *ptr1 = curMatch;
            ptr1 = pair.offset(1 as libc::c_int as isize);
            curMatch = *ptr1;
            len1 = len
        } else { *ptr0 = curMatch; ptr0 = pair; curMatch = *ptr0; len0 = len }
    };
}
unsafe extern "C" fn MatchFinder_MovePos(mut p: *mut CMatchFinder) {
    (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
    (*p).buffer = (*p).buffer.offset(1);
    (*p).pos = (*p).pos.wrapping_add(1);
    if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); };
}
unsafe extern "C" fn Bt2_MatchFinder_GetMatches(mut p: *mut CMatchFinder,
                                                mut distances: *mut UInt32)
 -> UInt32 {
    let mut offset: libc::c_uint = 0;
    let mut lenLimit: libc::c_uint = 0;
    let mut hv: UInt32 = 0;
    let mut cur: *const Byte = 0 as *const Byte;
    let mut curMatch: UInt32 = 0;
    lenLimit = (*p).lenLimit;
    if lenLimit < 2 as libc::c_int as libc::c_uint {
        MatchFinder_MovePos(p);
        return 0 as libc::c_int as UInt32
    }
    cur = (*p).buffer;
    hv =
        *cur.offset(0 as libc::c_int as isize) as libc::c_uint |
            (*cur.offset(1 as libc::c_int as isize) as UInt32) <<
                8 as libc::c_int;
    curMatch = *(*p).hash.offset(hv as isize);
    *(*p).hash.offset(hv as isize) = (*p).pos;
    offset = 0 as libc::c_int as libc::c_uint;
    offset =
        GetMatchesSpec1(lenLimit, curMatch, (*p).pos, (*p).buffer, (*p).son,
                        (*p).cyclicBufferPos, (*p).cyclicBufferSize,
                        (*p).cutValue, distances.offset(offset as isize),
                        1 as libc::c_int as
                            UInt32).wrapping_offset_from(distances) as
            libc::c_long as libc::c_uint;
    (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
    (*p).buffer = (*p).buffer.offset(1);
    (*p).pos = (*p).pos.wrapping_add(1);
    if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
    return offset;
}
#[no_mangle]
pub unsafe extern "C" fn Bt3Zip_MatchFinder_GetMatches(mut p:
                                                           *mut CMatchFinder,
                                                       mut distances:
                                                           *mut UInt32)
 -> UInt32 {
    let mut offset: libc::c_uint = 0;
    let mut lenLimit: libc::c_uint = 0;
    let mut hv: UInt32 = 0;
    let mut cur: *const Byte = 0 as *const Byte;
    let mut curMatch: UInt32 = 0;
    lenLimit = (*p).lenLimit;
    if lenLimit < 3 as libc::c_int as libc::c_uint {
        MatchFinder_MovePos(p);
        return 0 as libc::c_int as UInt32
    }
    cur = (*p).buffer;
    hv =
        ((*cur.offset(2 as libc::c_int as isize) as libc::c_uint |
              (*cur.offset(0 as libc::c_int as isize) as UInt32) <<
                  8 as libc::c_int) ^
             (*p).crc[*cur.offset(1 as libc::c_int as isize) as usize]) &
            0xffff as libc::c_int as libc::c_uint;
    curMatch = *(*p).hash.offset(hv as isize);
    *(*p).hash.offset(hv as isize) = (*p).pos;
    offset = 0 as libc::c_int as libc::c_uint;
    offset =
        GetMatchesSpec1(lenLimit, curMatch, (*p).pos, (*p).buffer, (*p).son,
                        (*p).cyclicBufferPos, (*p).cyclicBufferSize,
                        (*p).cutValue, distances.offset(offset as isize),
                        2 as libc::c_int as
                            UInt32).wrapping_offset_from(distances) as
            libc::c_long as libc::c_uint;
    (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
    (*p).buffer = (*p).buffer.offset(1);
    (*p).pos = (*p).pos.wrapping_add(1);
    if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
    return offset;
}
unsafe extern "C" fn Bt3_MatchFinder_GetMatches(mut p: *mut CMatchFinder,
                                                mut distances: *mut UInt32)
 -> UInt32 {
    let mut h2: UInt32 = 0;
    let mut d2: UInt32 = 0;
    let mut pos: UInt32 = 0;
    let mut maxLen: libc::c_uint = 0;
    let mut offset: libc::c_uint = 0;
    let mut hash: *mut UInt32 = 0 as *mut UInt32;
    let mut lenLimit: libc::c_uint = 0;
    let mut hv: UInt32 = 0;
    let mut cur: *const Byte = 0 as *const Byte;
    let mut curMatch: UInt32 = 0;
    lenLimit = (*p).lenLimit;
    if lenLimit < 3 as libc::c_int as libc::c_uint {
        MatchFinder_MovePos(p);
        return 0 as libc::c_int as UInt32
    }
    cur = (*p).buffer;
    let mut temp: UInt32 =
        (*p).crc[*cur.offset(0 as libc::c_int as isize) as usize] ^
            *cur.offset(1 as libc::c_int as isize) as libc::c_uint;
    h2 =
        temp &
            (((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int) as
                libc::c_uint;
    hv =
        (temp ^
             (*cur.offset(2 as libc::c_int as isize) as UInt32) <<
                 8 as libc::c_int) & (*p).hashMask;
    hash = (*p).hash;
    pos = (*p).pos;
    d2 = pos.wrapping_sub(*hash.offset(h2 as isize));
    curMatch =
        *hash.offset(((1 as libc::c_int) << 10 as libc::c_int) as
                         isize).offset(hv as isize);
    *hash.offset(h2 as isize) = pos;
    *hash.offset(((1 as libc::c_int) << 10 as libc::c_int) as
                     isize).offset(hv as isize) = pos;
    maxLen = 2 as libc::c_int as libc::c_uint;
    offset = 0 as libc::c_int as libc::c_uint;
    if d2 < (*p).cyclicBufferSize &&
           *cur.offset(-(d2 as isize)) as libc::c_int == *cur as libc::c_int {
        let mut diff: ptrdiff_t =
            0 as libc::c_int as ptrdiff_t - d2 as libc::c_long;
        let mut c: *const Byte = cur.offset(maxLen as isize);
        let mut lim: *const Byte = cur.offset(lenLimit as isize);
        while c != lim {
            if *c.offset(diff as isize) as libc::c_int != *c as libc::c_int {
                break ;
            }
            c = c.offset(1)
        }
        maxLen = c.wrapping_offset_from(cur) as libc::c_long as libc::c_uint;
        *distances.offset(0 as libc::c_int as isize) = maxLen;
        *distances.offset(1 as libc::c_int as isize) =
            d2.wrapping_sub(1 as libc::c_int as libc::c_uint);
        offset = 2 as libc::c_int as libc::c_uint;
        if maxLen == lenLimit {
            SkipMatchesSpec(lenLimit, curMatch, (*p).pos, (*p).buffer,
                            (*p).son, (*p).cyclicBufferPos,
                            (*p).cyclicBufferSize, (*p).cutValue);
            (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
            (*p).buffer = (*p).buffer.offset(1);
            (*p).pos = (*p).pos.wrapping_add(1);
            if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
            return offset
        }
    }
    offset =
        GetMatchesSpec1(lenLimit, curMatch, (*p).pos, (*p).buffer, (*p).son,
                        (*p).cyclicBufferPos, (*p).cyclicBufferSize,
                        (*p).cutValue, distances.offset(offset as isize),
                        maxLen).wrapping_offset_from(distances) as
            libc::c_long as libc::c_uint;
    (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
    (*p).buffer = (*p).buffer.offset(1);
    (*p).pos = (*p).pos.wrapping_add(1);
    if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
    return offset;
}
unsafe extern "C" fn Bt4_MatchFinder_GetMatches(mut p: *mut CMatchFinder,
                                                mut distances: *mut UInt32)
 -> UInt32 {
    let mut h2: UInt32 = 0;
    let mut h3: UInt32 = 0;
    let mut d2: UInt32 = 0;
    let mut d3: UInt32 = 0;
    let mut pos: UInt32 = 0;
    let mut maxLen: libc::c_uint = 0;
    let mut offset: libc::c_uint = 0;
    let mut hash: *mut UInt32 = 0 as *mut UInt32;
    let mut lenLimit: libc::c_uint = 0;
    let mut hv: UInt32 = 0;
    let mut cur: *const Byte = 0 as *const Byte;
    let mut curMatch: UInt32 = 0;
    lenLimit = (*p).lenLimit;
    if lenLimit < 4 as libc::c_int as libc::c_uint {
        MatchFinder_MovePos(p);
        return 0 as libc::c_int as UInt32
    }
    cur = (*p).buffer;
    let mut temp: UInt32 =
        (*p).crc[*cur.offset(0 as libc::c_int as isize) as usize] ^
            *cur.offset(1 as libc::c_int as isize) as libc::c_uint;
    h2 =
        temp &
            (((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int) as
                libc::c_uint;
    temp ^=
        (*cur.offset(2 as libc::c_int as isize) as UInt32) <<
            8 as libc::c_int;
    h3 =
        temp &
            (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int) as
                libc::c_uint;
    hv =
        (temp ^
             (*p).crc[*cur.offset(3 as libc::c_int as isize) as usize] <<
                 5 as libc::c_int) & (*p).hashMask;
    hash = (*p).hash;
    pos = (*p).pos;
    d2 = pos.wrapping_sub(*hash.offset(h2 as isize));
    d3 =
        pos.wrapping_sub(*hash.offset(((1 as libc::c_int) <<
                                           10 as libc::c_int) as
                                          isize).offset(h3 as isize));
    curMatch =
        *hash.offset((((1 as libc::c_int) << 10 as libc::c_int) +
                          ((1 as libc::c_int) << 16 as libc::c_int)) as
                         isize).offset(hv as isize);
    *hash.offset(h2 as isize) = pos;
    *hash.offset(((1 as libc::c_int) << 10 as libc::c_int) as
                     isize).offset(h3 as isize) = pos;
    *hash.offset((((1 as libc::c_int) << 10 as libc::c_int) +
                      ((1 as libc::c_int) << 16 as libc::c_int)) as
                     isize).offset(hv as isize) = pos;
    maxLen = 0 as libc::c_int as libc::c_uint;
    offset = 0 as libc::c_int as libc::c_uint;
    if d2 < (*p).cyclicBufferSize &&
           *cur.offset(-(d2 as isize)) as libc::c_int == *cur as libc::c_int {
        maxLen = 2 as libc::c_int as libc::c_uint;
        *distances.offset(0 as libc::c_int as isize) =
            2 as libc::c_int as UInt32;
        *distances.offset(1 as libc::c_int as isize) =
            d2.wrapping_sub(1 as libc::c_int as libc::c_uint);
        offset = 2 as libc::c_int as libc::c_uint
    }
    if d2 != d3 && d3 < (*p).cyclicBufferSize &&
           *cur.offset(-(d3 as isize)) as libc::c_int == *cur as libc::c_int {
        maxLen = 3 as libc::c_int as libc::c_uint;
        *distances.offset((offset as
                               size_t).wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong) as
                              isize) =
            d3.wrapping_sub(1 as libc::c_int as libc::c_uint);
        offset = offset.wrapping_add(2 as libc::c_int as libc::c_uint);
        d2 = d3
    }
    if offset != 0 as libc::c_int as libc::c_uint {
        let mut diff: ptrdiff_t =
            0 as libc::c_int as ptrdiff_t - d2 as libc::c_long;
        let mut c: *const Byte = cur.offset(maxLen as isize);
        let mut lim: *const Byte = cur.offset(lenLimit as isize);
        while c != lim {
            if *c.offset(diff as isize) as libc::c_int != *c as libc::c_int {
                break ;
            }
            c = c.offset(1)
        }
        maxLen = c.wrapping_offset_from(cur) as libc::c_long as libc::c_uint;
        *distances.offset((offset as
                               size_t).wrapping_sub(2 as libc::c_int as
                                                        libc::c_ulong) as
                              isize) = maxLen;
        if maxLen == lenLimit {
            SkipMatchesSpec(lenLimit, curMatch, (*p).pos, (*p).buffer,
                            (*p).son, (*p).cyclicBufferPos,
                            (*p).cyclicBufferSize, (*p).cutValue);
            (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
            (*p).buffer = (*p).buffer.offset(1);
            (*p).pos = (*p).pos.wrapping_add(1);
            if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
            return offset
        }
    }
    if maxLen < 3 as libc::c_int as libc::c_uint {
        maxLen = 3 as libc::c_int as libc::c_uint
    }
    offset =
        GetMatchesSpec1(lenLimit, curMatch, (*p).pos, (*p).buffer, (*p).son,
                        (*p).cyclicBufferPos, (*p).cyclicBufferSize,
                        (*p).cutValue, distances.offset(offset as isize),
                        maxLen).wrapping_offset_from(distances) as
            libc::c_long as libc::c_uint;
    (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
    (*p).buffer = (*p).buffer.offset(1);
    (*p).pos = (*p).pos.wrapping_add(1);
    if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
    return offset;
}
/*
static UInt32 Bt5_MatchFinder_GetMatches(CMatchFinder *p, UInt32 *distances)
{
  UInt32 h2, h3, h4, d2, d3, d4, maxLen, offset, pos;
  UInt32 *hash;
  GET_MATCHES_HEADER(5)

  HASH5_CALC;

  hash = p->hash;
  pos = p->pos;

  d2 = pos - hash                  [h2];
  d3 = pos - (hash + kFix3HashSize)[h3];
  d4 = pos - (hash + kFix4HashSize)[h4];

  curMatch = (hash + kFix5HashSize)[hv];

  hash                  [h2] = pos;
  (hash + kFix3HashSize)[h3] = pos;
  (hash + kFix4HashSize)[h4] = pos;
  (hash + kFix5HashSize)[hv] = pos;

  maxLen = 0;
  offset = 0;

  if (d2 < p->cyclicBufferSize && *(cur - d2) == *cur)
  {
    distances[0] = maxLen = 2;
    distances[1] = d2 - 1;
    offset = 2;
    if (*(cur - d2 + 2) == cur[2])
      distances[0] = maxLen = 3;
    else if (d3 < p->cyclicBufferSize && *(cur - d3) == *cur)
    {
      distances[2] = maxLen = 3;
      distances[3] = d3 - 1;
      offset = 4;
      d2 = d3;
    }
  }
  else if (d3 < p->cyclicBufferSize && *(cur - d3) == *cur)
  {
    distances[0] = maxLen = 3;
    distances[1] = d3 - 1;
    offset = 2;
    d2 = d3;
  }
  
  if (d2 != d4 && d4 < p->cyclicBufferSize
      && *(cur - d4) == *cur
      && *(cur - d4 + 3) == *(cur + 3))
  {
    maxLen = 4;
    distances[(size_t)offset + 1] = d4 - 1;
    offset += 2;
    d2 = d4;
  }
  
  if (offset != 0)
  {
    UPDATE_maxLen
    distances[(size_t)offset - 2] = maxLen;
    if (maxLen == lenLimit)
    {
      SkipMatchesSpec(lenLimit, curMatch, MF_PARAMS(p));
      MOVE_POS_RET;
    }
  }

  if (maxLen < 4)
    maxLen = 4;
  
  GET_MATCHES_FOOTER(offset, maxLen)
}
*/
unsafe extern "C" fn Hc4_MatchFinder_GetMatches(mut p: *mut CMatchFinder,
                                                mut distances: *mut UInt32)
 -> UInt32 {
    let mut h2: UInt32 = 0;
    let mut h3: UInt32 = 0;
    let mut d2: UInt32 = 0;
    let mut d3: UInt32 = 0;
    let mut pos: UInt32 = 0;
    let mut maxLen: libc::c_uint = 0;
    let mut offset: libc::c_uint = 0;
    let mut hash: *mut UInt32 = 0 as *mut UInt32;
    let mut lenLimit: libc::c_uint = 0;
    let mut hv: UInt32 = 0;
    let mut cur: *const Byte = 0 as *const Byte;
    let mut curMatch: UInt32 = 0;
    lenLimit = (*p).lenLimit;
    if lenLimit < 4 as libc::c_int as libc::c_uint {
        MatchFinder_MovePos(p);
        return 0 as libc::c_int as UInt32
    }
    cur = (*p).buffer;
    let mut temp: UInt32 =
        (*p).crc[*cur.offset(0 as libc::c_int as isize) as usize] ^
            *cur.offset(1 as libc::c_int as isize) as libc::c_uint;
    h2 =
        temp &
            (((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int) as
                libc::c_uint;
    temp ^=
        (*cur.offset(2 as libc::c_int as isize) as UInt32) <<
            8 as libc::c_int;
    h3 =
        temp &
            (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int) as
                libc::c_uint;
    hv =
        (temp ^
             (*p).crc[*cur.offset(3 as libc::c_int as isize) as usize] <<
                 5 as libc::c_int) & (*p).hashMask;
    hash = (*p).hash;
    pos = (*p).pos;
    d2 = pos.wrapping_sub(*hash.offset(h2 as isize));
    d3 =
        pos.wrapping_sub(*hash.offset(((1 as libc::c_int) <<
                                           10 as libc::c_int) as
                                          isize).offset(h3 as isize));
    curMatch =
        *hash.offset((((1 as libc::c_int) << 10 as libc::c_int) +
                          ((1 as libc::c_int) << 16 as libc::c_int)) as
                         isize).offset(hv as isize);
    *hash.offset(h2 as isize) = pos;
    *hash.offset(((1 as libc::c_int) << 10 as libc::c_int) as
                     isize).offset(h3 as isize) = pos;
    *hash.offset((((1 as libc::c_int) << 10 as libc::c_int) +
                      ((1 as libc::c_int) << 16 as libc::c_int)) as
                     isize).offset(hv as isize) = pos;
    maxLen = 0 as libc::c_int as libc::c_uint;
    offset = 0 as libc::c_int as libc::c_uint;
    if d2 < (*p).cyclicBufferSize &&
           *cur.offset(-(d2 as isize)) as libc::c_int == *cur as libc::c_int {
        maxLen = 2 as libc::c_int as libc::c_uint;
        *distances.offset(0 as libc::c_int as isize) =
            2 as libc::c_int as UInt32;
        *distances.offset(1 as libc::c_int as isize) =
            d2.wrapping_sub(1 as libc::c_int as libc::c_uint);
        offset = 2 as libc::c_int as libc::c_uint
    }
    if d2 != d3 && d3 < (*p).cyclicBufferSize &&
           *cur.offset(-(d3 as isize)) as libc::c_int == *cur as libc::c_int {
        maxLen = 3 as libc::c_int as libc::c_uint;
        *distances.offset((offset as
                               size_t).wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong) as
                              isize) =
            d3.wrapping_sub(1 as libc::c_int as libc::c_uint);
        offset = offset.wrapping_add(2 as libc::c_int as libc::c_uint);
        d2 = d3
    }
    if offset != 0 as libc::c_int as libc::c_uint {
        let mut diff: ptrdiff_t =
            0 as libc::c_int as ptrdiff_t - d2 as libc::c_long;
        let mut c: *const Byte = cur.offset(maxLen as isize);
        let mut lim: *const Byte = cur.offset(lenLimit as isize);
        while c != lim {
            if *c.offset(diff as isize) as libc::c_int != *c as libc::c_int {
                break ;
            }
            c = c.offset(1)
        }
        maxLen = c.wrapping_offset_from(cur) as libc::c_long as libc::c_uint;
        *distances.offset((offset as
                               size_t).wrapping_sub(2 as libc::c_int as
                                                        libc::c_ulong) as
                              isize) = maxLen;
        if maxLen == lenLimit {
            *(*p).son.offset((*p).cyclicBufferPos as isize) = curMatch;
            (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
            (*p).buffer = (*p).buffer.offset(1);
            (*p).pos = (*p).pos.wrapping_add(1);
            if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
            return offset
        }
    }
    if maxLen < 3 as libc::c_int as libc::c_uint {
        maxLen = 3 as libc::c_int as libc::c_uint
    }
    offset =
        Hc_GetMatchesSpec(lenLimit, curMatch, (*p).pos, (*p).buffer, (*p).son,
                          (*p).cyclicBufferPos, (*p).cyclicBufferSize,
                          (*p).cutValue, distances.offset(offset as isize),
                          maxLen).wrapping_offset_from(distances) as
            libc::c_long as libc::c_uint;
    (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
    (*p).buffer = (*p).buffer.offset(1);
    (*p).pos = (*p).pos.wrapping_add(1);
    if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
    return offset;
}
/*
static UInt32 Hc5_MatchFinder_GetMatches(CMatchFinder *p, UInt32 *distances)
{
  UInt32 h2, h3, h4, d2, d3, d4, maxLen, offset, pos
  UInt32 *hash;
  GET_MATCHES_HEADER(5)

  HASH5_CALC;

  hash = p->hash;
  pos = p->pos;
  
  d2 = pos - hash                  [h2];
  d3 = pos - (hash + kFix3HashSize)[h3];
  d4 = pos - (hash + kFix4HashSize)[h4];

  curMatch = (hash + kFix5HashSize)[hv];

  hash                  [h2] = pos;
  (hash + kFix3HashSize)[h3] = pos;
  (hash + kFix4HashSize)[h4] = pos;
  (hash + kFix5HashSize)[hv] = pos;

  maxLen = 0;
  offset = 0;

  if (d2 < p->cyclicBufferSize && *(cur - d2) == *cur)
  {
    distances[0] = maxLen = 2;
    distances[1] = d2 - 1;
    offset = 2;
    if (*(cur - d2 + 2) == cur[2])
      distances[0] = maxLen = 3;
    else if (d3 < p->cyclicBufferSize && *(cur - d3) == *cur)
    {
      distances[2] = maxLen = 3;
      distances[3] = d3 - 1;
      offset = 4;
      d2 = d3;
    }
  }
  else if (d3 < p->cyclicBufferSize && *(cur - d3) == *cur)
  {
    distances[0] = maxLen = 3;
    distances[1] = d3 - 1;
    offset = 2;
    d2 = d3;
  }
  
  if (d2 != d4 && d4 < p->cyclicBufferSize
      && *(cur - d4) == *cur
      && *(cur - d4 + 3) == *(cur + 3))
  {
    maxLen = 4;
    distances[(size_t)offset + 1] = d4 - 1;
    offset += 2;
    d2 = d4;
  }
  
  if (offset != 0)
  {
    UPDATE_maxLen
    distances[(size_t)offset - 2] = maxLen;
    if (maxLen == lenLimit)
    {
      p->son[p->cyclicBufferPos] = curMatch;
      MOVE_POS_RET;
    }
  }
  
  if (maxLen < 4)
    maxLen = 4;

  offset = (UInt32)(Hc_GetMatchesSpec(lenLimit, curMatch, MF_PARAMS(p),
      distances + offset, maxLen) - (distances));
  MOVE_POS_RET
}
*/
#[no_mangle]
pub unsafe extern "C" fn Hc3Zip_MatchFinder_GetMatches(mut p:
                                                           *mut CMatchFinder,
                                                       mut distances:
                                                           *mut UInt32)
 -> UInt32 {
    let mut offset: libc::c_uint = 0;
    let mut lenLimit: libc::c_uint = 0;
    let mut hv: UInt32 = 0;
    let mut cur: *const Byte = 0 as *const Byte;
    let mut curMatch: UInt32 = 0;
    lenLimit = (*p).lenLimit;
    if lenLimit < 3 as libc::c_int as libc::c_uint {
        MatchFinder_MovePos(p);
        return 0 as libc::c_int as UInt32
    }
    cur = (*p).buffer;
    hv =
        ((*cur.offset(2 as libc::c_int as isize) as libc::c_uint |
              (*cur.offset(0 as libc::c_int as isize) as UInt32) <<
                  8 as libc::c_int) ^
             (*p).crc[*cur.offset(1 as libc::c_int as isize) as usize]) &
            0xffff as libc::c_int as libc::c_uint;
    curMatch = *(*p).hash.offset(hv as isize);
    *(*p).hash.offset(hv as isize) = (*p).pos;
    offset =
        Hc_GetMatchesSpec(lenLimit, curMatch, (*p).pos, (*p).buffer, (*p).son,
                          (*p).cyclicBufferPos, (*p).cyclicBufferSize,
                          (*p).cutValue, distances,
                          2 as libc::c_int as
                              libc::c_uint).wrapping_offset_from(distances) as
            libc::c_long as libc::c_uint;
    (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
    (*p).buffer = (*p).buffer.offset(1);
    (*p).pos = (*p).pos.wrapping_add(1);
    if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
    return offset;
}
unsafe extern "C" fn Bt2_MatchFinder_Skip(mut p: *mut CMatchFinder,
                                          mut num: UInt32) {
    loop  {
        let mut lenLimit: libc::c_uint = 0;
        let mut hv: UInt32 = 0;
        let mut cur: *const Byte = 0 as *const Byte;
        let mut curMatch: UInt32 = 0;
        lenLimit = (*p).lenLimit;
        if lenLimit < 2 as libc::c_int as libc::c_uint {
            MatchFinder_MovePos(p);
        } else {
            cur = (*p).buffer;
            hv =
                *cur.offset(0 as libc::c_int as isize) as libc::c_uint |
                    (*cur.offset(1 as libc::c_int as isize) as UInt32) <<
                        8 as libc::c_int;
            curMatch = *(*p).hash.offset(hv as isize);
            *(*p).hash.offset(hv as isize) = (*p).pos;
            SkipMatchesSpec(lenLimit, curMatch, (*p).pos, (*p).buffer,
                            (*p).son, (*p).cyclicBufferPos,
                            (*p).cyclicBufferSize, (*p).cutValue);
            (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
            (*p).buffer = (*p).buffer.offset(1);
            (*p).pos = (*p).pos.wrapping_add(1);
            if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
        }
        num = num.wrapping_sub(1);
        if !(num != 0 as libc::c_int as libc::c_uint) { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Bt3Zip_MatchFinder_Skip(mut p: *mut CMatchFinder,
                                                 mut num: UInt32) {
    loop  {
        let mut lenLimit: libc::c_uint = 0;
        let mut hv: UInt32 = 0;
        let mut cur: *const Byte = 0 as *const Byte;
        let mut curMatch: UInt32 = 0;
        lenLimit = (*p).lenLimit;
        if lenLimit < 3 as libc::c_int as libc::c_uint {
            MatchFinder_MovePos(p);
        } else {
            cur = (*p).buffer;
            hv =
                ((*cur.offset(2 as libc::c_int as isize) as libc::c_uint |
                      (*cur.offset(0 as libc::c_int as isize) as UInt32) <<
                          8 as libc::c_int) ^
                     (*p).crc[*cur.offset(1 as libc::c_int as isize) as
                                  usize]) &
                    0xffff as libc::c_int as libc::c_uint;
            curMatch = *(*p).hash.offset(hv as isize);
            *(*p).hash.offset(hv as isize) = (*p).pos;
            SkipMatchesSpec(lenLimit, curMatch, (*p).pos, (*p).buffer,
                            (*p).son, (*p).cyclicBufferPos,
                            (*p).cyclicBufferSize, (*p).cutValue);
            (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
            (*p).buffer = (*p).buffer.offset(1);
            (*p).pos = (*p).pos.wrapping_add(1);
            if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
        }
        num = num.wrapping_sub(1);
        if !(num != 0 as libc::c_int as libc::c_uint) { break ; }
    };
}
unsafe extern "C" fn Bt3_MatchFinder_Skip(mut p: *mut CMatchFinder,
                                          mut num: UInt32) {
    loop  {
        let mut h2: UInt32 = 0;
        let mut hash: *mut UInt32 = 0 as *mut UInt32;
        let mut lenLimit: libc::c_uint = 0;
        let mut hv: UInt32 = 0;
        let mut cur: *const Byte = 0 as *const Byte;
        let mut curMatch: UInt32 = 0;
        lenLimit = (*p).lenLimit;
        if lenLimit < 3 as libc::c_int as libc::c_uint {
            MatchFinder_MovePos(p);
        } else {
            cur = (*p).buffer;
            let mut temp: UInt32 =
                (*p).crc[*cur.offset(0 as libc::c_int as isize) as usize] ^
                    *cur.offset(1 as libc::c_int as isize) as libc::c_uint;
            h2 =
                temp &
                    (((1 as libc::c_int) << 10 as libc::c_int) -
                         1 as libc::c_int) as libc::c_uint;
            hv =
                (temp ^
                     (*cur.offset(2 as libc::c_int as isize) as UInt32) <<
                         8 as libc::c_int) & (*p).hashMask;
            hash = (*p).hash;
            curMatch =
                *hash.offset(((1 as libc::c_int) << 10 as libc::c_int) as
                                 isize).offset(hv as isize);
            let ref mut fresh4 =
                *hash.offset(((1 as libc::c_int) << 10 as libc::c_int) as
                                 isize).offset(hv as isize);
            *fresh4 = (*p).pos;
            *hash.offset(h2 as isize) = *fresh4;
            SkipMatchesSpec(lenLimit, curMatch, (*p).pos, (*p).buffer,
                            (*p).son, (*p).cyclicBufferPos,
                            (*p).cyclicBufferSize, (*p).cutValue);
            (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
            (*p).buffer = (*p).buffer.offset(1);
            (*p).pos = (*p).pos.wrapping_add(1);
            if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
        }
        num = num.wrapping_sub(1);
        if !(num != 0 as libc::c_int as libc::c_uint) { break ; }
    };
}
unsafe extern "C" fn Bt4_MatchFinder_Skip(mut p: *mut CMatchFinder,
                                          mut num: UInt32) {
    loop  {
        let mut h2: UInt32 = 0;
        let mut h3: UInt32 = 0;
        let mut hash: *mut UInt32 = 0 as *mut UInt32;
        let mut lenLimit: libc::c_uint = 0;
        let mut hv: UInt32 = 0;
        let mut cur: *const Byte = 0 as *const Byte;
        let mut curMatch: UInt32 = 0;
        lenLimit = (*p).lenLimit;
        if lenLimit < 4 as libc::c_int as libc::c_uint {
            MatchFinder_MovePos(p);
        } else {
            cur = (*p).buffer;
            let mut temp: UInt32 =
                (*p).crc[*cur.offset(0 as libc::c_int as isize) as usize] ^
                    *cur.offset(1 as libc::c_int as isize) as libc::c_uint;
            h2 =
                temp &
                    (((1 as libc::c_int) << 10 as libc::c_int) -
                         1 as libc::c_int) as libc::c_uint;
            temp ^=
                (*cur.offset(2 as libc::c_int as isize) as UInt32) <<
                    8 as libc::c_int;
            h3 =
                temp &
                    (((1 as libc::c_int) << 16 as libc::c_int) -
                         1 as libc::c_int) as libc::c_uint;
            hv =
                (temp ^
                     (*p).crc[*cur.offset(3 as libc::c_int as isize) as usize]
                         << 5 as libc::c_int) & (*p).hashMask;
            hash = (*p).hash;
            curMatch =
                *hash.offset((((1 as libc::c_int) << 10 as libc::c_int) +
                                  ((1 as libc::c_int) << 16 as libc::c_int))
                                 as isize).offset(hv as isize);
            let ref mut fresh5 =
                *hash.offset((((1 as libc::c_int) << 10 as libc::c_int) +
                                  ((1 as libc::c_int) << 16 as libc::c_int))
                                 as isize).offset(hv as isize);
            *fresh5 = (*p).pos;
            let ref mut fresh6 =
                *hash.offset(((1 as libc::c_int) << 10 as libc::c_int) as
                                 isize).offset(h3 as isize);
            *fresh6 = *fresh5;
            *hash.offset(h2 as isize) = *fresh6;
            SkipMatchesSpec(lenLimit, curMatch, (*p).pos, (*p).buffer,
                            (*p).son, (*p).cyclicBufferPos,
                            (*p).cyclicBufferSize, (*p).cutValue);
            (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
            (*p).buffer = (*p).buffer.offset(1);
            (*p).pos = (*p).pos.wrapping_add(1);
            if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
        }
        num = num.wrapping_sub(1);
        if !(num != 0 as libc::c_int as libc::c_uint) { break ; }
    };
}
/*
static void Bt5_MatchFinder_Skip(CMatchFinder *p, UInt32 num)
{
  do
  {
    UInt32 h2, h3, h4;
    UInt32 *hash;
    SKIP_HEADER(5)
    HASH5_CALC;
    hash = p->hash;
    curMatch = (hash + kFix5HashSize)[hv];
    hash                  [h2] =
    (hash + kFix3HashSize)[h3] =
    (hash + kFix4HashSize)[h4] =
    (hash + kFix5HashSize)[hv] = p->pos;
    SKIP_FOOTER
  }
  while (--num != 0);
}
*/
unsafe extern "C" fn Hc4_MatchFinder_Skip(mut p: *mut CMatchFinder,
                                          mut num: UInt32) {
    loop  {
        let mut h2: UInt32 = 0;
        let mut h3: UInt32 = 0;
        let mut hash: *mut UInt32 = 0 as *mut UInt32;
        let mut lenLimit: libc::c_uint = 0;
        let mut hv: UInt32 = 0;
        let mut cur: *const Byte = 0 as *const Byte;
        let mut curMatch: UInt32 = 0;
        lenLimit = (*p).lenLimit;
        if lenLimit < 4 as libc::c_int as libc::c_uint {
            MatchFinder_MovePos(p);
        } else {
            cur = (*p).buffer;
            let mut temp: UInt32 =
                (*p).crc[*cur.offset(0 as libc::c_int as isize) as usize] ^
                    *cur.offset(1 as libc::c_int as isize) as libc::c_uint;
            h2 =
                temp &
                    (((1 as libc::c_int) << 10 as libc::c_int) -
                         1 as libc::c_int) as libc::c_uint;
            temp ^=
                (*cur.offset(2 as libc::c_int as isize) as UInt32) <<
                    8 as libc::c_int;
            h3 =
                temp &
                    (((1 as libc::c_int) << 16 as libc::c_int) -
                         1 as libc::c_int) as libc::c_uint;
            hv =
                (temp ^
                     (*p).crc[*cur.offset(3 as libc::c_int as isize) as usize]
                         << 5 as libc::c_int) & (*p).hashMask;
            hash = (*p).hash;
            curMatch =
                *hash.offset((((1 as libc::c_int) << 10 as libc::c_int) +
                                  ((1 as libc::c_int) << 16 as libc::c_int))
                                 as isize).offset(hv as isize);
            let ref mut fresh7 =
                *hash.offset((((1 as libc::c_int) << 10 as libc::c_int) +
                                  ((1 as libc::c_int) << 16 as libc::c_int))
                                 as isize).offset(hv as isize);
            *fresh7 = (*p).pos;
            let ref mut fresh8 =
                *hash.offset(((1 as libc::c_int) << 10 as libc::c_int) as
                                 isize).offset(h3 as isize);
            *fresh8 = *fresh7;
            *hash.offset(h2 as isize) = *fresh8;
            *(*p).son.offset((*p).cyclicBufferPos as isize) = curMatch;
            (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
            (*p).buffer = (*p).buffer.offset(1);
            (*p).pos = (*p).pos.wrapping_add(1);
            if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
        }
        num = num.wrapping_sub(1);
        if !(num != 0 as libc::c_int as libc::c_uint) { break ; }
    };
}
/*
static void Hc5_MatchFinder_Skip(CMatchFinder *p, UInt32 num)
{
  do
  {
    UInt32 h2, h3, h4;
    UInt32 *hash;
    SKIP_HEADER(5)
    HASH5_CALC;
    hash = p->hash;
    curMatch = hash + kFix5HashSize)[hv];
    hash                  [h2] =
    (hash + kFix3HashSize)[h3] =
    (hash + kFix4HashSize)[h4] =
    (hash + kFix5HashSize)[hv] = p->pos;
    p->son[p->cyclicBufferPos] = curMatch;
    MOVE_POS
  }
  while (--num != 0);
}
*/
#[no_mangle]
pub unsafe extern "C" fn Hc3Zip_MatchFinder_Skip(mut p: *mut CMatchFinder,
                                                 mut num: UInt32) {
    loop  {
        let mut lenLimit: libc::c_uint = 0;
        let mut hv: UInt32 = 0;
        let mut cur: *const Byte = 0 as *const Byte;
        let mut curMatch: UInt32 = 0;
        lenLimit = (*p).lenLimit;
        if lenLimit < 3 as libc::c_int as libc::c_uint {
            MatchFinder_MovePos(p);
        } else {
            cur = (*p).buffer;
            hv =
                ((*cur.offset(2 as libc::c_int as isize) as libc::c_uint |
                      (*cur.offset(0 as libc::c_int as isize) as UInt32) <<
                          8 as libc::c_int) ^
                     (*p).crc[*cur.offset(1 as libc::c_int as isize) as
                                  usize]) &
                    0xffff as libc::c_int as libc::c_uint;
            curMatch = *(*p).hash.offset(hv as isize);
            *(*p).hash.offset(hv as isize) = (*p).pos;
            *(*p).son.offset((*p).cyclicBufferPos as isize) = curMatch;
            (*p).cyclicBufferPos = (*p).cyclicBufferPos.wrapping_add(1);
            (*p).buffer = (*p).buffer.offset(1);
            (*p).pos = (*p).pos.wrapping_add(1);
            if (*p).pos == (*p).posLimit { MatchFinder_CheckLimits(p); }
        }
        num = num.wrapping_sub(1);
        if !(num != 0 as libc::c_int as libc::c_uint) { break ; }
    };
}
/* LzFind.h -- Match finder for LZ algorithms
2017-06-10 : Igor Pavlov : Public domain */
/* it must be = (historySize + 1) */
/* Conditions:
     historySize <= 3 GB
     keepAddBufferBefore + matchMaxLen + keepAddBufferAfter < 511MB
*/
/*
Conditions:
  Mf_GetNumAvailableBytes_Func must be called before each Mf_GetMatchLen_Func.
  Mf_GetPointerToCurrentPos_Func's result must be used only before any other function
*/
#[no_mangle]
pub unsafe extern "C" fn MatchFinder_CreateVTable(mut p: *mut CMatchFinder,
                                                  mut vTable:
                                                      *mut IMatchFinder) {
    (*vTable).Init =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut CMatchFinder)
                                           -> ()>,
                                Mf_Init_Func>(Some(MatchFinder_Init as
                                                       unsafe extern "C" fn(_:
                                                                                *mut CMatchFinder)
                                                           -> ()));
    (*vTable).GetNumAvailableBytes =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut CMatchFinder)
                                           -> UInt32>,
                                Mf_GetNumAvailableBytes_Func>(Some(MatchFinder_GetNumAvailableBytes
                                                                       as
                                                                       unsafe extern "C" fn(_:
                                                                                                *mut CMatchFinder)
                                                                           ->
                                                                               UInt32));
    (*vTable).GetPointerToCurrentPos =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut CMatchFinder)
                                           -> *mut Byte>,
                                Mf_GetPointerToCurrentPos_Func>(Some(MatchFinder_GetPointerToCurrentPos
                                                                         as
                                                                         unsafe extern "C" fn(_:
                                                                                                  *mut CMatchFinder)
                                                                             ->
                                                                                 *mut Byte));
    if (*p).btMode == 0 {
        /* if (p->numHashBytes <= 4) */
        (*vTable).GetMatches =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut CMatchFinder,
                                                                _:
                                                                    *mut UInt32)
                                               -> UInt32>,
                                    Mf_GetMatches_Func>(Some(Hc4_MatchFinder_GetMatches
                                                                 as
                                                                 unsafe extern "C" fn(_:
                                                                                          *mut CMatchFinder,
                                                                                      _:
                                                                                          *mut UInt32)
                                                                     ->
                                                                         UInt32));
        (*vTable).Skip =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut CMatchFinder,
                                                                _: UInt32)
                                               -> ()>,
                                    Mf_Skip_Func>(Some(Hc4_MatchFinder_Skip as
                                                           unsafe extern "C" fn(_:
                                                                                    *mut CMatchFinder,
                                                                                _:
                                                                                    UInt32)
                                                               -> ()))
        /*
    else
    {
      vTable->GetMatches = (Mf_GetMatches_Func)Hc5_MatchFinder_GetMatches;
      vTable->Skip = (Mf_Skip_Func)Hc5_MatchFinder_Skip;
    }
    */
    } else if (*p).numHashBytes == 2 as libc::c_int as libc::c_uint {
        (*vTable).GetMatches =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut CMatchFinder,
                                                                _:
                                                                    *mut UInt32)
                                               -> UInt32>,
                                    Mf_GetMatches_Func>(Some(Bt2_MatchFinder_GetMatches
                                                                 as
                                                                 unsafe extern "C" fn(_:
                                                                                          *mut CMatchFinder,
                                                                                      _:
                                                                                          *mut UInt32)
                                                                     ->
                                                                         UInt32));
        (*vTable).Skip =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut CMatchFinder,
                                                                _: UInt32)
                                               -> ()>,
                                    Mf_Skip_Func>(Some(Bt2_MatchFinder_Skip as
                                                           unsafe extern "C" fn(_:
                                                                                    *mut CMatchFinder,
                                                                                _:
                                                                                    UInt32)
                                                               -> ()))
    } else if (*p).numHashBytes == 3 as libc::c_int as libc::c_uint {
        (*vTable).GetMatches =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut CMatchFinder,
                                                                _:
                                                                    *mut UInt32)
                                               -> UInt32>,
                                    Mf_GetMatches_Func>(Some(Bt3_MatchFinder_GetMatches
                                                                 as
                                                                 unsafe extern "C" fn(_:
                                                                                          *mut CMatchFinder,
                                                                                      _:
                                                                                          *mut UInt32)
                                                                     ->
                                                                         UInt32));
        (*vTable).Skip =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut CMatchFinder,
                                                                _: UInt32)
                                               -> ()>,
                                    Mf_Skip_Func>(Some(Bt3_MatchFinder_Skip as
                                                           unsafe extern "C" fn(_:
                                                                                    *mut CMatchFinder,
                                                                                _:
                                                                                    UInt32)
                                                               -> ()))
    } else {
        /* if (p->numHashBytes == 4) */
        (*vTable).GetMatches =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut CMatchFinder,
                                                                _:
                                                                    *mut UInt32)
                                               -> UInt32>,
                                    Mf_GetMatches_Func>(Some(Bt4_MatchFinder_GetMatches
                                                                 as
                                                                 unsafe extern "C" fn(_:
                                                                                          *mut CMatchFinder,
                                                                                      _:
                                                                                          *mut UInt32)
                                                                     ->
                                                                         UInt32));
        (*vTable).Skip =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut CMatchFinder,
                                                                _: UInt32)
                                               -> ()>,
                                    Mf_Skip_Func>(Some(Bt4_MatchFinder_Skip as
                                                           unsafe extern "C" fn(_:
                                                                                    *mut CMatchFinder,
                                                                                _:
                                                                                    UInt32)
                                                               -> ()))
    };
    /*
  else
  {
    vTable->GetMatches = (Mf_GetMatches_Func)Bt5_MatchFinder_GetMatches;
    vTable->Skip = (Mf_Skip_Func)Bt5_MatchFinder_Skip;
  }
  */
}
