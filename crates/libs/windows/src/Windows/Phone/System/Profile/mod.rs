windows_core::imp::define_interface!(IRetailModeStatics, IRetailModeStatics_Vtbl, 0xd7ded029_fdda_43e7_93fb_e53ab6e89ec3);
impl windows_core::RuntimeType for IRetailModeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRetailModeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RetailModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
pub struct RetailMode;
impl RetailMode {
    pub fn RetailModeEnabled() -> windows_core::Result<bool> {
        Self::IRetailModeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetailModeEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IRetailModeStatics<R, F: FnOnce(&IRetailModeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RetailMode, IRetailModeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for RetailMode {
    const NAME: &'static str = "Windows.Phone.System.Profile.RetailMode";
}
