// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate bitflags;
extern crate libc;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer_sys as ffi;

#[macro_use]
extern crate glib;

extern crate num_rational;

use glib::translate::{from_glib, from_glib_full};

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

macro_rules! assert_initialized_main_thread {
    () => (
        assert_eq!(unsafe {ffi::gst_is_initialized()}, ::glib_ffi::GTRUE)
    )
}

macro_rules! skip_assert_initialized {
    () => (
    )
}

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

mod auto;
pub use auto::*;
pub use auto::traits::*;
pub use auto::functions::{parse_launch, parse_bin_from_description};
pub use auto::traits::ObjectExt as GstObjectExt;

pub mod miniobject;
pub use miniobject::GstRc;
pub mod message;
pub use message::Message;
pub use message::MessageView;
pub mod structure;
pub use structure::Structure;
pub mod caps;
pub use caps::Caps;
pub mod tags;
pub use tags::*;

mod element;
mod bin;
mod bus;
pub use bin::BinExtManual;

mod value;
pub use value::*;

use std::ptr;

pub fn init() -> Result<(), glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        if from_glib(ffi::gst_init_check(
            ptr::null_mut(),
            ptr::null_mut(),
            &mut error,
        )) {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}