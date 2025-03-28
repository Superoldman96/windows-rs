windows_targets::link!("kernel32.dll" "system" fn ActivatePackageVirtualizationContext(context : PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE, cookie : *mut usize) -> windows_sys::core::HRESULT);
windows_targets::link!("kernelbase.dll" "system" fn AddPackageDependency(packagedependencyid : windows_sys::core::PCWSTR, rank : i32, options : AddPackageDependencyOptions, packagedependencycontext : *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetClrCompat(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyClrCompat) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetCreateFileAccess(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyCreateFileAccess) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetLifecycleManagement(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyLifecycleManagement) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetMediaFoundationCodecLoading(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyMediaFoundationCodecLoading) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetProcessTerminationMethod(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyProcessTerminationMethod) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetShowDeveloperDiagnostic(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyShowDeveloperDiagnostic) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetThreadInitializationType(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyThreadInitializationType) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetWindowingModel(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyWindowingModel) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn CheckIsMSIXPackage(packagefullname : windows_sys::core::PCWSTR, ismsixpackage : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn ClosePackageInfo(packageinforeference : *const _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn CreatePackageVirtualizationContext(packagefamilyname : windows_sys::core::PCWSTR, context : *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn DeactivatePackageVirtualizationContext(cookie : usize));
windows_targets::link!("kernelbase.dll" "system" fn DeletePackageDependency(packagedependencyid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn DuplicatePackageVirtualizationContext(sourcecontext : PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE, destcontext : *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn FindPackagesByPackageFamily(packagefamilyname : windows_sys::core::PCWSTR, packagefilters : u32, count : *mut u32, packagefullnames : *mut windows_sys::core::PWSTR, bufferlength : *mut u32, buffer : windows_sys::core::PWSTR, packageproperties : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn FormatApplicationUserModelId(packagefamilyname : windows_sys::core::PCWSTR, packagerelativeapplicationid : windows_sys::core::PCWSTR, applicationusermodelidlength : *mut u32, applicationusermodelid : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetApplicationUserModelId(hprocess : super::super::super::Foundation:: HANDLE, applicationusermodelidlength : *mut u32, applicationusermodelid : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetApplicationUserModelIdFromToken(token : super::super::super::Foundation:: HANDLE, applicationusermodelidlength : *mut u32, applicationusermodelid : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentApplicationUserModelId(applicationusermodelidlength : *mut u32, applicationusermodelid : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageFamilyName(packagefamilynamelength : *mut u32, packagefamilyname : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageFullName(packagefullnamelength : *mut u32, packagefullname : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageId(bufferlength : *mut u32, buffer : *mut u8) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageInfo(flags : u32, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetCurrentPackageInfo2(flags : u32, packagepathtype : PackagePathType, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageInfo3(flags : u32, packageinfotype : PackageInfo3Type, bufferlength : *mut u32, buffer : *mut core::ffi::c_void, count : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackagePath(pathlength : *mut u32, path : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetCurrentPackagePath2(packagepathtype : PackagePathType, pathlength : *mut u32, path : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageVirtualizationContext() -> PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE);
windows_targets::link!("kernelbase.dll" "system" fn GetIdForPackageDependencyContext(packagedependencycontext : PACKAGEDEPENDENCY_CONTEXT, packagedependencyid : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn GetPackageApplicationIds(packageinforeference : *const _PACKAGE_INFO_REFERENCE, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetPackageFamilyName(hprocess : super::super::super::Foundation:: HANDLE, packagefamilynamelength : *mut u32, packagefamilyname : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetPackageFamilyNameFromToken(token : super::super::super::Foundation:: HANDLE, packagefamilynamelength : *mut u32, packagefamilyname : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetPackageFullName(hprocess : super::super::super::Foundation:: HANDLE, packagefullnamelength : *mut u32, packagefullname : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetPackageFullNameFromToken(token : super::super::super::Foundation:: HANDLE, packagefullnamelength : *mut u32, packagefullname : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-6.dll" "system" fn GetPackageGraphRevisionId() -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetPackageId(hprocess : super::super::super::Foundation:: HANDLE, bufferlength : *mut u32, buffer : *mut u8) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetPackageInfo(packageinforeference : *const _PACKAGE_INFO_REFERENCE, flags : u32, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetPackageInfo2(packageinforeference : *const _PACKAGE_INFO_REFERENCE, flags : u32, packagepathtype : PackagePathType, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetPackagePath(packageid : *const PACKAGE_ID, reserved : u32, pathlength : *mut u32, path : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetPackagePathByFullName(packagefullname : windows_sys::core::PCWSTR, pathlength : *mut u32, path : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetPackagePathByFullName2(packagefullname : windows_sys::core::PCWSTR, packagepathtype : PackagePathType, pathlength : *mut u32, path : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetPackagesByPackageFamily(packagefamilyname : windows_sys::core::PCWSTR, count : *mut u32, packagefullnames : *mut windows_sys::core::PWSTR, bufferlength : *mut u32, buffer : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetProcessesInVirtualizationContext(packagefamilyname : windows_sys::core::PCWSTR, count : *mut u32, processes : *mut *mut super::super::super::Foundation:: HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("kernelbase.dll" "system" fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid : windows_sys::core::PCWSTR, packagefullname : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetStagedPackageOrigin(packagefullname : windows_sys::core::PCWSTR, origin : *mut PackageOrigin) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn GetStagedPackagePathByFullName(packagefullname : windows_sys::core::PCWSTR, pathlength : *mut u32, path : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetStagedPackagePathByFullName2(packagefullname : windows_sys::core::PCWSTR, packagepathtype : PackagePathType, pathlength : *mut u32, path : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn OpenPackageInfoByFullName(packagefullname : windows_sys::core::PCWSTR, reserved : u32, packageinforeference : *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation:: WIN32_ERROR);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn OpenPackageInfoByFullNameForUser(usersid : super::super::super::Security:: PSID, packagefullname : windows_sys::core::PCWSTR, reserved : u32, packageinforeference : *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn PackageFamilyNameFromFullName(packagefullname : windows_sys::core::PCWSTR, packagefamilynamelength : *mut u32, packagefamilyname : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn PackageFamilyNameFromId(packageid : *const PACKAGE_ID, packagefamilynamelength : *mut u32, packagefamilyname : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn PackageFullNameFromId(packageid : *const PACKAGE_ID, packagefullnamelength : *mut u32, packagefullname : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn PackageIdFromFullName(packagefullname : windows_sys::core::PCWSTR, flags : u32, bufferlength : *mut u32, buffer : *mut u8) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname : windows_sys::core::PCWSTR, packagenamelength : *mut u32, packagename : windows_sys::core::PWSTR, packagepublisheridlength : *mut u32, packagepublisherid : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn ParseApplicationUserModelId(applicationusermodelid : windows_sys::core::PCWSTR, packagefamilynamelength : *mut u32, packagefamilyname : windows_sys::core::PWSTR, packagerelativeapplicationidlength : *mut u32, packagerelativeapplicationid : windows_sys::core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn ReleasePackageVirtualizationContext(context : PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE));
windows_targets::link!("kernelbase.dll" "system" fn RemovePackageDependency(packagedependencycontext : PACKAGEDEPENDENCY_CONTEXT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernelbase.dll" "system" fn TryCreatePackageDependency(user : super::super::super::Security:: PSID, packagefamilyname : windows_sys::core::PCWSTR, minversion : PACKAGE_VERSION, packagedependencyprocessorarchitectures : PackageDependencyProcessorArchitectures, lifetimekind : PackageDependencyLifetimeKind, lifetimeartifact : windows_sys::core::PCWSTR, options : CreatePackageDependencyOptions, packagedependencyid : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyApplicationUserModelId(applicationusermodelid : windows_sys::core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageFamilyName(packagefamilyname : windows_sys::core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageFullName(packagefullname : windows_sys::core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageId(packageid : *const PACKAGE_ID) -> super::super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid : windows_sys::core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
pub const APPLICATION_USER_MODEL_ID_MAX_LENGTH: u32 = 130u32;
pub const APPLICATION_USER_MODEL_ID_MIN_LENGTH: u32 = 20u32;
pub type APPX_BUNDLE_FOOTPRINT_FILE_TYPE = i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 1i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_FIRST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 0i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_LAST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 2i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 0i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 2i32;
pub type APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = i32;
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_APPLICATION: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = 0i32;
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_RESOURCE: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = 1i32;
pub type APPX_CAPABILITIES = i32;
pub const APPX_CAPABILITY_APPOINTMENTS: APPX_CAPABILITIES = 1024i32;
pub const APPX_CAPABILITY_CLASS_ALL: APPX_CAPABILITY_CLASS_TYPE = 7i32;
pub const APPX_CAPABILITY_CLASS_CUSTOM: APPX_CAPABILITY_CLASS_TYPE = 8i32;
pub const APPX_CAPABILITY_CLASS_DEFAULT: APPX_CAPABILITY_CLASS_TYPE = 0i32;
pub const APPX_CAPABILITY_CLASS_GENERAL: APPX_CAPABILITY_CLASS_TYPE = 1i32;
pub const APPX_CAPABILITY_CLASS_RESTRICTED: APPX_CAPABILITY_CLASS_TYPE = 2i32;
pub type APPX_CAPABILITY_CLASS_TYPE = i32;
pub const APPX_CAPABILITY_CLASS_WINDOWS: APPX_CAPABILITY_CLASS_TYPE = 4i32;
pub const APPX_CAPABILITY_CONTACTS: APPX_CAPABILITIES = 2048i32;
pub const APPX_CAPABILITY_DOCUMENTS_LIBRARY: APPX_CAPABILITIES = 8i32;
pub const APPX_CAPABILITY_ENTERPRISE_AUTHENTICATION: APPX_CAPABILITIES = 128i32;
pub const APPX_CAPABILITY_INTERNET_CLIENT: APPX_CAPABILITIES = 1i32;
pub const APPX_CAPABILITY_INTERNET_CLIENT_SERVER: APPX_CAPABILITIES = 2i32;
pub const APPX_CAPABILITY_MUSIC_LIBRARY: APPX_CAPABILITIES = 64i32;
pub const APPX_CAPABILITY_PICTURES_LIBRARY: APPX_CAPABILITIES = 16i32;
pub const APPX_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: APPX_CAPABILITIES = 4i32;
pub const APPX_CAPABILITY_REMOVABLE_STORAGE: APPX_CAPABILITIES = 512i32;
pub const APPX_CAPABILITY_SHARED_USER_CERTIFICATES: APPX_CAPABILITIES = 256i32;
pub const APPX_CAPABILITY_VIDEOS_LIBRARY: APPX_CAPABILITIES = 32i32;
pub type APPX_COMPRESSION_OPTION = i32;
pub const APPX_COMPRESSION_OPTION_FAST: APPX_COMPRESSION_OPTION = 3i32;
pub const APPX_COMPRESSION_OPTION_MAXIMUM: APPX_COMPRESSION_OPTION = 2i32;
pub const APPX_COMPRESSION_OPTION_NONE: APPX_COMPRESSION_OPTION = 0i32;
pub const APPX_COMPRESSION_OPTION_NORMAL: APPX_COMPRESSION_OPTION = 1i32;
pub const APPX_COMPRESSION_OPTION_SUPERFAST: APPX_COMPRESSION_OPTION = 4i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct APPX_ENCRYPTED_EXEMPTIONS {
    pub count: u32,
    pub plainTextFiles: *const windows_sys::core::PCWSTR,
}
impl Default for APPX_ENCRYPTED_EXEMPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type APPX_ENCRYPTED_PACKAGE_OPTIONS = i32;
pub const APPX_ENCRYPTED_PACKAGE_OPTION_DIFFUSION: APPX_ENCRYPTED_PACKAGE_OPTIONS = 1i32;
pub const APPX_ENCRYPTED_PACKAGE_OPTION_NONE: APPX_ENCRYPTED_PACKAGE_OPTIONS = 0i32;
pub const APPX_ENCRYPTED_PACKAGE_OPTION_PAGE_HASHING: APPX_ENCRYPTED_PACKAGE_OPTIONS = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS {
    pub keyLength: u32,
    pub encryptionAlgorithm: windows_sys::core::PCWSTR,
    pub useDiffusion: windows_sys::core::BOOL,
    pub blockMapHashAlgorithm: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    pub keyLength: u32,
    pub encryptionAlgorithm: windows_sys::core::PCWSTR,
    pub blockMapHashAlgorithm: *mut core::ffi::c_void,
    pub options: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type APPX_FOOTPRINT_FILE_TYPE = i32;
pub const APPX_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_FOOTPRINT_FILE_TYPE = 1i32;
pub const APPX_FOOTPRINT_FILE_TYPE_CODEINTEGRITY: APPX_FOOTPRINT_FILE_TYPE = 3i32;
pub const APPX_FOOTPRINT_FILE_TYPE_CONTENTGROUPMAP: APPX_FOOTPRINT_FILE_TYPE = 4i32;
pub const APPX_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_FOOTPRINT_FILE_TYPE = 0i32;
pub const APPX_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_FOOTPRINT_FILE_TYPE = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct APPX_KEY_INFO {
    pub keyLength: u32,
    pub keyIdLength: u32,
    pub key: *mut u8,
    pub keyId: *mut u8,
}
impl Default for APPX_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type APPX_PACKAGE_ARCHITECTURE = i32;
pub type APPX_PACKAGE_ARCHITECTURE2 = i32;
pub const APPX_PACKAGE_ARCHITECTURE2_ARM: APPX_PACKAGE_ARCHITECTURE2 = 5i32;
pub const APPX_PACKAGE_ARCHITECTURE2_ARM64: APPX_PACKAGE_ARCHITECTURE2 = 12i32;
pub const APPX_PACKAGE_ARCHITECTURE2_NEUTRAL: APPX_PACKAGE_ARCHITECTURE2 = 11i32;
pub const APPX_PACKAGE_ARCHITECTURE2_UNKNOWN: APPX_PACKAGE_ARCHITECTURE2 = 65535i32;
pub const APPX_PACKAGE_ARCHITECTURE2_X64: APPX_PACKAGE_ARCHITECTURE2 = 9i32;
pub const APPX_PACKAGE_ARCHITECTURE2_X86: APPX_PACKAGE_ARCHITECTURE2 = 0i32;
pub const APPX_PACKAGE_ARCHITECTURE2_X86_ON_ARM64: APPX_PACKAGE_ARCHITECTURE2 = 14i32;
pub const APPX_PACKAGE_ARCHITECTURE_ARM: APPX_PACKAGE_ARCHITECTURE = 5i32;
pub const APPX_PACKAGE_ARCHITECTURE_ARM64: APPX_PACKAGE_ARCHITECTURE = 12i32;
pub const APPX_PACKAGE_ARCHITECTURE_NEUTRAL: APPX_PACKAGE_ARCHITECTURE = 11i32;
pub const APPX_PACKAGE_ARCHITECTURE_X64: APPX_PACKAGE_ARCHITECTURE = 9i32;
pub const APPX_PACKAGE_ARCHITECTURE_X86: APPX_PACKAGE_ARCHITECTURE = 0i32;
pub type APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = i32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_LOCALIZED: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = 2i32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_NONE: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = 0i32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_SKIP_VALIDATION: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = 1i32;
pub type APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = i32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION_APPEND_DELTA: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = 0i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct APPX_PACKAGE_SETTINGS {
    pub forceZip32: windows_sys::core::BOOL,
    pub hashMethod: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for APPX_PACKAGE_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    pub inputStream: *mut core::ffi::c_void,
    pub fileName: windows_sys::core::PCWSTR,
    pub contentType: windows_sys::core::PCWSTR,
    pub compressionOption: APPX_COMPRESSION_OPTION,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type APPX_PACKAGING_CONTEXT_CHANGE_TYPE = i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_CHANGE: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 1i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_DETAILS: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 2i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_END: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 3i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_START: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 0i32;
pub type AddPackageDependencyOptions = i32;
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = 0i32;
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = 1i32;
pub type AppPolicyClrCompat = i32;
pub const AppPolicyClrCompat_ClassicDesktop: AppPolicyClrCompat = 1i32;
pub const AppPolicyClrCompat_Other: AppPolicyClrCompat = 0i32;
pub const AppPolicyClrCompat_PackagedDesktop: AppPolicyClrCompat = 3i32;
pub const AppPolicyClrCompat_Universal: AppPolicyClrCompat = 2i32;
pub type AppPolicyCreateFileAccess = i32;
pub const AppPolicyCreateFileAccess_Full: AppPolicyCreateFileAccess = 0i32;
pub const AppPolicyCreateFileAccess_Limited: AppPolicyCreateFileAccess = 1i32;
pub type AppPolicyLifecycleManagement = i32;
pub const AppPolicyLifecycleManagement_Managed: AppPolicyLifecycleManagement = 1i32;
pub const AppPolicyLifecycleManagement_Unmanaged: AppPolicyLifecycleManagement = 0i32;
pub type AppPolicyMediaFoundationCodecLoading = i32;
pub const AppPolicyMediaFoundationCodecLoading_All: AppPolicyMediaFoundationCodecLoading = 0i32;
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: AppPolicyMediaFoundationCodecLoading = 1i32;
pub type AppPolicyProcessTerminationMethod = i32;
pub const AppPolicyProcessTerminationMethod_ExitProcess: AppPolicyProcessTerminationMethod = 0i32;
pub const AppPolicyProcessTerminationMethod_TerminateProcess: AppPolicyProcessTerminationMethod = 1i32;
pub type AppPolicyShowDeveloperDiagnostic = i32;
pub const AppPolicyShowDeveloperDiagnostic_None: AppPolicyShowDeveloperDiagnostic = 0i32;
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: AppPolicyShowDeveloperDiagnostic = 1i32;
pub type AppPolicyThreadInitializationType = i32;
pub const AppPolicyThreadInitializationType_InitializeWinRT: AppPolicyThreadInitializationType = 1i32;
pub const AppPolicyThreadInitializationType_None: AppPolicyThreadInitializationType = 0i32;
pub type AppPolicyWindowingModel = i32;
pub const AppPolicyWindowingModel_ClassicDesktop: AppPolicyWindowingModel = 2i32;
pub const AppPolicyWindowingModel_ClassicPhone: AppPolicyWindowingModel = 3i32;
pub const AppPolicyWindowingModel_None: AppPolicyWindowingModel = 0i32;
pub const AppPolicyWindowingModel_Universal: AppPolicyWindowingModel = 1i32;
pub const AppxBundleFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x378e0446_5384_43b7_8877_e7dbdd883446);
pub const AppxEncryptionFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdc664fdd_d868_46ee_8780_8d196cb739f7);
pub const AppxFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5842a140_ff9f_4166_8f5c_62f5b7b0c781);
pub const AppxPackageEditor: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf004f2ca_aebc_4b0d_bf58_e516d5bcc0ab);
pub const AppxPackagingDiagnosticEventSinkManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x50ca0a46_1588_4161_8ed2_ef9e469ced5d);
pub type CreatePackageDependencyOptions = i32;
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = 1i32;
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = 0i32;
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = 2i32;
pub type DX_FEATURE_LEVEL = i32;
pub const DX_FEATURE_LEVEL_10: DX_FEATURE_LEVEL = 2i32;
pub const DX_FEATURE_LEVEL_11: DX_FEATURE_LEVEL = 3i32;
pub const DX_FEATURE_LEVEL_9: DX_FEATURE_LEVEL = 1i32;
pub const DX_FEATURE_LEVEL_UNSPECIFIED: DX_FEATURE_LEVEL = 0i32;
pub type PACKAGEDEPENDENCY_CONTEXT = *mut core::ffi::c_void;
pub const PACKAGE_APPLICATIONS_MAX_COUNT: u32 = 100u32;
pub const PACKAGE_APPLICATIONS_MIN_COUNT: u32 = 0u32;
pub const PACKAGE_ARCHITECTURE_MAX_LENGTH: u32 = 7u32;
pub const PACKAGE_ARCHITECTURE_MIN_LENGTH: u32 = 3u32;
pub const PACKAGE_DEPENDENCY_RANK_DEFAULT: u32 = 0u32;
pub const PACKAGE_FAMILY_MAX_RESOURCE_PACKAGES: u32 = 512u32;
pub const PACKAGE_FAMILY_MIN_RESOURCE_PACKAGES: u32 = 0u32;
pub const PACKAGE_FAMILY_NAME_MAX_LENGTH: u32 = 64u32;
pub const PACKAGE_FAMILY_NAME_MIN_LENGTH: u32 = 17u32;
pub const PACKAGE_FILTER_ALL_LOADED: u32 = 0u32;
pub const PACKAGE_FILTER_BUNDLE: u32 = 128u32;
pub const PACKAGE_FILTER_DIRECT: u32 = 32u32;
pub const PACKAGE_FILTER_DYNAMIC: u32 = 1048576u32;
pub const PACKAGE_FILTER_HEAD: u32 = 16u32;
pub const PACKAGE_FILTER_HOSTRUNTIME: u32 = 2097152u32;
pub const PACKAGE_FILTER_IS_IN_RELATED_SET: u32 = 262144u32;
pub const PACKAGE_FILTER_OPTIONAL: u32 = 131072u32;
pub const PACKAGE_FILTER_RESOURCE: u32 = 64u32;
pub const PACKAGE_FILTER_STATIC: u32 = 524288u32;
pub const PACKAGE_FULL_NAME_MAX_LENGTH: u32 = 127u32;
pub const PACKAGE_FULL_NAME_MIN_LENGTH: u32 = 30u32;
pub const PACKAGE_GRAPH_MAX_SIZE: u32 = 641u32;
pub const PACKAGE_GRAPH_MIN_SIZE: u32 = 1u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: windows_sys::core::PWSTR,
    pub publisher: windows_sys::core::PWSTR,
    pub resourceId: windows_sys::core::PWSTR,
    pub publisherId: windows_sys::core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: windows_sys::core::PWSTR,
    pub publisher: windows_sys::core::PWSTR,
    pub resourceId: windows_sys::core::PWSTR,
    pub publisherId: windows_sys::core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: windows_sys::core::PWSTR,
    pub packageFullName: windows_sys::core::PWSTR,
    pub packageFamilyName: windows_sys::core::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(target_arch = "x86")]
impl Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: windows_sys::core::PWSTR,
    pub packageFullName: windows_sys::core::PWSTR,
    pub packageFamilyName: windows_sys::core::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PACKAGE_INFORMATION_BASIC: u32 = 0u32;
pub const PACKAGE_INFORMATION_FULL: u32 = 256u32;
pub const PACKAGE_MAX_DEPENDENCIES: u32 = 128u32;
pub const PACKAGE_MIN_DEPENDENCIES: u32 = 0u32;
pub const PACKAGE_NAME_MAX_LENGTH: u32 = 50u32;
pub const PACKAGE_NAME_MIN_LENGTH: u32 = 3u32;
pub const PACKAGE_PROPERTY_BUNDLE: u32 = 4u32;
pub const PACKAGE_PROPERTY_DEVELOPMENT_MODE: u32 = 65536u32;
pub const PACKAGE_PROPERTY_DYNAMIC: u32 = 1048576u32;
pub const PACKAGE_PROPERTY_FRAMEWORK: u32 = 1u32;
pub const PACKAGE_PROPERTY_HOSTRUNTIME: u32 = 2097152u32;
pub const PACKAGE_PROPERTY_IS_IN_RELATED_SET: u32 = 262144u32;
pub const PACKAGE_PROPERTY_OPTIONAL: u32 = 8u32;
pub const PACKAGE_PROPERTY_RESOURCE: u32 = 2u32;
pub const PACKAGE_PROPERTY_STATIC: u32 = 524288u32;
pub const PACKAGE_PUBLISHERID_MAX_LENGTH: u32 = 13u32;
pub const PACKAGE_PUBLISHERID_MIN_LENGTH: u32 = 13u32;
pub const PACKAGE_PUBLISHER_MAX_LENGTH: u32 = 8192u32;
pub const PACKAGE_PUBLISHER_MIN_LENGTH: u32 = 3u32;
pub const PACKAGE_RELATIVE_APPLICATION_ID_MAX_LENGTH: u32 = 65u32;
pub const PACKAGE_RELATIVE_APPLICATION_ID_MIN_LENGTH: u32 = 2u32;
pub const PACKAGE_RESOURCEID_MAX_LENGTH: u32 = 30u32;
pub const PACKAGE_RESOURCEID_MIN_LENGTH: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl Default for PACKAGE_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub union PACKAGE_VERSION_0 {
    pub Version: u64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
impl Default for PACKAGE_VERSION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
pub const PACKAGE_VERSION_MAX_LENGTH: u32 = 23u32;
pub const PACKAGE_VERSION_MIN_LENGTH: u32 = 7u32;
pub type PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE = *mut core::ffi::c_void;
pub type PackageDependencyLifetimeKind = i32;
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = 1i32;
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = 0i32;
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = 2i32;
pub type PackageDependencyProcessorArchitectures = i32;
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = 8i32;
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = 16i32;
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = 1i32;
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = 0i32;
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = 4i32;
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = 2i32;
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = 32i32;
pub type PackageInfo3Type = i32;
pub const PackageInfo3Type_PackageInfoGeneration: PackageInfo3Type = 16i32;
pub type PackageOrigin = i32;
pub const PackageOrigin_DeveloperSigned: PackageOrigin = 5i32;
pub const PackageOrigin_DeveloperUnsigned: PackageOrigin = 4i32;
pub const PackageOrigin_Inbox: PackageOrigin = 2i32;
pub const PackageOrigin_LineOfBusiness: PackageOrigin = 6i32;
pub const PackageOrigin_Store: PackageOrigin = 3i32;
pub const PackageOrigin_Unknown: PackageOrigin = 0i32;
pub const PackageOrigin_Unsigned: PackageOrigin = 1i32;
pub type PackagePathType = i32;
pub const PackagePathType_Effective: PackagePathType = 2i32;
pub const PackagePathType_EffectiveExternal: PackagePathType = 5i32;
pub const PackagePathType_Install: PackagePathType = 0i32;
pub const PackagePathType_MachineExternal: PackagePathType = 3i32;
pub const PackagePathType_Mutable: PackagePathType = 1i32;
pub const PackagePathType_UserExternal: PackagePathType = 4i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut core::ffi::c_void,
}
impl Default for _PACKAGE_INFO_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
