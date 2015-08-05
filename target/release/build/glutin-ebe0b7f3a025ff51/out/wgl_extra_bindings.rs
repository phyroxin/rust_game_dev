
        mod __gl_imports {
            extern crate gl_common;
            extern crate libc;
            pub use std::mem;
            pub use std::marker::Send;
        }
    

        pub mod types {
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(dead_code)]
            #![allow(missing_copy_implementations)]
    
pub type GLenum = super::__gl_imports::libc::c_uint;
pub type GLboolean = super::__gl_imports::libc::c_uchar;
pub type GLbitfield = super::__gl_imports::libc::c_uint;
pub type GLvoid = super::__gl_imports::libc::c_void;
pub type GLbyte = super::__gl_imports::libc::c_char;
pub type GLshort = super::__gl_imports::libc::c_short;
pub type GLint = super::__gl_imports::libc::c_int;
pub type GLclampx = super::__gl_imports::libc::c_int;
pub type GLubyte = super::__gl_imports::libc::c_uchar;
pub type GLushort = super::__gl_imports::libc::c_ushort;
pub type GLuint = super::__gl_imports::libc::c_uint;
pub type GLsizei = super::__gl_imports::libc::c_int;
pub type GLfloat = super::__gl_imports::libc::c_float;
pub type GLclampf = super::__gl_imports::libc::c_float;
pub type GLdouble = super::__gl_imports::libc::c_double;
pub type GLclampd = super::__gl_imports::libc::c_double;
pub type GLeglImageOES = *const super::__gl_imports::libc::c_void;
pub type GLchar = super::__gl_imports::libc::c_char;
pub type GLcharARB = super::__gl_imports::libc::c_char;
#[cfg(target_os = "macos")] pub type GLhandleARB = *const super::__gl_imports::libc::c_void;
#[cfg(not(target_os = "macos"))] pub type GLhandleARB = super::__gl_imports::libc::c_uint;
pub type GLhalfARB = super::__gl_imports::libc::c_ushort;
pub type GLhalf = super::__gl_imports::libc::c_ushort;
pub type GLfixed = GLint;
pub type GLintptr = super::__gl_imports::libc::ptrdiff_t;
pub type GLsizeiptr = super::__gl_imports::libc::ptrdiff_t;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = super::__gl_imports::libc::ptrdiff_t;
pub type GLsizeiptrARB = super::__gl_imports::libc::ptrdiff_t;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;
#[repr(C)] pub struct __GLsync;
pub type GLsync = *const __GLsync;
#[repr(C)] pub struct _cl_context;
#[repr(C)] pub struct _cl_event;
pub type GLDEBUGPROC = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);
pub type GLDEBUGPROCARB = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);
pub type GLDEBUGPROCKHR = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);
pub type GLDEBUGPROCAMD = extern "system" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut super::__gl_imports::libc::c_void);
pub type GLhalfNV = super::__gl_imports::libc::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;
pub type CHAR = super::__gl_imports::libc::c_char;
pub type HANDLE = PVOID;
pub type LONG = super::__gl_imports::libc::c_long;
pub type LPCSTR = *const super::__gl_imports::libc::c_char;
pub type VOID = super::__gl_imports::libc::c_void;
pub type BOOL = super::__gl_imports::libc::c_int;
pub type BYTE = super::__gl_imports::libc::c_uchar;
pub type COLORREF = DWORD;
pub type FLOAT = super::__gl_imports::libc::c_float;
pub type HDC = HANDLE;
pub type HENHMETAFILE = HANDLE;
pub type HGLRC = *const super::__gl_imports::libc::c_void;
pub type INT = super::__gl_imports::libc::c_int;
pub type PVOID = *const super::__gl_imports::libc::c_void;
pub type LPVOID = *const super::__gl_imports::libc::c_void;
pub type PROC = extern "system" fn();

            #[repr(C)]
            pub struct RECT {
                left: LONG,
                top: LONG,
                right: LONG,
                bottom: LONG,
            }
        
