/* automatically generated by rust-bindgen */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub const PCAP_VERSION_MAJOR: u32 = 2;
pub const PCAP_VERSION_MINOR: u32 = 4;
pub const PCAP_ERRBUF_SIZE: u32 = 256;
pub const PCAP_IF_LOOPBACK: u32 = 1;
pub const PCAP_IF_UP: u32 = 2;
pub const PCAP_IF_RUNNING: u32 = 4;
pub const PCAP_ERROR: i32 = -1;
pub const PCAP_ERROR_BREAK: i32 = -2;
pub const PCAP_ERROR_NOT_ACTIVATED: i32 = -3;
pub const PCAP_ERROR_ACTIVATED: i32 = -4;
pub const PCAP_ERROR_NO_SUCH_DEVICE: i32 = -5;
pub const PCAP_ERROR_RFMON_NOTSUP: i32 = -6;
pub const PCAP_ERROR_NOT_RFMON: i32 = -7;
pub const PCAP_ERROR_PERM_DENIED: i32 = -8;
pub const PCAP_ERROR_IFACE_NOT_UP: i32 = -9;
pub const PCAP_ERROR_CANTSET_TSTAMP_TYPE: i32 = -10;
pub const PCAP_ERROR_PROMISC_PERM_DENIED: i32 = -11;
pub const PCAP_ERROR_TSTAMP_PRECISION_NOTSUP: i32 = -12;
pub const PCAP_WARNING: u32 = 1;
pub const PCAP_WARNING_PROMISC_NOTSUP: u32 = 2;
pub const PCAP_WARNING_TSTAMP_TYPE_NOTSUP: u32 = 3;
pub const PCAP_NETMASK_UNKNOWN: u32 = 4294967295;
pub const PCAP_TSTAMP_HOST: u32 = 0;
pub const PCAP_TSTAMP_HOST_LOWPREC: u32 = 1;
pub const PCAP_TSTAMP_HOST_HIPREC: u32 = 2;
pub const PCAP_TSTAMP_ADAPTER: u32 = 3;
pub const PCAP_TSTAMP_ADAPTER_UNSYNCED: u32 = 4;
pub const PCAP_TSTAMP_PRECISION_MICRO: u32 = 0;
pub const PCAP_TSTAMP_PRECISION_NANO: u32 = 1;
pub type USHORT = ::std::os::raw::c_ushort;
pub type CHAR = ::std::os::raw::c_char;
pub type HANDLE = *mut ::std::os::raw::c_void;
pub type u_char = ::std::os::raw::c_uchar;
pub type u_short = ::std::os::raw::c_ushort;
pub type u_int = ::std::os::raw::c_uint;
pub type ADDRESS_FAMILY = USHORT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
  pub sa_family: ADDRESS_FAMILY,
  pub sa_data: [CHAR; 14usize],
}
#[test]
fn bindgen_test_layout_sockaddr() {
  assert_eq!(
    ::std::mem::size_of::<sockaddr>(),
    16usize,
    concat!("Size of: ", stringify!(sockaddr))
  );
  assert_eq!(
    ::std::mem::align_of::<sockaddr>(),
    2usize,
    concat!("Alignment of ", stringify!(sockaddr))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_family as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(sockaddr),
      "::",
      stringify!(sa_family)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_data as *const _ as usize },
    2usize,
    concat!(
      "Offset of field: ",
      stringify!(sockaddr),
      "::",
      stringify!(sa_data)
    )
  );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
  pub tv_sec: ::std::os::raw::c_long,
  pub tv_usec: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_timeval() {
  assert_eq!(
    ::std::mem::size_of::<timeval>(),
    8usize,
    concat!("Size of: ", stringify!(timeval))
  );
  assert_eq!(
    ::std::mem::align_of::<timeval>(),
    4usize,
    concat!("Alignment of ", stringify!(timeval))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<timeval>())).tv_sec as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(timeval),
      "::",
      stringify!(tv_sec)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<timeval>())).tv_usec as *const _ as usize },
    4usize,
    concat!(
      "Offset of field: ",
      stringify!(timeval),
      "::",
      stringify!(tv_usec)
    )
  );
}
pub type bpf_int32 = ::std::os::raw::c_int;
pub type bpf_u_int32 = u_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_program {
  pub bf_len: u_int,
  pub bf_insns: *mut bpf_insn,
}
#[test]
fn bindgen_test_layout_bpf_program() {
  assert_eq!(
    ::std::mem::size_of::<bpf_program>(),
    16usize,
    concat!("Size of: ", stringify!(bpf_program))
  );
  assert_eq!(
    ::std::mem::align_of::<bpf_program>(),
    8usize,
    concat!("Alignment of ", stringify!(bpf_program))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<bpf_program>())).bf_len as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(bpf_program),
      "::",
      stringify!(bf_len)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<bpf_program>())).bf_insns as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(bpf_program),
      "::",
      stringify!(bf_insns)
    )
  );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_insn {
  pub code: u_short,
  pub jt: u_char,
  pub jf: u_char,
  pub k: bpf_u_int32,
}
#[test]
fn bindgen_test_layout_bpf_insn() {
  assert_eq!(
    ::std::mem::size_of::<bpf_insn>(),
    8usize,
    concat!("Size of: ", stringify!(bpf_insn))
  );
  assert_eq!(
    ::std::mem::align_of::<bpf_insn>(),
    4usize,
    concat!("Alignment of ", stringify!(bpf_insn))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<bpf_insn>())).code as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(bpf_insn),
      "::",
      stringify!(code)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<bpf_insn>())).jt as *const _ as usize },
    2usize,
    concat!(
      "Offset of field: ",
      stringify!(bpf_insn),
      "::",
      stringify!(jt)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<bpf_insn>())).jf as *const _ as usize },
    3usize,
    concat!(
      "Offset of field: ",
      stringify!(bpf_insn),
      "::",
      stringify!(jf)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<bpf_insn>())).k as *const _ as usize },
    4usize,
    concat!(
      "Offset of field: ",
      stringify!(bpf_insn),
      "::",
      stringify!(k)
    )
  );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _iobuf {
  pub _Placeholder: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__iobuf() {
  assert_eq!(
    ::std::mem::size_of::<_iobuf>(),
    8usize,
    concat!("Size of: ", stringify!(_iobuf))
  );
  assert_eq!(
    ::std::mem::align_of::<_iobuf>(),
    8usize,
    concat!("Alignment of ", stringify!(_iobuf))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_iobuf>()))._Placeholder as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(_iobuf),
      "::",
      stringify!(_Placeholder)
    )
  );
}
pub type FILE = _iobuf;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcap {
  _unused: [u8; 0],
}
pub type pcap_t = pcap;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcap_dumper {
  _unused: [u8; 0],
}
pub type pcap_dumper_t = pcap_dumper;
pub type pcap_if_t = pcap_if;
pub type pcap_addr_t = pcap_addr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcap_file_header {
  pub magic: bpf_u_int32,
  pub version_major: u_short,
  pub version_minor: u_short,
  pub thiszone: bpf_int32,
  pub sigfigs: bpf_u_int32,
  pub snaplen: bpf_u_int32,
  pub linktype: bpf_u_int32,
}
#[test]
fn bindgen_test_layout_pcap_file_header() {
  assert_eq!(
    ::std::mem::size_of::<pcap_file_header>(),
    24usize,
    concat!("Size of: ", stringify!(pcap_file_header))
  );
  assert_eq!(
    ::std::mem::align_of::<pcap_file_header>(),
    4usize,
    concat!("Alignment of ", stringify!(pcap_file_header))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_file_header>())).magic as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_file_header),
      "::",
      stringify!(magic)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_file_header>())).version_major as *const _ as usize },
    4usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_file_header),
      "::",
      stringify!(version_major)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_file_header>())).version_minor as *const _ as usize },
    6usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_file_header),
      "::",
      stringify!(version_minor)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_file_header>())).thiszone as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_file_header),
      "::",
      stringify!(thiszone)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_file_header>())).sigfigs as *const _ as usize },
    12usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_file_header),
      "::",
      stringify!(sigfigs)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_file_header>())).snaplen as *const _ as usize },
    16usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_file_header),
      "::",
      stringify!(snaplen)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_file_header>())).linktype as *const _ as usize },
    20usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_file_header),
      "::",
      stringify!(linktype)
    )
  );
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum pcap_direction_t {
  PCAP_D_INOUT = 0,
  PCAP_D_IN = 1,
  PCAP_D_OUT = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcap_pkthdr {
  pub ts: timeval,
  pub caplen: bpf_u_int32,
  pub len: bpf_u_int32,
}
#[test]
fn bindgen_test_layout_pcap_pkthdr() {
  assert_eq!(
    ::std::mem::size_of::<pcap_pkthdr>(),
    16usize,
    concat!("Size of: ", stringify!(pcap_pkthdr))
  );
  assert_eq!(
    ::std::mem::align_of::<pcap_pkthdr>(),
    4usize,
    concat!("Alignment of ", stringify!(pcap_pkthdr))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_pkthdr>())).ts as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_pkthdr),
      "::",
      stringify!(ts)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_pkthdr>())).caplen as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_pkthdr),
      "::",
      stringify!(caplen)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_pkthdr>())).len as *const _ as usize },
    12usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_pkthdr),
      "::",
      stringify!(len)
    )
  );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcap_stat {
  pub ps_recv: u_int,
  pub ps_drop: u_int,
  pub ps_ifdrop: u_int,
}
#[test]
fn bindgen_test_layout_pcap_stat() {
  assert_eq!(
    ::std::mem::size_of::<pcap_stat>(),
    12usize,
    concat!("Size of: ", stringify!(pcap_stat))
  );
  assert_eq!(
    ::std::mem::align_of::<pcap_stat>(),
    4usize,
    concat!("Alignment of ", stringify!(pcap_stat))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_stat>())).ps_recv as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_stat),
      "::",
      stringify!(ps_recv)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_stat>())).ps_drop as *const _ as usize },
    4usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_stat),
      "::",
      stringify!(ps_drop)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_stat>())).ps_ifdrop as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_stat),
      "::",
      stringify!(ps_ifdrop)
    )
  );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcap_if {
  pub next: *mut pcap_if,
  pub name: *mut ::std::os::raw::c_char,
  pub description: *mut ::std::os::raw::c_char,
  pub addresses: *mut pcap_addr,
  pub flags: bpf_u_int32,
}
#[test]
fn bindgen_test_layout_pcap_if() {
  assert_eq!(
    ::std::mem::size_of::<pcap_if>(),
    40usize,
    concat!("Size of: ", stringify!(pcap_if))
  );
  assert_eq!(
    ::std::mem::align_of::<pcap_if>(),
    8usize,
    concat!("Alignment of ", stringify!(pcap_if))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_if>())).next as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_if),
      "::",
      stringify!(next)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_if>())).name as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_if),
      "::",
      stringify!(name)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_if>())).description as *const _ as usize },
    16usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_if),
      "::",
      stringify!(description)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_if>())).addresses as *const _ as usize },
    24usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_if),
      "::",
      stringify!(addresses)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_if>())).flags as *const _ as usize },
    32usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_if),
      "::",
      stringify!(flags)
    )
  );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcap_addr {
  pub next: *mut pcap_addr,
  pub addr: *mut sockaddr,
  pub netmask: *mut sockaddr,
  pub broadaddr: *mut sockaddr,
  pub dstaddr: *mut sockaddr,
}
#[test]
fn bindgen_test_layout_pcap_addr() {
  assert_eq!(
    ::std::mem::size_of::<pcap_addr>(),
    40usize,
    concat!("Size of: ", stringify!(pcap_addr))
  );
  assert_eq!(
    ::std::mem::align_of::<pcap_addr>(),
    8usize,
    concat!("Alignment of ", stringify!(pcap_addr))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_addr>())).next as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_addr),
      "::",
      stringify!(next)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_addr>())).addr as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_addr),
      "::",
      stringify!(addr)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_addr>())).netmask as *const _ as usize },
    16usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_addr),
      "::",
      stringify!(netmask)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_addr>())).broadaddr as *const _ as usize },
    24usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_addr),
      "::",
      stringify!(broadaddr)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_addr>())).dstaddr as *const _ as usize },
    32usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_addr),
      "::",
      stringify!(dstaddr)
    )
  );
}
pub type pcap_handler = ::std::option::Option<
  unsafe extern "C" fn(arg1: *mut u_char, arg2: *const pcap_pkthdr, arg3: *const u_char),
