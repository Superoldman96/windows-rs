pub trait IChatItem_Impl: Sized + windows_core::IUnknownImpl {
    fn ItemKind(&self) -> windows_core::Result<ChatItemKind>;
}
impl windows_core::RuntimeName for IChatItem {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.IChatItem";
}
impl IChatItem_Vtbl {
    pub const fn new<Identity: IChatItem_Impl, const OFFSET: isize>() -> IChatItem_Vtbl {
        unsafe extern "system" fn ItemKind<Identity: IChatItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ChatItemKind) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChatItem_Impl::ItemKind(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IChatItem, OFFSET>(), ItemKind: ItemKind::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChatItem as windows_core::Interface>::IID
    }
}
