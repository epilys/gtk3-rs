// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventButton(::Event);

event_wrapper!(EventButton, GdkEventButton);
event_subtype!(EventButton, ButtonPress | _2buttonPress | _3buttonPress | ButtonRelease);
