// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FileChooser;
#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
use glib::translate::*;
use glib::IsA;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`FileChooser`](crate::FileChooser).
pub trait FileChooserExtManual: 'static {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gtk_file_chooser_add_choice")]
    fn add_choice(&self, id: &str, label: &str, options: &[(&str, &str)]);
}

impl<O: IsA<FileChooser>> FileChooserExtManual for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    fn add_choice(&self, id: &str, label: &str, options: &[(&str, &str)]) {
        unsafe {
            let stashes_ids = options
                .iter()
                .map(|o| o.0.to_glib_none())
                .collect::<Vec<_>>();
            let stashes_labels = options
                .iter()
                .map(|o| o.1.to_glib_none())
                .collect::<Vec<_>>();
            let (options_ids, options_labels) = (
                stashes_ids
                    .iter()
                    .map(|o| o.0)
                    .collect::<Vec<*const libc::c_char>>()
                    .as_ptr(),
                stashes_labels
                    .iter()
                    .map(|o| o.0)
                    .collect::<Vec<*const libc::c_char>>()
                    .as_ptr(),
            );

            ffi::gtk_file_chooser_add_choice(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                label.to_glib_none().0,
                mut_override(options_ids),
                mut_override(options_labels),
            );
        }
    }
}