>;
extern "C" {
  pub fn pcap_lookupdev(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
  pub fn pcap_lookupnet(
    arg1: *const ::std::os::raw::c_char,
    arg2: *mut bpf_u_int32,
    arg3: *mut bpf_u_int32,
    arg4: *mut ::std::os::raw::c_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_create(
    arg1: *const ::std::os::raw::c_char,
    arg2: *mut ::std::os::raw::c_char,
  ) -> *mut pcap_t;
}
extern "C" {
  pub fn pcap_set_snaplen(arg1: *mut pcap_t, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_set_promisc(arg1: *mut pcap_t, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_can_set_rfmon(arg1: *mut pcap_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_set_rfmon(arg1: *mut pcap_t, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_set_timeout(arg1: *mut pcap_t, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_set_tstamp_type(
    arg1: *mut pcap_t,
    arg2: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_set_immediate_mode(
    arg1: *mut pcap_t,
    arg2: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_set_buffer_size(
    arg1: *mut pcap_t,
    arg2: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_set_tstamp_precision(
    arg1: *mut pcap_t,
    arg2: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_get_tstamp_precision(arg1: *mut pcap_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_activate(arg1: *mut pcap_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_list_tstamp_types(
    arg1: *mut pcap_t,
    arg2: *mut *mut ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_free_tstamp_types(arg1: *mut ::std::os::raw::c_int);
}
extern "C" {
  pub fn pcap_tstamp_type_name_to_val(arg1: *const ::std::os::raw::c_char)
    -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_tstamp_type_val_to_name(arg1: ::std::os::raw::c_int)
    -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn pcap_tstamp_type_val_to_description(
    arg1: ::std::os::raw::c_int,
  ) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn pcap_open_live(
    arg1: *const ::std::os::raw::c_char,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
    arg5: *mut ::std::os::raw::c_char,
  ) -> *mut pcap_t;
}
extern "C" {
  pub fn pcap_open_dead(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int) -> *mut pcap_t;
}
extern "C" {
  pub fn pcap_open_dead_with_tstamp_precision(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
    arg3: u_int,
  ) -> *mut pcap_t;
}
extern "C" {
  pub fn pcap_open_offline_with_tstamp_precision(
    arg1: *const ::std::os::raw::c_char,
    arg2: u_int,
    arg3: *mut ::std::os::raw::c_char,
  ) -> *mut pcap_t;
}
extern "C" {
  pub fn pcap_open_offline(
    arg1: *const ::std::os::raw::c_char,
    arg2: *mut ::std::os::raw::c_char,
  ) -> *mut pcap_t;
}
extern "C" {
  pub fn pcap_hopen_offline_with_tstamp_precision(
    arg1: isize,
    arg2: u_int,
    arg3: *mut ::std::os::raw::c_char,
  ) -> *mut pcap_t;
}
extern "C" {
  pub fn pcap_hopen_offline(arg1: isize, arg2: *mut ::std::os::raw::c_char) -> *mut pcap_t;
}
extern "C" {
  pub fn pcap_close(arg1: *mut pcap_t);
}
extern "C" {
  pub fn pcap_loop(
    arg1: *mut pcap_t,
    arg2: ::std::os::raw::c_int,
    arg3: pcap_handler,
    arg4: *mut u_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_dispatch(
    arg1: *mut pcap_t,
    arg2: ::std::os::raw::c_int,
    arg3: pcap_handler,
    arg4: *mut u_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_next(arg1: *mut pcap_t, arg2: *mut pcap_pkthdr) -> *const u_char;
}
extern "C" {
  pub fn pcap_next_ex(
    arg1: *mut pcap_t,
    arg2: *mut *mut pcap_pkthdr,
    arg3: *mut *const u_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_breakloop(arg1: *mut pcap_t);
}
extern "C" {
  pub fn pcap_stats(arg1: *mut pcap_t, arg2: *mut pcap_stat) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_setfilter(arg1: *mut pcap_t, arg2: *mut bpf_program) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_setdirection(arg1: *mut pcap_t, arg2: pcap_direction_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_getnonblock(
    arg1: *mut pcap_t,
    arg2: *mut ::std::os::raw::c_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_setnonblock(
    arg1: *mut pcap_t,
    arg2: ::std::os::raw::c_int,
    arg3: *mut ::std::os::raw::c_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_inject(
    arg1: *mut pcap_t,
    arg2: *const ::std::os::raw::c_void,
    arg3: usize,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_sendpacket(
    arg1: *mut pcap_t,
    arg2: *const u_char,
    arg3: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_statustostr(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn pcap_strerror(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn pcap_geterr(arg1: *mut pcap_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
  pub fn pcap_perror(arg1: *mut pcap_t, arg2: *const ::std::os::raw::c_char);
}
extern "C" {
  pub fn pcap_compile(
    arg1: *mut pcap_t,
    arg2: *mut bpf_program,
    arg3: *const ::std::os::raw::c_char,
    arg4: ::std::os::raw::c_int,
    arg5: bpf_u_int32,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_compile_nopcap(
    arg1: ::std::os::raw::c_int,
    arg2: ::std::os::raw::c_int,
    arg3: *mut bpf_program,
    arg4: *const ::std::os::raw::c_char,
    arg5: ::std::os::raw::c_int,
    arg6: bpf_u_int32,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_freecode(arg1: *mut bpf_program);
}
extern "C" {
  pub fn pcap_offline_filter(
    arg1: *const bpf_program,
    arg2: *const pcap_pkthdr,
    arg3: *const u_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_datalink(arg1: *mut pcap_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_datalink_ext(arg1: *mut pcap_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_list_datalinks(
    arg1: *mut pcap_t,
    arg2: *mut *mut ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_set_datalink(arg1: *mut pcap_t, arg2: ::std::os::raw::c_int)
    -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_free_datalinks(arg1: *mut ::std::os::raw::c_int);
}
extern "C" {
  pub fn pcap_datalink_name_to_val(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_datalink_val_to_name(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn pcap_datalink_val_to_description(
    arg1: ::std::os::raw::c_int,
  ) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn pcap_snapshot(arg1: *mut pcap_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_is_swapped(arg1: *mut pcap_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_major_version(arg1: *mut pcap_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_minor_version(arg1: *mut pcap_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_file(arg1: *mut pcap_t) -> *mut FILE;
}
extern "C" {
  pub fn pcap_fileno(arg1: *mut pcap_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_wsockinit() -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_dump_open(
    arg1: *mut pcap_t,
    arg2: *const ::std::os::raw::c_char,
  ) -> *mut pcap_dumper_t;
}
extern "C" {
  pub fn pcap_dump_fopen(arg1: *mut pcap_t, fp: *mut FILE) -> *mut pcap_dumper_t;
}
extern "C" {
  pub fn pcap_dump_open_append(
    arg1: *mut pcap_t,
    arg2: *const ::std::os::raw::c_char,
  ) -> *mut pcap_dumper_t;
}
extern "C" {
  pub fn pcap_dump_file(arg1: *mut pcap_dumper_t) -> *mut FILE;
}
extern "C" {
  pub fn pcap_dump_ftell(arg1: *mut pcap_dumper_t) -> ::std::os::raw::c_long;
}
extern "C" {
  pub fn pcap_dump_flush(arg1: *mut pcap_dumper_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_dump_close(arg1: *mut pcap_dumper_t);
}
extern "C" {
  pub fn pcap_dump(arg1: *mut u_char, arg2: *const pcap_pkthdr, arg3: *const u_char);
}
extern "C" {
  pub fn pcap_findalldevs(
    arg1: *mut *mut pcap_if_t,
    arg2: *mut ::std::os::raw::c_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_freealldevs(arg1: *mut pcap_if_t);
}
extern "C" {
  pub fn pcap_lib_version() -> *const ::std::os::raw::c_char;
}
#[doc = "\\brief A queue of raw packets that will be sent to the network with pcap_sendqueue_transmit()."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcap_send_queue {
  pub maxlen: u_int,
  pub len: u_int,
  pub buffer: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_pcap_send_queue() {
  assert_eq!(
    ::std::mem::size_of::<pcap_send_queue>(),
    16usize,
    concat!("Size of: ", stringify!(pcap_send_queue))
  );
  assert_eq!(
    ::std::mem::align_of::<pcap_send_queue>(),
    8usize,
    concat!("Alignment of ", stringify!(pcap_send_queue))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_send_queue>())).maxlen as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_send_queue),
      "::",
      stringify!(maxlen)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_send_queue>())).len as *const _ as usize },
    4usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_send_queue),
      "::",
      stringify!(len)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<pcap_send_queue>())).buffer as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(pcap_send_queue),
      "::",
      stringify!(buffer)
    )
  );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _AirpcapHandle {
  _unused: [u8; 0],
}
pub type PAirpcapHandle = *mut _AirpcapHandle;
extern "C" {
  pub fn pcap_setbuff(p: *mut pcap_t, dim: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_setmode(p: *mut pcap_t, mode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_setmintocopy(p: *mut pcap_t, size: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_getevent(p: *mut pcap_t) -> HANDLE;
}
extern "C" {
  pub fn pcap_oid_get_request(
    arg1: *mut pcap_t,
    arg2: bpf_u_int32,
    arg3: *mut ::std::os::raw::c_void,
    arg4: *mut usize,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_oid_set_request(
    arg1: *mut pcap_t,
    arg2: bpf_u_int32,
    arg3: *const ::std::os::raw::c_void,
    arg4: *mut usize,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_sendqueue_alloc(memsize: u_int) -> *mut pcap_send_queue;
}
extern "C" {
  pub fn pcap_sendqueue_destroy(queue: *mut pcap_send_queue);
}
extern "C" {
  pub fn pcap_sendqueue_queue(
    queue: *mut pcap_send_queue,
    pkt_header: *const pcap_pkthdr,
    pkt_data: *const u_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_sendqueue_transmit(
    p: *mut pcap_t,
    queue: *mut pcap_send_queue,
    sync: ::std::os::raw::c_int,
  ) -> u_int;
}
extern "C" {
  pub fn pcap_stats_ex(
    p: *mut pcap_t,
    pcap_stat_size: *mut ::std::os::raw::c_int,
  ) -> *mut pcap_stat;
}
extern "C" {
  pub fn pcap_setuserbuffer(p: *mut pcap_t, size: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_live_dump(
    p: *mut pcap_t,
    filename: *mut ::std::os::raw::c_char,
    maxsize: ::std::os::raw::c_int,
    maxpacks: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_live_dump_ended(p: *mut pcap_t, sync: ::std::os::raw::c_int)
    -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_start_oem(
    err_str: *mut ::std::os::raw::c_char,
    flags: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn pcap_get_airpcap_handle(p: *mut pcap_t) -> PAirpcapHandle;
}
