// Take a look at the license at the top of the repository in the LICENSE file.

use std::{mem, ptr};

use glib::translate::*;
use gst::subclass::prelude::*;

use crate::{ffi, prelude::*, AudioDecoder};

pub trait AudioDecoderImpl: AudioDecoderImplExt + ElementImpl {
    fn open(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_open()
    }

    fn close(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_close()
    }

    fn start(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_start()
    }

    fn stop(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_stop()
    }

    fn set_format(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_format(caps)
    }

    fn parse(&self, adapter: &gst_base::Adapter) -> Result<(u32, u32), gst::FlowError> {
        self.parent_parse(adapter)
    }

    fn handle_frame(
        &self,
        buffer: Option<&gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_handle_frame(buffer)
    }

    fn pre_push(&self, buffer: gst::Buffer) -> Result<Option<gst::Buffer>, gst::FlowError> {
        self.parent_pre_push(buffer)
    }

    fn flush(&self, hard: bool) {
        self.parent_flush(hard)
    }

    fn negotiate(&self) -> Result<(), gst::LoggableError> {
        self.parent_negotiate()
    }

    fn caps(&self, filter: Option<&gst::Caps>) -> gst::Caps {
        self.parent_caps(filter)
    }

    fn sink_event(&self, event: gst::Event) -> bool {
        self.parent_sink_event(event)
    }

    fn sink_query(&self, query: &mut gst::QueryRef) -> bool {
        self.parent_sink_query(query)
    }

    fn src_event(&self, event: gst::Event) -> bool {
        self.parent_src_event(event)
    }

    fn src_query(&self, query: &mut gst::QueryRef) -> bool {
        self.parent_src_query(query)
    }

    fn propose_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        self.parent_propose_allocation(query)
    }

    fn decide_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        self.parent_decide_allocation(query)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AudioDecoderImplExt> Sealed for T {}
}

pub trait AudioDecoderImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_open(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .open
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<AudioDecoder>()
                        .to_glib_none()
                        .0))
                    {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `open` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_close(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .close
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<AudioDecoder>()
                        .to_glib_none()
                        .0))
                    {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `close` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_start(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<AudioDecoder>()
                        .to_glib_none()
                        .0))
                    {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `start` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_stop(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<AudioDecoder>()
                        .to_glib_none()
                        .0))
                    {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `stop` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_set_format(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .set_format
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<AudioDecoder>()
                                .to_glib_none()
                                .0,
                            caps.to_glib_none().0
                        ),
                        gst::CAT_RUST,
                        "parent function `set_format` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_parse(&self, adapter: &gst_base::Adapter) -> Result<(u32, u32), gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .parse
                .map(|f| {
                    let mut offset = mem::MaybeUninit::uninit();
                    let mut len = mem::MaybeUninit::uninit();
                    gst::FlowSuccess::try_from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<AudioDecoder>()
                            .to_glib_none()
                            .0,
                        adapter.to_glib_none().0,
                        offset.as_mut_ptr(),
                        len.as_mut_ptr(),
                    ))
                    .map(|_| {
                        let offset = offset.assume_init();
                        let len = len.assume_init();
                        assert!(offset >= 0);
                        assert!(len >= 0);
                        (offset as u32, len as u32)
                    })
                })
                .unwrap_or_else(|| Ok((0, adapter.available() as u32)))
        }
    }

    fn parent_handle_frame(
        &self,
        buffer: Option<&gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .handle_frame
                .map(|f| {
                    try_from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<AudioDecoder>()
                            .to_glib_none()
                            .0,
                        buffer
                            .map(|buffer| buffer.as_mut_ptr() as *mut *mut gst::ffi::GstBuffer)
                            .unwrap_or(ptr::null_mut()),
                    ))
                })
                .unwrap_or(Err(gst::FlowError::Error))
        }
    }

    fn parent_pre_push(&self, buffer: gst::Buffer) -> Result<Option<gst::Buffer>, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            if let Some(f) = (*parent_class).pre_push {
                let mut buffer = buffer.into_glib_ptr();
                gst::FlowSuccess::try_from_glib(f(
                    self.obj()
                        .unsafe_cast_ref::<AudioDecoder>()
                        .to_glib_none()
                        .0,
                    &mut buffer,
                ))
                .map(|_| from_glib_full(buffer))
            } else {
                Ok(Some(buffer))
            }
        }
    }

    fn parent_flush(&self, hard: bool) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .flush
                .map(|f| {
                    f(
                        self.obj()
                            .unsafe_cast_ref::<AudioDecoder>()
                            .to_glib_none()
                            .0,
                        hard.into_glib(),
                    )
                })
                .unwrap_or(())
        }
    }

    fn parent_negotiate(&self) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .negotiate
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(self
                            .obj()
                            .unsafe_cast_ref::<AudioDecoder>()
                            .to_glib_none()
                            .0),
                        gst::CAT_RUST,
                        "Parent function `negotiate` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_caps(&self, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .getcaps
                .map(|f| {
                    from_glib_full(f(
                        self.obj()
                            .unsafe_cast_ref::<AudioDecoder>()
                            .to_glib_none()
                            .0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or_else(|| {
                    self.obj()
                        .unsafe_cast_ref::<AudioDecoder>()
                        .proxy_getcaps(None, filter)
                })
        }
    }

    fn parent_sink_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            let f = (*parent_class)
                .sink_event
                .expect("Missing parent function `sink_event`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<AudioDecoder>()
                    .to_glib_none()
                    .0,
                event.into_glib_ptr(),
            ))
        }
    }

    fn parent_sink_query(&self, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            let f = (*parent_class)
                .sink_query
                .expect("Missing parent function `sink_query`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<AudioDecoder>()
                    .to_glib_none()
                    .0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn parent_src_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            let f = (*parent_class)
                .src_event
                .expect("Missing parent function `src_event`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<AudioDecoder>()
                    .to_glib_none()
                    .0,
                event.into_glib_ptr(),
            ))
        }
    }

    fn parent_src_query(&self, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            let f = (*parent_class)
                .src_query
                .expect("Missing parent function `src_query`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<AudioDecoder>()
                    .to_glib_none()
                    .0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn parent_propose_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .propose_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<AudioDecoder>()
                                .to_glib_none()
                                .0,
                            query.as_mut_ptr(),
                        ),
                        gst::CAT_RUST,
                        "Parent function `propose_allocation` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_decide_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .decide_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<AudioDecoder>()
                                .to_glib_none()
                                .0,
                            query.as_mut_ptr(),
                        ),
                        gst::CAT_RUST,
                        "Parent function `decide_allocation` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }
}

