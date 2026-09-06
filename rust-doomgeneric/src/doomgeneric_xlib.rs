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
use libc::memset;
use libc::strlen;
extern "C" {
    pub type _XDisplay;
    pub type _XGC;
    pub type _XrmHashBucketRec;
    pub type _XPrivate;
    static mut DG_ScreenBuffer: *mut pixel_t;
    fn doomgeneric_Tick();
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn tolower(__c: i32) -> i32;
    fn usleep(__useconds: __useconds_t) -> i32;
    fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::core::ffi::c_void,
    ) -> i32;
    fn XCreateImage(
        _: *mut Display,
        _: *mut Visual,
        _: u32,
        _: i32,
        _: i32,
        _: *mut ::core::ffi::c_char,
        _: u32,
        _: u32,
        _: i32,
        _: i32,
    ) -> *mut XImage;
    fn XOpenDisplay(_: *const ::core::ffi::c_char) -> *mut Display;
    fn XCreateGC(
        _: *mut Display,
        _: Drawable,
        _: u64,
        _: *mut XGCValues,
    ) -> GC;
    fn XCreateSimpleWindow(
        _: *mut Display,
        _: Window,
        _: i32,
        _: i32,
        _: u32,
        _: u32,
        _: u32,
        _: u64,
        _: u64,
    ) -> Window;
    fn XChangeProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: Atom,
        _: i32,
        _: i32,
        _: *const u8,
        _: i32,
    ) -> i32;
    fn XMapWindow(_: *mut Display, _: Window) -> i32;
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> i32;
    fn XPending(_: *mut Display) -> i32;
    fn XPutImage(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XImage,
        _: i32,
        _: i32,
        _: i32,
        _: i32,
        _: u32,
        _: u32,
    ) -> i32;
    fn XSelectInput(
        _: *mut Display,
        _: Window,
        _: i64,
    ) -> i32;
    fn XSetForeground(
        _: *mut Display,
        _: GC,
        _: u64,
    ) -> i32;
    fn XkbKeycodeToKeysym(
        _: *mut Display,
        _: KeyCode,
        _: i32,
        _: i32,
    ) -> KeySym;
    fn XkbSetDetectableAutoRepeat(
        _: *mut Display,
        _: i32,
        _: *mut i32,
    ) -> i32;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __time_t = i64;