pub type UINT = super::__gl_imports::libc::c_uint;
pub type USHORT = super::__gl_imports::libc::c_ushort;
pub type WORD = super::__gl_imports::libc::c_ushort;
pub type INT32 = i32;
pub type INT64 = i64;
pub type DWORD = super::__gl_imports::libc::c_ulong;

            #[repr(C)]
            pub struct POINTFLOAT {
                pub x: FLOAT,
                pub y: FLOAT,
            }
        

            #[repr(C)]
            pub struct GLYPHMETRICSFLOAT {
                pub gmfBlackBoxX: FLOAT,
                pub gmfBlackBoxY: FLOAT,
                pub gmfptGlyphOrigin: POINTFLOAT,
                pub gmfCellIncX: FLOAT,
                pub gmfCellIncY: FLOAT,
            }
        
pub type LPGLYPHMETRICSFLOAT = *const GLYPHMETRICSFLOAT;

            #[repr(C)]
            pub struct LAYERPLANEDESCRIPTOR {
                pub nSize: WORD,
                pub nVersion: WORD,
                pub dwFlags: DWORD,
                pub iPixelType: BYTE,
                pub cColorBits: BYTE,
                pub cRedBits: BYTE,
                pub cRedShift: BYTE,
                pub cGreenBits: BYTE,
                pub cGreenShift: BYTE,
                pub cBlueBits: BYTE,
                pub cBlueShift: BYTE,
                pub cAlphaBits: BYTE,
                pub cAlphaShift: BYTE,
                pub cAccumBits: BYTE,
                pub cAccumRedBits: BYTE,
                pub cAccumGreenBits: BYTE,
                pub cAccumBlueBits: BYTE,
                pub cAccumAlphaBits: BYTE,
                pub cDepthBits: BYTE,
                pub cStencilBits: BYTE,
                pub cAuxBuffers: BYTE,
                pub iLayerType: BYTE,
                pub bReserved: BYTE,
                pub crTransparent: COLORREF,
            }
        

            #[repr(C)]
            pub struct PIXELFORMATDESCRIPTOR {
                pub nSize: WORD,
                pub nVersion: WORD,
                pub dwFlags: DWORD,
                pub iPixelType: BYTE,
                pub cColorBits: BYTE,
                pub cRedBits: BYTE,
                pub cRedShift: BYTE,
                pub cGreenBits: BYTE,
                pub cGreenShift: BYTE,
                pub cBlueBits: BYTE,
                pub cBlueShift: BYTE,
                pub cAlphaBits: BYTE,
                pub cAlphaShift: BYTE,
                pub cAccumBits: BYTE,
                pub cAccumRedBits: BYTE,
                pub cAccumGreenBits: BYTE,
                pub cAccumBlueBits: BYTE,
                pub cAccumAlphaBits: BYTE,
                pub cDepthBits: BYTE,
                pub cStencilBits: BYTE,
                pub cAuxBuffers: BYTE,
                pub iLayerType: BYTE,
                pub bReserved: BYTE,
                pub dwLayerMask: DWORD,
                pub dwVisibleMask: DWORD,
                pub dwDamageMask: DWORD,
            }
        
pub type HPBUFFERARB = *const super::__gl_imports::libc::c_void;
pub type HPBUFFEREXT = *const super::__gl_imports::libc::c_void;
pub type HVIDEOOUTPUTDEVICENV = *const super::__gl_imports::libc::c_void;
pub type HPVIDEODEV = *const super::__gl_imports::libc::c_void;
pub type HPGPUNV = *const super::__gl_imports::libc::c_void;
pub type HGPUNV = *const super::__gl_imports::libc::c_void;
pub type HVIDEOINPUTDEVICENV = *const super::__gl_imports::libc::c_void;

            #[repr(C)]
            pub struct _GPU_DEVICE {
                cb: DWORD,
                DeviceName: [CHAR; 32],
                DeviceString: [CHAR; 128],
                Flags: DWORD,
                rcVirtualScreen: RECT,
            }
        
