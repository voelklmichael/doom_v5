#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, raw_ref_op)]
#[allow(unused_imports)]
use ::rust_doomgeneric;
extern "C" {
    pub type _XDisplay;
    pub type _XGC;
    pub type _XrmHashBucketRec;
    pub type _XPrivate;
    static mut DG_ScreenBuffer: *mut pixel_t;
    fn doomgeneric_Tick();
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn tolower(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn XCreateImage(
        _: *mut Display,
        _: *mut Visual,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut XImage;
    fn XOpenDisplay(_: *const ::core::ffi::c_char) -> *mut Display;
    fn XCreateGC(
        _: *mut Display,
        _: Drawable,
        _: ::core::ffi::c_ulong,
        _: *mut XGCValues,
    ) -> GC;
    fn XCreateSimpleWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_ulong,
        _: ::core::ffi::c_ulong,
    ) -> Window;
    fn XChangeProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: Atom,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> ::core::ffi::c_int;
    fn XPending(_: *mut Display) -> ::core::ffi::c_int;
    fn XPutImage(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XImage,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XSelectInput(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn XSetForeground(
        _: *mut Display,
        _: GC,
        _: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    fn XkbKeycodeToKeysym(
        _: *mut Display,
        _: KeyCode,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> KeySym;
    fn XkbSetDetectableAutoRepeat(
        _: *mut Display,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __suseconds_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint32_t = __uint32_t;
pub type pixel_t = uint32_t;
pub type XImage = _XImage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XImage {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub xoffset: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_char,
    pub byte_order: ::core::ffi::c_int,
    pub bitmap_unit: ::core::ffi::c_int,
    pub bitmap_bit_order: ::core::ffi::c_int,
    pub bitmap_pad: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub bytes_per_line: ::core::ffi::c_int,
    pub bits_per_pixel: ::core::ffi::c_int,
    pub red_mask: ::core::ffi::c_ulong,
    pub green_mask: ::core::ffi::c_ulong,
    pub blue_mask: ::core::ffi::c_ulong,
    pub obdata: XPointer,
    pub f: funcs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcs {
    pub create_image: Option<
        unsafe extern "C" fn(
            *mut _XDisplay,
            *mut Visual,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut _XImage,
    >,
    pub destroy_image: Option<unsafe extern "C" fn(*mut _XImage) -> ::core::ffi::c_int>,
    pub get_pixel: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulong,
    >,
    pub put_pixel: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_int,
    >,
    pub sub_image: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
        ) -> *mut _XImage,
    >,
    pub add_pixel: Option<
        unsafe extern "C" fn(*mut _XImage, ::core::ffi::c_long) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: ::core::ffi::c_int,
    pub red_mask: ::core::ffi::c_ulong,
    pub green_mask: ::core::ffi::c_ulong,
    pub blue_mask: ::core::ffi::c_ulong,
    pub bits_per_rgb: ::core::ffi::c_int,
    pub map_entries: ::core::ffi::c_int,
}
pub type VisualID = ::core::ffi::c_ulong;
pub type XExtData = _XExtData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: ::core::ffi::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(*mut _XExtData) -> ::core::ffi::c_int>,
    pub private_data: XPointer,
}
pub type XPointer = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub mwidth: ::core::ffi::c_int,
    pub mheight: ::core::ffi::c_int,
    pub ndepths: ::core::ffi::c_int,
    pub depths: *mut Depth,
    pub root_depth: ::core::ffi::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: ::core::ffi::c_ulong,
    pub black_pixel: ::core::ffi::c_ulong,
    pub max_maps: ::core::ffi::c_int,
    pub min_maps: ::core::ffi::c_int,
    pub backing_store: ::core::ffi::c_int,
    pub save_unders: ::core::ffi::c_int,
    pub root_input_mask: ::core::ffi::c_long,
}
pub type Colormap = XID;
pub type XID = ::core::ffi::c_ulong;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: ::core::ffi::c_int,
    pub nvisuals: ::core::ffi::c_int,
    pub visuals: *mut Visual,
}
pub type Window = XID;
pub type _XPrivDisplay = *mut C2RustUnnamed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: ::core::ffi::c_int,
    pub private2: ::core::ffi::c_int,
    pub proto_major_version: ::core::ffi::c_int,
    pub proto_minor_version: ::core::ffi::c_int,
    pub vendor: *mut ::core::ffi::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: ::core::ffi::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: ::core::ffi::c_int,
    pub bitmap_unit: ::core::ffi::c_int,
    pub bitmap_pad: ::core::ffi::c_int,
    pub bitmap_bit_order: ::core::ffi::c_int,
    pub nformats: ::core::ffi::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: ::core::ffi::c_int,
    pub release: ::core::ffi::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: ::core::ffi::c_int,
    pub last_request_read: ::core::ffi::c_ulong,
    pub request: ::core::ffi::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: ::core::ffi::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(*mut _XDisplay) -> ::core::ffi::c_int>,
    pub display_name: *mut ::core::ffi::c_char,
    pub default_screen: ::core::ffi::c_int,
    pub nscreens: ::core::ffi::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: ::core::ffi::c_ulong,
    pub private16: ::core::ffi::c_ulong,
    pub min_keycode: ::core::ffi::c_int,
    pub max_keycode: ::core::ffi::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: ::core::ffi::c_int,
    pub xdefaults: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: ::core::ffi::c_int,
    pub bits_per_pixel: ::core::ffi::c_int,
    pub scanline_pad: ::core::ffi::c_int,
}
pub type Display = _XDisplay;
pub type XEvent = _XEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: ::core::ffi::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [::core::ffi::c_long; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub extension: ::core::ffi::c_int,
    pub evtype: ::core::ffi::c_int,
    pub cookie: ::core::ffi::c_uint,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub extension: ::core::ffi::c_int,
    pub evtype: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: ::core::ffi::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: ::core::ffi::c_ulong,
    pub error_code: ::core::ffi::c_uchar,
    pub request_code: ::core::ffi::c_uchar,
    pub minor_code: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: ::core::ffi::c_int,
    pub first_keycode: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: ::core::ffi::c_int,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub b: [::core::ffi::c_char; 20],
    pub s: [::core::ffi::c_short; 10],
    pub l: [::core::ffi::c_long; 5],
}
pub type Atom = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: ::core::ffi::c_int,
    pub state: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
pub type Time = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub above: Window,
    pub detail: ::core::ffi::c_int,
    pub value_mask: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub above: Window,
    pub override_redirect: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub override_redirect: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub border_width: ::core::ffi::c_int,
    pub override_redirect: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: ::core::ffi::c_int,
    pub minor_code: ::core::ffi::c_int,
}
pub type Drawable = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub major_code: ::core::ffi::c_int,
    pub minor_code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: ::core::ffi::c_int,
    pub detail: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub mode: ::core::ffi::c_int,
    pub detail: ::core::ffi::c_int,
    pub same_screen: ::core::ffi::c_int,
    pub focus: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
    pub is_hint: ::core::ffi::c_char,
    pub same_screen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
    pub button: ::core::ffi::c_uint,
    pub same_screen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub x_root: ::core::ffi::c_int,
    pub y_root: ::core::ffi::c_int,
    pub state: ::core::ffi::c_uint,
    pub keycode: ::core::ffi::c_uint,
    pub same_screen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_ulong,
    pub send_event: ::core::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: ::core::ffi::c_int,
    pub plane_mask: ::core::ffi::c_ulong,
    pub foreground: ::core::ffi::c_ulong,
    pub background: ::core::ffi::c_ulong,
    pub line_width: ::core::ffi::c_int,
    pub line_style: ::core::ffi::c_int,
    pub cap_style: ::core::ffi::c_int,
    pub join_style: ::core::ffi::c_int,
    pub fill_style: ::core::ffi::c_int,
    pub fill_rule: ::core::ffi::c_int,
    pub arc_mode: ::core::ffi::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: ::core::ffi::c_int,
    pub ts_y_origin: ::core::ffi::c_int,
    pub font: Font,
    pub subwindow_mode: ::core::ffi::c_int,
    pub graphics_exposures: ::core::ffi::c_int,
    pub clip_x_origin: ::core::ffi::c_int,
    pub clip_y_origin: ::core::ffi::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: ::core::ffi::c_int,
    pub dashes: ::core::ffi::c_char,
}
pub type Pixmap = XID;
pub type Font = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: ::core::ffi::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: ::core::ffi::c_ulong,
    pub bit_gravity: ::core::ffi::c_int,
    pub win_gravity: ::core::ffi::c_int,
    pub backing_store: ::core::ffi::c_int,
    pub backing_planes: ::core::ffi::c_ulong,
    pub backing_pixel: ::core::ffi::c_ulong,
    pub save_under: ::core::ffi::c_int,
    pub event_mask: ::core::ffi::c_long,
    pub do_not_propagate_mask: ::core::ffi::c_long,
    pub override_redirect: ::core::ffi::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
pub type Cursor = XID;
pub type KeySym = XID;
pub type KeyCode = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: ::core::ffi::c_int,
    pub tz_dsttime: ::core::ffi::c_int,
}
pub const KEY_RIGHTARROW: ::core::ffi::c_int = 0xae as ::core::ffi::c_int;
pub const KEY_LEFTARROW: ::core::ffi::c_int = 0xac as ::core::ffi::c_int;
pub const KEY_UPARROW: ::core::ffi::c_int = 0xad as ::core::ffi::c_int;
pub const KEY_DOWNARROW: ::core::ffi::c_int = 0xaf as ::core::ffi::c_int;
pub const KEY_USE: ::core::ffi::c_int = 0xa2 as ::core::ffi::c_int;
pub const KEY_FIRE: ::core::ffi::c_int = 0xa3 as ::core::ffi::c_int;
pub const KEY_ESCAPE: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const KEY_ENTER: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const KEY_RSHIFT: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int
    + 0x36 as ::core::ffi::c_int;
pub const DOOMGENERIC_RESX: ::core::ffi::c_int = 640 as ::core::ffi::c_int;
pub const DOOMGENERIC_RESY: ::core::ffi::c_int = 400 as ::core::ffi::c_int;
pub const KeyPressMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long)
    << 0 as ::core::ffi::c_int;
