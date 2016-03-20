//! Macros

/// Rust string to UTF-8 conversion. See also `utf::u2s`.
/// 
/// # Example:
/// 
/// ```ignore
/// let (cstr, len) = s2u("hello"); // ffi::CString
/// libc::printf("%.*hs", len, cstr.as_ptr());
/// ```
/// 
#[macro_export]
macro_rules! s2u {
	($s:expr) => ( ::utf::s2un($s) )
}

/// Rust string to UTF-16 conversion. See also `utf::w2s`.
/// 
/// # Example:
/// 
/// ```ignore
/// let (cwstr, len) = s2w("hello"); // Vec<u16>
/// libc::printf("%.*ws", len, cwstr.as_ptr());
/// ```
/// 
#[macro_export]
macro_rules! s2w {
	($s:expr) => ( ::utf::s2vecn($s) )
}

/// UTF-16 to `String` conversion.
#[macro_export]
macro_rules! w2s {
	($s:expr) => ( ::utf::w2s($s) )
}

/// UTF-8 to `String` conversion.
#[macro_export]
macro_rules! u2s {
	($s:expr) => ( ::utf::u2s($s) )
}