pub struct GPU_DEVICE(_GPU_DEVICE);
pub struct PGPU_DEVICE(*const _GPU_DEVICE);
}
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_MAIN_PLANE: types::GLenum = 0x00000001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY1: types::GLenum = 0x00000002; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY2: types::GLenum = 0x00000004; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY3: types::GLenum = 0x00000008; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY4: types::GLenum = 0x00000010; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY5: types::GLenum = 0x00000020; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY6: types::GLenum = 0x00000040; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY7: types::GLenum = 0x00000080; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY8: types::GLenum = 0x00000100; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY9: types::GLenum = 0x00000200; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY10: types::GLenum = 0x00000400; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY11: types::GLenum = 0x00000800; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY12: types::GLenum = 0x00001000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY13: types::GLenum = 0x00002000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY14: types::GLenum = 0x00004000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_OVERLAY15: types::GLenum = 0x00008000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY1: types::GLenum = 0x00010000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY2: types::GLenum = 0x00020000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY3: types::GLenum = 0x00040000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY4: types::GLenum = 0x00080000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY5: types::GLenum = 0x00100000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY6: types::GLenum = 0x00200000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY7: types::GLenum = 0x00400000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY8: types::GLenum = 0x00800000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY9: types::GLenum = 0x01000000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY10: types::GLenum = 0x02000000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY11: types::GLenum = 0x04000000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY12: types::GLenum = 0x08000000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY13: types::GLenum = 0x10000000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY14: types::GLenum = 0x20000000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDERLAY15: types::GLenum = 0x40000000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_DEBUG_BIT_ARB: types::GLenum = 0x00000001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: types::GLenum = 0x00000002; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_ROBUST_ACCESS_BIT_ARB: types::GLenum = 0x00000004; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_CORE_PROFILE_BIT_ARB: types::GLenum = 0x00000001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: types::GLenum = 0x00000002; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_ES2_PROFILE_BIT_EXT: types::GLenum = 0x00000004; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FONT_LINES: types::GLenum = 0; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FONT_POLYGONS: types::GLenum = 1; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NUMBER_PIXEL_FORMATS_ARB: types::GLenum = 0x2000; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DRAW_TO_WINDOW_ARB: types::GLenum = 0x2001; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DRAW_TO_BITMAP_ARB: types::GLenum = 0x2002; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCELERATION_ARB: types::GLenum = 0x2003; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NEED_PALETTE_ARB: types::GLenum = 0x2004; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NEED_SYSTEM_PALETTE_ARB: types::GLenum = 0x2005; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_LAYER_BUFFERS_ARB: types::GLenum = 0x2006; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_METHOD_ARB: types::GLenum = 0x2007; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NUMBER_OVERLAYS_ARB: types::GLenum = 0x2008; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NUMBER_UNDERLAYS_ARB: types::GLenum = 0x2009; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRANSPARENT_ARB: types::GLenum = 0x200A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SHARE_DEPTH_ARB: types::GLenum = 0x200C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SHARE_STENCIL_ARB: types::GLenum = 0x200D; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SHARE_ACCUM_ARB: types::GLenum = 0x200E; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SUPPORT_GDI_ARB: types::GLenum = 0x200F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SUPPORT_OPENGL_ARB: types::GLenum = 0x2010; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DOUBLE_BUFFER_ARB: types::GLenum = 0x2011; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STEREO_ARB: types::GLenum = 0x2012; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const PIXEL_TYPE_ARB: types::GLenum = 0x2013; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const COLOR_BITS_ARB: types::GLenum = 0x2014; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RED_BITS_ARB: types::GLenum = 0x2015; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const RED_SHIFT_ARB: types::GLenum = 0x2016; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GREEN_BITS_ARB: types::GLenum = 0x2017; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GREEN_SHIFT_ARB: types::GLenum = 0x2018; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BLUE_BITS_ARB: types::GLenum = 0x2019; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const BLUE_SHIFT_ARB: types::GLenum = 0x201A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA_BITS_ARB: types::GLenum = 0x201B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ALPHA_SHIFT_ARB: types::GLenum = 0x201C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_BITS_ARB: types::GLenum = 0x201D; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_RED_BITS_ARB: types::GLenum = 0x201E; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_GREEN_BITS_ARB: types::GLenum = 0x201F; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_BLUE_BITS_ARB: types::GLenum = 0x2020; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const ACCUM_ALPHA_BITS_ARB: types::GLenum = 0x2021; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const DEPTH_BITS_ARB: types::GLenum = 0x2022; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const STENCIL_BITS_ARB: types::GLenum = 0x2023; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const AUX_BUFFERS_ARB: types::GLenum = 0x2024; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NO_ACCELERATION_ARB: types::GLenum = 0x2025; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const GENERIC_ACCELERATION_ARB: types::GLenum = 0x2026; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FULL_ACCELERATION_ARB: types::GLenum = 0x2027; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_EXCHANGE_ARB: types::GLenum = 0x2028; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_COPY_ARB: types::GLenum = 0x2029; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SWAP_UNDEFINED_ARB: types::GLenum = 0x202A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TYPE_RGBA_ARB: types::GLenum = 0x202B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TYPE_COLORINDEX_ARB: types::GLenum = 0x202C; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRANSPARENT_RED_VALUE_ARB: types::GLenum = 0x2037; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRANSPARENT_GREEN_VALUE_ARB: types::GLenum = 0x2038; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRANSPARENT_BLUE_VALUE_ARB: types::GLenum = 0x2039; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRANSPARENT_ALPHA_VALUE_ARB: types::GLenum = 0x203A; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const TRANSPARENT_INDEX_VALUE_ARB: types::GLenum = 0x203B; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SAMPLE_BUFFERS_ARB: types::GLenum = 0x2041; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const SAMPLES_ARB: types::GLenum = 0x2042; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_MAJOR_VERSION_ARB: types::GLenum = 0x2091; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_MINOR_VERSION_ARB: types::GLenum = 0x2092; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_LAYER_PLANE_ARB: types::GLenum = 0x2093; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_FLAGS_ARB: types::GLenum = 0x2094; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FRAMEBUFFER_SRGB_CAPABLE_ARB: types::GLenum = 0x20A9; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const FRAMEBUFFER_SRGB_CAPABLE_EXT: types::GLenum = 0x20A9; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const LOSE_CONTEXT_ON_RESET_ARB: types::GLenum = 0x8252; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB: types::GLenum = 0x8256; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const NO_RESET_NOTIFICATION_ARB: types::GLenum = 0x8261; 
#[allow(dead_code)]
        #[allow(non_upper_case_globals)]
        pub const CONTEXT_PROFILE_MASK_ARB: types::GLenum = 0x9126; 

        #[allow(dead_code)]
        #[allow(missing_copy_implementations)]
        #[allow(raw_pointer_derive)]
        #[derive(Clone)]
        pub struct FnPtr {
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::libc::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }

        impl FnPtr {
            /// Creates a `FnPtr` from a load attempt.
            fn new(ptr: *const __gl_imports::libc::c_void) -> FnPtr {
                if ptr.is_null() {
                    FnPtr {
                        f: missing_fn_panic as *const __gl_imports::libc::c_void,
                        is_loaded: false
                    }
                } else {
                    FnPtr { f: ptr, is_loaded: true }
                }
            }

            /// Returns `true` if the function has been successfully loaded.
            ///
            /// If it returns `false`, calling the corresponding function will fail.
            #[inline]
            #[allow(dead_code)]
            pub fn is_loaded(&self) -> bool {
                self.is_loaded
            }
        }
    