pub const KeyReleaseMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long)
    << 1 as ::core::ffi::c_int;
pub const ExposureMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long)
    << 15 as ::core::ffi::c_int;
pub const StructureNotifyMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long)
    << 17 as ::core::ffi::c_int;
pub const KeyPress: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const KeyRelease: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MapNotify: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const PropModeReplace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ZPixmap: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const XK_Return: ::core::ffi::c_uint = 65293 as ::core::ffi::c_uint;
pub const XK_Escape: ::core::ffi::c_uint = 65307 as ::core::ffi::c_uint;
pub const XK_Left: ::core::ffi::c_uint = 65361 as ::core::ffi::c_uint;
pub const XK_Up: ::core::ffi::c_uint = 65362 as ::core::ffi::c_uint;
pub const XK_Right: ::core::ffi::c_uint = 65363 as ::core::ffi::c_uint;
pub const XK_Down: ::core::ffi::c_uint = 65364 as ::core::ffi::c_uint;
pub const XK_Shift_L: ::core::ffi::c_uint = 65505 as ::core::ffi::c_uint;
pub const XK_Shift_R: ::core::ffi::c_uint = 65506 as ::core::ffi::c_uint;
pub const XK_Control_L: ::core::ffi::c_uint = 65507 as ::core::ffi::c_uint;
pub const XK_Control_R: ::core::ffi::c_uint = 65508 as ::core::ffi::c_uint;
pub const XK_space: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const XA_STRING: Atom = 31 as ::core::ffi::c_int as Atom;
pub const XA_WM_NAME: Atom = 39 as ::core::ffi::c_int as Atom;
static mut s_Display: *mut Display = ::core::ptr::null::<Display>() as *mut Display;
static mut s_Window: Window = 0 as Window;
static mut s_Screen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut s_Gc: GC = ::core::ptr::null::<_XGC>() as *mut _XGC;
static mut s_Image: *mut XImage = ::core::ptr::null::<XImage>() as *mut XImage;
pub const KEYQUEUE_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
static mut s_KeyQueue: [::core::ffi::c_ushort; 16] = [0; 16];
static mut s_KeyQueueWriteIndex: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
static mut s_KeyQueueReadIndex: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
unsafe extern "C" fn convertToDoomKey(
    mut key: ::core::ffi::c_uint,
) -> ::core::ffi::c_uchar {
    match key {
        65293 => {
            key = KEY_ENTER as ::core::ffi::c_uint;
        }
        65307 => {
            key = KEY_ESCAPE as ::core::ffi::c_uint;
        }
        65361 => {
            key = KEY_LEFTARROW as ::core::ffi::c_uint;
        }
        65363 => {
            key = KEY_RIGHTARROW as ::core::ffi::c_uint;
        }
        65362 => {
            key = KEY_UPARROW as ::core::ffi::c_uint;
        }
        65364 => {
            key = KEY_DOWNARROW as ::core::ffi::c_uint;
        }
        65507 | 65508 => {
            key = KEY_FIRE as ::core::ffi::c_uint;
        }
        32 => {
            key = KEY_USE as ::core::ffi::c_uint;
        }
        65505 | 65506 => {
            key = KEY_RSHIFT as ::core::ffi::c_uint;
        }
        _ => {
            key = ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_uint>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: ::core::ffi::c_int = key as ::core::ffi::c_int;
                        __res = (if __c < -(128 as ::core::ffi::c_int)
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = tolower(key as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(key as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) as ::core::ffi::c_uint;
        }
    }
    return key as ::core::ffi::c_uchar;
}
unsafe extern "C" fn addKeyToQueue(
    mut pressed: ::core::ffi::c_int,
    mut keyCode: ::core::ffi::c_uint,
) {
    let mut key: ::core::ffi::c_uchar = convertToDoomKey(keyCode);
    let mut keyData: ::core::ffi::c_ushort = (pressed << 8 as ::core::ffi::c_int
        | key as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    s_KeyQueue[s_KeyQueueWriteIndex as usize] = keyData;
    s_KeyQueueWriteIndex = s_KeyQueueWriteIndex.wrapping_add(1);
    s_KeyQueueWriteIndex = s_KeyQueueWriteIndex
        .wrapping_rem(KEYQUEUE_SIZE as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn DG_Init() {
    memset(
        &raw mut s_KeyQueue as *mut ::core::ffi::c_ushort as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (KEYQUEUE_SIZE as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_ushort>() as size_t),
    );
    s_Display = XOpenDisplay(::core::ptr::null::<::core::ffi::c_char>());
    s_Screen = (*(s_Display as _XPrivDisplay)).default_screen;
    let mut blackColor: ::core::ffi::c_int = (*(*(s_Display as _XPrivDisplay))
        .screens
        .offset(s_Screen as isize))
        .black_pixel as ::core::ffi::c_int;
    let mut whiteColor: ::core::ffi::c_int = (*(*(s_Display as _XPrivDisplay))
        .screens
        .offset(s_Screen as isize))
        .white_pixel as ::core::ffi::c_int;
    let mut attr: XSetWindowAttributes = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    memset(
        &raw mut attr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<XSetWindowAttributes>() as size_t,
    );
    attr.event_mask = ExposureMask | KeyPressMask;
    attr.background_pixel = (*(*(s_Display as _XPrivDisplay))
        .screens
        .offset(s_Screen as isize))
        .black_pixel;
    let mut depth: ::core::ffi::c_int = (*(*(s_Display as _XPrivDisplay))
        .screens
        .offset(s_Screen as isize))
        .root_depth;
    s_Window = XCreateSimpleWindow(
        s_Display,
        (*(*(s_Display as _XPrivDisplay))
            .screens
            .offset((*(s_Display as _XPrivDisplay)).default_screen as isize))
            .root,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        DOOMGENERIC_RESX as ::core::ffi::c_uint,
        DOOMGENERIC_RESY as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        blackColor as ::core::ffi::c_ulong,
        blackColor as ::core::ffi::c_ulong,
    );
    XSelectInput(
        s_Display,
        s_Window,
        StructureNotifyMask | KeyPressMask | KeyReleaseMask,
    );
    XMapWindow(s_Display, s_Window);
    s_Gc = XCreateGC(
        s_Display,
        s_Window as Drawable,
        0 as ::core::ffi::c_ulong,
        ::core::ptr::null_mut::<XGCValues>(),
    );
    XSetForeground(s_Display, s_Gc, whiteColor as ::core::ffi::c_ulong);
    XkbSetDetectableAutoRepeat(
        s_Display,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    loop {
        let mut e: XEvent = _XEvent { type_0: 0 };
        XNextEvent(s_Display, &raw mut e);
        if e.type_0 == MapNotify {
            break;
        }
    }
    s_Image = XCreateImage(
        s_Display,
        (*(*(s_Display as _XPrivDisplay)).screens.offset(s_Screen as isize)).root_visual,
        depth as ::core::ffi::c_uint,
        ZPixmap,
        0 as ::core::ffi::c_int,
        DG_ScreenBuffer as *mut ::core::ffi::c_char,
        DOOMGENERIC_RESX as ::core::ffi::c_uint,
        DOOMGENERIC_RESX as ::core::ffi::c_uint,
        32 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn DG_DrawFrame() {
    if !s_Display.is_null() {
        while XPending(s_Display) > 0 as ::core::ffi::c_int {
            let mut e: XEvent = _XEvent { type_0: 0 };
            XNextEvent(s_Display, &raw mut e);
            if e.type_0 == KeyPress {
                let mut sym: KeySym = XkbKeycodeToKeysym(
                    s_Display,
                    e.xkey.keycode as KeyCode,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                addKeyToQueue(1 as ::core::ffi::c_int, sym as ::core::ffi::c_uint);
            } else if e.type_0 == KeyRelease {
                let mut sym_0: KeySym = XkbKeycodeToKeysym(
                    s_Display,
                    e.xkey.keycode as KeyCode,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                addKeyToQueue(0 as ::core::ffi::c_int, sym_0 as ::core::ffi::c_uint);
            }
        }
        XPutImage(
            s_Display,
            s_Window as Drawable,
            s_Gc,
            s_Image,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            DOOMGENERIC_RESX as ::core::ffi::c_uint,
            DOOMGENERIC_RESY as ::core::ffi::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn DG_SleepMs(mut ms: uint32_t) {
    usleep((ms as __useconds_t).wrapping_mul(1000 as __useconds_t));
}
#[no_mangle]
pub unsafe extern "C" fn DG_GetTicksMs() -> uint32_t {
    let mut tp: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tzp: timezone = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    gettimeofday(&raw mut tp, &raw mut tzp as *mut ::core::ffi::c_void);
    return (tp.tv_sec as __suseconds_t * 1000 as __suseconds_t
        + tp.tv_usec / 1000 as __suseconds_t) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn DG_GetKey(
    mut pressed: *mut ::core::ffi::c_int,
    mut doomKey: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    if s_KeyQueueReadIndex == s_KeyQueueWriteIndex {
        return 0 as ::core::ffi::c_int
    } else {
        let mut keyData: ::core::ffi::c_ushort = s_KeyQueue[s_KeyQueueReadIndex
            as usize];
        s_KeyQueueReadIndex = s_KeyQueueReadIndex.wrapping_add(1);
        s_KeyQueueReadIndex = s_KeyQueueReadIndex
            .wrapping_rem(KEYQUEUE_SIZE as ::core::ffi::c_uint);
        *pressed = keyData as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
        *doomKey = (keyData as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        return 1 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn DG_SetWindowTitle(mut title: *const ::core::ffi::c_char) {
    if s_Window != 0 {
        XChangeProperty(
            s_Display,
            s_Window,
            XA_WM_NAME,
            XA_STRING,
            8 as ::core::ffi::c_int,
            PropModeReplace,
            title as *const ::core::ffi::c_uchar,
            strlen(title) as ::core::ffi::c_int,
        );
    }
}
pub fn main() {
    unsafe {
        ::rust_doomgeneric::src::doomgeneric::doomgeneric_Create(
            ::std::env::args().collect(),
        );
        loop {
            doomgeneric_Tick();
        }
    }
}
