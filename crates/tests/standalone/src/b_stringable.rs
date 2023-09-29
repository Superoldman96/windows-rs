// Bindings generated by `windows-bindgen` 0.52.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStringable(::windows_core::IUnknown);
impl IStringable {
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IStringable,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IStringable {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
}
unsafe impl ::windows_core::Interface for IStringable {
    type Vtable = IStringable_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStringable {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x96369f54_8eb6_48f0_abce_c1b211e627c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringable_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ToString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}