#[inline(never)]
        fn missing_fn_panic() -> ! {
            panic!("wgl function was not loaded")
        }

        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[derive(Clone)]
        pub struct Wgl {
pub ChoosePixelFormatARB: FnPtr,
pub CopyContext: FnPtr,
pub CreateContext: FnPtr,
pub CreateContextAttribsARB: FnPtr,
pub CreateLayerContext: FnPtr,
pub DeleteContext: FnPtr,
pub DescribeLayerPlane: FnPtr,
pub GetCurrentContext: FnPtr,
pub GetCurrentDC: FnPtr,
pub GetExtensionsStringARB: FnPtr,
pub GetExtensionsStringEXT: FnPtr,
pub GetLayerPaletteEntries: FnPtr,
pub GetPixelFormatAttribfvARB: FnPtr,
pub GetPixelFormatAttribivARB: FnPtr,
pub GetProcAddress: FnPtr,
pub GetSwapIntervalEXT: FnPtr,
pub MakeCurrent: FnPtr,
pub RealizeLayerPalette: FnPtr,
pub SetLayerPaletteEntries: FnPtr,
pub ShareLists: FnPtr,
pub SwapLayerBuffers: FnPtr,
pub SwapIntervalEXT: FnPtr,
pub UseFontBitmaps: FnPtr,
pub UseFontBitmapsA: FnPtr,
pub UseFontBitmapsW: FnPtr,
pub UseFontOutlines: FnPtr,
pub UseFontOutlinesA: FnPtr,
pub UseFontOutlinesW: FnPtr,
}
impl Wgl {
            /// Load each OpenGL symbol using a custom load function. This allows for the
            /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
            ///
            /// ~~~ignore
            /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
            /// ~~~
            #[allow(dead_code)]
            #[allow(unused_variables)]
            pub fn load_with<F>(mut loadfn: F) -> Wgl where F: FnMut(&str) -> *const __gl_imports::libc::c_void {
                let mut metaloadfn = |symbol: &str, symbols: &[&str]| {
                    let mut ptr = loadfn(symbol);
                    if ptr.is_null() {
                        for &sym in symbols.iter() {
                            ptr = loadfn(sym);
                            if !ptr.is_null() { break; }
                        }
                    }
                    ptr
                };
                Wgl {
ChoosePixelFormatARB: FnPtr::new(metaloadfn("wglChoosePixelFormatARB", &[])),
CopyContext: FnPtr::new(metaloadfn("wglCopyContext", &[])),
CreateContext: FnPtr::new(metaloadfn("wglCreateContext", &[])),
CreateContextAttribsARB: FnPtr::new(metaloadfn("wglCreateContextAttribsARB", &[])),
CreateLayerContext: FnPtr::new(metaloadfn("wglCreateLayerContext", &[])),
DeleteContext: FnPtr::new(metaloadfn("wglDeleteContext", &[])),
DescribeLayerPlane: FnPtr::new(metaloadfn("wglDescribeLayerPlane", &[])),
GetCurrentContext: FnPtr::new(metaloadfn("wglGetCurrentContext", &[])),
GetCurrentDC: FnPtr::new(metaloadfn("wglGetCurrentDC", &[])),
GetExtensionsStringARB: FnPtr::new(metaloadfn("wglGetExtensionsStringARB", &[])),
GetExtensionsStringEXT: FnPtr::new(metaloadfn("wglGetExtensionsStringEXT", &[])),
GetLayerPaletteEntries: FnPtr::new(metaloadfn("wglGetLayerPaletteEntries", &[])),
GetPixelFormatAttribfvARB: FnPtr::new(metaloadfn("wglGetPixelFormatAttribfvARB", &[])),
GetPixelFormatAttribivARB: FnPtr::new(metaloadfn("wglGetPixelFormatAttribivARB", &[])),
GetProcAddress: FnPtr::new(metaloadfn("wglGetProcAddress", &[])),
GetSwapIntervalEXT: FnPtr::new(metaloadfn("wglGetSwapIntervalEXT", &[])),
MakeCurrent: FnPtr::new(metaloadfn("wglMakeCurrent", &[])),
RealizeLayerPalette: FnPtr::new(metaloadfn("wglRealizeLayerPalette", &[])),
SetLayerPaletteEntries: FnPtr::new(metaloadfn("wglSetLayerPaletteEntries", &[])),
ShareLists: FnPtr::new(metaloadfn("wglShareLists", &[])),
SwapLayerBuffers: FnPtr::new(metaloadfn("wglSwapLayerBuffers", &[])),
SwapIntervalEXT: FnPtr::new(metaloadfn("wglSwapIntervalEXT", &[])),
UseFontBitmaps: FnPtr::new(metaloadfn("wglUseFontBitmaps", &[])),
UseFontBitmapsA: FnPtr::new(metaloadfn("wglUseFontBitmapsA", &[])),
UseFontBitmapsW: FnPtr::new(metaloadfn("wglUseFontBitmapsW", &[])),
UseFontOutlines: FnPtr::new(metaloadfn("wglUseFontOutlines", &[])),
UseFontOutlinesA: FnPtr::new(metaloadfn("wglUseFontOutlinesA", &[])),
UseFontOutlinesW: FnPtr::new(metaloadfn("wglUseFontOutlinesW", &[])),
}
        }