impl<T: AudioDecoderImpl> AudioDecoderImplExt for T {}

unsafe impl<T: AudioDecoderImpl> IsSubclassable<T> for AudioDecoder {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.open = Some(audio_decoder_open::<T>);
        klass.close = Some(audio_decoder_close::<T>);
        klass.start = Some(audio_decoder_start::<T>);
        klass.stop = Some(audio_decoder_stop::<T>);
        klass.set_format = Some(audio_decoder_set_format::<T>);
        klass.parse = Some(audio_decoder_parse::<T>);
        klass.handle_frame = Some(audio_decoder_handle_frame::<T>);
        klass.pre_push = Some(audio_decoder_pre_push::<T>);
        klass.flush = Some(audio_decoder_flush::<T>);
        klass.negotiate = Some(audio_decoder_negotiate::<T>);
        klass.getcaps = Some(audio_decoder_getcaps::<T>);
        klass.sink_event = Some(audio_decoder_sink_event::<T>);
        klass.src_event = Some(audio_decoder_src_event::<T>);
        klass.sink_query = Some(audio_decoder_sink_query::<T>);
        klass.src_query = Some(audio_decoder_src_query::<T>);
        klass.propose_allocation = Some(audio_decoder_propose_allocation::<T>);
        klass.decide_allocation = Some(audio_decoder_decide_allocation::<T>);
    }
}

unsafe extern "C" fn audio_decoder_open<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.open() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_close<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.close() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_start<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.start() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_stop<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.stop() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_set_format<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.set_format(&from_glib_borrow(caps)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_parse<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    adapter: *mut gst_base::ffi::GstAdapter,
    offset: *mut i32,
    len: *mut i32,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        match imp.parse(&from_glib_borrow(adapter)) {
            Ok((new_offset, new_len)) => {
                assert!(new_offset <= i32::MAX as u32);
                assert!(new_len <= i32::MAX as u32);
                *offset = new_offset as i32;
                *len = new_len as i32;
                Ok(gst::FlowSuccess::Ok)
            }
            Err(err) => Err(err),
        }
        .into()
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_handle_frame<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    buffer: *mut *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    // FIXME: Misgenerated in gstreamer-audio-sys
    let buffer = buffer as *mut gst::ffi::GstBuffer;
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        imp.handle_frame(Option::<gst::Buffer>::from_glib_none(buffer).as_ref())
            .into()
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_pre_push<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    buffer: *mut *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        match imp.pre_push(from_glib_full(*buffer)) {
            Ok(Some(new_buffer)) => {
                *buffer = new_buffer.into_glib_ptr();
                Ok(gst::FlowSuccess::Ok)
            }
            Ok(None) => {
                *buffer = ptr::null_mut();
                Ok(gst::FlowSuccess::Ok)
            }
            Err(err) => Err(err),
        }
        .into()
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_flush<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    hard: glib::ffi::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, (), { AudioDecoderImpl::flush(imp, from_glib(hard)) })
}

unsafe extern "C" fn audio_decoder_negotiate<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.negotiate() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_getcaps<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::Caps::new_empty(), {
        AudioDecoderImpl::caps(
            imp,
            Option::<gst::Caps>::from_glib_borrow(filter)
                .as_ref()
                .as_ref(),
        )
    })
    .into_glib_ptr()
}

unsafe extern "C" fn audio_decoder_sink_event<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { imp.sink_event(from_glib_full(event)) }).into_glib()
}

unsafe extern "C" fn audio_decoder_sink_query<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        imp.sink_query(gst::QueryRef::from_mut_ptr(query))
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_src_event<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { imp.src_event(from_glib_full(event)) }).into_glib()
}

unsafe extern "C" fn audio_decoder_src_query<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        imp.src_query(gst::QueryRef::from_mut_ptr(query))
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_propose_allocation<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let query = match gst::QueryRef::from_mut_ptr(query).view_mut() {
        gst::QueryViewMut::Allocation(allocation) => allocation,
        _ => unreachable!(),
    };

    gst::panic_to_error!(imp, false, {
        match imp.propose_allocation(query) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audio_decoder_decide_allocation<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let query = match gst::QueryRef::from_mut_ptr(query).view_mut() {
        gst::QueryViewMut::Allocation(allocation) => allocation,
        _ => unreachable!(),
    };

    gst::panic_to_error!(imp, false, {
        match imp.decide_allocation(query) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}
