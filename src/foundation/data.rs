use std::mem;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use std::slice;

use block2::{Block, ConcreteBlock};

use objc2::rc::Retained;
use objc2::runtime::Object;
use objc2::{class, msg_send, msg_send_id, sel};

use crate::foundation::{BOOL, NO, YES, id, to_bool};

/// Wrapper for a retained `NSData` object.
///
/// Supports constructing a new `NSData` from a `Vec<u8>`, wrapping and retaining an existing
/// pointer from the Objective-C side, and turning an `NSData` into a `Vec<u8>`.
///
/// This is an intentionally limited API.
pub use objc2_foundation::NSData;