        /// Load each OpenGL symbol using a custom load function.
        ///
        /// ~~~ignore
        /// let gl = Gl::load(&glfw);
        /// ~~~
        #[allow(dead_code)]
        #[allow(unused_variables)]
        pub fn load<T: __gl_imports::gl_common::GlFunctionsSource>(loader: &T) -> Wgl {
            Wgl::load_with(|name| loader.get_proc_addr(name))
        }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ChoosePixelFormatARB(&self, hdc: types::HDC, piAttribIList: *const __gl_imports::libc::c_int, pfAttribFList: *const types::FLOAT, nMaxFormats: types::UINT, piFormats: *mut __gl_imports::libc::c_int, nNumFormats: *mut types::UINT) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, *const __gl_imports::libc::c_int, *const types::FLOAT, types::UINT, *mut __gl_imports::libc::c_int, *mut types::UINT) -> types::BOOL>(self.ChoosePixelFormatARB.f)(hdc, piAttribIList, pfAttribFList, nMaxFormats, piFormats, nNumFormats) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CopyContext(&self, hglrcSrc: types::HGLRC, hglrcDst: types::HGLRC, mask: types::UINT) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HGLRC, types::HGLRC, types::UINT) -> types::BOOL>(self.CopyContext.f)(hglrcSrc, hglrcDst, mask) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CreateContext(&self, hDc: types::HDC) -> types::HGLRC { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC) -> types::HGLRC>(self.CreateContext.f)(hDc) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CreateContextAttribsARB(&self, hDC: types::HDC, hShareContext: types::HGLRC, attribList: *const __gl_imports::libc::c_int) -> types::HGLRC { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, types::HGLRC, *const __gl_imports::libc::c_int) -> types::HGLRC>(self.CreateContextAttribsARB.f)(hDC, hShareContext, attribList) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn CreateLayerContext(&self, hDc: types::HDC, level: __gl_imports::libc::c_int) -> types::HGLRC { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, __gl_imports::libc::c_int) -> types::HGLRC>(self.CreateLayerContext.f)(hDc, level) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DeleteContext(&self, oldContext: types::HGLRC) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HGLRC) -> types::BOOL>(self.DeleteContext.f)(oldContext) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn DescribeLayerPlane(&self, hDc: types::HDC, pixelFormat: __gl_imports::libc::c_int, layerPlane: __gl_imports::libc::c_int, nBytes: types::UINT, plpd: *const types::LAYERPLANEDESCRIPTOR) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, __gl_imports::libc::c_int, __gl_imports::libc::c_int, types::UINT, *const types::LAYERPLANEDESCRIPTOR) -> types::BOOL>(self.DescribeLayerPlane.f)(hDc, pixelFormat, layerPlane, nBytes, plpd) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetCurrentContext(&self, ) -> types::HGLRC { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::HGLRC>(self.GetCurrentContext.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetCurrentDC(&self, ) -> types::HDC { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::HDC>(self.GetCurrentDC.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetExtensionsStringARB(&self, hdc: types::HDC) -> *const __gl_imports::libc::c_char { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC) -> *const __gl_imports::libc::c_char>(self.GetExtensionsStringARB.f)(hdc) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetExtensionsStringEXT(&self, ) -> *const __gl_imports::libc::c_char { __gl_imports::mem::transmute::<_, extern "system" fn() -> *const __gl_imports::libc::c_char>(self.GetExtensionsStringEXT.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetLayerPaletteEntries(&self, hdc: types::HDC, iLayerPlane: __gl_imports::libc::c_int, iStart: __gl_imports::libc::c_int, cEntries: __gl_imports::libc::c_int, pcr: *const types::COLORREF) -> __gl_imports::libc::c_int { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, __gl_imports::libc::c_int, __gl_imports::libc::c_int, __gl_imports::libc::c_int, *const types::COLORREF) -> __gl_imports::libc::c_int>(self.GetLayerPaletteEntries.f)(hdc, iLayerPlane, iStart, cEntries, pcr) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetPixelFormatAttribfvARB(&self, hdc: types::HDC, iPixelFormat: __gl_imports::libc::c_int, iLayerPlane: __gl_imports::libc::c_int, nAttributes: types::UINT, piAttributes: *const __gl_imports::libc::c_int, pfValues: *mut types::FLOAT) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, __gl_imports::libc::c_int, __gl_imports::libc::c_int, types::UINT, *const __gl_imports::libc::c_int, *mut types::FLOAT) -> types::BOOL>(self.GetPixelFormatAttribfvARB.f)(hdc, iPixelFormat, iLayerPlane, nAttributes, piAttributes, pfValues) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetPixelFormatAttribivARB(&self, hdc: types::HDC, iPixelFormat: __gl_imports::libc::c_int, iLayerPlane: __gl_imports::libc::c_int, nAttributes: types::UINT, piAttributes: *const __gl_imports::libc::c_int, piValues: *mut __gl_imports::libc::c_int) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, __gl_imports::libc::c_int, __gl_imports::libc::c_int, types::UINT, *const __gl_imports::libc::c_int, *mut __gl_imports::libc::c_int) -> types::BOOL>(self.GetPixelFormatAttribivARB.f)(hdc, iPixelFormat, iLayerPlane, nAttributes, piAttributes, piValues) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetProcAddress(&self, lpszProc: types::LPCSTR) -> types::PROC { __gl_imports::mem::transmute::<_, extern "system" fn(types::LPCSTR) -> types::PROC>(self.GetProcAddress.f)(lpszProc) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn GetSwapIntervalEXT(&self, ) -> __gl_imports::libc::c_int { __gl_imports::mem::transmute::<_, extern "system" fn() -> __gl_imports::libc::c_int>(self.GetSwapIntervalEXT.f)() }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn MakeCurrent(&self, hDc: types::HDC, newContext: types::HGLRC) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, types::HGLRC) -> types::BOOL>(self.MakeCurrent.f)(hDc, newContext) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn RealizeLayerPalette(&self, hdc: types::HDC, iLayerPlane: __gl_imports::libc::c_int, bRealize: types::BOOL) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, __gl_imports::libc::c_int, types::BOOL) -> types::BOOL>(self.RealizeLayerPalette.f)(hdc, iLayerPlane, bRealize) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn SetLayerPaletteEntries(&self, hdc: types::HDC, iLayerPlane: __gl_imports::libc::c_int, iStart: __gl_imports::libc::c_int, cEntries: __gl_imports::libc::c_int, pcr: *const types::COLORREF) -> __gl_imports::libc::c_int { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, __gl_imports::libc::c_int, __gl_imports::libc::c_int, __gl_imports::libc::c_int, *const types::COLORREF) -> __gl_imports::libc::c_int>(self.SetLayerPaletteEntries.f)(hdc, iLayerPlane, iStart, cEntries, pcr) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn ShareLists(&self, hrcSrvShare: types::HGLRC, hrcSrvSource: types::HGLRC) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HGLRC, types::HGLRC) -> types::BOOL>(self.ShareLists.f)(hrcSrvShare, hrcSrvSource) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn SwapLayerBuffers(&self, hdc: types::HDC, fuFlags: types::UINT) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, types::UINT) -> types::BOOL>(self.SwapLayerBuffers.f)(hdc, fuFlags) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn SwapIntervalEXT(&self, interval: __gl_imports::libc::c_int) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(__gl_imports::libc::c_int) -> types::BOOL>(self.SwapIntervalEXT.f)(interval) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn UseFontBitmaps(&self, hDC: types::HDC, first: types::DWORD, count: types::DWORD, listBase: types::DWORD) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, types::DWORD, types::DWORD, types::DWORD) -> types::BOOL>(self.UseFontBitmaps.f)(hDC, first, count, listBase) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn UseFontBitmapsA(&self, hDC: types::HDC, first: types::DWORD, count: types::DWORD, listBase: types::DWORD) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, types::DWORD, types::DWORD, types::DWORD) -> types::BOOL>(self.UseFontBitmapsA.f)(hDC, first, count, listBase) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn UseFontBitmapsW(&self, hDC: types::HDC, first: types::DWORD, count: types::DWORD, listBase: types::DWORD) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, types::DWORD, types::DWORD, types::DWORD) -> types::BOOL>(self.UseFontBitmapsW.f)(hDC, first, count, listBase) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn UseFontOutlines(&self, hDC: types::HDC, first: types::DWORD, count: types::DWORD, listBase: types::DWORD, deviation: types::FLOAT, extrusion: types::FLOAT, format: __gl_imports::libc::c_int, lpgmf: types::LPGLYPHMETRICSFLOAT) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, types::DWORD, types::DWORD, types::DWORD, types::FLOAT, types::FLOAT, __gl_imports::libc::c_int, types::LPGLYPHMETRICSFLOAT) -> types::BOOL>(self.UseFontOutlines.f)(hDC, first, count, listBase, deviation, extrusion, format, lpgmf) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn UseFontOutlinesA(&self, hDC: types::HDC, first: types::DWORD, count: types::DWORD, listBase: types::DWORD, deviation: types::FLOAT, extrusion: types::FLOAT, format: __gl_imports::libc::c_int, lpgmf: types::LPGLYPHMETRICSFLOAT) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, types::DWORD, types::DWORD, types::DWORD, types::FLOAT, types::FLOAT, __gl_imports::libc::c_int, types::LPGLYPHMETRICSFLOAT) -> types::BOOL>(self.UseFontOutlinesA.f)(hDC, first, count, listBase, deviation, extrusion, format, lpgmf) }
#[allow(non_snake_case)] #[allow(unused_variables)] #[allow(dead_code)]
            #[inline] pub unsafe fn UseFontOutlinesW(&self, hDC: types::HDC, first: types::DWORD, count: types::DWORD, listBase: types::DWORD, deviation: types::FLOAT, extrusion: types::FLOAT, format: __gl_imports::libc::c_int, lpgmf: types::LPGLYPHMETRICSFLOAT) -> types::BOOL { __gl_imports::mem::transmute::<_, extern "system" fn(types::HDC, types::DWORD, types::DWORD, types::DWORD, types::FLOAT, types::FLOAT, __gl_imports::libc::c_int, types::LPGLYPHMETRICSFLOAT) -> types::BOOL>(self.UseFontOutlinesW.f)(hDC, first, count, listBase, deviation, extrusion, format, lpgmf) }
}

        unsafe impl __gl_imports::Send for Wgl {}
