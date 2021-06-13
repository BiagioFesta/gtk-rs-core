// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AskPasswordFlags;
use crate::MountOperationResult;
use crate::PasswordSave;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GMountOperation")]
    pub struct MountOperation(Object<ffi::GMountOperation, ffi::GMountOperationClass>);

    match fn {
        type_ => || ffi::g_mount_operation_get_type(),
    }
}

impl MountOperation {
    #[doc(alias = "g_mount_operation_new")]
    pub fn new() -> MountOperation {
        unsafe { from_glib_full(ffi::g_mount_operation_new()) }
    }
}

impl Default for MountOperation {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MOUNT_OPERATION: Option<&MountOperation> = None;

pub trait MountOperationExt: 'static {
    #[doc(alias = "g_mount_operation_get_anonymous")]
    #[doc(alias = "get_anonymous")]
    fn is_anonymous(&self) -> bool;

    #[doc(alias = "g_mount_operation_get_choice")]
    #[doc(alias = "get_choice")]
    fn choice(&self) -> i32;

    #[doc(alias = "g_mount_operation_get_domain")]
    #[doc(alias = "get_domain")]
    fn domain(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_mount_operation_get_is_tcrypt_hidden_volume")]
    #[doc(alias = "get_is_tcrypt_hidden_volume")]
    fn is_tcrypt_hidden_volume(&self) -> bool;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_mount_operation_get_is_tcrypt_system_volume")]
    #[doc(alias = "get_is_tcrypt_system_volume")]
    fn is_tcrypt_system_volume(&self) -> bool;

    #[doc(alias = "g_mount_operation_get_password")]
    #[doc(alias = "get_password")]
    fn password(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mount_operation_get_password_save")]
    #[doc(alias = "get_password_save")]
    fn password_save(&self) -> PasswordSave;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_mount_operation_get_pim")]
    #[doc(alias = "get_pim")]
    fn pim(&self) -> u32;

    #[doc(alias = "g_mount_operation_get_username")]
    #[doc(alias = "get_username")]
    fn username(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mount_operation_reply")]
    fn reply(&self, result: MountOperationResult);

    #[doc(alias = "g_mount_operation_set_anonymous")]
    fn set_anonymous(&self, anonymous: bool);

    #[doc(alias = "g_mount_operation_set_choice")]
    fn set_choice(&self, choice: i32);

    #[doc(alias = "g_mount_operation_set_domain")]
    fn set_domain(&self, domain: Option<&str>);

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_mount_operation_set_is_tcrypt_hidden_volume")]
    fn set_is_tcrypt_hidden_volume(&self, hidden_volume: bool);

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_mount_operation_set_is_tcrypt_system_volume")]
    fn set_is_tcrypt_system_volume(&self, system_volume: bool);

    #[doc(alias = "g_mount_operation_set_password")]
    fn set_password(&self, password: Option<&str>);

    #[doc(alias = "g_mount_operation_set_password_save")]
    fn set_password_save(&self, save: PasswordSave);

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_mount_operation_set_pim")]
    fn set_pim(&self, pim: u32);

    #[doc(alias = "g_mount_operation_set_username")]
    fn set_username(&self, username: Option<&str>);

    #[doc(alias = "aborted")]
    fn connect_aborted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "ask-password")]
    fn connect_ask_password<F: Fn(&Self, &str, &str, &str, AskPasswordFlags) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    //#[doc(alias = "ask-question")]
    //fn connect_ask_question<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "reply")]
    fn connect_reply<F: Fn(&Self, MountOperationResult) + 'static>(&self, f: F) -> SignalHandlerId;

    //#[doc(alias = "show-processes")]
    //fn connect_show_processes<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-unmount-progress")]
    fn connect_show_unmount_progress<F: Fn(&Self, &str, i64, i64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "anonymous")]
    fn connect_anonymous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "choice")]
    fn connect_choice_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "domain")]
    fn connect_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "is-tcrypt-hidden-volume")]
    fn connect_is_tcrypt_hidden_volume_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "is-tcrypt-system-volume")]
    fn connect_is_tcrypt_system_volume_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "password")]
    fn connect_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "password-save")]
    fn connect_password_save_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "pim")]
    fn connect_pim_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "username")]
    fn connect_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MountOperation>> MountOperationExt for O {
    fn is_anonymous(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mount_operation_get_anonymous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn choice(&self) -> i32 {
        unsafe { ffi::g_mount_operation_get_choice(self.as_ref().to_glib_none().0) }
    }

    fn domain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mount_operation_get_domain(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    fn is_tcrypt_hidden_volume(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mount_operation_get_is_tcrypt_hidden_volume(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    fn is_tcrypt_system_volume(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mount_operation_get_is_tcrypt_system_volume(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn password(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mount_operation_get_password(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn password_save(&self) -> PasswordSave {
        unsafe {
            from_glib(ffi::g_mount_operation_get_password_save(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    fn pim(&self) -> u32 {
        unsafe { ffi::g_mount_operation_get_pim(self.as_ref().to_glib_none().0) }
    }

    fn username(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mount_operation_get_username(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reply(&self, result: MountOperationResult) {
        unsafe {
            ffi::g_mount_operation_reply(self.as_ref().to_glib_none().0, result.into_glib());
        }
    }

    fn set_anonymous(&self, anonymous: bool) {
        unsafe {
            ffi::g_mount_operation_set_anonymous(
                self.as_ref().to_glib_none().0,
                anonymous.into_glib(),
            );
        }
    }

    fn set_choice(&self, choice: i32) {
        unsafe {
            ffi::g_mount_operation_set_choice(self.as_ref().to_glib_none().0, choice);
        }
    }

    fn set_domain(&self, domain: Option<&str>) {
        unsafe {
            ffi::g_mount_operation_set_domain(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    fn set_is_tcrypt_hidden_volume(&self, hidden_volume: bool) {
        unsafe {
            ffi::g_mount_operation_set_is_tcrypt_hidden_volume(
                self.as_ref().to_glib_none().0,
                hidden_volume.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    fn set_is_tcrypt_system_volume(&self, system_volume: bool) {
        unsafe {
            ffi::g_mount_operation_set_is_tcrypt_system_volume(
                self.as_ref().to_glib_none().0,
                system_volume.into_glib(),
            );
        }
    }

    fn set_password(&self, password: Option<&str>) {
        unsafe {
            ffi::g_mount_operation_set_password(
                self.as_ref().to_glib_none().0,
                password.to_glib_none().0,
            );
        }
    }

    fn set_password_save(&self, save: PasswordSave) {
        unsafe {
            ffi::g_mount_operation_set_password_save(
                self.as_ref().to_glib_none().0,
                save.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    fn set_pim(&self, pim: u32) {
        unsafe {
            ffi::g_mount_operation_set_pim(self.as_ref().to_glib_none().0, pim);
        }
    }

    fn set_username(&self, username: Option<&str>) {
        unsafe {
            ffi::g_mount_operation_set_username(
                self.as_ref().to_glib_none().0,
                username.to_glib_none().0,
            );
        }
    }

    fn connect_aborted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn aborted_trampoline<P: IsA<MountOperation>, F: Fn(&P) + 'static>(
            this: *mut ffi::GMountOperation,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"aborted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    aborted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ask_password<F: Fn(&Self, &str, &str, &str, AskPasswordFlags) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn ask_password_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P, &str, &str, &str, AskPasswordFlags) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            message: *mut libc::c_char,
            default_user: *mut libc::c_char,
            default_domain: *mut libc::c_char,
            flags: ffi::GAskPasswordFlags,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &MountOperation::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(message),
                &glib::GString::from_glib_borrow(default_user),
                &glib::GString::from_glib_borrow(default_domain),
                from_glib(flags),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"ask-password\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    ask_password_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_ask_question<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype choices: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    fn connect_reply<F: Fn(&Self, MountOperationResult) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn reply_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P, MountOperationResult) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            result: ffi::GMountOperationResult,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &MountOperation::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(result),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"reply\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    reply_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_show_processes<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype processes: *.Array TypeId { ns_id: 2, id: 4 }
    //    Empty ctype choices: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    fn connect_show_unmount_progress<F: Fn(&Self, &str, i64, i64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn show_unmount_progress_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P, &str, i64, i64) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            message: *mut libc::c_char,
            time_left: i64,
            bytes_left: i64,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &MountOperation::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(message),
                time_left,
                bytes_left,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-unmount-progress\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_unmount_progress_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_anonymous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_anonymous_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::anonymous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_anonymous_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_choice_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_choice_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::choice\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_choice_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_domain_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::domain\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_domain_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    fn connect_is_tcrypt_hidden_volume_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_tcrypt_hidden_volume_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-tcrypt-hidden-volume\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_tcrypt_hidden_volume_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    fn connect_is_tcrypt_system_volume_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_tcrypt_system_volume_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-tcrypt-system-volume\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_tcrypt_system_volume_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::password\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_password_save_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_save_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::password-save\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_save_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    fn connect_pim_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pim_trampoline<P: IsA<MountOperation>, F: Fn(&P) + 'static>(
            this: *mut ffi::GMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pim\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pim_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_username_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::username\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_username_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MountOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MountOperation")
    }
}