pub type __useconds_t = u32;
pub type __suseconds_t = i64;
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
    pub width: i32,
    pub height: i32,
    pub xoffset: i32,
    pub format: i32,
    pub data: *mut ::core::ffi::c_char,
    pub byte_order: i32,
    pub bitmap_unit: i32,
    pub bitmap_bit_order: i32,
    pub bitmap_pad: i32,
    pub depth: i32,
    pub bytes_per_line: i32,
    pub bits_per_pixel: i32,
    pub red_mask: u64,
    pub green_mask: u64,
    pub blue_mask: u64,
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
            u32,
            i32,
            i32,
            *mut ::core::ffi::c_char,
            u32,
            u32,
            i32,
            i32,
        ) -> *mut _XImage,
    >,
    pub destroy_image: Option<unsafe extern "C" fn(*mut _XImage) -> i32>,
    pub get_pixel: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            i32,
            i32,
        ) -> u64,
    >,
    pub put_pixel: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            i32,
            i32,
            u64,
        ) -> i32,
    >,
    pub sub_image: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            i32,
            i32,
            u32,
            u32,
        ) -> *mut _XImage,
    >,
    pub add_pixel: Option<
        unsafe extern "C" fn(*mut _XImage, i64) -> i32,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: i32,
    pub red_mask: u64,
    pub green_mask: u64,
    pub blue_mask: u64,
    pub bits_per_rgb: i32,
    pub map_entries: i32,
}
pub type VisualID = u64;
pub type XExtData = _XExtData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: i32,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(*mut _XExtData) -> i32>,
    pub private_data: XPointer,
}
pub type XPointer = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: i32,
    pub height: i32,
    pub mwidth: i32,
    pub mheight: i32,
    pub ndepths: i32,
    pub depths: *mut Depth,
    pub root_depth: i32,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: u64,
    pub black_pixel: u64,
    pub max_maps: i32,
    pub min_maps: i32,
    pub backing_store: i32,
    pub save_unders: i32,
    pub root_input_mask: i64,
}
pub type Colormap = XID;
pub type XID = u64;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: i32,
    pub nvisuals: i32,
    pub visuals: *mut Visual,
}
pub type Window = XID;
pub type _XPrivDisplay = *mut C2RustUnnamed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: i32,
    pub private2: i32,
    pub proto_major_version: i32,
    pub proto_minor_version: i32,
    pub vendor: *mut ::core::ffi::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: i32,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: i32,
    pub bitmap_unit: i32,
    pub bitmap_pad: i32,
    pub bitmap_bit_order: i32,
    pub nformats: i32,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: i32,
    pub release: i32,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: i32,
    pub last_request_read: u64,
    pub request: u64,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: u32,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(*mut _XDisplay) -> i32>,
    pub display_name: *mut ::core::ffi::c_char,
    pub default_screen: i32,
    pub nscreens: i32,
    pub screens: *mut Screen,
    pub motion_buffer: u64,
    pub private16: u64,
    pub min_keycode: i32,
    pub max_keycode: i32,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: i32,
    pub xdefaults: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: i32,
    pub bits_per_pixel: i32,
    pub scanline_pad: i32,
}
pub type Display = _XDisplay;
pub type XEvent = _XEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: i32,
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
    pub pad: [i64; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub extension: i32,
    pub evtype: i32,
    pub cookie: u32,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub extension: i32,
    pub evtype: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: i32,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: u64,
    pub error_code: u8,
    pub request_code: u8,
    pub minor_code: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub request: i32,
    pub first_keycode: i32,
    pub count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: i32,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub b: [::core::ffi::c_char; 20],
    pub s: [i16; 10],
    pub l: [i64; 5],
}
pub type Atom = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: i32,
    pub state: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
pub type Time = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
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
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub above: Window,
    pub detail: i32,
    pub value_mask: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub width: i32,
    pub height: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub above: Window,
    pub override_redirect: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: i32,
    pub y: i32,
    pub override_redirect: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub override_redirect: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub state: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: i32,
    pub minor_code: i32,
}
pub type Drawable = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub count: i32,
    pub major_code: i32,
    pub minor_code: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub mode: i32,
    pub detail: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub mode: i32,
    pub detail: i32,
    pub same_screen: i32,
    pub focus: i32,
    pub state: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub is_hint: ::core::ffi::c_char,
    pub same_screen: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub button: u32,
    pub same_screen: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub keycode: u32,
    pub same_screen: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: i32,
    pub plane_mask: u64,
    pub foreground: u64,
    pub background: u64,
    pub line_width: i32,
    pub line_style: i32,
    pub cap_style: i32,
    pub join_style: i32,
    pub fill_style: i32,
    pub fill_rule: i32,
    pub arc_mode: i32,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: i32,
    pub ts_y_origin: i32,
    pub font: Font,
    pub subwindow_mode: i32,
    pub graphics_exposures: i32,
    pub clip_x_origin: i32,
    pub clip_y_origin: i32,
    pub clip_mask: Pixmap,
    pub dash_offset: i32,
    pub dashes: ::core::ffi::c_char,
}
pub type Pixmap = XID;
pub type Font = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: u64,
    pub border_pixmap: Pixmap,
    pub border_pixel: u64,
    pub bit_gravity: i32,
    pub win_gravity: i32,
    pub backing_store: i32,
    pub backing_planes: u64,
    pub backing_pixel: u64,
    pub save_under: i32,
    pub event_mask: i64,
    pub do_not_propagate_mask: i64,
    pub override_redirect: i32,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
pub type Cursor = XID;
pub type KeySym = XID;
pub type KeyCode = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub const KEY_RIGHTARROW: i32 = 0xae;
pub const KEY_LEFTARROW: i32 = 0xac;
pub const KEY_UPARROW: i32 = 0xad;
pub const KEY_DOWNARROW: i32 = 0xaf;
pub const KEY_USE: i32 = 0xa2;
pub const KEY_FIRE: i32 = 0xa3;
pub const KEY_ESCAPE: i32 = 27;
pub const KEY_ENTER: i32 = 13;
pub const KEY_RSHIFT: i32 = 0x80
    + 0x36 as i32;
