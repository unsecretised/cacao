//! This module implements forwarding methods for standard `NSApplicationDelegate` calls. It also
//! creates a custom `NSApplication` subclass that currently does nothing; this is meant as a hook
//! for potential future use.

use objc2::runtime::AnyClass;

use crate::foundation::load_or_register_class;

/// Used for injecting a custom NSApplication. Currently does nothing.
pub(crate) fn register_app_class() -> &'static AnyClass {
    load_or_register_class("NSApplication", "RSTApplication", |_decl| {})
}