pub const DOOMGENERIC_RESX: i32 = 640;
pub const DOOMGENERIC_RESY: i32 = 400;
pub const KeyPressMask: i64 = (1 as i64)
    << 0 as i32;
pub const KeyReleaseMask: i64 = (1 as i64)
    << 1 as i32;
pub const ExposureMask: i64 = (1 as i64)
    << 15 as i32;
pub const StructureNotifyMask: i64 = (1 as i64)
    << 17 as i32;
pub const KeyPress: i32 = 2;
pub const KeyRelease: i32 = 3;
pub const MapNotify: i32 = 19;
pub const PropModeReplace: i32 = 0;
pub const ZPixmap: i32 = 2;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const XK_Return: u32 = 65293;
pub const XK_Escape: u32 = 65307;
pub const XK_Left: u32 = 65361;
pub const XK_Up: u32 = 65362;
pub const XK_Right: u32 = 65363;
pub const XK_Down: u32 = 65364;
pub const XK_Shift_L: u32 = 65505;
pub const XK_Shift_R: u32 = 65506;
pub const XK_Control_L: u32 = 65507;
pub const XK_Control_R: u32 = 65508;
pub const XK_space: u32 = 32;
pub const XA_STRING: Atom = 31 as i32 as Atom;
pub const XA_WM_NAME: Atom = 39 as i32 as Atom;
static mut s_Display: *mut Display = ::core::ptr::null::<Display>() as *mut Display;
static mut s_Window: Window = 0 as Window;
static mut s_Screen: i32 = 0 as i32;
static mut s_Gc: GC = ::core::ptr::null::<_XGC>() as *mut _XGC;
static mut s_Image: *mut XImage = ::core::ptr::null::<XImage>() as *mut XImage;
pub const KEYQUEUE_SIZE: i32 = 16;
static mut s_KeyQueue: [u16; 16] = [0; 16];
static mut s_KeyQueueWriteIndex: u32 = 0 as u32;
static mut s_KeyQueueReadIndex: u32 = 0 as u32;
unsafe extern "C" fn convertToDoomKey(
    mut key: u32,
) -> u8 {
    match key {
        65293 => {
            key = KEY_ENTER as u32;
        }
        65307 => {
            key = KEY_ESCAPE as u32;
        }
        65361 => {
            key = KEY_LEFTARROW as u32;
        }
        65363 => {
            key = KEY_RIGHTARROW as u32;
        }
        65362 => {
            key = KEY_UPARROW as u32;
        }
        65364 => {
            key = KEY_DOWNARROW as u32;
        }
        65507 | 65508 => {
            key = KEY_FIRE as u32;
        }
        32 => {
            key = KEY_USE as u32;
        }
        65505 | 65506 => {
            key = KEY_RSHIFT as u32;
        }
        _ => {
            key = ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u32>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: i32 = key as i32;
                        __res = (if __c < -(128 as i32)
                            || __c > 255 as i32
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as i32;
                    } else {
                        __res = tolower(key as i32);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(key as i32 as isize)
                        as i32;
                }
                __res
            }) as u32;
        }
    }
    return key as u8;
}
unsafe extern "C" fn addKeyToQueue(
    mut pressed: i32,
    mut keyCode: u32,
) {
    let mut key: u8 = convertToDoomKey(keyCode);
    let mut keyData: u16 = (pressed << 8 as i32
        | key as i32) as u16;
    s_KeyQueue[s_KeyQueueWriteIndex as usize] = keyData;
    s_KeyQueueWriteIndex = s_KeyQueueWriteIndex.wrapping_add(1);
    s_KeyQueueWriteIndex = s_KeyQueueWriteIndex
        .wrapping_rem(KEYQUEUE_SIZE as u32);
}
#[no_mangle]
pub unsafe extern "C" fn DG_Init() {
    memset(
        &raw mut s_KeyQueue as *mut u16 as *mut ::core::ffi::c_void,
        0 as i32,
        (KEYQUEUE_SIZE as size_t)
            .wrapping_mul(::core::mem::size_of::<u16>() as size_t),
    );
    s_Display = XOpenDisplay(::core::ptr::null::<::core::ffi::c_char>());
    s_Screen = (*(s_Display as _XPrivDisplay)).default_screen;
    let mut blackColor: i32 = (*(*(s_Display as _XPrivDisplay))
        .screens
        .offset(s_Screen as isize))
        .black_pixel as i32;
    let mut whiteColor: i32 = (*(*(s_Display as _XPrivDisplay))
        .screens
        .offset(s_Screen as isize))
        .white_pixel as i32;
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
        0 as i32,
        ::core::mem::size_of::<XSetWindowAttributes>() as size_t,
    );
    attr.event_mask = ExposureMask | KeyPressMask;
    attr.background_pixel = (*(*(s_Display as _XPrivDisplay))
        .screens
        .offset(s_Screen as isize))
        .black_pixel;
    let mut depth: i32 = (*(*(s_Display as _XPrivDisplay))
        .screens
        .offset(s_Screen as isize))
        .root_depth;
    s_Window = XCreateSimpleWindow(
        s_Display,
        (*(*(s_Display as _XPrivDisplay))
            .screens
            .offset((*(s_Display as _XPrivDisplay)).default_screen as isize))
            .root,
        0 as i32,
        0 as i32,
        DOOMGENERIC_RESX as u32,
        DOOMGENERIC_RESY as u32,
        0 as u32,
        blackColor as u64,
        blackColor as u64,
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
        0 as u64,
        ::core::ptr::null_mut::<XGCValues>(),
    );
    XSetForeground(s_Display, s_Gc, whiteColor as u64);
    XkbSetDetectableAutoRepeat(
        s_Display,
        1 as i32,
        ::core::ptr::null_mut::<i32>(),
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
        depth as u32,
        ZPixmap,
        0 as i32,
        DG_ScreenBuffer as *mut ::core::ffi::c_char,
        DOOMGENERIC_RESX as u32,
        DOOMGENERIC_RESX as u32,
        32 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn DG_DrawFrame() {
    if !s_Display.is_null() {
        while XPending(s_Display) > 0 as i32 {
            let mut e: XEvent = _XEvent { type_0: 0 };
            XNextEvent(s_Display, &raw mut e);
            if e.type_0 == KeyPress {
                let mut sym: KeySym = XkbKeycodeToKeysym(
                    s_Display,
                    e.xkey.keycode as KeyCode,
                    0 as i32,
                    0 as i32,
                );
                addKeyToQueue(1 as i32, sym as u32);
            } else if e.type_0 == KeyRelease {
                let mut sym_0: KeySym = XkbKeycodeToKeysym(
                    s_Display,
                    e.xkey.keycode as KeyCode,
                    0 as i32,
                    0 as i32,
                );
                addKeyToQueue(0 as i32, sym_0 as u32);
            }
        }
        XPutImage(
            s_Display,
            s_Window as Drawable,
            s_Gc,
            s_Image,
            0 as i32,
            0 as i32,
            0 as i32,
            0 as i32,
            DOOMGENERIC_RESX as u32,
            DOOMGENERIC_RESY as u32,
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
    mut pressed: *mut i32,
    mut doomKey: *mut u8,
) -> i32 {
    if s_KeyQueueReadIndex == s_KeyQueueWriteIndex {
        return 0 as i32
    } else {
        let mut keyData: u16 = s_KeyQueue[s_KeyQueueReadIndex
            as usize];
        s_KeyQueueReadIndex = s_KeyQueueReadIndex.wrapping_add(1);
        s_KeyQueueReadIndex = s_KeyQueueReadIndex
            .wrapping_rem(KEYQUEUE_SIZE as u32);
        *pressed = keyData as i32 >> 8 as i32;
        *doomKey = (keyData as i32 & 0xff as i32)
            as u8;
        return 1 as i32;
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
            8 as i32,
            PropModeReplace,
            title as *const u8,
            strlen(title) as i32,
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
