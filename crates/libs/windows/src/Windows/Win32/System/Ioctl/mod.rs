pub const ABL_5_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(64i32);
pub const ADR_1: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(90i32);
pub const ADR_2: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(91i32);
pub const AIT1_8mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(38i32);
pub const AIT_8mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(89i32);
pub const AME_8mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(37i32);
pub const ASSERT_ALTERNATE: u32 = 9u32;
pub const ASSERT_PRIMARY: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ASYNC_DUPLICATE_EXTENTS_STATUS {
    pub Version: u32,
    pub State: DUPLICATE_EXTENTS_STATE,
    pub SourceFileOffset: u64,
    pub TargetFileOffset: u64,
    pub ByteCount: u64,
    pub BytesDuplicated: u64,
}
pub const ATAPI_ID_CMD: u32 = 161u32;
pub const AVATAR_F2: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(78i32);
pub const AllElements: ELEMENT_TYPE = ELEMENT_TYPE(0i32);
pub const AtaDataTypeIdentify: STORAGE_PROTOCOL_ATA_DATA_TYPE = STORAGE_PROTOCOL_ATA_DATA_TYPE(1i32);
pub const AtaDataTypeLogPage: STORAGE_PROTOCOL_ATA_DATA_TYPE = STORAGE_PROTOCOL_ATA_DATA_TYPE(2i32);
pub const AtaDataTypeUnknown: STORAGE_PROTOCOL_ATA_DATA_TYPE = STORAGE_PROTOCOL_ATA_DATA_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BIN_COUNT {
    pub BinRange: BIN_RANGE,
    pub BinCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BIN_RANGE {
    pub StartValue: i64,
    pub Length: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BIN_RESULTS {
    pub NumberOfBins: u32,
    pub BinCounts: [BIN_COUNT; 1],
}
impl Default for BIN_RESULTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BIN_TYPES(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BOOT_AREA_INFO {
    pub BootSectorCount: u32,
    pub BootSectors: [BOOT_AREA_INFO_0; 2],
}
impl Default for BOOT_AREA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BOOT_AREA_INFO_0 {
    pub Offset: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BULK_SECURITY_TEST_DATA {
    pub DesiredAccess: u32,
    pub SecurityIds: [u32; 1],
}
impl Default for BULK_SECURITY_TEST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CAP_ATAPI_ID_CMD: u32 = 2u32;
pub const CAP_ATA_ID_CMD: u32 = 1u32;
pub const CAP_SMART_CMD: u32 = 4u32;
pub const CDB_SIZE: u32 = 16u32;
pub const CD_R: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(52i32);
pub const CD_ROM: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(51i32);
pub const CD_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(53i32);
pub const CHANGER_BAR_CODE_SCANNER_INSTALLED: CHANGER_FEATURES = CHANGER_FEATURES(1u32);
pub const CHANGER_CARTRIDGE_MAGAZINE: CHANGER_FEATURES = CHANGER_FEATURES(256u32);
pub const CHANGER_CLEANER_ACCESS_NOT_VALID: CHANGER_FEATURES = CHANGER_FEATURES(262144u32);
pub const CHANGER_CLEANER_AUTODISMOUNT: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483652u32);
pub const CHANGER_CLEANER_OPS_NOT_SUPPORTED: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483712u32);
pub const CHANGER_CLEANER_SLOT: CHANGER_FEATURES = CHANGER_FEATURES(64u32);
pub const CHANGER_CLOSE_IEPORT: CHANGER_FEATURES = CHANGER_FEATURES(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_DEVICE_PROBLEM_TYPE(pub i32);
pub const CHANGER_DEVICE_REINITIALIZE_CAPABLE: CHANGER_FEATURES = CHANGER_FEATURES(134217728u32);
pub const CHANGER_DRIVE_CLEANING_REQUIRED: CHANGER_FEATURES = CHANGER_FEATURES(65536u32);
pub const CHANGER_DRIVE_EMPTY_ON_DOOR_ACCESS: CHANGER_FEATURES = CHANGER_FEATURES(536870912u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHANGER_ELEMENT {
    pub ElementType: ELEMENT_TYPE,
    pub ElementAddress: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHANGER_ELEMENT_LIST {
    pub Element: CHANGER_ELEMENT,
    pub NumberOfElements: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CHANGER_ELEMENT_STATUS {
    pub Element: CHANGER_ELEMENT,
    pub SrcElementAddress: CHANGER_ELEMENT,
    pub Flags: CHANGER_ELEMENT_STATUS_FLAGS,
    pub ExceptionCode: u32,
    pub TargetId: u8,
    pub Lun: u8,
    pub Reserved: u16,
    pub PrimaryVolumeID: [u8; 36],
    pub AlternateVolumeID: [u8; 36],
}
impl Default for CHANGER_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CHANGER_ELEMENT_STATUS_EX {
    pub Element: CHANGER_ELEMENT,
    pub SrcElementAddress: CHANGER_ELEMENT,
    pub Flags: CHANGER_ELEMENT_STATUS_FLAGS,
    pub ExceptionCode: u32,
    pub TargetId: u8,
    pub Lun: u8,
    pub Reserved: u16,
    pub PrimaryVolumeID: [u8; 36],
    pub AlternateVolumeID: [u8; 36],
    pub VendorIdentification: [u8; 8],
    pub ProductIdentification: [u8; 16],
    pub SerialNumber: [u8; 32],
}
impl Default for CHANGER_ELEMENT_STATUS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_ELEMENT_STATUS_FLAGS(pub u32);
impl CHANGER_ELEMENT_STATUS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CHANGER_ELEMENT_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CHANGER_ELEMENT_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CHANGER_ELEMENT_STATUS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CHANGER_ELEMENT_STATUS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CHANGER_ELEMENT_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CHANGER_EXCHANGE_MEDIA: CHANGER_FEATURES = CHANGER_FEATURES(32u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHANGER_EXCHANGE_MEDIUM {
    pub Transport: CHANGER_ELEMENT,
    pub Source: CHANGER_ELEMENT,
    pub Destination1: CHANGER_ELEMENT,
    pub Destination2: CHANGER_ELEMENT,
    pub Flip1: bool,
    pub Flip2: bool,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_FEATURES(pub u32);
impl CHANGER_FEATURES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CHANGER_FEATURES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CHANGER_FEATURES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CHANGER_FEATURES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CHANGER_FEATURES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CHANGER_FEATURES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CHANGER_IEPORT_USER_CONTROL_CLOSE: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483904u32);
pub const CHANGER_IEPORT_USER_CONTROL_OPEN: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483776u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHANGER_INITIALIZE_ELEMENT_STATUS {
    pub ElementList: CHANGER_ELEMENT_LIST,
    pub BarCodeScan: bool,
}
pub const CHANGER_INIT_ELEM_STAT_WITH_RANGE: CHANGER_FEATURES = CHANGER_FEATURES(2u32);
pub const CHANGER_KEYPAD_ENABLE_DISABLE: CHANGER_FEATURES = CHANGER_FEATURES(268435456u32);
pub const CHANGER_LOCK_UNLOCK: CHANGER_FEATURES = CHANGER_FEATURES(128u32);
pub const CHANGER_MEDIUM_FLIP: CHANGER_FEATURES = CHANGER_FEATURES(512u32);
pub const CHANGER_MOVE_EXTENDS_IEPORT: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147484160u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHANGER_MOVE_MEDIUM {
    pub Transport: CHANGER_ELEMENT,
    pub Source: CHANGER_ELEMENT,
    pub Destination: CHANGER_ELEMENT,
    pub Flip: bool,
}
pub const CHANGER_MOVE_RETRACTS_IEPORT: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147484672u32);
pub const CHANGER_OPEN_IEPORT: CHANGER_FEATURES = CHANGER_FEATURES(8u32);
pub const CHANGER_POSITION_TO_ELEMENT: CHANGER_FEATURES = CHANGER_FEATURES(1024u32);
pub const CHANGER_PREDISMOUNT_ALIGN_TO_DRIVE: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483650u32);
pub const CHANGER_PREDISMOUNT_ALIGN_TO_SLOT: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483649u32);
pub const CHANGER_PREDISMOUNT_EJECT_REQUIRED: CHANGER_FEATURES = CHANGER_FEATURES(131072u32);
pub const CHANGER_PREMOUNT_EJECT_REQUIRED: CHANGER_FEATURES = CHANGER_FEATURES(524288u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CHANGER_PRODUCT_DATA {
    pub VendorId: [u8; 8],
    pub ProductId: [u8; 16],
    pub Revision: [u8; 4],
    pub SerialNumber: [u8; 32],
    pub DeviceType: u8,
}
impl Default for CHANGER_PRODUCT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHANGER_READ_ELEMENT_STATUS {
    pub ElementList: CHANGER_ELEMENT_LIST,
    pub VolumeTagInfo: bool,
}
pub const CHANGER_REPORT_IEPORT_STATE: CHANGER_FEATURES = CHANGER_FEATURES(2048u32);
pub const CHANGER_RESERVED_BIT: u32 = 2147483648u32;
pub const CHANGER_RTN_MEDIA_TO_ORIGINAL_ADDR: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483680u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CHANGER_SEND_VOLUME_TAG_INFORMATION {
    pub StartingElement: CHANGER_ELEMENT,
    pub ActionCode: u32,
    pub VolumeIDTemplate: [u8; 40],
}
impl Default for CHANGER_SEND_VOLUME_TAG_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CHANGER_SERIAL_NUMBER_VALID: CHANGER_FEATURES = CHANGER_FEATURES(67108864u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHANGER_SET_ACCESS {
    pub Element: CHANGER_ELEMENT,
    pub Control: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHANGER_SET_POSITION {
    pub Transport: CHANGER_ELEMENT,
    pub Destination: CHANGER_ELEMENT,
    pub Flip: bool,
}
pub const CHANGER_SLOTS_USE_TRAYS: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483664u32);
pub const CHANGER_STATUS_NON_VOLATILE: CHANGER_FEATURES = CHANGER_FEATURES(16u32);
pub const CHANGER_STORAGE_DRIVE: CHANGER_FEATURES = CHANGER_FEATURES(4096u32);
pub const CHANGER_STORAGE_IEPORT: CHANGER_FEATURES = CHANGER_FEATURES(8192u32);
pub const CHANGER_STORAGE_SLOT: CHANGER_FEATURES = CHANGER_FEATURES(16384u32);
pub const CHANGER_STORAGE_TRANSPORT: CHANGER_FEATURES = CHANGER_FEATURES(32768u32);
pub const CHANGER_TO_DRIVE: u32 = 8u32;
pub const CHANGER_TO_IEPORT: u32 = 4u32;
pub const CHANGER_TO_SLOT: u32 = 2u32;
pub const CHANGER_TO_TRANSPORT: u32 = 1u32;
pub const CHANGER_TRUE_EXCHANGE_CAPABLE: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483656u32);
pub const CHANGER_VOLUME_ASSERT: CHANGER_FEATURES = CHANGER_FEATURES(4194304u32);
pub const CHANGER_VOLUME_IDENTIFICATION: CHANGER_FEATURES = CHANGER_FEATURES(1048576u32);
pub const CHANGER_VOLUME_REPLACE: CHANGER_FEATURES = CHANGER_FEATURES(8388608u32);
pub const CHANGER_VOLUME_SEARCH: CHANGER_FEATURES = CHANGER_FEATURES(2097152u32);
pub const CHANGER_VOLUME_UNDEFINE: CHANGER_FEATURES = CHANGER_FEATURES(16777216u32);
pub const CHECKSUM_TYPE_CRC32: u32 = 1u32;
pub const CHECKSUM_TYPE_CRC64: u32 = 2u32;
pub const CHECKSUM_TYPE_ECC: u32 = 3u32;
pub const CHECKSUM_TYPE_FIRST_UNUSED_TYPE: u32 = 5u32;
pub const CHECKSUM_TYPE_NONE: u32 = 0u32;
pub const CHECKSUM_TYPE_SHA256: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLASS_MEDIA_CHANGE_CONTEXT {
    pub MediaChangeCount: u32,
    pub NewState: u32,
}
pub const CLEANER_CARTRIDGE: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(50i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_RANGE {
    pub StartingCluster: i64,
    pub ClusterCount: i64,
}
pub const CONTAINER_ROOT_INFO_FLAG_BIND_DO_NOT_MAP_NAME: u32 = 256u32;
pub const CONTAINER_ROOT_INFO_FLAG_BIND_EXCEPTION_ROOT: u32 = 128u32;
pub const CONTAINER_ROOT_INFO_FLAG_BIND_ROOT: u32 = 32u32;
pub const CONTAINER_ROOT_INFO_FLAG_BIND_TARGET_ROOT: u32 = 64u32;
pub const CONTAINER_ROOT_INFO_FLAG_LAYER_ROOT: u32 = 2u32;
pub const CONTAINER_ROOT_INFO_FLAG_SCRATCH_ROOT: u32 = 1u32;
pub const CONTAINER_ROOT_INFO_FLAG_UNION_LAYER_ROOT: u32 = 512u32;
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_EXCEPTION_ROOT: u32 = 16u32;
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_ROOT: u32 = 4u32;
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_TARGET_ROOT: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONTAINER_ROOT_INFO_INPUT {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONTAINER_ROOT_INFO_OUTPUT {
    pub ContainerRootIdLength: u16,
    pub ContainerRootId: [u8; 1],
}
impl Default for CONTAINER_ROOT_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONTAINER_ROOT_INFO_VALID_FLAGS: u32 = 1023u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONTAINER_VOLUME_STATE {
    pub Flags: u32,
}
pub const CONTAINER_VOLUME_STATE_HOSTING_CONTAINER: u32 = 1u32;
pub const COPYFILE_SIS_FLAGS: u32 = 3u32;
pub const COPYFILE_SIS_LINK: u32 = 1u32;
pub const COPYFILE_SIS_REPLACE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CREATE_DISK {
    pub PartitionStyle: PARTITION_STYLE,
    pub Anonymous: CREATE_DISK_0,
}
impl Default for CREATE_DISK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CREATE_DISK_0 {
    pub Mbr: CREATE_DISK_MBR,
    pub Gpt: CREATE_DISK_GPT,
}
impl Default for CREATE_DISK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CREATE_DISK_GPT {
    pub DiskId: windows_core::GUID,
    pub MaxPartitionCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CREATE_DISK_MBR {
    pub Signature: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CREATE_USN_JOURNAL_DATA {
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSVFS_DISK_CONNECTIVITY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_CONTROL_OP(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_CONTROL_PARAM {
    pub Operation: CSV_CONTROL_OP,
    pub Unused: i64,
}
pub const CSV_INVALID_DEVICE_NUMBER: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_IS_OWNED_BY_CSVFS {
    pub OwnedByCSVFS: bool,
}
pub const CSV_MGMTLOCK_CHECK_VOLUME_REDIRECTED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_MGMT_LOCK {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_NAMESPACE_INFO {
    pub Version: u32,
    pub DeviceNumber: u32,
    pub StartingOffset: i64,
    pub SectorSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CSV_QUERY_FILE_REVISION {
    pub FileId: i64,
    pub FileRevision: [i64; 3],
}
impl Default for CSV_QUERY_FILE_REVISION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    pub FileId: super::super::Storage::FileSystem::FILE_ID_128,
    pub FileRevision: [i64; 3],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CSV_QUERY_MDS_PATH {
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub PathLength: u32,
    pub Path: [u16; 1],
}
impl Default for CSV_QUERY_MDS_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CSV_QUERY_MDS_PATH_FLAG_CSV_DIRECT_IO_ENABLED: u32 = 2u32;
pub const CSV_QUERY_MDS_PATH_FLAG_SMB_BYPASS_CSV_ENABLED: u32 = 4u32;
pub const CSV_QUERY_MDS_PATH_FLAG_STORAGE_ON_THIS_NODE_IS_CONNECTED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_QUERY_MDS_PATH_V2 {
    pub Version: i64,
    pub RequiredSize: u32,
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub Flags: u32,
    pub DiskConnectivity: CSVFS_DISK_CONNECTIVITY,
    pub VolumeId: windows_core::GUID,
    pub IpAddressOffset: u32,
    pub IpAddressLength: u32,
    pub PathOffset: u32,
    pub PathLength: u32,
}
pub const CSV_QUERY_MDS_PATH_V2_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_QUERY_REDIRECT_STATE {
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub FileRedirected: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    pub VetoedFromAltitudeIntegral: u64,
    pub VetoedFromAltitudeDecimal: u64,
    pub Reason: [u16; 256],
}
impl Default for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_QUERY_VOLUME_ID {
    pub VolumeId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_QUERY_VOLUME_REDIRECT_STATE {
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub IsDiskConnected: bool,
    pub ClusterEnableDirectIo: bool,
    pub DiskConnectivity: CSVFS_DISK_CONNECTIVITY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_SET_VOLUME_ID {
    pub VolumeId: windows_core::GUID,
}
pub const CYGNET_12_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(69i32);
pub const ChangerDoor: ELEMENT_TYPE = ELEMENT_TYPE(5i32);
pub const ChangerDrive: ELEMENT_TYPE = ELEMENT_TYPE(4i32);
pub const ChangerIEPort: ELEMENT_TYPE = ELEMENT_TYPE(3i32);
pub const ChangerKeypad: ELEMENT_TYPE = ELEMENT_TYPE(6i32);
pub const ChangerMaxElement: ELEMENT_TYPE = ELEMENT_TYPE(7i32);
pub const ChangerSlot: ELEMENT_TYPE = ELEMENT_TYPE(2i32);
pub const ChangerTransport: ELEMENT_TYPE = ELEMENT_TYPE(1i32);
pub const CsvControlDisableCaching: CSV_CONTROL_OP = CSV_CONTROL_OP(19i32);
pub const CsvControlEnableCaching: CSV_CONTROL_OP = CSV_CONTROL_OP(20i32);
pub const CsvControlEnableUSNRangeModificationTracking: CSV_CONTROL_OP = CSV_CONTROL_OP(13i32);
pub const CsvControlGetCsvFsMdsPathV2: CSV_CONTROL_OP = CSV_CONTROL_OP(18i32);
pub const CsvControlMarkHandleLocalVolumeMount: CSV_CONTROL_OP = CSV_CONTROL_OP(14i32);
pub const CsvControlQueryFileRevision: CSV_CONTROL_OP = CSV_CONTROL_OP(6i32);
pub const CsvControlQueryFileRevisionFileId128: CSV_CONTROL_OP = CSV_CONTROL_OP(9i32);
pub const CsvControlQueryMdsPath: CSV_CONTROL_OP = CSV_CONTROL_OP(8i32);
pub const CsvControlQueryMdsPathNoPause: CSV_CONTROL_OP = CSV_CONTROL_OP(23i32);
pub const CsvControlQueryRedirectState: CSV_CONTROL_OP = CSV_CONTROL_OP(4i32);
pub const CsvControlQueryVolumeId: CSV_CONTROL_OP = CSV_CONTROL_OP(25i32);
pub const CsvControlQueryVolumeRedirectState: CSV_CONTROL_OP = CSV_CONTROL_OP(10i32);
pub const CsvControlSetVolumeId: CSV_CONTROL_OP = CSV_CONTROL_OP(24i32);
pub const CsvControlStartForceDFO: CSV_CONTROL_OP = CSV_CONTROL_OP(21i32);
pub const CsvControlStartRedirectFile: CSV_CONTROL_OP = CSV_CONTROL_OP(2i32);
pub const CsvControlStopForceDFO: CSV_CONTROL_OP = CSV_CONTROL_OP(22i32);
pub const CsvControlStopRedirectFile: CSV_CONTROL_OP = CSV_CONTROL_OP(3i32);
pub const CsvControlUnmarkHandleLocalVolumeMount: CSV_CONTROL_OP = CSV_CONTROL_OP(15i32);
pub const CsvFsDiskConnectivityAllNodes: CSVFS_DISK_CONNECTIVITY = CSVFS_DISK_CONNECTIVITY(3i32);
pub const CsvFsDiskConnectivityMdsNodeOnly: CSVFS_DISK_CONNECTIVITY = CSVFS_DISK_CONNECTIVITY(1i32);
pub const CsvFsDiskConnectivityNone: CSVFS_DISK_CONNECTIVITY = CSVFS_DISK_CONNECTIVITY(0i32);
pub const CsvFsDiskConnectivitySubsetOfNodes: CSVFS_DISK_CONNECTIVITY = CSVFS_DISK_CONNECTIVITY(2i32);
pub const DAX_ALLOC_ALIGNMENT_FLAG_FALLBACK_SPECIFIED: u32 = 2u32;
pub const DAX_ALLOC_ALIGNMENT_FLAG_MANDATORY: u32 = 1u32;
pub const DDS_4mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(32i32);
pub const DDUMP_FLAG_DATA_READ_FROM_DEVICE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DECRYPTION_STATUS_BUFFER {
    pub NoEncryptedStreams: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DELETE_USN_JOURNAL_DATA {
    pub UsnJournalID: u64,
    pub DeleteFlags: USN_DELETE_FLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DETECTION_TYPE(pub i32);
pub const DEVICEDUMP_CAP_PRIVATE_SECTION: u32 = 1u32;
pub const DEVICEDUMP_CAP_RESTRICTED_SECTION: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE(pub i32);
pub const DEVICEDUMP_MAX_IDSTRING: u32 = 32u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_PRIVATE_SUBSECTION {
    pub dwFlags: u32,
    pub GPLogId: GP_LOG_PAGE_DESCRIPTOR,
    pub bData: [u8; 1],
}
impl Default for DEVICEDUMP_PRIVATE_SUBSECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_PUBLIC_SUBSECTION {
    pub dwFlags: u32,
    pub GPLogTable: [GP_LOG_PAGE_DESCRIPTOR; 16],
    pub szDescription: [i8; 16],
    pub bData: [u8; 1],
}
impl Default for DEVICEDUMP_PUBLIC_SUBSECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICEDUMP_RESTRICTED_SUBSECTION {
    pub bData: [u8; 1],
}
impl Default for DEVICEDUMP_RESTRICTED_SUBSECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_SECTION_HEADER {
    pub guidDeviceDataId: windows_core::GUID,
    pub sOrganizationID: [u8; 16],
    pub dwFirmwareRevision: u32,
    pub sModelNumber: [u8; 32],
    pub szDeviceManufacturingID: [u8; 32],
    pub dwFlags: u32,
    pub bRestrictedPrivateDataVersion: u32,
    pub dwFirmwareIssueId: u32,
    pub szIssueDescriptionString: [u8; 132],
}
impl Default for DEVICEDUMP_SECTION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_STORAGEDEVICE_DATA {
    pub Descriptor: DEVICEDUMP_STRUCTURE_VERSION,
    pub SectionHeader: DEVICEDUMP_SECTION_HEADER,
    pub dwBufferSize: u32,
    pub dwReasonForCollection: u32,
    pub PublicData: DEVICEDUMP_SUBSECTION_POINTER,
    pub RestrictedData: DEVICEDUMP_SUBSECTION_POINTER,
    pub PrivateData: DEVICEDUMP_SUBSECTION_POINTER,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    pub Descriptor: DEVICEDUMP_STRUCTURE_VERSION,
    pub dwReasonForCollection: u32,
    pub cDriverName: [u8; 16],
    pub uiNumRecords: u32,
    pub RecordArray: [DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD; 1],
}
impl Default for DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    pub Cdb: [u8; 16],
    pub Command: [u8; 16],
    pub StartTime: u64,
    pub EndTime: u64,
    pub OperationStatus: u32,
    pub OperationError: u32,
    pub StackSpecific: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0,
}
impl Default for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {
    pub ExternalStack: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0,
    pub AtaPort: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1,
    pub StorPort: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2,
}
impl Default for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1 {
    pub dwAtaPortSpecific: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0 {
    pub dwReserved: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2 {
    pub SrbTag: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_STRUCTURE_VERSION {
    pub dwSignature: u32,
    pub dwVersion: u32,
    pub dwSize: u32,
}
pub const DEVICEDUMP_STRUCTURE_VERSION_V1: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_SUBSECTION_POINTER {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub MaximumTokenLifetime: u32,
    pub DefaultTokenLifetime: u32,
    pub MaximumTransferSize: u64,
    pub OptimalTransferCount: u64,
    pub MaximumDataDescriptors: u32,
    pub MaximumTransferLengthPerDescriptor: u32,
    pub OptimalTransferLengthPerDescriptor: u32,
    pub OptimalTransferLengthGranularity: u16,
    pub Reserved: [u8; 2],
}
impl Default for DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub OutputVersion: u32,
}
pub const DEVICE_DATA_SET_LBP_STATE_PARAMETERS_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    pub Size: u32,
    pub Version: u32,
    pub SlabSizeInBytes: u64,
    pub SlabOffsetDeltaInBytes: u32,
    pub SlabAllocationBitMapBitCount: u32,
    pub SlabAllocationBitMapLength: u32,
    pub SlabAllocationBitMap: [u32; 1],
}
impl Default for DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    pub Size: u32,
    pub Version: u32,
    pub SlabSizeInBytes: u64,
    pub SlabOffsetDeltaInBytes: u64,
    pub SlabAllocationBitMapBitCount: u32,
    pub SlabAllocationBitMapLength: u32,
    pub SlabAllocationBitMap: [u32; 1],
}
impl Default for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_DATA_SET_RANGE {
    pub StartingOffset: i64,
    pub LengthInBytes: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_DATA_SET_REPAIR_OUTPUT {
    pub ParityExtent: DEVICE_DATA_SET_RANGE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DATA_SET_REPAIR_PARAMETERS {
    pub NumberOfRepairCopies: u32,
    pub SourceCopy: u32,
    pub RepairCopies: [u32; 1],
}
impl Default for DEVICE_DATA_SET_REPAIR_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    pub BytesProcessed: u64,
    pub BytesRepaired: u64,
    pub BytesFailed: u64,
    pub ParityExtent: DEVICE_DATA_SET_RANGE,
    pub BytesScrubbed: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_DATA_SET_SCRUB_OUTPUT {
    pub BytesProcessed: u64,
    pub BytesRepaired: u64,
    pub BytesFailed: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    pub TopologyRangeBytes: u64,
    pub TopologyId: [u8; 16],
}
impl Default for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_DSM_CONVERSION_OUTPUT {
    pub Version: u32,
    pub Source: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_DSM_DEFINITION {
    pub Action: u32,
    pub SingleRange: bool,
    pub ParameterBlockAlignment: u32,
    pub ParameterBlockLength: u32,
    pub HasOutput: bool,
    pub OutputBlockAlignment: u32,
    pub OutputBlockLength: u32,
}
pub const DEVICE_DSM_FLAG_ALLOCATION_CONSOLIDATEABLE_ONLY: u32 = 1073741824u32;
pub const DEVICE_DSM_FLAG_ENTIRE_DATA_SET_RANGE: u32 = 1u32;
pub const DEVICE_DSM_FLAG_PHYSICAL_ADDRESSES_OMIT_TOTAL_RANGES: u32 = 268435456u32;
pub const DEVICE_DSM_FLAG_REPAIR_INPUT_TOPOLOGY_ID_PRESENT: u32 = 1073741824u32;
pub const DEVICE_DSM_FLAG_REPAIR_OUTPUT_PARITY_EXTENT: u32 = 536870912u32;
pub const DEVICE_DSM_FLAG_SCRUB_OUTPUT_PARITY_EXTENT: u32 = 536870912u32;
pub const DEVICE_DSM_FLAG_SCRUB_SKIP_IN_SYNC: u32 = 268435456u32;
pub const DEVICE_DSM_FLAG_TRIM_BYPASS_RZAT: u32 = 1073741824u32;
pub const DEVICE_DSM_FLAG_TRIM_NOT_FS_ALLOCATED: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_DSM_FREE_SPACE_OUTPUT {
    pub Version: u32,
    pub FreeSpace: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DSM_LOST_QUERY_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Alignment: u64,
    pub NumberOfBits: u32,
    pub BitMap: [u32; 1],
}
impl Default for DEVICE_DSM_LOST_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_DSM_LOST_QUERY_PARAMETERS {
    pub Version: u32,
    pub Granularity: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DSM_NOTIFICATION_PARAMETERS {
    pub Size: u32,
    pub Flags: u32,
    pub NumFileTypeIDs: u32,
    pub FileTypeID: [windows_core::GUID; 1],
}
impl Default for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVICE_DSM_NOTIFY_FLAG_BEGIN: u32 = 1u32;
pub const DEVICE_DSM_NOTIFY_FLAG_END: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    pub Size: u32,
    pub TargetPriority: u8,
    pub Reserved: [u8; 3],
}
impl Default for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    pub Flags: u32,
    pub TimeToLive: u32,
    pub Reserved: [u32; 2],
}
impl Default for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    pub Flags: u32,
    pub Reserved: u32,
    pub TokenOffset: u64,
    pub Token: STORAGE_OFFLOAD_TOKEN,
}
impl Default for DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVICE_DSM_PARAMETERS_V1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    pub Version: u32,
    pub Flags: u32,
    pub TotalNumberOfRanges: u32,
    pub NumberOfRangesReturned: u32,
    pub Ranges: [DEVICE_STORAGE_ADDRESS_RANGE; 1],
}
impl Default for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_V1: u32 = 1u32;
pub const DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_DSM_RANGE_ERROR_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub TotalNumberOfRanges: u32,
    pub NumberOfRangesReturned: u32,
    pub Ranges: [DEVICE_STORAGE_RANGE_ATTRIBUTES; 1],
}
impl Default for DEVICE_DSM_RANGE_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVICE_DSM_RANGE_ERROR_INFO_VERSION_V1: u32 = 1u32;
pub const DEVICE_DSM_RANGE_ERROR_OUTPUT_V1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DSM_REPORT_ZONES_DATA {
    pub Size: u32,
    pub ZoneCount: u32,
    pub Attributes: STORAGE_ZONES_ATTRIBUTES,
    pub Reserved0: u32,
    pub ZoneDescriptors: [STORAGE_ZONE_DESCRIPTOR; 1],
}
impl Default for DEVICE_DSM_REPORT_ZONES_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    pub Size: u32,
    pub ReportOption: u8,
    pub Partial: u8,
    pub Reserved: [u8; 2],
}
impl Default for DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DSM_TIERING_QUERY_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub NumberOfTierIds: u32,
    pub TierIds: [windows_core::GUID; 1],
}
impl Default for DEVICE_DSM_TIERING_QUERY_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_DSM_TIERING_QUERY_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Alignment: u64,
    pub TotalNumberOfRegions: u32,
    pub NumberOfRegionsReturned: u32,
    pub Regions: [STORAGE_TIER_REGION; 1],
}
impl Default for DEVICE_DSM_TIERING_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_INTERNAL_STATUS_DATA {
    pub Version: u32,
    pub Size: u32,
    pub T10VendorId: u64,
    pub DataSet1Length: u32,
    pub DataSet2Length: u32,
    pub DataSet3Length: u32,
    pub DataSet4Length: u32,
    pub StatusDataVersion: u8,
    pub Reserved: [u8; 3],
    pub ReasonIdentifier: [u8; 128],
    pub StatusDataLength: u32,
    pub StatusData: [u8; 1],
}
impl Default for DEVICE_INTERNAL_STATUS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_INTERNAL_STATUS_DATA_SET(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_LB_PROVISIONING_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub _bitfield: u8,
    pub Reserved1: [u8; 7],
    pub OptimalUnmapGranularity: u64,
    pub UnmapGranularityAlignment: u64,
    pub MaxUnmapLbaCount: u32,
    pub MaxUnmapBlockDescriptorCount: u32,
}
impl Default for DEVICE_LB_PROVISIONING_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_LOCATION {
    pub Socket: u32,
    pub Slot: u32,
    pub Adapter: u32,
    pub Port: u32,
    pub Anonymous: DEVICE_LOCATION_0,
}
impl Default for DEVICE_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVICE_LOCATION_0 {
    pub Anonymous1: DEVICE_LOCATION_0_0,
    pub Anonymous2: DEVICE_LOCATION_0_1,
}
impl Default for DEVICE_LOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_LOCATION_0_0 {
    pub Channel: u32,
    pub Device: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_LOCATION_0_1 {
    pub Target: u32,
    pub Lun: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    pub Size: u32,
    pub Action: u32,
    pub Flags: u32,
    pub ParameterBlockOffset: u32,
    pub ParameterBlockLength: u32,
    pub DataSetRangesOffset: u32,
    pub DataSetRangesLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    pub Size: u32,
    pub Action: u32,
    pub Flags: u32,
    pub OperationStatus: u32,
    pub ExtendedError: u32,
    pub TargetDetailedError: u32,
    pub ReservedStatus: u32,
    pub OutputBlockOffset: u32,
    pub OutputBlockLength: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy)]
pub struct DEVICE_MEDIA_INFO {
    pub DeviceSpecific: DEVICE_MEDIA_INFO_0,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for DEVICE_MEDIA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy)]
pub union DEVICE_MEDIA_INFO_0 {
    pub DiskInfo: DEVICE_MEDIA_INFO_0_0,
    pub RemovableDiskInfo: DEVICE_MEDIA_INFO_0_1,
    pub TapeInfo: DEVICE_MEDIA_INFO_0_2,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for DEVICE_MEDIA_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_MEDIA_INFO_0_0 {
    pub Cylinders: i64,
    pub MediaType: STORAGE_MEDIA_TYPE,
    pub TracksPerCylinder: u32,
    pub SectorsPerTrack: u32,
    pub BytesPerSector: u32,
    pub NumberMediaSides: u32,
    pub MediaCharacteristics: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_MEDIA_INFO_0_1 {
    pub Cylinders: i64,
    pub MediaType: STORAGE_MEDIA_TYPE,
    pub TracksPerCylinder: u32,
    pub SectorsPerTrack: u32,
    pub BytesPerSector: u32,
    pub NumberMediaSides: u32,
    pub MediaCharacteristics: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy)]
pub struct DEVICE_MEDIA_INFO_0_2 {
    pub MediaType: STORAGE_MEDIA_TYPE,
    pub MediaCharacteristics: u32,
    pub CurrentBlockSize: u32,
    pub BusType: super::super::Storage::FileSystem::STORAGE_BUS_TYPE,
    pub BusSpecificData: DEVICE_MEDIA_INFO_0_2_0,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for DEVICE_MEDIA_INFO_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy)]
pub union DEVICE_MEDIA_INFO_0_2_0 {
    pub ScsiInformation: DEVICE_MEDIA_INFO_0_2_0_0,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for DEVICE_MEDIA_INFO_0_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_MEDIA_INFO_0_2_0_0 {
    pub MediumType: u8,
    pub DensityCode: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_POWER_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub DeviceAttentionSupported: bool,
    pub AsynchronousNotificationSupported: bool,
    pub IdlePowerManagementEnabled: bool,
    pub D3ColdEnabled: bool,
    pub D3ColdSupported: bool,
    pub NoVerifyDuringIdlePower: bool,
    pub Reserved: [u8; 2],
    pub IdleTimeoutInMS: u32,
}
impl Default for DEVICE_POWER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_SEEK_PENALTY_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub IncursSeekPenalty: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_STORAGE_ADDRESS_RANGE {
    pub StartAddress: i64,
    pub LengthInBytes: u64,
}
pub const DEVICE_STORAGE_NO_ERRORS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_STORAGE_RANGE_ATTRIBUTES {
    pub LengthInBytes: u64,
    pub Anonymous: DEVICE_STORAGE_RANGE_ATTRIBUTES_0,
    pub Reserved: u32,
}
impl Default for DEVICE_STORAGE_RANGE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {
    pub AllFlags: u32,
    pub Anonymous: DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0,
}
impl Default for DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_TRIM_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub TrimEnabled: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub BenefitsFromWriteAggregation: bool,
}
pub const DEVPKEY_Storage_Disk_Number: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 5 };
pub const DEVPKEY_Storage_Gpt_Name: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 9 };
pub const DEVPKEY_Storage_Gpt_Type: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 8 };
pub const DEVPKEY_Storage_Mbr_Type: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 7 };
pub const DEVPKEY_Storage_Partition_Number: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 6 };
pub const DEVPKEY_Storage_Portable: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 2 };
pub const DEVPKEY_Storage_Removable_Media: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 3 };
pub const DEVPKEY_Storage_System_Critical: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 4 };
pub const DISABLE_SMART: u32 = 217u32;
pub const DISK_ATTRIBUTE_OFFLINE: u64 = 1u64;
pub const DISK_ATTRIBUTE_READ_ONLY: u64 = 2u64;
pub const DISK_BINNING: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISK_CACHE_INFORMATION {
    pub ParametersSavable: bool,
    pub ReadCacheEnabled: bool,
    pub WriteCacheEnabled: bool,
    pub ReadRetentionPriority: DISK_CACHE_RETENTION_PRIORITY,
    pub WriteRetentionPriority: DISK_CACHE_RETENTION_PRIORITY,
    pub DisablePrefetchTransferLength: u16,
    pub PrefetchScalar: bool,
    pub Anonymous: DISK_CACHE_INFORMATION_0,
}
impl Default for DISK_CACHE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISK_CACHE_INFORMATION_0 {
    pub ScalarPrefetch: DISK_CACHE_INFORMATION_0_0,
    pub BlockPrefetch: DISK_CACHE_INFORMATION_0_1,
}
impl Default for DISK_CACHE_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_CACHE_INFORMATION_0_1 {
    pub Minimum: u16,
    pub Maximum: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_CACHE_INFORMATION_0_0 {
    pub Minimum: u16,
    pub Maximum: u16,
    pub MaximumBlocks: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISK_CACHE_RETENTION_PRIORITY(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_CONTROLLER_NUMBER {
    pub ControllerNumber: u32,
    pub DiskNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISK_DETECTION_INFO {
    pub SizeOfDetectInfo: u32,
    pub DetectionType: DETECTION_TYPE,
    pub Anonymous: DISK_DETECTION_INFO_0,
}
impl Default for DISK_DETECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISK_DETECTION_INFO_0 {
    pub Anonymous: DISK_DETECTION_INFO_0_0,
}
impl Default for DISK_DETECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_DETECTION_INFO_0_0 {
    pub Int13: DISK_INT13_INFO,
    pub ExInt13: DISK_EX_INT13_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_EXTENT {
    pub DiskNumber: u32,
    pub StartingOffset: i64,
    pub ExtentLength: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_EX_INT13_INFO {
    pub ExBufferSize: u16,
    pub ExFlags: u16,
    pub ExCylinders: u32,
    pub ExHeads: u32,
    pub ExSectorsPerTrack: u32,
    pub ExSectorsPerDrive: u64,
    pub ExSectorSize: u16,
    pub ExReserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_GEOMETRY {
    pub Cylinders: i64,
    pub MediaType: MEDIA_TYPE,
    pub TracksPerCylinder: u32,
    pub SectorsPerTrack: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISK_GEOMETRY_EX {
    pub Geometry: DISK_GEOMETRY,
    pub DiskSize: i64,
    pub Data: [u8; 1],
}
impl Default for DISK_GEOMETRY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_GROW_PARTITION {
    pub PartitionNumber: u32,
    pub BytesToGrow: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISK_HISTOGRAM {
    pub DiskSize: i64,
    pub Start: i64,
    pub End: i64,
    pub Average: i64,
    pub AverageRead: i64,
    pub AverageWrite: i64,
    pub Granularity: u32,
    pub Size: u32,
    pub ReadCount: u32,
    pub WriteCount: u32,
    pub Histogram: *mut HISTOGRAM_BUCKET,
}
impl Default for DISK_HISTOGRAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_INT13_INFO {
    pub DriveSelect: u16,
    pub MaxCylinders: u32,
    pub SectorsPerTrack: u16,
    pub MaxHeads: u16,
    pub NumberDrives: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISK_LOGGING {
    pub Function: u8,
    pub BufferAddress: *mut core::ffi::c_void,
    pub BufferSize: u32,
}
impl Default for DISK_LOGGING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISK_LOGGING_DUMP: u32 = 2u32;
pub const DISK_LOGGING_START: u32 = 0u32;
pub const DISK_LOGGING_STOP: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISK_PARTITION_INFO {
    pub SizeOfPartitionInfo: u32,
    pub PartitionStyle: PARTITION_STYLE,
    pub Anonymous: DISK_PARTITION_INFO_0,
}
impl Default for DISK_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISK_PARTITION_INFO_0 {
    pub Mbr: DISK_PARTITION_INFO_0_0,
    pub Gpt: DISK_PARTITION_INFO_0_1,
}
impl Default for DISK_PARTITION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_PARTITION_INFO_0_1 {
    pub DiskId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISK_PARTITION_INFO_0_0 {
    pub Signature: u32,
    pub CheckSum: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISK_PERFORMANCE {
    pub BytesRead: i64,
    pub BytesWritten: i64,
    pub ReadTime: i64,
    pub WriteTime: i64,
    pub IdleTime: i64,
    pub ReadCount: u32,
    pub WriteCount: u32,
    pub QueueDepth: u32,
    pub SplitCount: u32,
    pub QueryTime: i64,
    pub StorageDeviceNumber: u32,
    pub StorageManagerName: [u16; 8],
}
impl Default for DISK_PERFORMANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISK_RECORD {
    pub ByteOffset: i64,
    pub StartTime: i64,
    pub EndTime: i64,
    pub VirtualAddress: *mut core::ffi::c_void,
    pub NumberOfBytes: u32,
    pub DeviceNumber: u8,
    pub ReadRequest: bool,
}
impl Default for DISK_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DLT: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(39i32);
pub const DMI: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(48i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DRIVERSTATUS {
    pub bDriverError: u8,
    pub bIDEError: u8,
    pub bReserved: [u8; 2],
    pub dwReserved: [u32; 2],
}
impl Default for DRIVERSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVE_LAYOUT_INFORMATION {
    pub PartitionCount: u32,
    pub Signature: u32,
    pub PartitionEntry: [PARTITION_INFORMATION; 1],
}
impl Default for DRIVE_LAYOUT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVE_LAYOUT_INFORMATION_EX {
    pub PartitionStyle: u32,
    pub PartitionCount: u32,
    pub Anonymous: DRIVE_LAYOUT_INFORMATION_EX_0,
    pub PartitionEntry: [PARTITION_INFORMATION_EX; 1],
}
impl Default for DRIVE_LAYOUT_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DRIVE_LAYOUT_INFORMATION_EX_0 {
    pub Mbr: DRIVE_LAYOUT_INFORMATION_MBR,
    pub Gpt: DRIVE_LAYOUT_INFORMATION_GPT,
}
impl Default for DRIVE_LAYOUT_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVE_LAYOUT_INFORMATION_GPT {
    pub DiskId: windows_core::GUID,
    pub StartingUsableOffset: i64,
    pub UsableLength: i64,
    pub MaxPartitionCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVE_LAYOUT_INFORMATION_MBR {
    pub Signature: u32,
    pub CheckSum: u32,
}
pub const DST_L: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(82i32);
pub const DST_M: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(81i32);
pub const DST_S: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(80i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DUPLICATE_EXTENTS_DATA {
    pub FileHandle: super::super::Foundation::HANDLE,
    pub SourceFileOffset: i64,
    pub TargetFileOffset: i64,
    pub ByteCount: i64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DUPLICATE_EXTENTS_DATA32 {
    pub FileHandle: u32,
    pub SourceFileOffset: i64,
    pub TargetFileOffset: i64,
    pub ByteCount: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DUPLICATE_EXTENTS_DATA_EX {
    pub Size: usize,
    pub FileHandle: super::super::Foundation::HANDLE,
    pub SourceFileOffset: i64,
    pub TargetFileOffset: i64,
    pub ByteCount: i64,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DUPLICATE_EXTENTS_DATA_EX32 {
    pub Size: u32,
    pub FileHandle: u32,
    pub SourceFileOffset: i64,
    pub TargetFileOffset: i64,
    pub ByteCount: i64,
    pub Flags: u32,
}
pub const DUPLICATE_EXTENTS_DATA_EX_ASYNC: u32 = 2u32;
pub const DUPLICATE_EXTENTS_DATA_EX_SOURCE_ATOMIC: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DUPLICATE_EXTENTS_STATE(pub i32);
pub const DVD_R: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(55i32);
pub const DVD_RAM: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(88i32);
pub const DVD_ROM: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(54i32);
pub const DVD_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(56i32);
pub const DV_6mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(47i32);
pub const DetectExInt13: DETECTION_TYPE = DETECTION_TYPE(2i32);
pub const DetectInt13: DETECTION_TYPE = DETECTION_TYPE(1i32);
pub const DetectNone: DETECTION_TYPE = DETECTION_TYPE(0i32);
pub const DeviceCurrentInternalStatusData: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(2i32);
pub const DeviceCurrentInternalStatusDataHeader: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(1i32);
pub const DeviceDsmActionFlag_NonDestructive: u32 = 2147483648u32;
pub const DeviceInternalStatusDataRequestTypeUndefined: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(0i32);
pub const DeviceProblemCHMError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(2i32);
pub const DeviceProblemCHMMoveError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(6i32);
pub const DeviceProblemCHMZeroError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(7i32);
pub const DeviceProblemCalibrationError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(4i32);
pub const DeviceProblemCartridgeEjectError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(11i32);
pub const DeviceProblemCartridgeInsertError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(8i32);
pub const DeviceProblemDoorOpen: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(3i32);
pub const DeviceProblemDriveError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(13i32);
pub const DeviceProblemGripperError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(12i32);
pub const DeviceProblemHardware: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(1i32);
pub const DeviceProblemNone: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(0i32);
pub const DeviceProblemPositionError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(9i32);
pub const DeviceProblemSensorError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(10i32);
pub const DeviceProblemTargetFailure: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(5i32);
pub const DeviceSavedInternalStatusData: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(4i32);
pub const DeviceSavedInternalStatusDataHeader: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(3i32);
pub const DeviceStatusDataSet1: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(1i32);
pub const DeviceStatusDataSet2: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(2i32);
pub const DeviceStatusDataSet3: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(3i32);
pub const DeviceStatusDataSet4: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(4i32);
pub const DeviceStatusDataSetMax: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(5i32);
pub const DeviceStatusDataSetUndefined: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(0i32);
pub const DiskHealthHealthy: STORAGE_DISK_HEALTH_STATUS = STORAGE_DISK_HEALTH_STATUS(3i32);
pub const DiskHealthMax: STORAGE_DISK_HEALTH_STATUS = STORAGE_DISK_HEALTH_STATUS(4i32);
pub const DiskHealthUnhealthy: STORAGE_DISK_HEALTH_STATUS = STORAGE_DISK_HEALTH_STATUS(1i32);
pub const DiskHealthUnknown: STORAGE_DISK_HEALTH_STATUS = STORAGE_DISK_HEALTH_STATUS(0i32);
pub const DiskHealthWarning: STORAGE_DISK_HEALTH_STATUS = STORAGE_DISK_HEALTH_STATUS(2i32);
pub const DiskOpReasonBackgroundOperation: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(12i32);
pub const DiskOpReasonComponent: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(10i32);
pub const DiskOpReasonConfiguration: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(7i32);
pub const DiskOpReasonDataPersistenceLossImminent: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(18i32);
pub const DiskOpReasonDeviceController: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(8i32);
pub const DiskOpReasonDisabledByPlatform: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(16i32);
pub const DiskOpReasonEnergySource: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(6i32);
pub const DiskOpReasonHealthCheck: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(14i32);
pub const DiskOpReasonInvalidFirmware: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(13i32);
pub const DiskOpReasonIo: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(3i32);
pub const DiskOpReasonLostData: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(5i32);
pub const DiskOpReasonLostDataPersistence: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(15i32);
pub const DiskOpReasonLostWritePersistence: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(17i32);
pub const DiskOpReasonMax: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(20i32);
pub const DiskOpReasonMedia: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(2i32);
pub const DiskOpReasonMediaController: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(9i32);
pub const DiskOpReasonNVDIMM_N: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(11i32);
pub const DiskOpReasonScsiSenseCode: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(1i32);
pub const DiskOpReasonThresholdExceeded: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(4i32);
pub const DiskOpReasonUnknown: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(0i32);
pub const DiskOpReasonWritePersistenceLossImminent: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(19i32);
pub const DiskOpStatusHardwareError: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(5i32);
pub const DiskOpStatusInService: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(4i32);
pub const DiskOpStatusMissing: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(8i32);
pub const DiskOpStatusNone: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(0i32);
pub const DiskOpStatusNotUsable: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(6i32);
pub const DiskOpStatusOk: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(2i32);
pub const DiskOpStatusPredictingFailure: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(3i32);
pub const DiskOpStatusTransientError: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(7i32);
pub const DiskOpStatusUnknown: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(1i32);
pub const EFS_TRACKED_OFFSET_HEADER_FLAG: u32 = 1u32;
pub const ELEMENT_STATUS_ACCESS: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(8u32);
pub const ELEMENT_STATUS_AVOLTAG: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(536870912u32);
pub const ELEMENT_STATUS_EXCEPT: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(4u32);
pub const ELEMENT_STATUS_EXENAB: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(16u32);
pub const ELEMENT_STATUS_FULL: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(1u32);
pub const ELEMENT_STATUS_ID_VALID: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(8192u32);
pub const ELEMENT_STATUS_IMPEXP: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(2u32);
pub const ELEMENT_STATUS_INENAB: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(32u32);
pub const ELEMENT_STATUS_INVERT: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(4194304u32);
pub const ELEMENT_STATUS_LUN_VALID: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(4096u32);
pub const ELEMENT_STATUS_NOT_BUS: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(32768u32);
pub const ELEMENT_STATUS_PRODUCT_DATA: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(64u32);
pub const ELEMENT_STATUS_PVOLTAG: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(268435456u32);
pub const ELEMENT_STATUS_SVALID: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(8388608u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ELEMENT_TYPE(pub i32);
pub const ENABLE_DISABLE_AUTOSAVE: u32 = 210u32;
pub const ENABLE_DISABLE_AUTO_OFFLINE: u32 = 219u32;
pub const ENABLE_SMART: u32 = 216u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCRYPTED_DATA_INFO {
    pub StartingFileOffset: u64,
    pub OutputBufferOffset: u32,
    pub BytesWithinFileSize: u32,
    pub BytesWithinValidDataLength: u32,
    pub CompressionFormat: u16,
    pub DataUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub EncryptionFormat: u8,
    pub NumberOfDataBlocks: u16,
    pub DataBlockSize: [u32; 1],
}
impl Default for ENCRYPTED_DATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENCRYPTED_DATA_INFO_SPARSE_FILE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCRYPTION_BUFFER {
    pub EncryptionOperation: u32,
    pub Private: [u8; 1],
}
impl Default for ENCRYPTION_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENCRYPTION_FORMAT_DEFAULT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENCRYPTION_KEY_CTRL_INPUT {
    pub HeaderSize: u32,
    pub StructureSize: u32,
    pub KeyOffset: u16,
    pub KeySize: u16,
    pub DplLock: u32,
    pub DplUserId: u64,
    pub DplCredentialId: u64,
}
pub const ERROR_DRIVE_NOT_INSTALLED: u32 = 8u32;
pub const ERROR_HISTORY_DIRECTORY_ENTRY_DEFAULT_COUNT: u32 = 8u32;
pub const ERROR_INIT_STATUS_NEEDED: u32 = 17u32;
pub const ERROR_LABEL_QUESTIONABLE: u32 = 2u32;
pub const ERROR_LABEL_UNREADABLE: u32 = 1u32;
pub const ERROR_SLOT_NOT_PRESENT: u32 = 4u32;
pub const ERROR_TRAY_MALFUNCTION: u32 = 16u32;
pub const ERROR_UNHANDLED_ERROR: u32 = 4294967295u32;
pub const EXECUTE_OFFLINE_DIAGS: u32 = 212u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXFAT_STATISTICS {
    pub CreateHits: u32,
    pub SuccessfulCreates: u32,
    pub FailedCreates: u32,
    pub NonCachedReads: u32,
    pub NonCachedReadBytes: u32,
    pub NonCachedWrites: u32,
    pub NonCachedWriteBytes: u32,
    pub NonCachedDiskReads: u32,
    pub NonCachedDiskWrites: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXTENDED_ENCRYPTED_DATA_INFO {
    pub ExtendedCode: u32,
    pub Length: u32,
    pub Flags: u32,
    pub Reserved: u32,
}
pub const EXTEND_IEPORT: u32 = 2u32;
pub const EqualPriority: DISK_CACHE_RETENTION_PRIORITY = DISK_CACHE_RETENTION_PRIORITY(0i32);
pub const F3_120M_512: MEDIA_TYPE = MEDIA_TYPE(13i32);
pub const F3_128Mb_512: MEDIA_TYPE = MEDIA_TYPE(20i32);
pub const F3_1Pt23_1024: MEDIA_TYPE = MEDIA_TYPE(18i32);
pub const F3_1Pt2_512: MEDIA_TYPE = MEDIA_TYPE(17i32);
pub const F3_1Pt44_512: MEDIA_TYPE = MEDIA_TYPE(2i32);
pub const F3_200Mb_512: MEDIA_TYPE = MEDIA_TYPE(23i32);
pub const F3_20Pt8_512: MEDIA_TYPE = MEDIA_TYPE(4i32);
pub const F3_230Mb_512: MEDIA_TYPE = MEDIA_TYPE(21i32);
pub const F3_240M_512: MEDIA_TYPE = MEDIA_TYPE(24i32);
pub const F3_2Pt88_512: MEDIA_TYPE = MEDIA_TYPE(3i32);
pub const F3_32M_512: MEDIA_TYPE = MEDIA_TYPE(25i32);
pub const F3_640_512: MEDIA_TYPE = MEDIA_TYPE(14i32);
pub const F3_720_512: MEDIA_TYPE = MEDIA_TYPE(5i32);
pub const F5_160_512: MEDIA_TYPE = MEDIA_TYPE(10i32);
pub const F5_180_512: MEDIA_TYPE = MEDIA_TYPE(9i32);
pub const F5_1Pt23_1024: MEDIA_TYPE = MEDIA_TYPE(19i32);
pub const F5_1Pt2_512: MEDIA_TYPE = MEDIA_TYPE(1i32);
pub const F5_320_1024: MEDIA_TYPE = MEDIA_TYPE(8i32);
pub const F5_320_512: MEDIA_TYPE = MEDIA_TYPE(7i32);
pub const F5_360_512: MEDIA_TYPE = MEDIA_TYPE(6i32);
pub const F5_640_512: MEDIA_TYPE = MEDIA_TYPE(15i32);
pub const F5_720_512: MEDIA_TYPE = MEDIA_TYPE(16i32);
pub const F8_256_128: MEDIA_TYPE = MEDIA_TYPE(22i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAT_STATISTICS {
    pub CreateHits: u32,
    pub SuccessfulCreates: u32,
    pub FailedCreates: u32,
    pub NonCachedReads: u32,
    pub NonCachedReadBytes: u32,
    pub NonCachedWrites: u32,
    pub NonCachedWriteBytes: u32,
    pub NonCachedDiskReads: u32,
    pub NonCachedDiskWrites: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILESYSTEM_STATISTICS {
    pub FileSystemType: FILESYSTEM_STATISTICS_TYPE,
    pub Version: u16,
    pub SizeOfCompleteStructure: u32,
    pub UserFileReads: u32,
    pub UserFileReadBytes: u32,
    pub UserDiskReads: u32,
    pub UserFileWrites: u32,
    pub UserFileWriteBytes: u32,
    pub UserDiskWrites: u32,
    pub MetaDataReads: u32,
    pub MetaDataReadBytes: u32,
    pub MetaDataDiskReads: u32,
    pub MetaDataWrites: u32,
    pub MetaDataWriteBytes: u32,
    pub MetaDataDiskWrites: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILESYSTEM_STATISTICS_EX {
    pub FileSystemType: FILESYSTEM_STATISTICS_TYPE,
    pub Version: u16,
    pub SizeOfCompleteStructure: u32,
    pub UserFileReads: u64,
    pub UserFileReadBytes: u64,
    pub UserDiskReads: u64,
    pub UserFileWrites: u64,
    pub UserFileWriteBytes: u64,
    pub UserDiskWrites: u64,
    pub MetaDataReads: u64,
    pub MetaDataReadBytes: u64,
    pub MetaDataDiskReads: u64,
    pub MetaDataWrites: u64,
    pub MetaDataWriteBytes: u64,
    pub MetaDataDiskWrites: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILESYSTEM_STATISTICS_TYPE(pub u16);
pub const FILESYSTEM_STATISTICS_TYPE_EXFAT: FILESYSTEM_STATISTICS_TYPE = FILESYSTEM_STATISTICS_TYPE(3u16);
pub const FILESYSTEM_STATISTICS_TYPE_FAT: FILESYSTEM_STATISTICS_TYPE = FILESYSTEM_STATISTICS_TYPE(2u16);
pub const FILESYSTEM_STATISTICS_TYPE_NTFS: FILESYSTEM_STATISTICS_TYPE = FILESYSTEM_STATISTICS_TYPE(1u16);
pub const FILESYSTEM_STATISTICS_TYPE_REFS: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_ALLOCATED_RANGE_BUFFER {
    pub FileOffset: i64,
    pub Length: i64,
}
pub const FILE_ANY_ACCESS: u32 = 0u32;
pub const FILE_CLEAR_ENCRYPTION: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    pub Class: FILE_STORAGE_TIER_CLASS,
    pub Flags: u32,
}
pub const FILE_DEVICE_8042_PORT: u32 = 39u32;
pub const FILE_DEVICE_ACPI: u32 = 50u32;
pub const FILE_DEVICE_BATTERY: u32 = 41u32;
pub const FILE_DEVICE_BEEP: u32 = 1u32;
pub const FILE_DEVICE_BIOMETRIC: u32 = 68u32;
pub const FILE_DEVICE_BLUETOOTH: u32 = 65u32;
pub const FILE_DEVICE_BUS_EXTENDER: u32 = 42u32;
pub const FILE_DEVICE_CD_ROM_FILE_SYSTEM: u32 = 3u32;
pub const FILE_DEVICE_CHANGER: u32 = 48u32;
pub const FILE_DEVICE_CONSOLE: u32 = 80u32;
pub const FILE_DEVICE_CONTROLLER: u32 = 4u32;
pub const FILE_DEVICE_CRYPT_PROVIDER: u32 = 63u32;
pub const FILE_DEVICE_DATALINK: u32 = 5u32;
pub const FILE_DEVICE_DEVAPI: u32 = 71u32;
pub const FILE_DEVICE_DFS: u32 = 6u32;
pub const FILE_DEVICE_DFS_FILE_SYSTEM: u32 = 53u32;
pub const FILE_DEVICE_DFS_VOLUME: u32 = 54u32;
pub const FILE_DEVICE_DISK_FILE_SYSTEM: u32 = 8u32;
pub const FILE_DEVICE_EHSTOR: u32 = 70u32;
pub const FILE_DEVICE_EVENT_COLLECTOR: u32 = 95u32;
pub const FILE_DEVICE_FILE_SYSTEM: u32 = 9u32;
pub const FILE_DEVICE_FIPS: u32 = 58u32;
pub const FILE_DEVICE_FULLSCREEN_VIDEO: u32 = 52u32;
pub const FILE_DEVICE_GPIO: u32 = 72u32;
pub const FILE_DEVICE_HOLOGRAPHIC: u32 = 91u32;
pub const FILE_DEVICE_INFINIBAND: u32 = 59u32;
pub const FILE_DEVICE_INPORT_PORT: u32 = 10u32;
pub const FILE_DEVICE_KEYBOARD: u32 = 11u32;
pub const FILE_DEVICE_KS: u32 = 47u32;
pub const FILE_DEVICE_KSEC: u32 = 57u32;
pub const FILE_DEVICE_MAILSLOT: u32 = 12u32;
pub const FILE_DEVICE_MASS_STORAGE: u32 = 45u32;
pub const FILE_DEVICE_MIDI_IN: u32 = 13u32;
pub const FILE_DEVICE_MIDI_OUT: u32 = 14u32;
pub const FILE_DEVICE_MODEM: u32 = 43u32;
pub const FILE_DEVICE_MOUSE: u32 = 15u32;
pub const FILE_DEVICE_MT_COMPOSITE: u32 = 66u32;
pub const FILE_DEVICE_MT_TRANSPORT: u32 = 67u32;
pub const FILE_DEVICE_MULTI_UNC_PROVIDER: u32 = 16u32;
pub const FILE_DEVICE_NAMED_PIPE: u32 = 17u32;
pub const FILE_DEVICE_NETWORK: u32 = 18u32;
pub const FILE_DEVICE_NETWORK_BROWSER: u32 = 19u32;
pub const FILE_DEVICE_NETWORK_FILE_SYSTEM: u32 = 20u32;
pub const FILE_DEVICE_NETWORK_REDIRECTOR: u32 = 40u32;
pub const FILE_DEVICE_NFP: u32 = 81u32;
pub const FILE_DEVICE_NULL: u32 = 21u32;
pub const FILE_DEVICE_NVDIMM: u32 = 90u32;
pub const FILE_DEVICE_PARALLEL_PORT: u32 = 22u32;
pub const FILE_DEVICE_PERSISTENT_MEMORY: u32 = 89u32;
pub const FILE_DEVICE_PHYSICAL_NETCARD: u32 = 23u32;
pub const FILE_DEVICE_PMI: u32 = 69u32;
pub const FILE_DEVICE_POINT_OF_SERVICE: u32 = 84u32;
pub const FILE_DEVICE_PRINTER: u32 = 24u32;
pub const FILE_DEVICE_PRM: u32 = 94u32;
pub const FILE_DEVICE_SCANNER: u32 = 25u32;
pub const FILE_DEVICE_SCREEN: u32 = 28u32;
pub const FILE_DEVICE_SDFXHCI: u32 = 92u32;
pub const FILE_DEVICE_SERENUM: u32 = 55u32;
pub const FILE_DEVICE_SERIAL_MOUSE_PORT: u32 = 26u32;
pub const FILE_DEVICE_SERIAL_PORT: u32 = 27u32;
pub const FILE_DEVICE_SMB: u32 = 46u32;
pub const FILE_DEVICE_SOUND: u32 = 29u32;
pub const FILE_DEVICE_SOUNDWIRE: u32 = 97u32;
pub const FILE_DEVICE_STORAGE_REPLICATION: u32 = 85u32;
pub const FILE_DEVICE_STREAMS: u32 = 30u32;
pub const FILE_DEVICE_SYSENV: u32 = 82u32;
pub const FILE_DEVICE_TAPE_FILE_SYSTEM: u32 = 32u32;
pub const FILE_DEVICE_TERMSRV: u32 = 56u32;
pub const FILE_DEVICE_TRANSPORT: u32 = 33u32;
pub const FILE_DEVICE_TRUST_ENV: u32 = 86u32;
pub const FILE_DEVICE_UCM: u32 = 87u32;
pub const FILE_DEVICE_UCMTCPCI: u32 = 88u32;
pub const FILE_DEVICE_UCMUCSI: u32 = 93u32;
pub const FILE_DEVICE_UNKNOWN: u32 = 34u32;
pub const FILE_DEVICE_USB4: u32 = 96u32;
pub const FILE_DEVICE_USBEX: u32 = 73u32;
pub const FILE_DEVICE_VDM: u32 = 44u32;
pub const FILE_DEVICE_VIDEO: u32 = 35u32;
pub const FILE_DEVICE_VIRTUAL_BLOCK: u32 = 83u32;
pub const FILE_DEVICE_VIRTUAL_DISK: u32 = 36u32;
pub const FILE_DEVICE_VMBUS: u32 = 62u32;
pub const FILE_DEVICE_WAVE_IN: u32 = 37u32;
pub const FILE_DEVICE_WAVE_OUT: u32 = 38u32;
pub const FILE_DEVICE_WPD: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    pub VolumeFlags: u32,
    pub FlagMask: u32,
    pub Version: u32,
    pub Reserved: u32,
}
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NON_RESIDENT: u64 = 137438953472u64;
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NOT_FOUND: u64 = 4096u64;
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_TOO_SMALL: u64 = 68719476736u64;
pub const FILE_INITIATE_REPAIR_HINT1_CLUSTERS_ALREADY_IN_USE: u64 = 32768u64;
pub const FILE_INITIATE_REPAIR_HINT1_DENY_DEFRAG: u64 = 274877906944u64;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_IS_BASE_RECORD: u64 = 524288u64;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_BASE_RECORD: u64 = 8u64;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_EXIST: u64 = 4u64;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_IN_USE: u64 = 1u64;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_ORPHAN: u64 = 262144u64;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_REUSED: u64 = 2u64;
pub const FILE_INITIATE_REPAIR_HINT1_INDEX_ENTRY_MISMATCH: u64 = 1099511627776u64;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_ARRAY_LENGTH_COUNT: u64 = 1048576u64;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_LCN: u64 = 4294967296u64;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_ORPHAN_RECOVERY_NAME: u64 = 2199023255552u64;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_PARENT: u64 = 8388608u64;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_RUN_LENGTH: u64 = 131072u64;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_VCN: u64 = 8589934592u64;
pub const FILE_INITIATE_REPAIR_HINT1_LCN_NOT_EXIST: u64 = 65536u64;
pub const FILE_INITIATE_REPAIR_HINT1_MULTIPLE_FILE_NAME_ATTRIBUTES: u64 = 4398046511104u64;
pub const FILE_INITIATE_REPAIR_HINT1_NAME_CONFLICT: u64 = 17179869184u64;
pub const FILE_INITIATE_REPAIR_HINT1_NOTHING_WRONG: u64 = 2048u64;
pub const FILE_INITIATE_REPAIR_HINT1_NOT_IMPLEMENTED: u64 = 32u64;
pub const FILE_INITIATE_REPAIR_HINT1_ORPHAN: u64 = 34359738368u64;
pub const FILE_INITIATE_REPAIR_HINT1_ORPHAN_GENERATED: u64 = 512u64;
pub const FILE_INITIATE_REPAIR_HINT1_OUT_OF_GENERIC_NAMES: u64 = 1073741824u64;
pub const FILE_INITIATE_REPAIR_HINT1_OUT_OF_RESOURCE: u64 = 2147483648u64;
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_BASE_RECORD: u64 = 134217728u64;
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_EXIST: u64 = 67108864u64;
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_INDEX: u64 = 268435456u64;
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_IN_USE: u64 = 16777216u64;
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_REUSED: u64 = 33554432u64;
pub const FILE_INITIATE_REPAIR_HINT1_POTENTIAL_CROSSLINK: u64 = 8192u64;
pub const FILE_INITIATE_REPAIR_HINT1_PREVIOUS_PARENT_STILL_VALID: u64 = 549755813888u64;
pub const FILE_INITIATE_REPAIR_HINT1_RECURSIVELY_CORRUPTED: u64 = 256u64;
pub const FILE_INITIATE_REPAIR_HINT1_REPAIRED: u64 = 1024u64;
pub const FILE_INITIATE_REPAIR_HINT1_REPAIR_DISABLED: u64 = 128u64;
pub const FILE_INITIATE_REPAIR_HINT1_SID_MISMATCH: u64 = 4194304u64;
pub const FILE_INITIATE_REPAIR_HINT1_SID_VALID: u64 = 2097152u64;
pub const FILE_INITIATE_REPAIR_HINT1_STALE_INFORMATION: u64 = 16384u64;
pub const FILE_INITIATE_REPAIR_HINT1_SYSTEM_FILE: u64 = 16u64;
pub const FILE_INITIATE_REPAIR_HINT1_UNABLE_TO_REPAIR: u64 = 64u64;
pub const FILE_INITIATE_REPAIR_HINT1_VALID_INDEX_ENTRY: u64 = 536870912u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    pub Hint1: u64,
    pub Hint2: u64,
    pub Clsn: u64,
    pub Status: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_LAYOUT_ENTRY {
    pub Version: u32,
    pub NextFileOffset: u32,
    pub Flags: u32,
    pub FileAttributes: u32,
    pub FileReferenceNumber: u64,
    pub FirstNameOffset: u32,
    pub FirstStreamOffset: u32,
    pub ExtraInfoOffset: u32,
    pub ExtraInfoLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_LAYOUT_INFO_ENTRY {
    pub BasicInformation: FILE_LAYOUT_INFO_ENTRY_0,
    pub OwnerId: u32,
    pub SecurityId: u32,
    pub Usn: i64,
    pub StorageReserveId: STORAGE_RESERVE_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_LAYOUT_INFO_ENTRY_0 {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_LAYOUT_NAME_ENTRY {
    pub NextNameOffset: u32,
    pub Flags: u32,
    pub ParentFileReferenceNumber: u64,
    pub FileNameLength: u32,
    pub Reserved: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_LAYOUT_NAME_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_LAYOUT_NAME_ENTRY_DOS: u32 = 2u32;
pub const FILE_LAYOUT_NAME_ENTRY_PRIMARY: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_LEVEL_TRIM {
    pub Key: u32,
    pub NumRanges: u32,
    pub Ranges: [FILE_LEVEL_TRIM_RANGE; 1],
}
impl Default for FILE_LEVEL_TRIM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_LEVEL_TRIM_OUTPUT {
    pub NumRangesProcessed: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_LEVEL_TRIM_RANGE {
    pub Offset: u64,
    pub Length: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_MAKE_COMPATIBLE_BUFFER {
    pub CloseDisc: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_OBJECTID_BUFFER {
    pub ObjectId: [u8; 16],
    pub Anonymous: FILE_OBJECTID_BUFFER_0,
}
impl Default for FILE_OBJECTID_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_OBJECTID_BUFFER_0 {
    pub Anonymous: FILE_OBJECTID_BUFFER_0_0,
    pub ExtendedInfo: [u8; 48],
}
impl Default for FILE_OBJECTID_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_OBJECTID_BUFFER_0_0 {
    pub BirthVolumeId: [u8; 16],
    pub BirthObjectId: [u8; 16],
    pub DomainId: [u8; 16],
}
impl Default for FILE_OBJECTID_BUFFER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_PREFETCH {
    pub Type: u32,
    pub Count: u32,
    pub Prefetch: [u64; 1],
}
impl Default for FILE_PREFETCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_PREFETCH_EX {
    pub Type: u32,
    pub Count: u32,
    pub Context: *mut core::ffi::c_void,
    pub Prefetch: [u64; 1],
}
impl Default for FILE_PREFETCH_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_PREFETCH_TYPE_FOR_CREATE: u32 = 1u32;
pub const FILE_PREFETCH_TYPE_FOR_CREATE_EX: u32 = 3u32;
pub const FILE_PREFETCH_TYPE_FOR_DIRENUM: u32 = 2u32;
pub const FILE_PREFETCH_TYPE_FOR_DIRENUM_EX: u32 = 4u32;
pub const FILE_PREFETCH_TYPE_MAX: u32 = 4u32;
pub const FILE_PROVIDER_COMPRESSION_MAXIMUM: u32 = 4u32;
pub const FILE_PROVIDER_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_PROVIDER_EXTERNAL_INFO_V0 {
    pub Version: u32,
    pub Algorithm: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_PROVIDER_EXTERNAL_INFO_V1 {
    pub Version: u32,
    pub Algorithm: u32,
    pub Flags: u32,
}
pub const FILE_PROVIDER_FLAG_COMPRESS_ON_WRITE: u32 = 1u32;
pub const FILE_PROVIDER_SINGLE_FILE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    pub DirectoryCount: i64,
    pub FileCount: i64,
    pub FsFormatMajVersion: u16,
    pub FsFormatMinVersion: u16,
    pub FsFormatName: [u16; 12],
    pub FormatTime: i64,
    pub LastUpdateTime: i64,
    pub CopyrightInfo: [u16; 34],
    pub AbstractInfo: [u16; 34],
    pub FormattingImplementationInfo: [u16; 34],
    pub LastModifyingImplementationInfo: [u16; 34],
}
impl Default for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_QUERY_SPARING_BUFFER {
    pub SparingUnitBytes: u32,
    pub SoftwareSparing: bool,
    pub TotalSpareBlocks: u32,
    pub FreeSpareBlocks: u32,
}
pub const FILE_READ_ACCESS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_REFERENCE_RANGE {
    pub StartingFileReferenceNumber: u64,
    pub EndingFileReferenceNumber: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_REGION_INFO {
    pub FileOffset: i64,
    pub Length: i64,
    pub Usage: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_REGION_INPUT {
    pub FileOffset: i64,
    pub Length: i64,
    pub DesiredUsage: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_REGION_OUTPUT {
    pub Flags: u32,
    pub TotalRegionEntryCount: u32,
    pub RegionEntryCount: u32,
    pub Reserved: u32,
    pub Region: [FILE_REGION_INFO; 1],
}
impl Default for FILE_REGION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_REGION_USAGE_HUGE_PAGE_ALIGNMENT: u32 = 16u32;
pub const FILE_REGION_USAGE_LARGE_PAGE_ALIGNMENT: u32 = 8u32;
pub const FILE_REGION_USAGE_OTHER_PAGE_ALIGNMENT: u32 = 4u32;
pub const FILE_REGION_USAGE_QUERY_ALIGNMENT: u32 = 8u32;
pub const FILE_REGION_USAGE_VALID_CACHED_DATA: u32 = 1u32;
pub const FILE_REGION_USAGE_VALID_NONCACHED_DATA: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_SET_DEFECT_MGMT_BUFFER {
    pub Disable: bool,
}
pub const FILE_SET_ENCRYPTION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_SET_SPARSE_BUFFER {
    pub SetSparse: bool,
}
pub const FILE_SPECIAL_ACCESS: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_STORAGE_TIER {
    pub Id: windows_core::GUID,
    pub Name: [u16; 256],
    pub Description: [u16; 256],
    pub Flags: u64,
    pub ProvisionedCapacity: u64,
    pub MediaType: FILE_STORAGE_TIER_MEDIA_TYPE,
    pub Class: FILE_STORAGE_TIER_CLASS,
}
impl Default for FILE_STORAGE_TIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_STORAGE_TIER_CLASS(pub i32);
pub const FILE_STORAGE_TIER_DESCRIPTION_LENGTH: u32 = 512u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_STORAGE_TIER_FLAGS(pub u32);
impl FILE_STORAGE_TIER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FILE_STORAGE_TIER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FILE_STORAGE_TIER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FILE_STORAGE_TIER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FILE_STORAGE_TIER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FILE_STORAGE_TIER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const FILE_STORAGE_TIER_FLAG_NO_SEEK_PENALTY: FILE_STORAGE_TIER_FLAGS = FILE_STORAGE_TIER_FLAGS(131072u32);
pub const FILE_STORAGE_TIER_FLAG_PARITY: u32 = 8388608u32;
pub const FILE_STORAGE_TIER_FLAG_READ_CACHE: u32 = 4194304u32;
pub const FILE_STORAGE_TIER_FLAG_SMR: u32 = 16777216u32;
pub const FILE_STORAGE_TIER_FLAG_WRITE_BACK_CACHE: u32 = 2097152u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_STORAGE_TIER_MEDIA_TYPE(pub i32);
pub const FILE_STORAGE_TIER_NAME_LENGTH: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_STORAGE_TIER_REGION {
    pub TierId: windows_core::GUID,
    pub Offset: u64,
    pub Length: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_SYSTEM_RECOGNITION_INFORMATION {
    pub FileSystem: [i8; 9],
}
impl Default for FILE_SYSTEM_RECOGNITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_BEGIN: u32 = 1u32;
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_END: u32 = 2u32;
pub const FILE_TYPE_NOTIFICATION_GUID_CRASHDUMP_FILE: windows_core::GUID = windows_core::GUID::from_u128(0x9d453eb7_d2a6_4dbd_a2e3_fbd0ed9109a9);
pub const FILE_TYPE_NOTIFICATION_GUID_HIBERNATION_FILE: windows_core::GUID = windows_core::GUID::from_u128(0xb7624d64_b9a3_4cf8_8011_5b86c940e7b7);
pub const FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE: windows_core::GUID = windows_core::GUID::from_u128(0x0d0a64a1_38fc_4db8_9fe7_3f4352cd7c5c);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_TYPE_NOTIFICATION_INPUT {
    pub Flags: u32,
    pub NumFileTypeIDs: u32,
    pub FileTypeID: [windows_core::GUID; 1],
}
impl Default for FILE_TYPE_NOTIFICATION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_WRITE_ACCESS: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_ZERO_DATA_INFORMATION {
    pub FileOffset: i64,
    pub BeyondFinalZero: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_ZERO_DATA_INFORMATION_EX {
    pub FileOffset: i64,
    pub BeyondFinalZero: i64,
    pub Flags: u32,
}
pub const FILE_ZERO_DATA_INFORMATION_FLAG_PRESERVE_CACHED_DATA: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FIND_BY_SID_DATA {
    pub Restart: u32,
    pub Sid: super::super::Security::SID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FIND_BY_SID_OUTPUT {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl Default for FIND_BY_SID_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLAG_USN_TRACK_MODIFIED_RANGES_ENABLE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FORMAT_EX_PARAMETERS {
    pub MediaType: MEDIA_TYPE,
    pub StartCylinderNumber: u32,
    pub EndCylinderNumber: u32,
    pub StartHeadNumber: u32,
    pub EndHeadNumber: u32,
    pub FormatGapLength: u16,
    pub SectorsPerTrack: u16,
    pub SectorNumber: [u16; 1],
}
impl Default for FORMAT_EX_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FORMAT_PARAMETERS {
    pub MediaType: MEDIA_TYPE,
    pub StartCylinderNumber: u32,
    pub EndCylinderNumber: u32,
    pub StartHeadNumber: u32,
    pub EndHeadNumber: u32,
}
pub const FSBPIO_INFL_None: FS_BPIO_INFLAGS = FS_BPIO_INFLAGS(0i32);
pub const FSBPIO_INFL_SKIP_STORAGE_STACK_QUERY: FS_BPIO_INFLAGS = FS_BPIO_INFLAGS(1i32);
pub const FSBPIO_OUTFL_COMPATIBLE_STORAGE_DRIVER: FS_BPIO_OUTFLAGS = FS_BPIO_OUTFLAGS(8i32);
pub const FSBPIO_OUTFL_FILTER_ATTACH_BLOCKED: FS_BPIO_OUTFLAGS = FS_BPIO_OUTFLAGS(4i32);
pub const FSBPIO_OUTFL_None: FS_BPIO_OUTFLAGS = FS_BPIO_OUTFLAGS(0i32);
pub const FSBPIO_OUTFL_STREAM_BYPASS_PAUSED: FS_BPIO_OUTFLAGS = FS_BPIO_OUTFLAGS(2i32);
pub const FSBPIO_OUTFL_VOLUME_STACK_BYPASS_PAUSED: FS_BPIO_OUTFLAGS = FS_BPIO_OUTFLAGS(1i32);
pub const FSCTL_ADD_OVERLAY: u32 = 623408u32;
pub const FSCTL_ADVANCE_FILE_ID: u32 = 590532u32;
pub const FSCTL_ALLOW_EXTENDED_DASD_IO: u32 = 589955u32;
pub const FSCTL_CLEAN_VOLUME_METADATA: u32 = 590716u32;
pub const FSCTL_CLEAR_ALL_LCN_WEAK_REFERENCES: u32 = 590956u32;
pub const FSCTL_CLEAR_LCN_WEAK_REFERENCE: u32 = 590948u32;
pub const FSCTL_CORRUPTION_HANDLING: u32 = 590432u32;
pub const FSCTL_CREATE_LCN_WEAK_REFERENCE: u32 = 590944u32;
pub const FSCTL_CREATE_OR_GET_OBJECT_ID: u32 = 590016u32;
pub const FSCTL_CREATE_USN_JOURNAL: u32 = 590055u32;
pub const FSCTL_CSC_INTERNAL: u32 = 590255u32;
pub const FSCTL_CSV_CONTROL: u32 = 590548u32;
pub const FSCTL_CSV_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT: u32 = 590420u32;
pub const FSCTL_CSV_GET_VOLUME_PATH_NAME: u32 = 590416u32;
pub const FSCTL_CSV_GET_VOLUME_PATH_NAMES_FOR_VOLUME_NAME: u32 = 590424u32;
pub const FSCTL_CSV_H_BREAKING_SYNC_TUNNEL_REQUEST: u32 = 590564u32;
pub const FSCTL_CSV_INTERNAL: u32 = 590444u32;
pub const FSCTL_CSV_MGMT_LOCK: u32 = 590524u32;
pub const FSCTL_CSV_QUERY_DOWN_LEVEL_FILE_SYSTEM_CHARACTERISTICS: u32 = 590528u32;
pub const FSCTL_CSV_QUERY_VETO_FILE_DIRECT_IO: u32 = 590540u32;
pub const FSCTL_CSV_SYNC_TUNNEL_REQUEST: u32 = 590536u32;
pub const FSCTL_CSV_TUNNEL_REQUEST: u32 = 590404u32;
pub const FSCTL_DELETE_CORRUPTED_REFS_CONTAINER: u32 = 590836u32;
pub const FSCTL_DELETE_EXTERNAL_BACKING: u32 = 590612u32;
pub const FSCTL_DELETE_OBJECT_ID: u32 = 589984u32;
pub const FSCTL_DELETE_REPARSE_POINT: u32 = 589996u32;
pub const FSCTL_DELETE_USN_JOURNAL: u32 = 590072u32;
pub const FSCTL_DFSR_SET_GHOST_HANDLE_STATE: u32 = 590264u32;
pub const FSCTL_DISABLE_LOCAL_BUFFERING: u32 = 590520u32;
pub const FSCTL_DISMOUNT_VOLUME: u32 = 589856u32;
pub const FSCTL_DUPLICATE_CLUSTER: u32 = 590940u32;
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE: u32 = 623428u32;
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE_EX: u32 = 623592u32;
pub const FSCTL_ENABLE_PER_IO_FLAGS: u32 = 590892u32;
pub const FSCTL_ENABLE_UPGRADE: u32 = 622800u32;
pub const FSCTL_ENCRYPTION_FSCTL_IO: u32 = 590043u32;
pub const FSCTL_ENCRYPTION_KEY_CONTROL: u32 = 590852u32;
pub const FSCTL_ENUM_EXTERNAL_BACKING: u32 = 590616u32;
pub const FSCTL_ENUM_OVERLAY: u32 = 590623u32;
pub const FSCTL_ENUM_USN_DATA: u32 = 590003u32;
pub const FSCTL_EXTEND_VOLUME: u32 = 590064u32;
pub const FSCTL_FILESYSTEM_GET_STATISTICS: u32 = 589920u32;
pub const FSCTL_FILESYSTEM_GET_STATISTICS_EX: u32 = 590732u32;
pub const FSCTL_FILE_LEVEL_TRIM: u32 = 623112u32;
pub const FSCTL_FILE_PREFETCH: u32 = 590112u32;
pub const FSCTL_FILE_TYPE_NOTIFICATION: u32 = 590340u32;
pub const FSCTL_FIND_FILES_BY_SID: u32 = 589967u32;
pub const FSCTL_GET_BOOT_AREA_INFO: u32 = 590384u32;
pub const FSCTL_GET_COMPRESSION: u32 = 589884u32;
pub const FSCTL_GET_EXTERNAL_BACKING: u32 = 590608u32;
pub const FSCTL_GET_FILTER_FILE_IDENTIFIER: u32 = 590788u32;
pub const FSCTL_GET_INTEGRITY_INFORMATION: u32 = 590460u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    pub ChecksumAlgorithm: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub ChecksumChunkSizeInBytes: u32,
    pub ClusterSizeInBytes: u32,
}
pub const FSCTL_GET_NTFS_FILE_RECORD: u32 = 589928u32;
pub const FSCTL_GET_NTFS_VOLUME_DATA: u32 = 589924u32;
pub const FSCTL_GET_OBJECT_ID: u32 = 589980u32;
pub const FSCTL_GET_REFS_VOLUME_DATA: u32 = 590552u32;
pub const FSCTL_GET_REPAIR: u32 = 590236u32;
pub const FSCTL_GET_REPARSE_POINT: u32 = 589992u32;
pub const FSCTL_GET_RETRIEVAL_POINTERS: u32 = 589939u32;
pub const FSCTL_GET_RETRIEVAL_POINTERS_AND_REFCOUNT: u32 = 590803u32;
pub const FSCTL_GET_RETRIEVAL_POINTER_BASE: u32 = 590388u32;
pub const FSCTL_GET_RETRIEVAL_POINTER_COUNT: u32 = 590891u32;
pub const FSCTL_GET_VOLUME_BITMAP: u32 = 589935u32;
pub const FSCTL_GET_WOF_VERSION: u32 = 590696u32;
pub const FSCTL_GHOST_FILE_EXTENTS: u32 = 623532u32;
pub const FSCTL_HCS_ASYNC_TUNNEL_REQUEST: u32 = 590704u32;
pub const FSCTL_HCS_SYNC_NO_WRITE_TUNNEL_REQUEST: u32 = 590776u32;
pub const FSCTL_HCS_SYNC_TUNNEL_REQUEST: u32 = 590700u32;
pub const FSCTL_INITIATE_FILE_METADATA_OPTIMIZATION: u32 = 590684u32;
pub const FSCTL_INITIATE_REPAIR: u32 = 590248u32;
pub const FSCTL_INTEGRITY_FLAG_CHECKSUM_ENFORCEMENT_OFF: u32 = 1u32;
pub const FSCTL_INVALIDATE_VOLUMES: u32 = 589908u32;
pub const FSCTL_IS_CSV_FILE: u32 = 590408u32;
pub const FSCTL_IS_FILE_ON_CSV_VOLUME: u32 = 590428u32;
pub const FSCTL_IS_PATHNAME_VALID: u32 = 589868u32;
pub const FSCTL_IS_VOLUME_DIRTY: u32 = 589944u32;
pub const FSCTL_IS_VOLUME_MOUNTED: u32 = 589864u32;
pub const FSCTL_IS_VOLUME_OWNED_BYCSVFS: u32 = 590456u32;
pub const FSCTL_LMR_QUERY_INFO: u32 = 590968u32;
pub const FSCTL_LOCK_VOLUME: u32 = 589848u32;
pub const FSCTL_LOOKUP_STREAM_FROM_CLUSTER: u32 = 590332u32;
pub const FSCTL_MAKE_MEDIA_COMPATIBLE: u32 = 622896u32;
pub const FSCTL_MANAGE_BYPASS_IO: u32 = 590920u32;
pub const FSCTL_MARK_AS_SYSTEM_HIVE: u32 = 589903u32;
pub const FSCTL_MARK_HANDLE: u32 = 590076u32;
pub const FSCTL_MARK_VOLUME_DIRTY: u32 = 589872u32;
pub const FSCTL_MOVE_FILE: u32 = 589940u32;
pub const FSCTL_NOTIFY_DATA_CHANGE: u32 = 590844u32;
pub const FSCTL_NOTIFY_STORAGE_SPACE_ALLOCATION: u32 = 590748u32;
pub const FSCTL_OFFLOAD_READ: u32 = 606820u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSCTL_OFFLOAD_READ_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub TokenTimeToLive: u32,
    pub Reserved: u32,
    pub FileOffset: u64,
    pub CopyLength: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSCTL_OFFLOAD_READ_OUTPUT {
    pub Size: u32,
    pub Flags: u32,
    pub TransferLength: u64,
    pub Token: [u8; 512],
}
impl Default for FSCTL_OFFLOAD_READ_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_OFFLOAD_WRITE: u32 = 623208u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSCTL_OFFLOAD_WRITE_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub FileOffset: u64,
    pub CopyLength: u64,
    pub TransferOffset: u64,
    pub Token: [u8; 512],
}
impl Default for FSCTL_OFFLOAD_WRITE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSCTL_OFFLOAD_WRITE_OUTPUT {
    pub Size: u32,
    pub Flags: u32,
    pub LengthWritten: u64,
}
pub const FSCTL_OPBATCH_ACK_CLOSE_PENDING: u32 = 589840u32;
pub const FSCTL_OPLOCK_BREAK_ACKNOWLEDGE: u32 = 589836u32;
pub const FSCTL_OPLOCK_BREAK_ACK_NO_2: u32 = 589904u32;
pub const FSCTL_OPLOCK_BREAK_NOTIFY: u32 = 589844u32;
pub const FSCTL_QUERY_ALLOCATED_RANGES: u32 = 606415u32;
pub const FSCTL_QUERY_ASYNC_DUPLICATE_EXTENTS_STATUS: u32 = 590896u32;
pub const FSCTL_QUERY_BAD_RANGES: u32 = 590828u32;
pub const FSCTL_QUERY_DEPENDENT_VOLUME: u32 = 590320u32;
pub const FSCTL_QUERY_DIRECT_ACCESS_EXTENTS: u32 = 590747u32;
pub const FSCTL_QUERY_DIRECT_IMAGE_ORIGINAL_BASE: u32 = 590756u32;
pub const FSCTL_QUERY_EXTENT_READ_CACHE_INFO: u32 = 590711u32;
pub const FSCTL_QUERY_FAT_BPB: u32 = 589912u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSCTL_QUERY_FAT_BPB_BUFFER {
    pub First0x24BytesOfBootSector: [u8; 36],
}
impl Default for FSCTL_QUERY_FAT_BPB_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_QUERY_FILE_LAYOUT: u32 = 590455u32;
pub const FSCTL_QUERY_FILE_METADATA_OPTIMIZATION: u32 = 590688u32;
pub const FSCTL_QUERY_FILE_REGIONS: u32 = 590468u32;
pub const FSCTL_QUERY_FILE_SYSTEM_RECOGNITION: u32 = 590412u32;
pub const FSCTL_QUERY_GHOSTED_FILE_EXTENTS: u32 = 590768u32;
pub const FSCTL_QUERY_LCN_WEAK_REFERENCE: u32 = 590952u32;
pub const FSCTL_QUERY_ON_DISK_VOLUME_INFO: u32 = 590140u32;
pub const FSCTL_QUERY_PAGEFILE_ENCRYPTION: u32 = 590312u32;
pub const FSCTL_QUERY_PERSISTENT_VOLUME_STATE: u32 = 590396u32;
pub const FSCTL_QUERY_REFS_SMR_VOLUME_INFO: u32 = 590812u32;
pub const FSCTL_QUERY_REFS_VOLUME_COUNTER_INFO: u32 = 590715u32;
pub const FSCTL_QUERY_REGION_INFO: u32 = 590576u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSCTL_QUERY_REGION_INFO_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub NumberOfTierIds: u32,
    pub TierIds: [windows_core::GUID; 1],
}
impl Default for FSCTL_QUERY_REGION_INFO_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSCTL_QUERY_REGION_INFO_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Alignment: u64,
    pub TotalNumberOfRegions: u32,
    pub NumberOfRegionsReturned: u32,
    pub Regions: [FILE_STORAGE_TIER_REGION; 1],
}
impl Default for FSCTL_QUERY_REGION_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_QUERY_RETRIEVAL_POINTERS: u32 = 589883u32;
pub const FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT: u32 = 590592u32;
pub const FSCTL_QUERY_SPARING_INFO: u32 = 590136u32;
pub const FSCTL_QUERY_STORAGE_CLASSES: u32 = 590572u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: FILE_STORAGE_TIER_FLAGS,
    pub TotalNumberOfTiers: u32,
    pub NumberOfTiersReturned: u32,
    pub Tiers: [FILE_STORAGE_TIER; 1],
}
impl Default for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_QUERY_USN_JOURNAL: u32 = 590068u32;
pub const FSCTL_QUERY_VOLUME_CONTAINER_STATE: u32 = 590736u32;
pub const FSCTL_QUERY_VOLUME_NUMA_INFO: u32 = 590804u32;
pub const FSCTL_READ_FILE_USN_DATA: u32 = 590059u32;
pub const FSCTL_READ_FROM_PLEX: u32 = 606494u32;
pub const FSCTL_READ_RAW_ENCRYPTED: u32 = 590051u32;
pub const FSCTL_READ_UNPRIVILEGED_USN_JOURNAL: u32 = 590763u32;
pub const FSCTL_READ_USN_JOURNAL: u32 = 590011u32;
pub const FSCTL_REARRANGE_FILE: u32 = 640032u32;
pub const FSCTL_RECALL_FILE: u32 = 590103u32;
pub const FSCTL_REFS_CHECKPOINT_VOLUME: u32 = 590972u32;
pub const FSCTL_REFS_DEALLOCATE_RANGES: u32 = 590808u32;
pub const FSCTL_REFS_DEALLOCATE_RANGES_EX: u32 = 590924u32;
pub const FSCTL_REFS_QUERY_VOLUME_COMPRESSION_INFO: u32 = 590936u32;
pub const FSCTL_REFS_QUERY_VOLUME_DEDUP_INFO: u32 = 590964u32;
pub const FSCTL_REFS_QUERY_VOLUME_IO_METRICS_INFO: u32 = 590988u32;
pub const FSCTL_REFS_QUERY_VOLUME_TOTAL_SHARED_LCNS: u32 = 590976u32;
pub const FSCTL_REFS_SET_VOLUME_COMPRESSION_INFO: u32 = 590932u32;
pub const FSCTL_REFS_SET_VOLUME_DEDUP_INFO: u32 = 590960u32;
pub const FSCTL_REFS_SET_VOLUME_IO_METRICS_INFO: u32 = 590984u32;
pub const FSCTL_REFS_STREAM_SNAPSHOT_MANAGEMENT: u32 = 590912u32;
pub const FSCTL_REMOVE_OVERLAY: u32 = 623412u32;
pub const FSCTL_REPAIR_COPIES: u32 = 639668u32;
pub const FSCTL_REQUEST_BATCH_OPLOCK: u32 = 589832u32;
pub const FSCTL_REQUEST_FILTER_OPLOCK: u32 = 589916u32;
pub const FSCTL_REQUEST_OPLOCK: u32 = 590400u32;
pub const FSCTL_REQUEST_OPLOCK_LEVEL_1: u32 = 589824u32;
pub const FSCTL_REQUEST_OPLOCK_LEVEL_2: u32 = 589828u32;
pub const FSCTL_RESET_VOLUME_ALLOCATION_HINTS: u32 = 590316u32;
pub const FSCTL_RKF_INTERNAL: u32 = 590511u32;
pub const FSCTL_SCRUB_DATA: u32 = 590512u32;
pub const FSCTL_SCRUB_UNDISCOVERABLE_ID: u32 = 590840u32;
pub const FSCTL_SD_GLOBAL_CHANGE: u32 = 590324u32;
pub const FSCTL_SECURITY_ID_CHECK: u32 = 606391u32;
pub const FSCTL_SET_BOOTLOADER_ACCESSED: u32 = 589903u32;
pub const FSCTL_SET_CACHED_RUNS_STATE: u32 = 590928u32;
pub const FSCTL_SET_COMPRESSION: u32 = 639040u32;
pub const FSCTL_SET_DAX_ALLOC_ALIGNMENT_HINT: u32 = 590832u32;
pub const FSCTL_SET_DEFECT_MANAGEMENT: u32 = 622900u32;
pub const FSCTL_SET_ENCRYPTION: u32 = 590039u32;
pub const FSCTL_SET_EXTERNAL_BACKING: u32 = 590604u32;
pub const FSCTL_SET_INTEGRITY_INFORMATION: u32 = 639616u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    pub ChecksumAlgorithm: u16,
    pub Reserved: u16,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    pub EnableIntegrity: u8,
    pub KeepIntegrityStateUnchanged: u8,
    pub Reserved: u16,
    pub Flags: u32,
    pub Version: u8,
    pub Reserved2: [u8; 7],
}
impl Default for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_SET_INTEGRITY_INFORMATION_EX: u32 = 590720u32;
pub const FSCTL_SET_LAYER_ROOT: u32 = 590740u32;
pub const FSCTL_SET_OBJECT_ID: u32 = 589976u32;
pub const FSCTL_SET_OBJECT_ID_EXTENDED: u32 = 590012u32;
pub const FSCTL_SET_PERSISTENT_VOLUME_STATE: u32 = 590392u32;
pub const FSCTL_SET_PURGE_FAILURE_MODE: u32 = 590448u32;
pub const FSCTL_SET_REFS_FILE_STRICTLY_SEQUENTIAL: u32 = 590820u32;
pub const FSCTL_SET_REFS_SMR_VOLUME_GC_PARAMETERS: u32 = 590816u32;
pub const FSCTL_SET_REPAIR: u32 = 590232u32;
pub const FSCTL_SET_REPARSE_POINT: u32 = 589988u32;
pub const FSCTL_SET_REPARSE_POINT_EX: u32 = 590860u32;
pub const FSCTL_SET_SHORT_NAME_BEHAVIOR: u32 = 590260u32;
pub const FSCTL_SET_SPARSE: u32 = 590020u32;
pub const FSCTL_SET_VOLUME_COMPRESSION_STATE: u32 = 590144u32;
pub const FSCTL_SET_ZERO_DATA: u32 = 622792u32;
pub const FSCTL_SET_ZERO_ON_DEALLOCATION: u32 = 590228u32;
pub const FSCTL_SHRINK_VOLUME: u32 = 590256u32;
pub const FSCTL_SHUFFLE_FILE: u32 = 639808u32;
pub const FSCTL_SIS_COPYFILE: u32 = 590080u32;
pub const FSCTL_SIS_LINK_FILES: u32 = 639236u32;
pub const FSCTL_SMB_SHARE_FLUSH_AND_PURGE: u32 = 590908u32;
pub const FSCTL_SPARSE_OVERALLOCATE: u32 = 590668u32;
pub const FSCTL_SSDI_STORAGE_REQUEST: u32 = 590752u32;
pub const FSCTL_START_VIRTUALIZATION_INSTANCE: u32 = 590784u32;
pub const FSCTL_START_VIRTUALIZATION_INSTANCE_EX: u32 = 590848u32;
pub const FSCTL_STORAGE_QOS_CONTROL: u32 = 590672u32;
pub const FSCTL_STREAMS_ASSOCIATE_ID: u32 = 590792u32;
pub const FSCTL_STREAMS_QUERY_ID: u32 = 590796u32;
pub const FSCTL_STREAMS_QUERY_PARAMETERS: u32 = 590788u32;
pub const FSCTL_SUSPEND_OVERLAY: u32 = 590724u32;
pub const FSCTL_SVHDX_ASYNC_TUNNEL_REQUEST: u32 = 590692u32;
pub const FSCTL_SVHDX_SET_INITIATOR_INFORMATION: u32 = 590600u32;
pub const FSCTL_SVHDX_SYNC_TUNNEL_REQUEST: u32 = 590596u32;
pub const FSCTL_TXFS_CREATE_MINIVERSION: u32 = 622972u32;
pub const FSCTL_TXFS_CREATE_SECONDARY_RM: u32 = 622952u32;
pub const FSCTL_TXFS_GET_METADATA_INFO: u32 = 606572u32;
pub const FSCTL_TXFS_GET_TRANSACTED_VERSION: u32 = 606576u32;
pub const FSCTL_TXFS_LIST_TRANSACTIONS: u32 = 606692u32;
pub const FSCTL_TXFS_LIST_TRANSACTION_LOCKED_FILES: u32 = 606688u32;
pub const FSCTL_TXFS_MODIFY_RM: u32 = 622916u32;
pub const FSCTL_TXFS_QUERY_RM_INFORMATION: u32 = 606536u32;
pub const FSCTL_TXFS_READ_BACKUP_INFORMATION: u32 = 606560u32;
pub const FSCTL_TXFS_READ_BACKUP_INFORMATION2: u32 = 590328u32;
pub const FSCTL_TXFS_ROLLFORWARD_REDO: u32 = 622928u32;
pub const FSCTL_TXFS_ROLLFORWARD_UNDO: u32 = 622932u32;
pub const FSCTL_TXFS_SAVEPOINT_INFORMATION: u32 = 622968u32;
pub const FSCTL_TXFS_SHUTDOWN_RM: u32 = 622940u32;
pub const FSCTL_TXFS_START_RM: u32 = 622936u32;
pub const FSCTL_TXFS_TRANSACTION_ACTIVE: u32 = 606604u32;
pub const FSCTL_TXFS_WRITE_BACKUP_INFORMATION: u32 = 622948u32;
pub const FSCTL_TXFS_WRITE_BACKUP_INFORMATION2: u32 = 590336u32;
pub const FSCTL_UNLOCK_VOLUME: u32 = 589852u32;
pub const FSCTL_UNMAP_SPACE: u32 = 590772u32;
pub const FSCTL_UPDATE_OVERLAY: u32 = 623416u32;
pub const FSCTL_UPGRADE_VOLUME: u32 = 590980u32;
pub const FSCTL_USN_TRACK_MODIFIED_RANGES: u32 = 590580u32;
pub const FSCTL_VIRTUAL_STORAGE_PASSTHROUGH: u32 = 590884u32;
pub const FSCTL_VIRTUAL_STORAGE_QUERY_PROPERTY: u32 = 590728u32;
pub const FSCTL_VIRTUAL_STORAGE_SET_BEHAVIOR: u32 = 590856u32;
pub const FSCTL_WAIT_FOR_REPAIR: u32 = 590240u32;
pub const FSCTL_WRITE_RAW_ENCRYPTED: u32 = 590047u32;
pub const FSCTL_WRITE_USN_CLOSE_RECORD: u32 = 590063u32;
pub const FSCTL_WRITE_USN_REASON: u32 = 590544u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FS_BPIO_INFLAGS(pub i32);
impl FS_BPIO_INFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FS_BPIO_INFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FS_BPIO_INFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FS_BPIO_INFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FS_BPIO_INFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FS_BPIO_INFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FS_BPIO_INFO {
    pub ActiveBypassIoCount: u32,
    pub StorageDriverNameLen: u16,
    pub StorageDriverName: [u16; 32],
}
impl Default for FS_BPIO_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FS_BPIO_INPUT {
    pub Operation: FS_BPIO_OPERATIONS,
    pub InFlags: FS_BPIO_INFLAGS,
    pub Reserved1: u64,
    pub Reserved2: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FS_BPIO_OPERATIONS(pub i32);
pub const FS_BPIO_OP_DISABLE: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(2i32);
pub const FS_BPIO_OP_ENABLE: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(1i32);
pub const FS_BPIO_OP_GET_INFO: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(8i32);
pub const FS_BPIO_OP_MAX_OPERATION: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(9i32);
pub const FS_BPIO_OP_QUERY: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(3i32);
pub const FS_BPIO_OP_STREAM_PAUSE: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(6i32);
pub const FS_BPIO_OP_STREAM_RESUME: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(7i32);
pub const FS_BPIO_OP_VOLUME_STACK_PAUSE: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(4i32);
pub const FS_BPIO_OP_VOLUME_STACK_RESUME: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FS_BPIO_OUTFLAGS(pub i32);
impl FS_BPIO_OUTFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FS_BPIO_OUTFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FS_BPIO_OUTFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FS_BPIO_OUTFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FS_BPIO_OUTFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FS_BPIO_OUTFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FS_BPIO_OUTPUT {
    pub Operation: FS_BPIO_OPERATIONS,
    pub OutFlags: FS_BPIO_OUTFLAGS,
    pub Reserved1: u64,
    pub Reserved2: u64,
    pub Anonymous: FS_BPIO_OUTPUT_0,
}
impl Default for FS_BPIO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FS_BPIO_OUTPUT_0 {
    pub Enable: FS_BPIO_RESULTS,
    pub Query: FS_BPIO_RESULTS,
    pub VolumeStackResume: FS_BPIO_RESULTS,
    pub StreamResume: FS_BPIO_RESULTS,
    pub GetInfo: FS_BPIO_INFO,
}
impl Default for FS_BPIO_OUTPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FS_BPIO_RESULTS {
    pub OpStatus: u32,
    pub FailingDriverNameLen: u16,
    pub FailingDriverName: [u16; 32],
    pub FailureReasonLen: u16,
    pub FailureReason: [u16; 128],
}
impl Default for FS_BPIO_RESULTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FW_ISSUEID_NO_ISSUE: u32 = 0u32;
pub const FW_ISSUEID_UNKNOWN: u32 = 4294967295u32;
pub const FileSnapStateInactive: DUPLICATE_EXTENTS_STATE = DUPLICATE_EXTENTS_STATE(0i32);
pub const FileSnapStateSource: DUPLICATE_EXTENTS_STATE = DUPLICATE_EXTENTS_STATE(1i32);
pub const FileSnapStateTarget: DUPLICATE_EXTENTS_STATE = DUPLICATE_EXTENTS_STATE(2i32);
pub const FileStorageTierClassCapacity: FILE_STORAGE_TIER_CLASS = FILE_STORAGE_TIER_CLASS(1i32);
pub const FileStorageTierClassMax: FILE_STORAGE_TIER_CLASS = FILE_STORAGE_TIER_CLASS(3i32);
pub const FileStorageTierClassPerformance: FILE_STORAGE_TIER_CLASS = FILE_STORAGE_TIER_CLASS(2i32);
pub const FileStorageTierClassUnspecified: FILE_STORAGE_TIER_CLASS = FILE_STORAGE_TIER_CLASS(0i32);
pub const FileStorageTierMediaTypeDisk: FILE_STORAGE_TIER_MEDIA_TYPE = FILE_STORAGE_TIER_MEDIA_TYPE(1i32);
pub const FileStorageTierMediaTypeMax: FILE_STORAGE_TIER_MEDIA_TYPE = FILE_STORAGE_TIER_MEDIA_TYPE(5i32);
pub const FileStorageTierMediaTypeScm: FILE_STORAGE_TIER_MEDIA_TYPE = FILE_STORAGE_TIER_MEDIA_TYPE(4i32);
pub const FileStorageTierMediaTypeSsd: FILE_STORAGE_TIER_MEDIA_TYPE = FILE_STORAGE_TIER_MEDIA_TYPE(2i32);
pub const FileStorageTierMediaTypeUnspecified: FILE_STORAGE_TIER_MEDIA_TYPE = FILE_STORAGE_TIER_MEDIA_TYPE(0i32);
pub const FixedMedia: MEDIA_TYPE = MEDIA_TYPE(12i32);
pub const FormFactor1_8: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(3i32);
pub const FormFactor1_8Less: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(4i32);
pub const FormFactor2_5: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(2i32);
pub const FormFactor3_5: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(1i32);
pub const FormFactorDimm: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(10i32);
pub const FormFactorEmbedded: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(5i32);
pub const FormFactorM_2: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(8i32);
pub const FormFactorMemoryCard: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(6i32);
pub const FormFactorPCIeBoard: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(9i32);
pub const FormFactorUnknown: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(0i32);
pub const FormFactormSata: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(7i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct GETVERSIONINPARAMS {
    pub bVersion: u8,
    pub bRevision: u8,
    pub bReserved: u8,
    pub bIDEDeviceMap: u8,
    pub fCapabilities: u32,
    pub dwReserved: [u32; 4],
}
impl Default for GETVERSIONINPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GET_CHANGER_PARAMETERS {
    pub Size: u32,
    pub NumberTransportElements: u16,
    pub NumberStorageElements: u16,
    pub NumberCleanerSlots: u16,
    pub NumberIEElements: u16,
    pub NumberDataTransferElements: u16,
    pub NumberOfDoors: u16,
    pub FirstSlotNumber: u16,
    pub FirstDriveNumber: u16,
    pub FirstTransportNumber: u16,
    pub FirstIEPortNumber: u16,
    pub FirstCleanerSlotAddress: u16,
    pub MagazineSize: u16,
    pub DriveCleanTimeout: u32,
    pub Features0: CHANGER_FEATURES,
    pub Features1: GET_CHANGER_PARAMETERS_FEATURES1,
    pub MoveFromTransport: u8,
    pub MoveFromSlot: u8,
    pub MoveFromIePort: u8,
    pub MoveFromDrive: u8,
    pub ExchangeFromTransport: u8,
    pub ExchangeFromSlot: u8,
    pub ExchangeFromIePort: u8,
    pub ExchangeFromDrive: u8,
    pub LockUnlockCapabilities: u8,
    pub PositionCapabilities: u8,
    pub Reserved1: [u8; 2],
    pub Reserved2: [u32; 2],
}
impl Default for GET_CHANGER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_CHANGER_PARAMETERS_FEATURES1(pub u32);
impl GET_CHANGER_PARAMETERS_FEATURES1 {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GET_CHANGER_PARAMETERS_FEATURES1 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GET_CHANGER_PARAMETERS_FEATURES1 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GET_CHANGER_PARAMETERS_FEATURES1 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub RequestDataType: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE,
    pub RequestDataSet: DEVICE_INTERNAL_STATUS_DATA_SET,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GET_DISK_ATTRIBUTES {
    pub Version: u32,
    pub Reserved1: u32,
    pub Attributes: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GET_FILTER_FILE_IDENTIFIER_INPUT {
    pub AltitudeLength: u16,
    pub Altitude: [u16; 1],
}
impl Default for GET_FILTER_FILE_IDENTIFIER_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    pub FilterFileIdentifierLength: u16,
    pub FilterFileIdentifier: [u8; 1],
}
impl Default for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GET_LENGTH_INFORMATION {
    pub Length: i64,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy)]
pub struct GET_MEDIA_TYPES {
    pub DeviceType: u32,
    pub MediaInfoCount: u32,
    pub MediaInfo: [DEVICE_MEDIA_INFO; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for GET_MEDIA_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GET_VOLUME_BITMAP_FLAG_MASK_METADATA: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPT_ATTRIBUTES(pub u64);
impl GPT_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GPT_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GPT_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GPT_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GPT_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GPT_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GPT_ATTRIBUTE_LEGACY_BIOS_BOOTABLE: u64 = 4u64;
pub const GPT_ATTRIBUTE_NO_BLOCK_IO_PROTOCOL: u64 = 2u64;
pub const GPT_ATTRIBUTE_PLATFORM_REQUIRED: GPT_ATTRIBUTES = GPT_ATTRIBUTES(1u64);
pub const GPT_BASIC_DATA_ATTRIBUTE_DAX: u64 = 288230376151711744u64;
pub const GPT_BASIC_DATA_ATTRIBUTE_HIDDEN: GPT_ATTRIBUTES = GPT_ATTRIBUTES(4611686018427387904u64);
pub const GPT_BASIC_DATA_ATTRIBUTE_NO_DRIVE_LETTER: GPT_ATTRIBUTES = GPT_ATTRIBUTES(9223372036854775808u64);
pub const GPT_BASIC_DATA_ATTRIBUTE_OFFLINE: u64 = 576460752303423488u64;
pub const GPT_BASIC_DATA_ATTRIBUTE_READ_ONLY: GPT_ATTRIBUTES = GPT_ATTRIBUTES(1152921504606846976u64);
pub const GPT_BASIC_DATA_ATTRIBUTE_SERVICE: u64 = 144115188075855872u64;
pub const GPT_BASIC_DATA_ATTRIBUTE_SHADOW_COPY: GPT_ATTRIBUTES = GPT_ATTRIBUTES(2305843009213693952u64);
pub const GPT_SPACES_ATTRIBUTE_NO_METADATA: u64 = 9223372036854775808u64;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct GP_LOG_PAGE_DESCRIPTOR {
    pub LogAddress: u16,
    pub LogSectors: u16,
}
pub const GUID_DEVICEDUMP_DRIVER_STORAGE_PORT: windows_core::GUID = windows_core::GUID::from_u128(0xda82441d_7142_4bc1_b844_0807c5a4b67f);
pub const GUID_DEVICEDUMP_STORAGE_DEVICE: windows_core::GUID = windows_core::GUID::from_u128(0xd8e2592f_1aab_4d56_a746_1f7585df40f4);
pub const GUID_DEVINTERFACE_CDCHANGER: windows_core::GUID = windows_core::GUID::from_u128(0x53f56312_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_CDROM: windows_core::GUID = windows_core::GUID::from_u128(0x53f56308_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_COMPORT: windows_core::GUID = windows_core::GUID::from_u128(0x86e0d1e0_8089_11d0_9ce4_08003e301f73);
pub const GUID_DEVINTERFACE_DISK: windows_core::GUID = windows_core::GUID::from_u128(0x53f56307_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_FLOPPY: windows_core::GUID = windows_core::GUID::from_u128(0x53f56311_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_HIDDEN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x7f108a28_9833_4b3b_b780_2c6b5fa5c062);
pub const GUID_DEVINTERFACE_MEDIUMCHANGER: windows_core::GUID = windows_core::GUID::from_u128(0x53f56310_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_PARTITION: windows_core::GUID = windows_core::GUID::from_u128(0x53f5630a_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_SCM_PHYSICAL_DEVICE: windows_core::GUID = windows_core::GUID::from_u128(0x4283609d_4dc2_43be_bbb4_4f15dfce2c61);
pub const GUID_DEVINTERFACE_SERENUM_BUS_ENUMERATOR: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e978_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVINTERFACE_SERVICE_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x6ead3d82_25ec_46bc_b7fd_c1f0df8f5037);
pub const GUID_DEVINTERFACE_SES: windows_core::GUID = windows_core::GUID::from_u128(0x1790c9ec_47d5_4df3_b5af_9adf3cf23e48);
pub const GUID_DEVINTERFACE_STORAGEPORT: windows_core::GUID = windows_core::GUID::from_u128(0x2accfe60_c130_11d2_b082_00a0c91efb8b);
pub const GUID_DEVINTERFACE_TAPE: windows_core::GUID = windows_core::GUID::from_u128(0x53f5630b_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_UNIFIED_ACCESS_RPMB: windows_core::GUID = windows_core::GUID::from_u128(0x27447c21_bcc3_4d07_a05b_a3395bb4eee7);
pub const GUID_DEVINTERFACE_VMLUN: windows_core::GUID = windows_core::GUID::from_u128(0x6f416619_9f29_42a5_b20b_37e219ca02b0);
pub const GUID_DEVINTERFACE_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x53f5630d_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_WRITEONCEDISK: windows_core::GUID = windows_core::GUID::from_u128(0x53f5630c_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_ZNSDISK: windows_core::GUID = windows_core::GUID::from_u128(0xb87941c5_ffdb_43c7_b6b1_20b632f0b109);
pub const GUID_SCM_PD_HEALTH_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0x9da2d386_72f5_4ee3_8155_eca0678e3b06);
pub const GUID_SCM_PD_PASSTHROUGH_INVDIMM: windows_core::GUID = windows_core::GUID::from_u128(0x4309ac30_0d11_11e4_9191_0800200c9a66);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HISTOGRAM_BUCKET {
    pub Reads: u32,
    pub Writes: u32,
}
pub const HIST_NO_OF_BUCKETS: u32 = 24u32;
pub const HITACHI_12_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(68i32);
pub const HealthStatusDisabled: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(4i32);
pub const HealthStatusFailed: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(5i32);
pub const HealthStatusNormal: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(1i32);
pub const HealthStatusThrottled: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(2i32);
pub const HealthStatusUnknown: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(0i32);
pub const HealthStatusWarning: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(3i32);
pub const IBM_3480: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(41i32);
pub const IBM_3490E: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(42i32);
pub const IBM_Magstar_3590: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(43i32);
pub const IBM_Magstar_MP: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(44i32);
pub const IDENTIFY_BUFFER_SIZE: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IDEREGS {
    pub bFeaturesReg: u8,
    pub bSectorCountReg: u8,
    pub bSectorNumberReg: u8,
    pub bCylLowReg: u8,
    pub bCylHighReg: u8,
    pub bDriveHeadReg: u8,
    pub bCommandReg: u8,
    pub bReserved: u8,
}
pub const ID_CMD: u32 = 236u32;
pub const IOCTL_CHANGER_BASE: u32 = 48u32;
pub const IOCTL_CHANGER_EXCHANGE_MEDIUM: u32 = 3162144u32;
pub const IOCTL_CHANGER_GET_ELEMENT_STATUS: u32 = 3194900u32;
pub const IOCTL_CHANGER_GET_PARAMETERS: u32 = 3162112u32;
pub const IOCTL_CHANGER_GET_PRODUCT_DATA: u32 = 3162120u32;
pub const IOCTL_CHANGER_GET_STATUS: u32 = 3162116u32;
pub const IOCTL_CHANGER_INITIALIZE_ELEMENT_STATUS: u32 = 3162136u32;
pub const IOCTL_CHANGER_MOVE_MEDIUM: u32 = 3162148u32;
pub const IOCTL_CHANGER_QUERY_VOLUME_TAGS: u32 = 3194924u32;
pub const IOCTL_CHANGER_REINITIALIZE_TRANSPORT: u32 = 3162152u32;
pub const IOCTL_CHANGER_SET_ACCESS: u32 = 3194896u32;
pub const IOCTL_CHANGER_SET_POSITION: u32 = 3162140u32;
pub const IOCTL_DISK_BASE: u32 = 7u32;
pub const IOCTL_DISK_CHECK_VERIFY: u32 = 477184u32;
pub const IOCTL_DISK_CONTROLLER_NUMBER: u32 = 458820u32;
pub const IOCTL_DISK_CREATE_DISK: u32 = 507992u32;
pub const IOCTL_DISK_DELETE_DRIVE_LAYOUT: u32 = 508160u32;
pub const IOCTL_DISK_EJECT_MEDIA: u32 = 477192u32;
pub const IOCTL_DISK_FIND_NEW_DEVICES: u32 = 477208u32;
pub const IOCTL_DISK_FORMAT_DRIVE: u32 = 508876u32;
pub const IOCTL_DISK_FORMAT_TRACKS: u32 = 507928u32;
pub const IOCTL_DISK_FORMAT_TRACKS_EX: u32 = 507948u32;
pub const IOCTL_DISK_GET_CACHE_INFORMATION: u32 = 475348u32;
pub const IOCTL_DISK_GET_DISK_ATTRIBUTES: u32 = 458992u32;
pub const IOCTL_DISK_GET_DRIVE_GEOMETRY: u32 = 458752u32;
pub const IOCTL_DISK_GET_DRIVE_GEOMETRY_EX: u32 = 458912u32;
pub const IOCTL_DISK_GET_DRIVE_LAYOUT: u32 = 475148u32;
pub const IOCTL_DISK_GET_DRIVE_LAYOUT_EX: u32 = 458832u32;
pub const IOCTL_DISK_GET_LENGTH_INFO: u32 = 475228u32;
pub const IOCTL_DISK_GET_MEDIA_TYPES: u32 = 461824u32;
pub const IOCTL_DISK_GET_PARTITION_INFO: u32 = 475140u32;
pub const IOCTL_DISK_GET_PARTITION_INFO_EX: u32 = 458824u32;
pub const IOCTL_DISK_GET_WRITE_CACHE_STATE: u32 = 475356u32;
pub const IOCTL_DISK_GROW_PARTITION: u32 = 508112u32;
pub const IOCTL_DISK_HISTOGRAM_DATA: u32 = 458804u32;
pub const IOCTL_DISK_HISTOGRAM_RESET: u32 = 458808u32;
pub const IOCTL_DISK_HISTOGRAM_STRUCTURE: u32 = 458800u32;
pub const IOCTL_DISK_IS_WRITABLE: u32 = 458788u32;
pub const IOCTL_DISK_LOAD_MEDIA: u32 = 477196u32;
pub const IOCTL_DISK_LOGGING: u32 = 458792u32;
pub const IOCTL_DISK_MEDIA_REMOVAL: u32 = 477188u32;
pub const IOCTL_DISK_PERFORMANCE: u32 = 458784u32;
pub const IOCTL_DISK_PERFORMANCE_OFF: u32 = 458848u32;
pub const IOCTL_DISK_REASSIGN_BLOCKS: u32 = 507932u32;
pub const IOCTL_DISK_REASSIGN_BLOCKS_EX: u32 = 508068u32;
pub const IOCTL_DISK_RELEASE: u32 = 477204u32;
pub const IOCTL_DISK_REQUEST_DATA: u32 = 458816u32;
pub const IOCTL_DISK_REQUEST_STRUCTURE: u32 = 458812u32;
pub const IOCTL_DISK_RESERVE: u32 = 477200u32;
pub const IOCTL_DISK_RESET_SNAPSHOT_INFO: u32 = 508432u32;
pub const IOCTL_DISK_SENSE_DEVICE: u32 = 459744u32;
pub const IOCTL_DISK_SET_CACHE_INFORMATION: u32 = 508120u32;
pub const IOCTL_DISK_SET_DISK_ATTRIBUTES: u32 = 508148u32;
pub const IOCTL_DISK_SET_DRIVE_LAYOUT: u32 = 507920u32;
pub const IOCTL_DISK_SET_DRIVE_LAYOUT_EX: u32 = 507988u32;
pub const IOCTL_DISK_SET_PARTITION_INFO: u32 = 507912u32;
pub const IOCTL_DISK_SET_PARTITION_INFO_EX: u32 = 507980u32;
pub const IOCTL_DISK_UPDATE_DRIVE_SIZE: u32 = 508104u32;
pub const IOCTL_DISK_UPDATE_PROPERTIES: u32 = 459072u32;
pub const IOCTL_DISK_VERIFY: u32 = 458772u32;
pub const IOCTL_SCMBUS_BASE: u32 = 89u32;
pub const IOCTL_SCMBUS_DEVICE_FUNCTION_BASE: u32 = 0u32;
pub const IOCTL_SCM_BUS_GET_LOGICAL_DEVICES: u32 = 5832704u32;
pub const IOCTL_SCM_BUS_GET_PHYSICAL_DEVICES: u32 = 5832708u32;
pub const IOCTL_SCM_BUS_GET_REGIONS: u32 = 5832712u32;
pub const IOCTL_SCM_BUS_QUERY_PROPERTY: u32 = 5832716u32;
pub const IOCTL_SCM_BUS_REFRESH_NAMESPACE: u32 = 5832728u32;
pub const IOCTL_SCM_BUS_RUNTIME_FW_ACTIVATE: u32 = 5865488u32;
pub const IOCTL_SCM_BUS_SET_PROPERTY: u32 = 5865492u32;
pub const IOCTL_SCM_LD_GET_INTERLEAVE_SET: u32 = 5835776u32;
pub const IOCTL_SCM_LOGICAL_DEVICE_FUNCTION_BASE: u32 = 768u32;
pub const IOCTL_SCM_PD_FIRMWARE_ACTIVATE: u32 = 5871624u32;
pub const IOCTL_SCM_PD_FIRMWARE_DOWNLOAD: u32 = 5871620u32;
pub const IOCTL_SCM_PD_PASSTHROUGH: u32 = 5888012u32;
pub const IOCTL_SCM_PD_QUERY_PROPERTY: u32 = 5838848u32;
pub const IOCTL_SCM_PD_REINITIALIZE_MEDIA: u32 = 5871636u32;
pub const IOCTL_SCM_PD_SET_PROPERTY: u32 = 5871640u32;
pub const IOCTL_SCM_PD_UPDATE_MANAGEMENT_STATUS: u32 = 5838864u32;
pub const IOCTL_SCM_PHYSICAL_DEVICE_FUNCTION_BASE: u32 = 1536u32;
pub const IOCTL_SERENUM_EXPOSE_HARDWARE: u32 = 3604992u32;
pub const IOCTL_SERENUM_GET_PORT_NAME: u32 = 3605004u32;
pub const IOCTL_SERENUM_PORT_DESC: u32 = 3605000u32;
pub const IOCTL_SERENUM_REMOVE_HARDWARE: u32 = 3604996u32;
pub const IOCTL_SERIAL_LSRMST_INSERT: u32 = 1769596u32;
pub const IOCTL_STORAGE_ALLOCATE_BC_STREAM: u32 = 3004420u32;
pub const IOCTL_STORAGE_ATTRIBUTE_MANAGEMENT: u32 = 3005596u32;
pub const IOCTL_STORAGE_BASE: u32 = 45u32;
pub const IOCTL_STORAGE_BC_VERSION: u32 = 1u32;
pub const IOCTL_STORAGE_BREAK_RESERVATION: u32 = 2969620u32;
pub const IOCTL_STORAGE_CHECK_PRIORITY_HINT_SUPPORT: u32 = 2955392u32;
pub const IOCTL_STORAGE_CHECK_VERIFY: u32 = 2967552u32;
pub const IOCTL_STORAGE_CHECK_VERIFY2: u32 = 2951168u32;
pub const IOCTL_STORAGE_DEVICE_POWER_CAP: u32 = 2956436u32;
pub const IOCTL_STORAGE_DEVICE_TELEMETRY_NOTIFY: u32 = 3002820u32;
pub const IOCTL_STORAGE_DEVICE_TELEMETRY_QUERY_CAPS: u32 = 3002824u32;
pub const IOCTL_STORAGE_DIAGNOSTIC: u32 = 2956448u32;
pub const IOCTL_STORAGE_EJECTION_CONTROL: u32 = 2951488u32;
pub const IOCTL_STORAGE_EJECT_MEDIA: u32 = 2967560u32;
pub const IOCTL_STORAGE_ENABLE_IDLE_POWER: u32 = 2956416u32;
pub const IOCTL_STORAGE_EVENT_NOTIFICATION: u32 = 2956432u32;
pub const IOCTL_STORAGE_FAILURE_PREDICTION_CONFIG: u32 = 2953476u32;
pub const IOCTL_STORAGE_FIND_NEW_DEVICES: u32 = 2967576u32;
pub const IOCTL_STORAGE_FIRMWARE_ACTIVATE: u32 = 3005448u32;
pub const IOCTL_STORAGE_FIRMWARE_DOWNLOAD: u32 = 3005444u32;
pub const IOCTL_STORAGE_FIRMWARE_GET_INFO: u32 = 2956288u32;
pub const IOCTL_STORAGE_FREE_BC_STREAM: u32 = 3004424u32;
pub const IOCTL_STORAGE_GET_BC_PROPERTIES: u32 = 2971648u32;
pub const IOCTL_STORAGE_GET_COUNTERS: u32 = 2953480u32;
pub const IOCTL_STORAGE_GET_DEVICE_INTERNAL_LOG: u32 = 2956484u32;
pub const IOCTL_STORAGE_GET_DEVICE_NUMBER: u32 = 2953344u32;
pub const IOCTL_STORAGE_GET_DEVICE_NUMBER_EX: u32 = 2953348u32;
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY: u32 = 3002816u32;
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY_RAW: u32 = 3002828u32;
pub const IOCTL_STORAGE_GET_HOTPLUG_INFO: u32 = 2952212u32;
pub const IOCTL_STORAGE_GET_IDLE_POWERUP_REASON: u32 = 2956420u32;
pub const IOCTL_STORAGE_GET_LB_PROVISIONING_MAP_RESOURCES: u32 = 2970632u32;
pub const IOCTL_STORAGE_GET_MEDIA_SERIAL_NUMBER: u32 = 2952208u32;
pub const IOCTL_STORAGE_GET_MEDIA_TYPES: u32 = 2952192u32;
pub const IOCTL_STORAGE_GET_MEDIA_TYPES_EX: u32 = 2952196u32;
pub const IOCTL_STORAGE_GET_PHYSICAL_ELEMENT_STATUS: u32 = 2956452u32;
pub const IOCTL_STORAGE_LOAD_MEDIA: u32 = 2967564u32;
pub const IOCTL_STORAGE_LOAD_MEDIA2: u32 = 2951180u32;
pub const IOCTL_STORAGE_MANAGE_BYPASS_IO: u32 = 2951360u32;
pub const IOCTL_STORAGE_MANAGE_DATA_SET_ATTRIBUTES: u32 = 2987012u32;
pub const IOCTL_STORAGE_MCN_CONTROL: u32 = 2951492u32;
pub const IOCTL_STORAGE_MEDIA_REMOVAL: u32 = 2967556u32;
pub const IOCTL_STORAGE_PERSISTENT_RESERVE_IN: u32 = 2969624u32;
pub const IOCTL_STORAGE_PERSISTENT_RESERVE_OUT: u32 = 3002396u32;
pub const IOCTL_STORAGE_POWER_ACTIVE: u32 = 2956424u32;
pub const IOCTL_STORAGE_POWER_IDLE: u32 = 2956428u32;
pub const IOCTL_STORAGE_PREDICT_FAILURE: u32 = 2953472u32;
pub const IOCTL_STORAGE_PROTOCOL_COMMAND: u32 = 3003328u32;
pub const IOCTL_STORAGE_QUERY_PROPERTY: u32 = 2954240u32;
pub const IOCTL_STORAGE_READ_CAPACITY: u32 = 2969920u32;
pub const IOCTL_STORAGE_REINITIALIZE_MEDIA: u32 = 2987584u32;
pub const IOCTL_STORAGE_RELEASE: u32 = 2967572u32;
pub const IOCTL_STORAGE_REMOVE_ELEMENT_AND_TRUNCATE: u32 = 2956480u32;
pub const IOCTL_STORAGE_RESERVE: u32 = 2967568u32;
pub const IOCTL_STORAGE_RESET_BUS: u32 = 2969600u32;
pub const IOCTL_STORAGE_RESET_DEVICE: u32 = 2969604u32;
pub const IOCTL_STORAGE_RPMB_COMMAND: u32 = 2956440u32;
pub const IOCTL_STORAGE_SET_HOTPLUG_INFO: u32 = 3001368u32;
pub const IOCTL_STORAGE_SET_PROPERTY: u32 = 2987004u32;
pub const IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD: u32 = 3002880u32;
pub const IOCTL_STORAGE_START_DATA_INTEGRITY_CHECK: u32 = 3004548u32;
pub const IOCTL_STORAGE_STOP_DATA_INTEGRITY_CHECK: u32 = 3004552u32;
pub const IOMEGA_JAZ: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(74i32);
pub const IOMEGA_ZIP: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(73i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IO_IRP_EXT_TRACK_OFFSET_HEADER {
    pub Validation: u16,
    pub Flags: u16,
    pub TrackedOffsetCallback: PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK,
}
pub const KODAK_14_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(70i32);
pub const KeepPrefetchedData: DISK_CACHE_RETENTION_PRIORITY = DISK_CACHE_RETENTION_PRIORITY(1i32);
pub const KeepReadData: DISK_CACHE_RETENTION_PRIORITY = DISK_CACHE_RETENTION_PRIORITY(2i32);
pub const LMRQuerySessionInfo: LMR_QUERY_INFO_CLASS = LMR_QUERY_INFO_CLASS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LMR_QUERY_INFO_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LMR_QUERY_INFO_PARAM {
    pub Operation: LMR_QUERY_INFO_CLASS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LMR_QUERY_SESSION_INFO {
    pub SessionId: u64,
}
pub const LOCK_ELEMENT: u32 = 0u32;
pub const LOCK_UNLOCK_DOOR: u32 = 2u32;
pub const LOCK_UNLOCK_IEPORT: u32 = 1u32;
pub const LOCK_UNLOCK_KEYPAD: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    pub OffsetToNext: u32,
    pub Flags: u32,
    pub Reserved: i64,
    pub Cluster: i64,
    pub FileName: [u16; 1],
}
impl Default for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_DATA: u32 = 16777216u32;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_INDEX: u32 = 33554432u32;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_MASK: u32 = 4278190080u32;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_SYSTEM: u32 = 50331648u32;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_DENY_DEFRAG_SET: u32 = 2u32;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_FS_SYSTEM_FILE: u32 = 4u32;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_PAGE_FILE: u32 = 1u32;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_TXF_SYSTEM_FILE: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    pub Flags: u32,
    pub NumberOfClusters: u32,
    pub Cluster: [i64; 1],
}
impl Default for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    pub Offset: u32,
    pub NumberOfMatches: u32,
    pub BufferSizeRequired: u32,
}
pub const LTO_Accelis: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(87i32);
pub const LTO_Ultrium: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(86i32);
pub const MARK_HANDLE_CLOUD_SYNC: u32 = 2048u32;
pub const MARK_HANDLE_DISABLE_FILE_METADATA_OPTIMIZATION: u32 = 4096u32;
pub const MARK_HANDLE_ENABLE_CPU_CACHE: u32 = 268435456u32;
pub const MARK_HANDLE_ENABLE_USN_SOURCE_ON_PAGING_IO: u32 = 8192u32;
pub const MARK_HANDLE_FILTER_METADATA: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MARK_HANDLE_INFO {
    pub Anonymous: MARK_HANDLE_INFO_0,
    pub VolumeHandle: super::super::Foundation::HANDLE,
    pub HandleInfo: u32,
}
impl Default for MARK_HANDLE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MARK_HANDLE_INFO_0 {
    pub UsnSourceInfo: u32,
    pub CopyNumber: u32,
}
impl Default for MARK_HANDLE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct MARK_HANDLE_INFO32 {
    pub Anonymous: MARK_HANDLE_INFO32_0,
    pub VolumeHandle: u32,
    pub HandleInfo: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MARK_HANDLE_INFO32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union MARK_HANDLE_INFO32_0 {
    pub UsnSourceInfo: u32,
    pub CopyNumber: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MARK_HANDLE_INFO32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MARK_HANDLE_NOT_READ_COPY: u32 = 256u32;
pub const MARK_HANDLE_NOT_REALTIME: u32 = 64u32;
pub const MARK_HANDLE_NOT_TXF_SYSTEM_LOG: u32 = 8u32;
pub const MARK_HANDLE_PROTECT_CLUSTERS: u32 = 1u32;
pub const MARK_HANDLE_READ_COPY: u32 = 128u32;
pub const MARK_HANDLE_REALTIME: u32 = 32u32;
pub const MARK_HANDLE_RETURN_PURGE_FAILURE: u32 = 1024u32;
pub const MARK_HANDLE_SKIP_COHERENCY_SYNC_DISALLOW_WRITES: u32 = 16384u32;
pub const MARK_HANDLE_SUPPRESS_VOLUME_OPEN_FLUSH: u32 = 32768u32;
pub const MARK_HANDLE_TXF_SYSTEM_LOG: u32 = 4u32;
pub const MAXIMUM_ENCRYPTION_VALUE: u32 = 4u32;
pub const MAX_FW_BUCKET_ID_LENGTH: u32 = 132u32;
pub const MAX_INTERFACE_CODES: u32 = 8u32;
pub const MAX_VOLUME_ID_SIZE: u32 = 36u32;
pub const MAX_VOLUME_TEMPLATE_SIZE: u32 = 40u32;
pub const MEDIA_CURRENTLY_MOUNTED: u32 = 2147483648u32;
pub const MEDIA_ERASEABLE: u32 = 1u32;
pub const MEDIA_READ_ONLY: u32 = 4u32;
pub const MEDIA_READ_WRITE: u32 = 8u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEDIA_TYPE(pub i32);
pub const MEDIA_WRITE_ONCE: u32 = 2u32;
pub const MEDIA_WRITE_PROTECTED: u32 = 256u32;
pub const METHOD_BUFFERED: u32 = 0u32;
pub const METHOD_DIRECT_FROM_HARDWARE: u32 = 2u32;
pub const METHOD_DIRECT_TO_HARDWARE: u32 = 1u32;
pub const METHOD_IN_DIRECT: u32 = 1u32;
pub const METHOD_NEITHER: u32 = 3u32;
pub const METHOD_OUT_DIRECT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MFT_ENUM_DATA_V0 {
    pub StartFileReferenceNumber: u64,
    pub LowUsn: i64,
    pub HighUsn: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MFT_ENUM_DATA_V1 {
    pub StartFileReferenceNumber: u64,
    pub LowUsn: i64,
    pub HighUsn: i64,
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MOVE_FILE_DATA {
    pub FileHandle: super::super::Foundation::HANDLE,
    pub StartingVcn: i64,
    pub StartingLcn: i64,
    pub ClusterCount: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MOVE_FILE_DATA32 {
    pub FileHandle: u32,
    pub StartingVcn: i64,
    pub StartingLcn: i64,
    pub ClusterCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MOVE_FILE_RECORD_DATA {
    pub FileHandle: super::super::Foundation::HANDLE,
    pub SourceFileRecord: i64,
    pub TargetFileRecord: i64,
}
pub const MO_3_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(57i32);
pub const MO_5_LIMDOW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(60i32);
pub const MO_5_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(59i32);
pub const MO_5_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(58i32);
pub const MO_NFR_525: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(71i32);
pub const MP2_8mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(79i32);
pub const MP_8mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(36i32);
pub const MiniQic: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(33i32);
pub const NCTP: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(40i32);
pub const NIKON_12_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(72i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_EXTENDED_VOLUME_DATA {
    pub ByteCount: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BytesPerPhysicalSector: u32,
    pub LfsMajorVersion: u16,
    pub LfsMinorVersion: u16,
    pub MaxDeviceTrimExtentCount: u32,
    pub MaxDeviceTrimByteCount: u32,
    pub MaxVolumeTrimExtentCount: u32,
    pub MaxVolumeTrimByteCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_FILE_RECORD_INPUT_BUFFER {
    pub FileReferenceNumber: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NTFS_FILE_RECORD_OUTPUT_BUFFER {
    pub FileReferenceNumber: i64,
    pub FileRecordLength: u32,
    pub FileRecordBuffer: [u8; 1],
}
impl Default for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS {
    pub LogFileFullExceptions: u32,
    pub OtherExceptions: u32,
    pub MftReads: u32,
    pub MftReadBytes: u32,
    pub MftWrites: u32,
    pub MftWriteBytes: u32,
    pub MftWritesUserLevel: NTFS_STATISTICS_0,
    pub MftWritesFlushForLogFileFull: u16,
    pub MftWritesLazyWriter: u16,
    pub MftWritesUserRequest: u16,
    pub Mft2Writes: u32,
    pub Mft2WriteBytes: u32,
    pub Mft2WritesUserLevel: NTFS_STATISTICS_1,
    pub Mft2WritesFlushForLogFileFull: u16,
    pub Mft2WritesLazyWriter: u16,
    pub Mft2WritesUserRequest: u16,
    pub RootIndexReads: u32,
    pub RootIndexReadBytes: u32,
    pub RootIndexWrites: u32,
    pub RootIndexWriteBytes: u32,
    pub BitmapReads: u32,
    pub BitmapReadBytes: u32,
    pub BitmapWrites: u32,
    pub BitmapWriteBytes: u32,
    pub BitmapWritesFlushForLogFileFull: u16,
    pub BitmapWritesLazyWriter: u16,
    pub BitmapWritesUserRequest: u16,
    pub BitmapWritesUserLevel: NTFS_STATISTICS_2,
    pub MftBitmapReads: u32,
    pub MftBitmapReadBytes: u32,
    pub MftBitmapWrites: u32,
    pub MftBitmapWriteBytes: u32,
    pub MftBitmapWritesFlushForLogFileFull: u16,
    pub MftBitmapWritesLazyWriter: u16,
    pub MftBitmapWritesUserRequest: u16,
    pub MftBitmapWritesUserLevel: NTFS_STATISTICS_3,
    pub UserIndexReads: u32,
    pub UserIndexReadBytes: u32,
    pub UserIndexWrites: u32,
    pub UserIndexWriteBytes: u32,
    pub LogFileReads: u32,
    pub LogFileReadBytes: u32,
    pub LogFileWrites: u32,
    pub LogFileWriteBytes: u32,
    pub Allocate: NTFS_STATISTICS_4,
    pub DiskResourcesExhausted: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_4 {
    pub Calls: u32,
    pub Clusters: u32,
    pub Hints: u32,
    pub RunsReturned: u32,
    pub HintsHonored: u32,
    pub HintsClusters: u32,
    pub Cache: u32,
    pub CacheClusters: u32,
    pub CacheMiss: u32,
    pub CacheMissClusters: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_2 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_1 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
    pub Flush: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_3 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
    pub Flush: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_0 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
    pub Flush: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_EX {
    pub LogFileFullExceptions: u32,
    pub OtherExceptions: u32,
    pub MftReads: u64,
    pub MftReadBytes: u64,
    pub MftWrites: u64,
    pub MftWriteBytes: u64,
    pub MftWritesUserLevel: NTFS_STATISTICS_EX_0,
    pub MftWritesFlushForLogFileFull: u32,
    pub MftWritesLazyWriter: u32,
    pub MftWritesUserRequest: u32,
    pub Mft2Writes: u64,
    pub Mft2WriteBytes: u64,
    pub Mft2WritesUserLevel: NTFS_STATISTICS_EX_1,
    pub Mft2WritesFlushForLogFileFull: u32,
    pub Mft2WritesLazyWriter: u32,
    pub Mft2WritesUserRequest: u32,
    pub RootIndexReads: u64,
    pub RootIndexReadBytes: u64,
    pub RootIndexWrites: u64,
    pub RootIndexWriteBytes: u64,
    pub BitmapReads: u64,
    pub BitmapReadBytes: u64,
    pub BitmapWrites: u64,
    pub BitmapWriteBytes: u64,
    pub BitmapWritesFlushForLogFileFull: u32,
    pub BitmapWritesLazyWriter: u32,
    pub BitmapWritesUserRequest: u32,
    pub BitmapWritesUserLevel: NTFS_STATISTICS_EX_2,
    pub MftBitmapReads: u64,
    pub MftBitmapReadBytes: u64,
    pub MftBitmapWrites: u64,
    pub MftBitmapWriteBytes: u64,
    pub MftBitmapWritesFlushForLogFileFull: u32,
    pub MftBitmapWritesLazyWriter: u32,
    pub MftBitmapWritesUserRequest: u32,
    pub MftBitmapWritesUserLevel: NTFS_STATISTICS_EX_3,
    pub UserIndexReads: u64,
    pub UserIndexReadBytes: u64,
    pub UserIndexWrites: u64,
    pub UserIndexWriteBytes: u64,
    pub LogFileReads: u64,
    pub LogFileReadBytes: u64,
    pub LogFileWrites: u64,
    pub LogFileWriteBytes: u64,
    pub Allocate: NTFS_STATISTICS_EX_4,
    pub DiskResourcesExhausted: u32,
    pub VolumeTrimCount: u64,
    pub VolumeTrimTime: u64,
    pub VolumeTrimByteCount: u64,
    pub FileLevelTrimCount: u64,
    pub FileLevelTrimTime: u64,
    pub FileLevelTrimByteCount: u64,
    pub VolumeTrimSkippedCount: u64,
    pub VolumeTrimSkippedByteCount: u64,
    pub NtfsFillStatInfoFromMftRecordCalledCount: u64,
    pub NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount: u64,
    pub NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_EX_4 {
    pub Calls: u32,
    pub RunsReturned: u32,
    pub Hints: u32,
    pub HintsHonored: u32,
    pub Cache: u32,
    pub CacheMiss: u32,
    pub Clusters: u64,
    pub HintsClusters: u64,
    pub CacheClusters: u64,
    pub CacheMissClusters: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_EX_2 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_EX_1 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_EX_3 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_STATISTICS_EX_0 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NTFS_VOLUME_DATA_BUFFER {
    pub VolumeSerialNumber: i64,
    pub NumberSectors: i64,
    pub TotalClusters: i64,
    pub FreeClusters: i64,
    pub TotalReserved: i64,
    pub BytesPerSector: u32,
    pub BytesPerCluster: u32,
    pub BytesPerFileRecordSegment: u32,
    pub ClustersPerFileRecordSegment: u32,
    pub MftValidDataLength: i64,
    pub MftStartLcn: i64,
    pub Mft2StartLcn: i64,
    pub MftZoneStart: i64,
    pub MftZoneEnd: i64,
}
pub const NVMeDataTypeFeature: STORAGE_PROTOCOL_NVME_DATA_TYPE = STORAGE_PROTOCOL_NVME_DATA_TYPE(3i32);
pub const NVMeDataTypeIdentify: STORAGE_PROTOCOL_NVME_DATA_TYPE = STORAGE_PROTOCOL_NVME_DATA_TYPE(1i32);
pub const NVMeDataTypeLogPage: STORAGE_PROTOCOL_NVME_DATA_TYPE = STORAGE_PROTOCOL_NVME_DATA_TYPE(2i32);
pub const NVMeDataTypeUnknown: STORAGE_PROTOCOL_NVME_DATA_TYPE = STORAGE_PROTOCOL_NVME_DATA_TYPE(0i32);
pub const OBSOLETE_DISK_GET_WRITE_CACHE_STATE: u32 = 475356u32;
pub const OBSOLETE_IOCTL_STORAGE_RESET_BUS: u32 = 3002368u32;
pub const OBSOLETE_IOCTL_STORAGE_RESET_DEVICE: u32 = 3002372u32;
pub const OFFLOAD_READ_FLAG_ALL_ZERO_BEYOND_CURRENT_RANGE: u32 = 1u32;
pub const OPLOCK_LEVEL_CACHE_HANDLE: u32 = 2u32;
pub const OPLOCK_LEVEL_CACHE_READ: u32 = 1u32;
pub const OPLOCK_LEVEL_CACHE_WRITE: u32 = 4u32;
pub const PARTIITON_OS_DATA: u32 = 41u32;
pub const PARTITION_BSP: u32 = 43u32;
pub const PARTITION_DM: u32 = 84u32;
pub const PARTITION_DPP: u32 = 44u32;
pub const PARTITION_ENTRY_UNUSED: u32 = 0u32;
pub const PARTITION_EXTENDED: u32 = 5u32;
pub const PARTITION_EZDRIVE: u32 = 85u32;
pub const PARTITION_FAT32: u32 = 11u32;
pub const PARTITION_FAT32_XINT13: u32 = 12u32;
pub const PARTITION_FAT_12: u32 = 1u32;
pub const PARTITION_FAT_16: u32 = 4u32;
pub const PARTITION_GPT: u32 = 238u32;
pub const PARTITION_HUGE: u32 = 6u32;
pub const PARTITION_IFS: u32 = 7u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PARTITION_INFORMATION {
    pub StartingOffset: i64,
    pub PartitionLength: i64,
    pub HiddenSectors: u32,
    pub PartitionNumber: u32,
    pub PartitionType: u8,
    pub BootIndicator: bool,
    pub RecognizedPartition: bool,
    pub RewritePartition: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PARTITION_INFORMATION_EX {
    pub PartitionStyle: PARTITION_STYLE,
    pub StartingOffset: i64,
    pub PartitionLength: i64,
    pub PartitionNumber: u32,
    pub RewritePartition: bool,
    pub IsServicePartition: bool,
    pub Anonymous: PARTITION_INFORMATION_EX_0,
}
impl Default for PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PARTITION_INFORMATION_EX_0 {
    pub Mbr: PARTITION_INFORMATION_MBR,
    pub Gpt: PARTITION_INFORMATION_GPT,
}
impl Default for PARTITION_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PARTITION_INFORMATION_GPT {
    pub PartitionType: windows_core::GUID,
    pub PartitionId: windows_core::GUID,
    pub Attributes: GPT_ATTRIBUTES,
    pub Name: [u16; 36],
}
impl Default for PARTITION_INFORMATION_GPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PARTITION_INFORMATION_MBR {
    pub PartitionType: u8,
    pub BootIndicator: bool,
    pub RecognizedPartition: bool,
    pub HiddenSectors: u32,
    pub PartitionId: windows_core::GUID,
}
pub const PARTITION_LDM: u32 = 66u32;
pub const PARTITION_MAIN_OS: u32 = 40u32;
pub const PARTITION_MSFT_RECOVERY: u32 = 39u32;
pub const PARTITION_NTFT: u32 = 128u32;
pub const PARTITION_OS2BOOTMGR: u32 = 10u32;
pub const PARTITION_PREP: u32 = 65u32;
pub const PARTITION_PRE_INSTALLED: u32 = 42u32;
pub const PARTITION_SPACES: u32 = 231u32;
pub const PARTITION_SPACES_DATA: u32 = 215u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PARTITION_STYLE(pub i32);
pub const PARTITION_STYLE_GPT: PARTITION_STYLE = PARTITION_STYLE(1i32);
pub const PARTITION_STYLE_MBR: PARTITION_STYLE = PARTITION_STYLE(0i32);
pub const PARTITION_STYLE_RAW: PARTITION_STYLE = PARTITION_STYLE(2i32);
pub const PARTITION_SYSTEM: u32 = 239u32;
pub const PARTITION_UNIX: u32 = 99u32;
pub const PARTITION_WINDOWS_SYSTEM: u32 = 45u32;
pub const PARTITION_XENIX_1: u32 = 2u32;
pub const PARTITION_XENIX_2: u32 = 3u32;
pub const PARTITION_XINT13: u32 = 14u32;
pub const PARTITION_XINT13_EXTENDED: u32 = 15u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PATHNAME_BUFFER {
    pub PathNameLength: u32,
    pub Name: [u16; 1],
}
impl Default for PATHNAME_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PC_5_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(62i32);
pub const PC_5_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(61i32);
pub const PD_5_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(63i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PERF_BIN {
    pub NumberOfBins: u32,
    pub TypeOfBin: u32,
    pub BinsRanges: [BIN_RANGE; 1],
}
impl Default for PERF_BIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PERSISTENT_RESERVE_COMMAND {
    pub Version: u32,
    pub Size: u32,
    pub Anonymous: PERSISTENT_RESERVE_COMMAND_0,
}
impl Default for PERSISTENT_RESERVE_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PERSISTENT_RESERVE_COMMAND_0 {
    pub PR_IN: PERSISTENT_RESERVE_COMMAND_0_0,
    pub PR_OUT: PERSISTENT_RESERVE_COMMAND_0_1,
}
impl Default for PERSISTENT_RESERVE_COMMAND_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERSISTENT_RESERVE_COMMAND_0_0 {
    pub _bitfield: u8,
    pub AllocationLength: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PERSISTENT_RESERVE_COMMAND_0_1 {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub ParameterList: [u8; 1],
}
impl Default for PERSISTENT_RESERVE_COMMAND_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PERSISTENT_VOLUME_STATE_BACKED_BY_WIM: u32 = 64u32;
pub const PERSISTENT_VOLUME_STATE_CHKDSK_RAN_ONCE: u32 = 1024u32;
pub const PERSISTENT_VOLUME_STATE_CONTAINS_BACKING_WIM: u32 = 32u32;
pub const PERSISTENT_VOLUME_STATE_DAX_FORMATTED: u32 = 4096u32;
pub const PERSISTENT_VOLUME_STATE_DEV_VOLUME: u32 = 8192u32;
pub const PERSISTENT_VOLUME_STATE_GLOBAL_METADATA_NO_SEEK_PENALTY: u32 = 4u32;
pub const PERSISTENT_VOLUME_STATE_LOCAL_METADATA_NO_SEEK_PENALTY: u32 = 8u32;
pub const PERSISTENT_VOLUME_STATE_MODIFIED_BY_CHKDSK: u32 = 2048u32;
pub const PERSISTENT_VOLUME_STATE_NO_HEAT_GATHERING: u32 = 16u32;
pub const PERSISTENT_VOLUME_STATE_NO_WRITE_AUTO_TIERING: u32 = 128u32;
pub const PERSISTENT_VOLUME_STATE_REALLOCATE_ALL_DATA_WRITES: u32 = 512u32;
pub const PERSISTENT_VOLUME_STATE_SHORT_NAME_CREATION_DISABLED: u32 = 1u32;
pub const PERSISTENT_VOLUME_STATE_TRUSTED_VOLUME: u32 = 16384u32;
pub const PERSISTENT_VOLUME_STATE_TXF_DISABLED: u32 = 256u32;
pub const PERSISTENT_VOLUME_STATE_VOLUME_SCRUB_DISABLED: u32 = 2u32;
pub const PHILIPS_12_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(67i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PHYSICAL_ELEMENT_STATUS {
    pub Version: u32,
    pub Size: u32,
    pub DescriptorCount: u32,
    pub ReturnedDescriptorCount: u32,
    pub ElementIdentifierBeingDepoped: u32,
    pub Reserved: u32,
    pub Descriptors: [PHYSICAL_ELEMENT_STATUS_DESCRIPTOR; 1],
}
impl Default for PHYSICAL_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub ElementIdentifier: u32,
    pub PhysicalElementType: u8,
    pub PhysicalElementHealth: u8,
    pub Reserved1: [u8; 2],
    pub AssociatedCapacity: u64,
    pub Reserved2: [u32; 4],
}
impl Default for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PHYSICAL_ELEMENT_STATUS_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub StartingElement: u32,
    pub Filter: u8,
    pub ReportType: u8,
    pub Reserved: [u8; 2],
}
impl Default for PHYSICAL_ELEMENT_STATUS_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PINNACLE_APEX_5_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(65i32);
pub type PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK = Option<unsafe extern "system" fn(sourcecontext: *const IO_IRP_EXT_TRACK_OFFSET_HEADER, targetcontext: *mut IO_IRP_EXT_TRACK_OFFSET_HEADER, relativeoffset: i64)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PLEX_READ_DATA_REQUEST {
    pub ByteOffset: i64,
    pub ByteLength: u32,
    pub PlexNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PREVENT_MEDIA_REMOVAL {
    pub PreventMediaRemoval: bool,
}
pub const PRODUCT_ID_LENGTH: u32 = 16u32;
pub const PROJFS_PROTOCOL_VERSION: u32 = 3u32;
pub const PropertyExistsQuery: STORAGE_QUERY_TYPE = STORAGE_QUERY_TYPE(1i32);
pub const PropertyExistsSet: STORAGE_SET_TYPE = STORAGE_SET_TYPE(1i32);
pub const PropertyMaskQuery: STORAGE_QUERY_TYPE = STORAGE_QUERY_TYPE(2i32);
pub const PropertyQueryMaxDefined: STORAGE_QUERY_TYPE = STORAGE_QUERY_TYPE(3i32);
pub const PropertySetMaxDefined: STORAGE_SET_TYPE = STORAGE_SET_TYPE(2i32);
pub const PropertyStandardQuery: STORAGE_QUERY_TYPE = STORAGE_QUERY_TYPE(0i32);
pub const PropertyStandardSet: STORAGE_SET_TYPE = STORAGE_SET_TYPE(0i32);
pub const ProtocolTypeAta: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(2i32);
pub const ProtocolTypeMaxReserved: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(127i32);
pub const ProtocolTypeNvme: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(3i32);
pub const ProtocolTypeProprietary: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(126i32);
pub const ProtocolTypeScsi: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(1i32);
pub const ProtocolTypeSd: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(4i32);
pub const ProtocolTypeUfs: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(5i32);
pub const ProtocolTypeUnknown: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(0i32);
pub const QIC: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(35i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QUERY_BAD_RANGES_INPUT {
    pub Flags: u32,
    pub NumRanges: u32,
    pub Ranges: [QUERY_BAD_RANGES_INPUT_RANGE; 1],
}
impl Default for QUERY_BAD_RANGES_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_BAD_RANGES_INPUT_RANGE {
    pub StartOffset: u64,
    pub LengthInBytes: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QUERY_BAD_RANGES_OUTPUT {
    pub Flags: u32,
    pub NumBadRanges: u32,
    pub NextOffsetToLookUp: u64,
    pub BadRanges: [QUERY_BAD_RANGES_OUTPUT_RANGE; 1],
}
impl Default for QUERY_BAD_RANGES_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_BAD_RANGES_OUTPUT_RANGE {
    pub Flags: u32,
    pub Reserved: u32,
    pub StartOffset: u64,
    pub LengthInBytes: u64,
}
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_GUEST_VOLUMES: u32 = 2u32;
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_HOST_VOLUMES: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_FILE_LAYOUT_FILTER_TYPE(pub i32);
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_CLUSTERS: QUERY_FILE_LAYOUT_FILTER_TYPE = QUERY_FILE_LAYOUT_FILTER_TYPE(1i32);
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_FILEID: QUERY_FILE_LAYOUT_FILTER_TYPE = QUERY_FILE_LAYOUT_FILTER_TYPE(2i32);
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_NONE: QUERY_FILE_LAYOUT_FILTER_TYPE = QUERY_FILE_LAYOUT_FILTER_TYPE(0i32);
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_STORAGE_RESERVE_ID: QUERY_FILE_LAYOUT_FILTER_TYPE = QUERY_FILE_LAYOUT_FILTER_TYPE(3i32);
pub const QUERY_FILE_LAYOUT_INCLUDE_EXTENTS: u32 = 8u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_EXTRA_INFO: u32 = 16u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_FILES_WITH_DSC_ATTRIBUTE: u32 = 4096u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_FULL_PATH_IN_NAMES: u32 = 64u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_NAMES: u32 = 2u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_ONLY_FILES_WITH_SPECIFIC_ATTRIBUTES: u32 = 2048u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAMS: u32 = 4u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAMS_WITH_NO_CLUSTERS_ALLOCATED: u32 = 32u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION: u32 = 128u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DATA_ATTRIBUTE: u32 = 8192u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DSC_ATTRIBUTE: u32 = 256u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EA_ATTRIBUTE: u32 = 32768u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EFS_ATTRIBUTE: u32 = 1024u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_REPARSE_ATTRIBUTE: u32 = 16384u32;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_TXF_ATTRIBUTE: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QUERY_FILE_LAYOUT_INPUT {
    pub Anonymous: QUERY_FILE_LAYOUT_INPUT_0,
    pub Flags: u32,
    pub FilterType: QUERY_FILE_LAYOUT_FILTER_TYPE,
    pub Reserved: u32,
    pub Filter: QUERY_FILE_LAYOUT_INPUT_1,
}
impl Default for QUERY_FILE_LAYOUT_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union QUERY_FILE_LAYOUT_INPUT_0 {
    pub FilterEntryCount: u32,
    pub NumberOfPairs: u32,
}
impl Default for QUERY_FILE_LAYOUT_INPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union QUERY_FILE_LAYOUT_INPUT_1 {
    pub ClusterRanges: [CLUSTER_RANGE; 1],
    pub FileReferenceRanges: [FILE_REFERENCE_RANGE; 1],
    pub StorageReserveIds: [STORAGE_RESERVE_ID; 1],
}
impl Default for QUERY_FILE_LAYOUT_INPUT_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const QUERY_FILE_LAYOUT_NUM_FILTER_TYPES: QUERY_FILE_LAYOUT_FILTER_TYPE = QUERY_FILE_LAYOUT_FILTER_TYPE(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_FILE_LAYOUT_OUTPUT {
    pub FileEntryCount: u32,
    pub FirstFileOffset: u32,
    pub Flags: u32,
    pub Reserved: u32,
}
pub const QUERY_FILE_LAYOUT_REPARSE_DATA_INVALID: u32 = 1u32;
pub const QUERY_FILE_LAYOUT_REPARSE_TAG_INVALID: u32 = 2u32;
pub const QUERY_FILE_LAYOUT_RESTART: u32 = 1u32;
pub const QUERY_FILE_LAYOUT_SINGLE_INSTANCED: u32 = 1u32;
pub const QUERY_STORAGE_CLASSES_FLAGS_MEASURE_READ: u32 = 1073741824u32;
pub const QUERY_STORAGE_CLASSES_FLAGS_MEASURE_WRITE: u32 = 2147483648u32;
pub const QUERY_STORAGE_CLASSES_FLAGS_NO_DEFRAG_VOLUME: u32 = 536870912u32;
pub const READ_ATTRIBUTES: u32 = 208u32;
pub const READ_ATTRIBUTE_BUFFER_SIZE: u32 = 512u32;
pub const READ_COMPRESSION_INFO_VALID: u32 = 32u32;
pub const READ_COPY_NUMBER_BYPASS_CACHE_FLAG: u32 = 256u32;
pub const READ_COPY_NUMBER_KEY: u32 = 1380142592u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct READ_ELEMENT_ADDRESS_INFO {
    pub NumberOfElements: u32,
    pub ElementStatus: [CHANGER_ELEMENT_STATUS; 1],
}
impl Default for READ_ELEMENT_ADDRESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct READ_FILE_USN_DATA {
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
pub const READ_THRESHOLDS: u32 = 209u32;
pub const READ_THRESHOLD_BUFFER_SIZE: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct READ_USN_JOURNAL_DATA_V0 {
    pub StartUsn: i64,
    pub ReasonMask: u32,
    pub ReturnOnlyOnClose: u32,
    pub Timeout: u64,
    pub BytesToWaitFor: u64,
    pub UsnJournalID: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct READ_USN_JOURNAL_DATA_V1 {
    pub StartUsn: i64,
    pub ReasonMask: u32,
    pub ReturnOnlyOnClose: u32,
    pub Timeout: u64,
    pub BytesToWaitFor: u64,
    pub UsnJournalID: u64,
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REASSIGN_BLOCKS {
    pub Reserved: u16,
    pub Count: u16,
    pub BlockNumber: [u32; 1],
}
impl Default for REASSIGN_BLOCKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct REASSIGN_BLOCKS_EX {
    pub Reserved: u16,
    pub Count: u16,
    pub BlockNumber: [i64; 1],
}
impl Default for REASSIGN_BLOCKS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RECOVERED_READS_VALID: u32 = 4u32;
pub const RECOVERED_WRITES_VALID: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_SMR_VOLUME_GC_ACTION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_SMR_VOLUME_GC_METHOD(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REFS_SMR_VOLUME_GC_PARAMETERS {
    pub Version: u32,
    pub Flags: u32,
    pub Action: REFS_SMR_VOLUME_GC_ACTION,
    pub Method: REFS_SMR_VOLUME_GC_METHOD,
    pub IoGranularity: u32,
    pub CompressionFormat: u32,
    pub Unused: [u64; 8],
}
impl Default for REFS_SMR_VOLUME_GC_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_SMR_VOLUME_GC_PARAMETERS_VERSION_V1: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_SMR_VOLUME_GC_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REFS_SMR_VOLUME_INFO_OUTPUT {
    pub Version: u32,
    pub Flags: u32,
    pub SizeOfRandomlyWritableTier: i64,
    pub FreeSpaceInRandomlyWritableTier: i64,
    pub SizeofSMRTier: i64,
    pub FreeSpaceInSMRTier: i64,
    pub UsableFreeSpaceInSMRTier: i64,
    pub VolumeGcState: REFS_SMR_VOLUME_GC_STATE,
    pub VolumeGcLastStatus: u32,
    pub CurrentGcBandFillPercentage: u32,
    pub Unused: [u64; 6],
}
impl Default for REFS_SMR_VOLUME_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V0: u32 = 0u32;
pub const REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REFS_VOLUME_DATA_BUFFER {
    pub ByteCount: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub BytesPerPhysicalSector: u32,
    pub VolumeSerialNumber: i64,
    pub NumberSectors: i64,
    pub TotalClusters: i64,
    pub FreeClusters: i64,
    pub TotalReserved: i64,
    pub BytesPerSector: u32,
    pub BytesPerCluster: u32,
    pub MaximumSizeOfResidentFile: i64,
    pub FastTierDataFillRatio: u16,
    pub SlowTierDataFillRatio: u16,
    pub DestagesFastTierToSlowTierRate: u32,
    pub MetadataChecksumType: u16,
    pub Reserved0: [u8; 6],
    pub Reserved: [i64; 8],
}
impl Default for REFS_VOLUME_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub RequestCapacity: u64,
    pub ElementIdentifier: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REPAIR_COPIES_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub FileOffset: i64,
    pub Length: u32,
    pub SourceCopy: u32,
    pub NumberOfRepairCopies: u32,
    pub RepairCopies: [u32; 1],
}
impl Default for REPAIR_COPIES_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPAIR_COPIES_OUTPUT {
    pub Size: u32,
    pub Status: u32,
    pub ResumeFileOffset: i64,
}
pub const REPLACE_ALTERNATE: u32 = 11u32;
pub const REPLACE_PRIMARY: u32 = 10u32;
pub const REQUEST_OPLOCK_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REQUEST_OPLOCK_INPUT_BUFFER {
    pub StructureVersion: u16,
    pub StructureLength: u16,
    pub RequestedOplockLevel: u32,
    pub Flags: u32,
}
pub const REQUEST_OPLOCK_INPUT_FLAG_ACK: u32 = 2u32;
pub const REQUEST_OPLOCK_INPUT_FLAG_COMPLETE_ACK_ON_CLOSE: u32 = 4u32;
pub const REQUEST_OPLOCK_INPUT_FLAG_REQUEST: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REQUEST_OPLOCK_OUTPUT_BUFFER {
    pub StructureVersion: u16,
    pub StructureLength: u16,
    pub OriginalOplockLevel: u32,
    pub NewOplockLevel: u32,
    pub Flags: u32,
    pub AccessMode: u32,
    pub ShareMode: u16,
}
pub const REQUEST_OPLOCK_OUTPUT_FLAG_ACK_REQUIRED: u32 = 1u32;
pub const REQUEST_OPLOCK_OUTPUT_FLAG_MODES_PROVIDED: u32 = 2u32;
pub const REQUEST_OPLOCK_OUTPUT_FLAG_WRITABLE_SECTION_PRESENT: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REQUEST_RAW_ENCRYPTED_DATA {
    pub FileOffset: i64,
    pub Length: u32,
}
pub const RETRACT_IEPORT: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    pub ExtentCount: u32,
    pub StartingVcn: i64,
    pub Extents: [RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0; 1],
}
impl Default for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    pub NextVcn: i64,
    pub Lcn: i64,
    pub ReferenceCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RETRIEVAL_POINTERS_BUFFER {
    pub ExtentCount: u32,
    pub StartingVcn: i64,
    pub Extents: [RETRIEVAL_POINTERS_BUFFER_0; 1],
}
impl Default for RETRIEVAL_POINTERS_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RETRIEVAL_POINTERS_BUFFER_0 {
    pub NextVcn: i64,
    pub Lcn: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RETRIEVAL_POINTER_BASE {
    pub FileAreaOffset: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RETRIEVAL_POINTER_COUNT {
    pub ExtentCount: u32,
}
pub const RETURN_SMART_STATUS: u32 = 218u32;
pub const REVISION_LENGTH: u32 = 4u32;
pub const RemovableMedia: MEDIA_TYPE = MEDIA_TYPE(11i32);
pub const RequestLocation: BIN_TYPES = BIN_TYPES(1i32);
pub const RequestSize: BIN_TYPES = BIN_TYPES(0i32);
pub const SAIT: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(93i32);
pub const SAVE_ATTRIBUTE_VALUES: u32 = 211u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    pub Version: u32,
    pub Size: u32,
    pub DeviceCount: u32,
    pub Devices: [SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO; 1],
}
impl Default for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    pub DeviceGuid: windows_core::GUID,
    pub DeviceNumber: u32,
    pub Flags: SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0,
    pub DeviceSize: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_BUS_DEDICATED_MEMORY_STATE {
    pub ActivateState: bool,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_BUS_FIRMWARE_ACTIVATION_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_BUS_PROPERTY_ID(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_BUS_PROPERTY_QUERY {
    pub Version: u32,
    pub Size: u32,
    pub PropertyId: SCM_BUS_PROPERTY_ID,
    pub QueryType: SCM_BUS_QUERY_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl Default for SCM_BUS_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_BUS_PROPERTY_SET {
    pub Version: u32,
    pub Size: u32,
    pub PropertyId: SCM_BUS_PROPERTY_ID,
    pub SetType: SCM_BUS_SET_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl Default for SCM_BUS_PROPERTY_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_BUS_QUERY_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    pub Version: u32,
    pub Size: u32,
    pub RuntimeFwActivationSupported: bool,
    pub FirmwareActivationState: SCM_BUS_FIRMWARE_ACTIVATION_STATE,
    pub FirmwareActivationCapability: SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0,
    pub EstimatedFirmwareActivationTimeInUSecs: u64,
    pub EstimatedProcessorAccessQuiesceTimeInUSecs: u64,
    pub EstimatedIOAccessQuiesceTimeInUSecs: u64,
    pub PlatformSupportedMaxIOAccessQuiesceTimeInUSecs: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    pub _bitfield: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_BUS_SET_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_INTERLEAVED_PD_INFO {
    pub DeviceHandle: u32,
    pub DeviceGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_LD_INTERLEAVE_SET_INFO {
    pub Version: u32,
    pub Size: u32,
    pub InterleaveSetSize: u32,
    pub InterleaveSet: [SCM_INTERLEAVED_PD_INFO; 1],
}
impl Default for SCM_LD_INTERLEAVE_SET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_LOGICAL_DEVICES {
    pub Version: u32,
    pub Size: u32,
    pub DeviceCount: u32,
    pub Devices: [SCM_LOGICAL_DEVICE_INSTANCE; 1],
}
impl Default for SCM_LOGICAL_DEVICES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_LOGICAL_DEVICE_INSTANCE {
    pub Version: u32,
    pub Size: u32,
    pub DeviceGuid: windows_core::GUID,
    pub SymbolicLink: [u16; 256],
}
impl Default for SCM_LOGICAL_DEVICE_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCM_MAX_SYMLINK_LEN_IN_CHARS: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_PD_DESCRIPTOR_HEADER {
    pub Version: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_PD_DEVICE_HANDLE {
    pub Version: u32,
    pub Size: u32,
    pub DeviceGuid: windows_core::GUID,
    pub DeviceHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_DEVICE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub DeviceGuid: windows_core::GUID,
    pub UnsafeShutdownCount: u32,
    pub PersistentMemorySizeInBytes: u64,
    pub VolatileMemorySizeInBytes: u64,
    pub TotalMemorySizeInBytes: u64,
    pub SlotNumber: u32,
    pub DeviceHandle: u32,
    pub PhysicalId: u16,
    pub NumberOfFormatInterfaceCodes: u8,
    pub FormatInterfaceCodes: [u16; 8],
    pub VendorId: u32,
    pub ProductId: u32,
    pub SubsystemDeviceId: u32,
    pub SubsystemVendorId: u32,
    pub ManufacturingLocation: u8,
    pub ManufacturingWeek: u8,
    pub ManufacturingYear: u8,
    pub SerialNumber4Byte: u32,
    pub SerialNumberLengthInChars: u32,
    pub SerialNumber: [i8; 1],
}
impl Default for SCM_PD_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_DEVICE_SPECIFIC_INFO {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfProperties: u32,
    pub DeviceSpecificProperties: [SCM_PD_DEVICE_SPECIFIC_PROPERTY; 1],
}
impl Default for SCM_PD_DEVICE_SPECIFIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    pub Name: [u16; 128],
    pub Value: i64,
}
impl Default for SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_PD_FIRMWARE_ACTIVATE {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Slot: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_FIRMWARE_ACTIVATION_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_FIRMWARE_DOWNLOAD {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub Offset: u64,
    pub FirmwareImageSizeInBytes: u32,
    pub FirmwareImage: [u8; 1],
}
impl Default for SCM_PD_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_FIRMWARE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub ActiveSlot: u8,
    pub NextActiveSlot: u8,
    pub SlotCount: u8,
    pub Slots: [SCM_PD_FIRMWARE_SLOT_INFO; 1],
}
impl Default for SCM_PD_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCM_PD_FIRMWARE_LAST_DOWNLOAD: u32 = 1u32;
pub const SCM_PD_FIRMWARE_REVISION_LENGTH_BYTES: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_FIRMWARE_SLOT_INFO {
    pub Version: u32,
    pub Size: u32,
    pub SlotNumber: u8,
    pub _bitfield: u8,
    pub Reserved1: [u8; 6],
    pub Revision: [u8; 32],
}
impl Default for SCM_PD_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_FRU_ID_STRING {
    pub Version: u32,
    pub Size: u32,
    pub IdentifierSize: u32,
    pub Identifier: [u8; 1],
}
impl Default for SCM_PD_FRU_ID_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_PD_HEALTH_NOTIFICATION_DATA {
    pub DeviceGuid: windows_core::GUID,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_HEALTH_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_LAST_FW_ACTIVATION_STATUS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_LOCATION_STRING {
    pub Version: u32,
    pub Size: u32,
    pub Location: [u16; 1],
}
impl Default for SCM_PD_LOCATION_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_MANAGEMENT_STATUS {
    pub Version: u32,
    pub Size: u32,
    pub Health: SCM_PD_HEALTH_STATUS,
    pub NumberOfOperationalStatus: u32,
    pub NumberOfAdditionalReasons: u32,
    pub OperationalStatus: [SCM_PD_OPERATIONAL_STATUS; 16],
    pub AdditionalReasons: [SCM_PD_OPERATIONAL_STATUS_REASON; 1],
}
impl Default for SCM_PD_MANAGEMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCM_PD_MAX_OPERATIONAL_STATUS: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_MEDIA_REINITIALIZATION_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_OPERATIONAL_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_OPERATIONAL_STATUS_REASON(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_PASSTHROUGH_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub ProtocolGuid: windows_core::GUID,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl Default for SCM_PD_PASSTHROUGH_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    pub Opcode: u32,
    pub OpcodeParametersLength: u32,
    pub OpcodeParameters: [u8; 1],
}
impl Default for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    pub GeneralStatus: u16,
    pub ExtendedStatus: u16,
    pub OutputDataLength: u32,
    pub OutputData: [u8; 1],
}
impl Default for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_PASSTHROUGH_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub ProtocolGuid: windows_core::GUID,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl Default for SCM_PD_PASSTHROUGH_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_PROPERTY_ID(pub i32);
pub const SCM_PD_PROPERTY_NAME_LENGTH_IN_CHARS: u32 = 128u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_PROPERTY_QUERY {
    pub Version: u32,
    pub Size: u32,
    pub PropertyId: SCM_PD_PROPERTY_ID,
    pub QueryType: SCM_PD_QUERY_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl Default for SCM_PD_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PD_PROPERTY_SET {
    pub Version: u32,
    pub Size: u32,
    pub PropertyId: SCM_PD_PROPERTY_ID,
    pub SetType: SCM_PD_SET_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl Default for SCM_PD_PROPERTY_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_QUERY_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_PD_REINITIALIZE_MEDIA_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub Options: SCM_PD_REINITIALIZE_MEDIA_INPUT_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Status: SCM_PD_MEDIA_REINITIALIZATION_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    pub ArmState: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    pub Version: u32,
    pub Size: u32,
    pub LastFirmwareActivationStatus: SCM_PD_LAST_FW_ACTIVATION_STATUS,
    pub FirmwareActivationState: SCM_PD_FIRMWARE_ACTIVATION_STATE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_SET_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PHYSICAL_DEVICES {
    pub Version: u32,
    pub Size: u32,
    pub DeviceCount: u32,
    pub Devices: [SCM_PHYSICAL_DEVICE_INSTANCE; 1],
}
impl Default for SCM_PHYSICAL_DEVICES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_PHYSICAL_DEVICE_INSTANCE {
    pub Version: u32,
    pub Size: u32,
    pub NfitHandle: u32,
    pub SymbolicLink: [u16; 256],
}
impl Default for SCM_PHYSICAL_DEVICE_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCM_REGION {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub NfitHandle: u32,
    pub LogicalDeviceGuid: windows_core::GUID,
    pub AddressRangeType: windows_core::GUID,
    pub AssociatedId: u32,
    pub Length: u64,
    pub StartingDPA: u64,
    pub BaseSPA: u64,
    pub SPAOffset: u64,
    pub RegionOffset: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCM_REGIONS {
    pub Version: u32,
    pub Size: u32,
    pub RegionCount: u32,
    pub Regions: [SCM_REGION; 1],
}
impl Default for SCM_REGIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_REGION_FLAG(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SD_CHANGE_MACHINE_SID_INPUT {
    pub CurrentMachineSIDOffset: u16,
    pub CurrentMachineSIDLength: u16,
    pub NewMachineSIDOffset: u16,
    pub NewMachineSIDLength: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SD_CHANGE_MACHINE_SID_OUTPUT {
    pub NumSDChangedSuccess: u64,
    pub NumSDChangedFail: u64,
    pub NumSDUnused: u64,
    pub NumSDTotal: u64,
    pub NumMftSDChangedSuccess: u64,
    pub NumMftSDChangedFail: u64,
    pub NumMftSDTotal: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SD_ENUM_SDS_ENTRY {
    pub Hash: u32,
    pub SecurityId: u32,
    pub Offset: u64,
    pub Length: u32,
    pub Descriptor: [u8; 1],
}
impl Default for SD_ENUM_SDS_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SD_ENUM_SDS_INPUT {
    pub StartingOffset: u64,
    pub MaxSDEntriesToReturn: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SD_ENUM_SDS_OUTPUT {
    pub NextOffset: u64,
    pub NumSDEntriesReturned: u64,
    pub NumSDBytesReturned: u64,
    pub SDEntry: [SD_ENUM_SDS_ENTRY; 1],
}
impl Default for SD_ENUM_SDS_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SD_GLOBAL_CHANGE_INPUT {
    pub Flags: u32,
    pub ChangeType: u32,
    pub Anonymous: SD_GLOBAL_CHANGE_INPUT_0,
}
impl Default for SD_GLOBAL_CHANGE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SD_GLOBAL_CHANGE_INPUT_0 {
    pub SdChange: SD_CHANGE_MACHINE_SID_INPUT,
    pub SdQueryStats: SD_QUERY_STATS_INPUT,
    pub SdEnumSds: SD_ENUM_SDS_INPUT,
}
impl Default for SD_GLOBAL_CHANGE_INPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SD_GLOBAL_CHANGE_OUTPUT {
    pub Flags: u32,
    pub ChangeType: u32,
    pub Anonymous: SD_GLOBAL_CHANGE_OUTPUT_0,
}
impl Default for SD_GLOBAL_CHANGE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SD_GLOBAL_CHANGE_OUTPUT_0 {
    pub SdChange: SD_CHANGE_MACHINE_SID_OUTPUT,
    pub SdQueryStats: SD_QUERY_STATS_OUTPUT,
    pub SdEnumSds: SD_ENUM_SDS_OUTPUT,
}
impl Default for SD_GLOBAL_CHANGE_OUTPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SD_GLOBAL_CHANGE_TYPE_ENUM_SDS: u32 = 131072u32;
pub const SD_GLOBAL_CHANGE_TYPE_MACHINE_SID: u32 = 1u32;
pub const SD_GLOBAL_CHANGE_TYPE_QUERY_STATS: u32 = 65536u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SD_QUERY_STATS_INPUT {
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SD_QUERY_STATS_OUTPUT {
    pub SdsStreamSize: u64,
    pub SdsAllocationSize: u64,
    pub SiiStreamSize: u64,
    pub SiiAllocationSize: u64,
    pub SdhStreamSize: u64,
    pub SdhAllocationSize: u64,
    pub NumSDTotal: u64,
    pub NumSDUnused: u64,
}
pub const SEARCH_ALL: u32 = 0u32;
pub const SEARCH_ALL_NO_SEQ: u32 = 4u32;
pub const SEARCH_ALTERNATE: u32 = 2u32;
pub const SEARCH_ALT_NO_SEQ: u32 = 6u32;
pub const SEARCH_PRIMARY: u32 = 1u32;
pub const SEARCH_PRI_NO_SEQ: u32 = 5u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SENDCMDINPARAMS {
    pub cBufferSize: u32,
    pub irDriveRegs: IDEREGS,
    pub bDriveNumber: u8,
    pub bReserved: [u8; 3],
    pub dwReserved: [u32; 4],
    pub bBuffer: [u8; 1],
}
impl Default for SENDCMDINPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SENDCMDOUTPARAMS {
    pub cBufferSize: u32,
    pub DriverStatus: DRIVERSTATUS,
    pub bBuffer: [u8; 1],
}
impl Default for SENDCMDOUTPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERIAL_IOC_FCR_DMA_MODE: u32 = 8u32;
pub const SERIAL_IOC_FCR_FIFO_ENABLE: u32 = 1u32;
pub const SERIAL_IOC_FCR_RCVR_RESET: u32 = 2u32;
pub const SERIAL_IOC_FCR_RCVR_TRIGGER_LSB: u32 = 64u32;
pub const SERIAL_IOC_FCR_RCVR_TRIGGER_MSB: u32 = 128u32;
pub const SERIAL_IOC_FCR_RES1: u32 = 16u32;
pub const SERIAL_IOC_FCR_RES2: u32 = 32u32;
pub const SERIAL_IOC_FCR_XMIT_RESET: u32 = 4u32;
pub const SERIAL_IOC_MCR_DTR: u32 = 1u32;
pub const SERIAL_IOC_MCR_LOOP: u32 = 16u32;
pub const SERIAL_IOC_MCR_OUT1: u32 = 4u32;
pub const SERIAL_IOC_MCR_OUT2: u32 = 8u32;
pub const SERIAL_IOC_MCR_RTS: u32 = 2u32;
pub const SERIAL_NUMBER_LENGTH: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    pub Flags: u32,
    pub AlignmentShift: u32,
    pub FileOffsetToAlign: u64,
    pub FallbackAlignmentShift: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SET_DISK_ATTRIBUTES {
    pub Version: u32,
    pub Persist: bool,
    pub Reserved1: [u8; 3],
    pub Attributes: u64,
    pub AttributesMask: u64,
    pub Reserved2: [u32; 4],
}
impl Default for SET_DISK_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SET_PARTITION_INFORMATION {
    pub PartitionType: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SET_PARTITION_INFORMATION_EX {
    pub PartitionStyle: PARTITION_STYLE,
    pub Anonymous: SET_PARTITION_INFORMATION_EX_0,
}
impl Default for SET_PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SET_PARTITION_INFORMATION_EX_0 {
    pub Mbr: SET_PARTITION_INFORMATION,
    pub Gpt: PARTITION_INFORMATION_GPT,
}
impl Default for SET_PARTITION_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SET_PURGE_FAILURE_MODE_DISABLED: u32 = 2u32;
pub const SET_PURGE_FAILURE_MODE_ENABLED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SET_PURGE_FAILURE_MODE_INPUT {
    pub Flags: u32,
}
pub const SET_REPAIR_DISABLED_AND_BUGCHECK_ON_CORRUPT: u32 = 16u32;
pub const SET_REPAIR_ENABLED: u32 = 1u32;
pub const SET_REPAIR_VALID_MASK: u32 = 25u32;
pub const SET_REPAIR_WARN_ABOUT_DATA_LOSS: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SHRINK_VOLUME_INFORMATION {
    pub ShrinkRequestType: SHRINK_VOLUME_REQUEST_TYPES,
    pub Flags: u64,
    pub NewNumberOfSectors: i64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHRINK_VOLUME_REQUEST_TYPES(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SI_COPYFILE {
    pub SourceFileNameLength: u32,
    pub DestinationFileNameLength: u32,
    pub Flags: u32,
    pub FileNameBuffer: [u16; 1],
}
impl Default for SI_COPYFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SMART_ABORT_OFFLINE_SELFTEST: u32 = 127u32;
pub const SMART_CMD: u32 = 176u32;
pub const SMART_CYL_HI: u32 = 194u32;
pub const SMART_CYL_LOW: u32 = 79u32;
pub const SMART_ERROR_NO_MEM: u32 = 7u32;
pub const SMART_EXTENDED_SELFTEST_CAPTIVE: u32 = 130u32;
pub const SMART_EXTENDED_SELFTEST_OFFLINE: u32 = 2u32;
pub const SMART_GET_VERSION: u32 = 475264u32;
pub const SMART_IDE_ERROR: u32 = 1u32;
pub const SMART_INVALID_BUFFER: u32 = 4u32;
pub const SMART_INVALID_COMMAND: u32 = 3u32;
pub const SMART_INVALID_DRIVE: u32 = 5u32;
pub const SMART_INVALID_FLAG: u32 = 2u32;
pub const SMART_INVALID_IOCTL: u32 = 6u32;
pub const SMART_INVALID_REGISTER: u32 = 8u32;
pub const SMART_LOG_SECTOR_SIZE: u32 = 512u32;
pub const SMART_NOT_SUPPORTED: u32 = 9u32;
pub const SMART_NO_ERROR: u32 = 0u32;
pub const SMART_NO_IDE_DEVICE: u32 = 10u32;
pub const SMART_OFFLINE_ROUTINE_OFFLINE: u32 = 0u32;
pub const SMART_RCV_DRIVE_DATA: u32 = 508040u32;
pub const SMART_RCV_DRIVE_DATA_EX: u32 = 458892u32;
pub const SMART_READ_LOG: u32 = 213u32;
pub const SMART_SEND_DRIVE_COMMAND: u32 = 508036u32;
pub const SMART_SHORT_SELFTEST_CAPTIVE: u32 = 129u32;
pub const SMART_SHORT_SELFTEST_OFFLINE: u32 = 1u32;
pub const SMART_WRITE_LOG: u32 = 214u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    pub Version: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    pub cEntriesPurged: u32,
}
pub const SONY_12_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(66i32);
pub const SONY_D2: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(49i32);
pub const SONY_DTF: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(46i32);
pub const SPACES_TRACKED_OFFSET_HEADER_FLAG: u32 = 2u32;
pub const SRB_TYPE_SCSI_REQUEST_BLOCK: u32 = 0u32;
pub const SRB_TYPE_STORAGE_REQUEST_BLOCK: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STARTING_LCN_INPUT_BUFFER {
    pub StartingLcn: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STARTING_LCN_INPUT_BUFFER_EX {
    pub StartingLcn: i64,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STARTING_VCN_INPUT_BUFFER {
    pub StartingVcn: i64,
}
pub const STK_9840: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(85i32);
pub const STK_9940: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(92i32);
pub const STK_DATA_D3: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(45i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub BytesPerCacheLine: u32,
    pub BytesOffsetForCacheAlignment: u32,
    pub BytesPerLogicalSector: u32,
    pub BytesPerPhysicalSector: u32,
    pub BytesOffsetForSectorAlignment: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_ADAPTER_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub MaximumTransferLength: u32,
    pub MaximumPhysicalPages: u32,
    pub AlignmentMask: u32,
    pub AdapterUsesPio: bool,
    pub AdapterScansDown: bool,
    pub CommandQueueing: bool,
    pub AcceleratedTransfer: bool,
    pub BusType: u8,
    pub BusMajorVersion: u16,
    pub BusMinorVersion: u16,
    pub SrbType: u8,
    pub AddressType: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_ADAPTER_SERIAL_NUMBER {
    pub Version: u32,
    pub Size: u32,
    pub SerialNumber: [u16; 128],
}
impl Default for STORAGE_ADAPTER_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_MAX_LENGTH: u32 = 128u32;
pub const STORAGE_ADDRESS_TYPE_BTL8: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_ALLOCATE_BC_STREAM_INPUT {
    pub Version: u32,
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: bool,
    pub Discardable: bool,
    pub Reserved1: [bool; 2],
    pub AccessType: u32,
    pub AccessMode: u32,
}
impl Default for STORAGE_ALLOCATE_BC_STREAM_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    pub RequestSize: u64,
    pub NumOutStandingRequests: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ASSOCIATION_TYPE(pub i32);
pub const STORAGE_ATTRIBUTE_ASYNC_EVENT_NOTIFICATION: u32 = 16u32;
pub const STORAGE_ATTRIBUTE_BLOCK_IO: u32 = 2u32;
pub const STORAGE_ATTRIBUTE_BYTE_ADDRESSABLE_IO: u32 = 1u32;
pub const STORAGE_ATTRIBUTE_DYNAMIC_PERSISTENCE: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_ATTRIBUTE_MGMT {
    pub Version: u32,
    pub Size: u32,
    pub Action: STORAGE_ATTRIBUTE_MGMT_ACTION,
    pub Attribute: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ATTRIBUTE_MGMT_ACTION(pub i32);
pub const STORAGE_ATTRIBUTE_PERF_SIZE_INDEPENDENT: u32 = 32u32;
pub const STORAGE_ATTRIBUTE_VOLATILE: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_BREAK_RESERVATION_REQUEST {
    pub Length: u32,
    pub _unused: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_BUS_RESET_REQUEST {
    pub PathId: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_COMPONENT_HEALTH_STATUS(pub i32);
pub const STORAGE_COMPONENT_ROLE_CACHE: u32 = 1u32;
pub const STORAGE_COMPONENT_ROLE_DATA: u32 = 4u32;
pub const STORAGE_COMPONENT_ROLE_TIERING: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_COUNTER {
    pub Type: STORAGE_COUNTER_TYPE,
    pub Value: STORAGE_COUNTER_0,
}
impl Default for STORAGE_COUNTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_COUNTER_0 {
    pub ManufactureDate: STORAGE_COUNTER_0_0,
    pub AsUlonglong: u64,
}
impl Default for STORAGE_COUNTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_COUNTER_0_0 {
    pub Week: u32,
    pub Year: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_COUNTERS {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfCounters: u32,
    pub Counters: [STORAGE_COUNTER; 1],
}
impl Default for STORAGE_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_COUNTER_TYPE(pub i32);
pub const STORAGE_CRASH_TELEMETRY_REGKEY: windows_core::PCWSTR = windows_core::w!("\\Registry\\Machine\\System\\CurrentControlSet\\Control\\CrashControl\\StorageTelemetry");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_CRYPTO_ALGORITHM_ID(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_CRYPTO_CAPABILITY {
    pub Version: u32,
    pub Size: u32,
    pub CryptoCapabilityIndex: u32,
    pub AlgorithmId: STORAGE_CRYPTO_ALGORITHM_ID,
    pub KeySize: STORAGE_CRYPTO_KEY_SIZE,
    pub DataUnitSizeBitmask: u32,
}
pub const STORAGE_CRYPTO_CAPABILITY_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_CRYPTO_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NumKeysSupported: u32,
    pub NumCryptoCapabilities: u32,
    pub CryptoCapabilities: [STORAGE_CRYPTO_CAPABILITY; 1],
}
impl Default for STORAGE_CRYPTO_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_CRYPTO_DESCRIPTOR_VERSION_1: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_CRYPTO_KEY_SIZE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DESCRIPTOR_HEADER {
    pub Version: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub Attributes: u64,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_DEVICE_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub DeviceType: u8,
    pub DeviceTypeModifier: u8,
    pub RemovableMedia: bool,
    pub CommandQueueing: bool,
    pub VendorIdOffset: u32,
    pub ProductIdOffset: u32,
    pub ProductRevisionOffset: u32,
    pub SerialNumberOffset: u32,
    pub BusType: super::super::Storage::FileSystem::STORAGE_BUS_TYPE,
    pub RawPropertiesLength: u32,
    pub RawDeviceProperties: [u8; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for STORAGE_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfFaultDomains: u32,
    pub FaultDomainIds: [windows_core::GUID; 1],
}
impl Default for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_DEVICE_FLAGS_PAGE_83_DEVICEGUID: u32 = 4u32;
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_CONFLICT: u32 = 1u32;
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_NOHWID: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_FORM_FACTOR(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_DEVICE_ID_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfIdentifiers: u32,
    pub Identifiers: [u8; 1],
}
impl Default for STORAGE_DEVICE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub LunMaxIoCount: u32,
    pub AdapterMaxIoCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub State: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub Location: DEVICE_LOCATION,
    pub StringOffset: u32,
}
impl Default for STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_DEVICE_MANAGEMENT_STATUS {
    pub Version: u32,
    pub Size: u32,
    pub Health: STORAGE_DISK_HEALTH_STATUS,
    pub NumberOfOperationalStatus: u32,
    pub NumberOfAdditionalReasons: u32,
    pub OperationalStatus: [STORAGE_DISK_OPERATIONAL_STATUS; 16],
    pub AdditionalReasons: [STORAGE_OPERATIONAL_REASON; 1],
}
impl Default for STORAGE_DEVICE_MANAGEMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_DEVICE_MAX_OPERATIONAL_STATUS: u32 = 16u32;
pub const STORAGE_DEVICE_NUMA_NODE_UNKNOWN: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_NUMA_PROPERTY {
    pub Version: u32,
    pub Size: u32,
    pub NumaNode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_NUMBER {
    pub DeviceType: u32,
    pub DeviceNumber: u32,
    pub PartitionNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_DEVICE_NUMBERS {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfDevices: u32,
    pub Devices: [STORAGE_DEVICE_NUMBER; 1],
}
impl Default for STORAGE_DEVICE_NUMBERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_NUMBER_EX {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub DeviceType: u32,
    pub DeviceNumber: u32,
    pub DeviceGuid: windows_core::GUID,
    pub PartitionNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_POWER_CAP {
    pub Version: u32,
    pub Size: u32,
    pub Units: STORAGE_DEVICE_POWER_CAP_UNITS,
    pub MaxPower: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_POWER_CAP_UNITS(pub i32);
pub const STORAGE_DEVICE_POWER_CAP_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NameOffset: u32,
    pub NumberOfLogicalCopies: u32,
    pub NumberOfPhysicalCopies: u32,
    pub PhysicalDiskRedundancy: u32,
    pub NumberOfColumns: u32,
    pub Interleave: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    pub Version: u32,
    pub Size: u32,
    pub SupportsSelfEncryption: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY_V2 {
    pub Version: u32,
    pub Size: u32,
    pub SupportsSelfEncryption: bool,
    pub EncryptionType: STORAGE_ENCRYPTION_TYPE,
}
pub const STORAGE_DEVICE_TELEMETRY_REGKEY: windows_core::PCWSTR = windows_core::w!("\\Registry\\Machine\\System\\CurrentControlSet\\Control\\Storage\\StorageTelemetry");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_DEVICE_TIERING_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub TotalNumberOfTiers: u32,
    pub NumberOfTiersReturned: u32,
    pub Tiers: [STORAGE_TIER; 1],
}
impl Default for STORAGE_DEVICE_TIERING_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    pub Version: u32,
    pub Size: u32,
    pub UnsafeShutdownCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_DIAGNOSTIC_DATA {
    pub Version: u32,
    pub Size: u32,
    pub ProviderId: windows_core::GUID,
    pub BufferSize: u32,
    pub Reserved: u32,
    pub DiagnosticDataBuffer: [u8; 1],
}
impl Default for STORAGE_DIAGNOSTIC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_DIAGNOSTIC_FLAG_ADAPTER_REQUEST: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DIAGNOSTIC_LEVEL(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_DIAGNOSTIC_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub TargetType: STORAGE_DIAGNOSTIC_TARGET_TYPE,
    pub Level: STORAGE_DIAGNOSTIC_LEVEL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DIAGNOSTIC_TARGET_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DISK_HEALTH_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DISK_OPERATIONAL_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ENCRYPTION_TYPE(pub i32);
pub const STORAGE_EVENT_DEVICE_OPERATION: u64 = 4u64;
pub const STORAGE_EVENT_DEVICE_STATUS: u64 = 2u64;
pub const STORAGE_EVENT_MEDIA_STATUS: u64 = 1u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_EVENT_NOTIFICATION {
    pub Version: u32,
    pub Size: u32,
    pub Events: u64,
}
pub const STORAGE_EVENT_NOTIFICATION_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_FAILURE_PREDICTION_CONFIG {
    pub Version: u32,
    pub Size: u32,
    pub Set: bool,
    pub Enabled: bool,
    pub Reserved: u16,
}
pub const STORAGE_FAILURE_PREDICTION_CONFIG_V1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_FRU_ID_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub IdentifierSize: u32,
    pub Identifier: [u8; 1],
}
impl Default for STORAGE_FRU_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_GET_BC_PROPERTIES_OUTPUT {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MaximumRequestSize: u64,
    pub EstimatedTimePerRequest: u32,
    pub NumOutStandingRequests: u32,
    pub RequestSize: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_HOTPLUG_INFO {
    pub Size: u32,
    pub MediaRemovable: bool,
    pub MediaHotplug: bool,
    pub DeviceHotplug: bool,
    pub WriteCacheEnableOverride: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub EnduranceInfo: STORAGE_HW_ENDURANCE_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_HW_ENDURANCE_INFO {
    pub ValidFields: u32,
    pub GroupId: u32,
    pub Flags: STORAGE_HW_ENDURANCE_INFO_0,
    pub LifePercentage: u32,
    pub BytesReadCount: [u8; 16],
    pub ByteWriteCount: [u8; 16],
}
impl Default for STORAGE_HW_ENDURANCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_HW_ENDURANCE_INFO_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_ACTIVATE {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Slot: u8,
    pub Reserved0: [u8; 3],
}
impl Default for STORAGE_HW_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_DOWNLOAD {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub Offset: u64,
    pub BufferSize: u64,
    pub ImageBuffer: [u8; 1],
}
impl Default for STORAGE_HW_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub Offset: u64,
    pub BufferSize: u64,
    pub ImageSize: u32,
    pub Reserved2: u32,
    pub ImageBuffer: [u8; 1],
}
impl Default for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub _bitfield: u8,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub FirmwareShared: bool,
    pub Reserved: [u8; 3],
    pub ImagePayloadAlignment: u32,
    pub ImagePayloadMaxSize: u32,
    pub Slot: [STORAGE_HW_FIRMWARE_SLOT_INFO; 1],
}
impl Default for STORAGE_HW_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_INFO_QUERY {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Reserved: u32,
}
pub const STORAGE_HW_FIRMWARE_INVALID_SLOT: u32 = 255u32;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_CONTROLLER: u32 = 1u32;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: u32 = 4u32;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: u32 = 2u32;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_REPLACE_EXISTING_IMAGE: u32 = 1073741824u32;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648u32;
pub const STORAGE_HW_FIRMWARE_REVISION_LENGTH: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_SLOT_INFO {
    pub Version: u32,
    pub Size: u32,
    pub SlotNumber: u8,
    pub _bitfield: u8,
    pub Reserved1: [u8; 6],
    pub Revision: [u8; 16],
}
impl Default for STORAGE_HW_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_IDENTIFIER {
    pub CodeSet: STORAGE_IDENTIFIER_CODE_SET,
    pub Type: STORAGE_IDENTIFIER_TYPE,
    pub IdentifierSize: u16,
    pub NextOffset: u16,
    pub Association: STORAGE_ASSOCIATION_TYPE,
    pub Identifier: [u8; 1],
}
impl Default for STORAGE_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_IDENTIFIER_CODE_SET(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_IDENTIFIER_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_IDLE_POWER {
    pub Version: u32,
    pub Size: u32,
    pub _bitfield: u32,
    pub D3IdleTimeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_IDLE_POWERUP_REASON {
    pub Version: u32,
    pub Size: u32,
    pub PowerupReason: STORAGE_POWERUP_REASON_TYPE,
}
pub const STORAGE_IDLE_POWERUP_REASON_VERSION_V1: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ID_NAA_FORMAT(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    pub Size: u32,
    pub Version: u32,
    pub _bitfield1: u8,
    pub Reserved1: [u8; 3],
    pub _bitfield2: u8,
    pub Reserved3: [u8; 3],
    pub AvailableMappingResources: u64,
    pub UsedMappingResources: u64,
}
impl Default for STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    pub Reserved: u16,
    pub SerialNumberLength: u16,
    pub SerialNumber: [u8; 1],
}
impl Default for STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_MEDIA_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub MediumProductType: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_MINIPORT_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub Portdriver: STORAGE_PORT_CODE_SET,
    pub LUNResetSupported: bool,
    pub TargetResetSupported: bool,
    pub IoTimeoutValue: u16,
    pub ExtraIoInfoSupported: bool,
    pub Flags: STORAGE_MINIPORT_DESCRIPTOR_0,
    pub Reserved0: [u8; 2],
    pub Reserved1: u32,
}
impl Default for STORAGE_MINIPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_MINIPORT_DESCRIPTOR_0 {
    pub Anonymous: STORAGE_MINIPORT_DESCRIPTOR_0_0,
    pub AsBYTE: u8,
}
impl Default for STORAGE_MINIPORT_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_MINIPORT_DESCRIPTOR_0_0 {
    pub _bitfield: u8,
}
pub const STORAGE_OFFLOAD_MAX_TOKEN_LENGTH: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_OFFLOAD_READ_OUTPUT {
    pub OffloadReadFlags: u32,
    pub Reserved: u32,
    pub LengthProtected: u64,
    pub TokenLength: u32,
    pub Token: STORAGE_OFFLOAD_TOKEN,
}
impl Default for STORAGE_OFFLOAD_READ_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_OFFLOAD_READ_RANGE_TRUNCATED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_OFFLOAD_TOKEN {
    pub TokenType: [u8; 4],
    pub Reserved: [u8; 2],
    pub TokenIdLength: [u8; 2],
    pub Anonymous: STORAGE_OFFLOAD_TOKEN_0,
}
impl Default for STORAGE_OFFLOAD_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_OFFLOAD_TOKEN_0 {
    pub StorageOffloadZeroDataToken: STORAGE_OFFLOAD_TOKEN_0_0,
    pub Token: [u8; 504],
}
impl Default for STORAGE_OFFLOAD_TOKEN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_OFFLOAD_TOKEN_0_0 {
    pub Reserved2: [u8; 504],
}
impl Default for STORAGE_OFFLOAD_TOKEN_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_OFFLOAD_TOKEN_ID_LENGTH: u32 = 504u32;
pub const STORAGE_OFFLOAD_TOKEN_INVALID: u32 = 2u32;
pub const STORAGE_OFFLOAD_TOKEN_TYPE_ZERO_DATA: u32 = 4294901761u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_OFFLOAD_WRITE_OUTPUT {
    pub OffloadWriteFlags: u32,
    pub Reserved: u32,
    pub LengthCopied: u64,
}
pub const STORAGE_OFFLOAD_WRITE_RANGE_TRUNCATED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_OPERATIONAL_REASON {
    pub Version: u32,
    pub Size: u32,
    pub Reason: STORAGE_OPERATIONAL_STATUS_REASON,
    pub RawBytes: STORAGE_OPERATIONAL_REASON_0,
}
impl Default for STORAGE_OPERATIONAL_REASON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_OPERATIONAL_REASON_0 {
    pub ScsiSenseKey: STORAGE_OPERATIONAL_REASON_0_0,
    pub NVDIMM_N: STORAGE_OPERATIONAL_REASON_0_1,
    pub AsUlong: u32,
}
impl Default for STORAGE_OPERATIONAL_REASON_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_OPERATIONAL_REASON_0_1 {
    pub CriticalHealth: u8,
    pub ModuleHealth: [u8; 2],
    pub ErrorThresholdStatus: u8,
}
impl Default for STORAGE_OPERATIONAL_REASON_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_OPERATIONAL_REASON_0_0 {
    pub SenseKey: u8,
    pub ASC: u8,
    pub ASCQ: u8,
    pub Reserved: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_OPERATIONAL_STATUS_REASON(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_PHYSICAL_ADAPTER_DATA {
    pub AdapterId: u32,
    pub HealthStatus: STORAGE_COMPONENT_HEALTH_STATUS,
    pub CommandProtocol: STORAGE_PROTOCOL_TYPE,
    pub SpecVersion: STORAGE_SPEC_VERSION,
    pub Vendor: [u8; 8],
    pub Model: [u8; 40],
    pub FirmwareRevision: [u8; 16],
    pub PhysicalLocation: [u8; 32],
    pub ExpanderConnected: bool,
    pub Reserved0: [u8; 3],
    pub Reserved1: [u32; 3],
}
impl Default for STORAGE_PHYSICAL_ADAPTER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_PHYSICAL_DEVICE_DATA {
    pub DeviceId: u32,
    pub Role: u32,
    pub HealthStatus: STORAGE_COMPONENT_HEALTH_STATUS,
    pub CommandProtocol: STORAGE_PROTOCOL_TYPE,
    pub SpecVersion: STORAGE_SPEC_VERSION,
    pub FormFactor: STORAGE_DEVICE_FORM_FACTOR,
    pub Vendor: [u8; 8],
    pub Model: [u8; 40],
    pub FirmwareRevision: [u8; 16],
    pub Capacity: u64,
    pub PhysicalLocation: [u8; 32],
    pub Reserved: [u32; 2],
}
impl Default for STORAGE_PHYSICAL_DEVICE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_PHYSICAL_NODE_DATA {
    pub NodeId: u32,
    pub AdapterCount: u32,
    pub AdapterDataLength: u32,
    pub AdapterDataOffset: u32,
    pub DeviceCount: u32,
    pub DeviceDataLength: u32,
    pub DeviceDataOffset: u32,
    pub Reserved: [u32; 3],
}
impl Default for STORAGE_PHYSICAL_NODE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NodeCount: u32,
    pub Reserved: u32,
    pub Node: [STORAGE_PHYSICAL_NODE_DATA; 1],
}
impl Default for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PORT_CODE_SET(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_POWERUP_REASON_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_PREDICT_FAILURE {
    pub PredictFailure: u32,
    pub VendorSpecific: [u8; 512],
}
impl Default for STORAGE_PREDICT_FAILURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_PRIORITY_HINT_SUPPORT {
    pub SupportFlags: u32,
}
pub const STORAGE_PRIORITY_HINT_SUPPORTED: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PROPERTY_ID(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_PROPERTY_QUERY {
    pub PropertyId: STORAGE_PROPERTY_ID,
    pub QueryType: STORAGE_QUERY_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl Default for STORAGE_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_PROPERTY_SET {
    pub PropertyId: STORAGE_PROPERTY_ID,
    pub SetType: STORAGE_SET_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl Default for STORAGE_PROPERTY_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PROTOCOL_ATA_DATA_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_PROTOCOL_COMMAND {
    pub Version: u32,
    pub Length: u32,
    pub ProtocolType: STORAGE_PROTOCOL_TYPE,
    pub Flags: u32,
    pub ReturnStatus: u32,
    pub ErrorCode: u32,
    pub CommandLength: u32,
    pub ErrorInfoLength: u32,
    pub DataToDeviceTransferLength: u32,
    pub DataFromDeviceTransferLength: u32,
    pub TimeOutValue: u32,
    pub ErrorInfoOffset: u32,
    pub DataToDeviceBufferOffset: u32,
    pub DataFromDeviceBufferOffset: u32,
    pub CommandSpecific: u32,
    pub Reserved0: u32,
    pub FixedProtocolReturnData: u32,
    pub Reserved1: [u32; 3],
    pub Command: [u8; 1],
}
impl Default for STORAGE_PROTOCOL_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST: u32 = 2147483648u32;
pub const STORAGE_PROTOCOL_COMMAND_LENGTH_NVME: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub ProtocolSpecificData: STORAGE_PROTOCOL_SPECIFIC_DATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    pub Version: u32,
    pub Size: u32,
    pub ProtocolSpecificData: STORAGE_PROTOCOL_SPECIFIC_DATA_EXT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {
    pub Anonymous: STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0,
    pub AsUlong: u32,
}
impl Default for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PROTOCOL_NVME_DATA_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_PROTOCOL_SPECIFIC_DATA {
    pub ProtocolType: STORAGE_PROTOCOL_TYPE,
    pub DataType: u32,
    pub ProtocolDataRequestValue: u32,
    pub ProtocolDataRequestSubValue: u32,
    pub ProtocolDataOffset: u32,
    pub ProtocolDataLength: u32,
    pub FixedProtocolReturnData: u32,
    pub ProtocolDataRequestSubValue2: u32,
    pub ProtocolDataRequestSubValue3: u32,
    pub ProtocolDataRequestSubValue4: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    pub ProtocolType: STORAGE_PROTOCOL_TYPE,
    pub DataType: u32,
    pub ProtocolDataValue: u32,
    pub ProtocolDataSubValue: u32,
    pub ProtocolDataOffset: u32,
    pub ProtocolDataLength: u32,
    pub FixedProtocolReturnData: u32,
    pub ProtocolDataSubValue2: u32,
    pub ProtocolDataSubValue3: u32,
    pub ProtocolDataSubValue4: u32,
    pub ProtocolDataSubValue5: u32,
    pub Reserved: [u32; 5],
}
impl Default for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND: u32 = 1u32;
pub const STORAGE_PROTOCOL_SPECIFIC_NVME_NVM_COMMAND: u32 = 2u32;
pub const STORAGE_PROTOCOL_STATUS_BUSY: u32 = 5u32;
pub const STORAGE_PROTOCOL_STATUS_DATA_OVERRUN: u32 = 6u32;
pub const STORAGE_PROTOCOL_STATUS_ERROR: u32 = 2u32;
pub const STORAGE_PROTOCOL_STATUS_INSUFFICIENT_RESOURCES: u32 = 7u32;
pub const STORAGE_PROTOCOL_STATUS_INVALID_REQUEST: u32 = 3u32;
pub const STORAGE_PROTOCOL_STATUS_NOT_SUPPORTED: u32 = 255u32;
pub const STORAGE_PROTOCOL_STATUS_NO_DEVICE: u32 = 4u32;
pub const STORAGE_PROTOCOL_STATUS_PENDING: u32 = 0u32;
pub const STORAGE_PROTOCOL_STATUS_SUCCESS: u32 = 1u32;
pub const STORAGE_PROTOCOL_STATUS_THROTTLED_REQUEST: u32 = 8u32;
pub const STORAGE_PROTOCOL_STRUCTURE_VERSION: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PROTOCOL_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PROTOCOL_UFS_DATA_TYPE(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Storage_Vhd")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    pub EntryLength: u32,
    pub DependencyTypeFlags: u32,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: super::super::Storage::Vhd::VIRTUAL_STORAGE_TYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_Vhd")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    pub EntryLength: u32,
    pub DependencyTypeFlags: u32,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: super::super::Storage::Vhd::VIRTUAL_STORAGE_TYPE,
    pub AncestorLevel: u32,
    pub HostVolumeNameOffset: u32,
    pub HostVolumeNameSize: u32,
    pub DependentVolumeNameOffset: u32,
    pub DependentVolumeNameSize: u32,
    pub RelativePathOffset: u32,
    pub RelativePathSize: u32,
    pub DependentDeviceNameOffset: u32,
    pub DependentDeviceNameSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    pub RequestLevel: u32,
    pub RequestFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_Vhd")]
#[derive(Clone, Copy)]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    pub ResponseLevel: u32,
    pub NumberEntries: u32,
    pub Anonymous: STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0,
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl Default for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_Vhd")]
#[derive(Clone, Copy)]
pub union STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {
    pub Lev1Depends: [STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY; 1],
    pub Lev2Depends: [STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY; 1],
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl Default for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_QUERY_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_READ_CAPACITY {
    pub Version: u32,
    pub Size: u32,
    pub BlockLength: u32,
    pub NumberOfBlocks: i64,
    pub DiskLength: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_REINITIALIZE_MEDIA {
    pub Version: u32,
    pub Size: u32,
    pub TimeoutInSeconds: u32,
    pub SanitizeOption: STORAGE_REINITIALIZE_MEDIA_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_REINITIALIZE_MEDIA_0 {
    pub _bitfield: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_RESERVE_ID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_RPMB_COMMAND_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_RPMB_DATA_FRAME {
    pub Stuff: [u8; 196],
    pub KeyOrMAC: [u8; 32],
    pub Data: [u8; 256],
    pub Nonce: [u8; 16],
    pub WriteCounter: [u8; 4],
    pub Address: [u8; 2],
    pub BlockCount: [u8; 2],
    pub OperationResult: [u8; 2],
    pub RequestOrResponseType: [u8; 2],
}
impl Default for STORAGE_RPMB_DATA_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_RPMB_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub SizeInBytes: u32,
    pub MaxReliableWriteSizeInBytes: u32,
    pub FrameFormat: STORAGE_RPMB_FRAME_TYPE,
}
pub const STORAGE_RPMB_DESCRIPTOR_VERSION_1: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_RPMB_FRAME_TYPE(pub i32);
pub const STORAGE_RPMB_MINIMUM_RELIABLE_WRITE_SIZE: u32 = 512u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_SANITIZE_METHOD(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_SET_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_SPEC_VERSION {
    pub Anonymous: STORAGE_SPEC_VERSION_0,
    pub AsUlong: u32,
}
impl Default for STORAGE_SPEC_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_SPEC_VERSION_0 {
    pub MinorVersion: STORAGE_SPEC_VERSION_0_0,
    pub MajorVersion: u16,
}
impl Default for STORAGE_SPEC_VERSION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_SPEC_VERSION_0_0 {
    pub Anonymous: STORAGE_SPEC_VERSION_0_0_0,
    pub AsUshort: u16,
}
impl Default for STORAGE_SPEC_VERSION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_SPEC_VERSION_0_0_0 {
    pub SubMinor: u8,
    pub Minor: u8,
}
pub const STORAGE_SUPPORTED_FEATURES_BYPASS_IO: u32 = 1u32;
pub const STORAGE_SUPPORTED_FEATURES_MASK: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub CriticalTemperature: i16,
    pub WarningTemperature: i16,
    pub InfoCount: u16,
    pub Reserved0: [u8; 2],
    pub Reserved1: [u32; 2],
    pub TemperatureInfo: [STORAGE_TEMPERATURE_INFO; 1],
}
impl Default for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_TEMPERATURE_INFO {
    pub Index: u16,
    pub Temperature: i16,
    pub OverThreshold: i16,
    pub UnderThreshold: i16,
    pub OverThresholdChangable: bool,
    pub UnderThresholdChangable: bool,
    pub EventGenerated: bool,
    pub Reserved0: u8,
    pub Reserved1: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_TEMPERATURE_THRESHOLD {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u16,
    pub Index: u16,
    pub Threshold: i16,
    pub OverThreshold: bool,
    pub Reserved: u8,
}
pub const STORAGE_TEMPERATURE_THRESHOLD_FLAG_ADAPTER_REQUEST: u32 = 1u32;
pub const STORAGE_TEMPERATURE_VALUE_NOT_REPORTED: u32 = 32768u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_TIER {
    pub Id: windows_core::GUID,
    pub Name: [u16; 256],
    pub Description: [u16; 256],
    pub Flags: u64,
    pub ProvisionedCapacity: u64,
    pub MediaType: STORAGE_TIER_MEDIA_TYPE,
    pub Class: STORAGE_TIER_CLASS,
}
impl Default for STORAGE_TIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_TIER_CLASS(pub i32);
pub const STORAGE_TIER_DESCRIPTION_LENGTH: u32 = 512u32;
pub const STORAGE_TIER_FLAG_NO_SEEK_PENALTY: u32 = 131072u32;
pub const STORAGE_TIER_FLAG_PARITY: u32 = 8388608u32;
pub const STORAGE_TIER_FLAG_READ_CACHE: u32 = 4194304u32;
pub const STORAGE_TIER_FLAG_SMR: u32 = 16777216u32;
pub const STORAGE_TIER_FLAG_WRITE_BACK_CACHE: u32 = 2097152u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_TIER_MEDIA_TYPE(pub i32);
pub const STORAGE_TIER_NAME_LENGTH: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_TIER_REGION {
    pub TierId: windows_core::GUID,
    pub Offset: u64,
    pub Length: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_WRITE_CACHE_PROPERTY {
    pub Version: u32,
    pub Size: u32,
    pub WriteCacheType: WRITE_CACHE_TYPE,
    pub WriteCacheEnabled: WRITE_CACHE_ENABLE,
    pub WriteCacheChangeable: WRITE_CACHE_CHANGE,
    pub WriteThroughSupported: WRITE_THROUGH,
    pub FlushCacheSupported: bool,
    pub UserDefinedPowerProtection: bool,
    pub NVCacheEnabled: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_ZONED_DEVICE_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub DeviceType: STORAGE_ZONED_DEVICE_TYPES,
    pub ZoneCount: u32,
    pub ZoneAttributes: STORAGE_ZONED_DEVICE_DESCRIPTOR_0,
    pub ZoneGroupCount: u32,
    pub ZoneGroup: [STORAGE_ZONE_GROUP; 1],
}
impl Default for STORAGE_ZONED_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {
    pub SequentialRequiredZone: STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0,
    pub SequentialPreferredZone: STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1,
}
impl Default for STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1 {
    pub OptimalOpenZoneCount: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {
    pub MaxOpenZoneCount: u32,
    pub UnrestrictedRead: bool,
    pub Reserved: [u8; 3],
}
impl Default for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ZONED_DEVICE_TYPES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ZONES_ATTRIBUTES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ZONE_CONDITION(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STORAGE_ZONE_DESCRIPTOR {
    pub Size: u32,
    pub ZoneType: STORAGE_ZONE_TYPES,
    pub ZoneCondition: STORAGE_ZONE_CONDITION,
    pub ResetWritePointerRecommend: bool,
    pub Reserved0: [u8; 3],
    pub ZoneSize: u64,
    pub WritePointerOffset: u64,
}
impl Default for STORAGE_ZONE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STORAGE_ZONE_GROUP {
    pub ZoneCount: u32,
    pub ZoneType: STORAGE_ZONE_TYPES,
    pub ZoneSize: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ZONE_TYPES(pub i32);
pub const STORATTRIBUTE_MANAGEMENT_STATE: u32 = 1u32;
pub const STORATTRIBUTE_NONE: u32 = 0u32;
pub const STREAMS_ASSOCIATE_ID_CLEAR: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    pub Flags: u32,
    pub StreamId: u32,
}
pub const STREAMS_ASSOCIATE_ID_SET: u32 = 2u32;
pub const STREAMS_INVALID_ID: u32 = 0u32;
pub const STREAMS_MAX_ID: u32 = 65535u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STREAMS_QUERY_ID_OUTPUT_BUFFER {
    pub StreamId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    pub OptimalWriteSize: u32,
    pub StreamGranularitySize: u32,
    pub StreamIdMin: u32,
    pub StreamIdMax: u32,
}
pub const STREAM_CLEAR_ENCRYPTION: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STREAM_EXTENT_ENTRY {
    pub Flags: u32,
    pub ExtentInformation: STREAM_EXTENT_ENTRY_0,
}
impl Default for STREAM_EXTENT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STREAM_EXTENT_ENTRY_0 {
    pub RetrievalPointers: RETRIEVAL_POINTERS_BUFFER,
}
impl Default for STREAM_EXTENT_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STREAM_EXTENT_ENTRY_ALL_EXTENTS: u32 = 2u32;
pub const STREAM_EXTENT_ENTRY_AS_RETRIEVAL_POINTERS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STREAM_INFORMATION_ENTRY {
    pub Version: u32,
    pub Flags: u32,
    pub StreamInformation: STREAM_INFORMATION_ENTRY_0,
}
impl Default for STREAM_INFORMATION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STREAM_INFORMATION_ENTRY_0 {
    pub DesiredStorageClass: STREAM_INFORMATION_ENTRY_0_0,
    pub DataStream: STREAM_INFORMATION_ENTRY_0_1,
    pub Reparse: STREAM_INFORMATION_ENTRY_0_2,
    pub Ea: STREAM_INFORMATION_ENTRY_0_3,
}
impl Default for STREAM_INFORMATION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STREAM_INFORMATION_ENTRY_0_1 {
    pub Length: u16,
    pub Flags: u16,
    pub Reserved: u32,
    pub Vdl: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STREAM_INFORMATION_ENTRY_0_0 {
    pub Class: FILE_STORAGE_TIER_CLASS,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STREAM_INFORMATION_ENTRY_0_3 {
    pub Length: u16,
    pub Flags: u16,
    pub EaSize: u32,
    pub EaInformationOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STREAM_INFORMATION_ENTRY_0_2 {
    pub Length: u16,
    pub Flags: u16,
    pub ReparseDataSize: u32,
    pub ReparseDataOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STREAM_LAYOUT_ENTRY {
    pub Version: u32,
    pub NextStreamOffset: u32,
    pub Flags: u32,
    pub ExtentInformationOffset: u32,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub StreamInformationOffset: u32,
    pub AttributeTypeCode: u32,
    pub AttributeFlags: u32,
    pub StreamIdentifierLength: u32,
    pub StreamIdentifier: [u16; 1],
}
impl Default for STREAM_LAYOUT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STREAM_LAYOUT_ENTRY_HAS_INFORMATION: u32 = 16u32;
pub const STREAM_LAYOUT_ENTRY_IMMOVABLE: u32 = 1u32;
pub const STREAM_LAYOUT_ENTRY_NO_CLUSTERS_ALLOCATED: u32 = 8u32;
pub const STREAM_LAYOUT_ENTRY_PINNED: u32 = 2u32;
pub const STREAM_LAYOUT_ENTRY_RESIDENT: u32 = 4u32;
pub const STREAM_SET_ENCRYPTION: u32 = 3u32;
pub const SYQUEST_EZ135: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(75i32);
pub const SYQUEST_EZFLYER: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(76i32);
pub const SYQUEST_SYJET: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(77i32);
pub const ScmBusFirmwareActivationState_Armed: SCM_BUS_FIRMWARE_ACTIVATION_STATE = SCM_BUS_FIRMWARE_ACTIVATION_STATE(1i32);
pub const ScmBusFirmwareActivationState_Busy: SCM_BUS_FIRMWARE_ACTIVATION_STATE = SCM_BUS_FIRMWARE_ACTIVATION_STATE(2i32);
pub const ScmBusFirmwareActivationState_Idle: SCM_BUS_FIRMWARE_ACTIVATION_STATE = SCM_BUS_FIRMWARE_ACTIVATION_STATE(0i32);
pub const ScmBusProperty_DedicatedMemoryInfo: SCM_BUS_PROPERTY_ID = SCM_BUS_PROPERTY_ID(1i32);
pub const ScmBusProperty_DedicatedMemoryState: SCM_BUS_PROPERTY_ID = SCM_BUS_PROPERTY_ID(2i32);
pub const ScmBusProperty_Max: SCM_BUS_PROPERTY_ID = SCM_BUS_PROPERTY_ID(3i32);
pub const ScmBusProperty_RuntimeFwActivationInfo: SCM_BUS_PROPERTY_ID = SCM_BUS_PROPERTY_ID(0i32);
pub const ScmBusQuery_Descriptor: SCM_BUS_QUERY_TYPE = SCM_BUS_QUERY_TYPE(0i32);
pub const ScmBusQuery_IsSupported: SCM_BUS_QUERY_TYPE = SCM_BUS_QUERY_TYPE(1i32);
pub const ScmBusQuery_Max: SCM_BUS_QUERY_TYPE = SCM_BUS_QUERY_TYPE(2i32);
pub const ScmBusSet_Descriptor: SCM_BUS_SET_TYPE = SCM_BUS_SET_TYPE(0i32);
pub const ScmBusSet_IsSupported: SCM_BUS_SET_TYPE = SCM_BUS_SET_TYPE(1i32);
pub const ScmBusSet_Max: SCM_BUS_SET_TYPE = SCM_BUS_SET_TYPE(2i32);
pub const ScmPdFirmwareActivationState_Armed: SCM_PD_FIRMWARE_ACTIVATION_STATE = SCM_PD_FIRMWARE_ACTIVATION_STATE(1i32);
pub const ScmPdFirmwareActivationState_Busy: SCM_PD_FIRMWARE_ACTIVATION_STATE = SCM_PD_FIRMWARE_ACTIVATION_STATE(2i32);
pub const ScmPdFirmwareActivationState_Idle: SCM_PD_FIRMWARE_ACTIVATION_STATE = SCM_PD_FIRMWARE_ACTIVATION_STATE(0i32);
pub const ScmPdLastFwActivaitonStatus_ActivationInProgress: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(4i32);
pub const ScmPdLastFwActivaitonStatus_FwUnsupported: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(6i32);
pub const ScmPdLastFwActivaitonStatus_Retry: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(5i32);
pub const ScmPdLastFwActivaitonStatus_UnknownError: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(7i32);
pub const ScmPdLastFwActivationStatus_ColdRebootRequired: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(3i32);
pub const ScmPdLastFwActivationStatus_FwNotFound: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(2i32);
pub const ScmPdLastFwActivationStatus_None: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(0i32);
pub const ScmPdLastFwActivationStatus_Success: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(1i32);
pub const ScmPhysicalDeviceHealth_Healthy: SCM_PD_HEALTH_STATUS = SCM_PD_HEALTH_STATUS(3i32);
pub const ScmPhysicalDeviceHealth_Max: SCM_PD_HEALTH_STATUS = SCM_PD_HEALTH_STATUS(4i32);
pub const ScmPhysicalDeviceHealth_Unhealthy: SCM_PD_HEALTH_STATUS = SCM_PD_HEALTH_STATUS(1i32);
pub const ScmPhysicalDeviceHealth_Unknown: SCM_PD_HEALTH_STATUS = SCM_PD_HEALTH_STATUS(0i32);
pub const ScmPhysicalDeviceHealth_Warning: SCM_PD_HEALTH_STATUS = SCM_PD_HEALTH_STATUS(2i32);
pub const ScmPhysicalDeviceOpReason_BackgroundOperation: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(9i32);
pub const ScmPhysicalDeviceOpReason_Component: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(8i32);
pub const ScmPhysicalDeviceOpReason_Configuration: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(5i32);
pub const ScmPhysicalDeviceOpReason_DataPersistenceLossImminent: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(17i32);
pub const ScmPhysicalDeviceOpReason_DeviceController: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(6i32);
pub const ScmPhysicalDeviceOpReason_DisabledByPlatform: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(13i32);
pub const ScmPhysicalDeviceOpReason_EnergySource: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(4i32);
pub const ScmPhysicalDeviceOpReason_ExcessiveTemperature: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(21i32);
pub const ScmPhysicalDeviceOpReason_FatalError: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(16i32);
pub const ScmPhysicalDeviceOpReason_HealthCheck: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(11i32);
pub const ScmPhysicalDeviceOpReason_InternalFailure: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(22i32);
pub const ScmPhysicalDeviceOpReason_InvalidFirmware: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(10i32);
pub const ScmPhysicalDeviceOpReason_LostData: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(3i32);
pub const ScmPhysicalDeviceOpReason_LostDataPersistence: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(12i32);
pub const ScmPhysicalDeviceOpReason_LostWritePersistence: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(15i32);
pub const ScmPhysicalDeviceOpReason_Max: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(23i32);
pub const ScmPhysicalDeviceOpReason_Media: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(1i32);
pub const ScmPhysicalDeviceOpReason_MediaController: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(7i32);
pub const ScmPhysicalDeviceOpReason_MediaRemainingSpareBlock: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(19i32);
pub const ScmPhysicalDeviceOpReason_PerformanceDegradation: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(20i32);
pub const ScmPhysicalDeviceOpReason_PermanentError: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(14i32);
pub const ScmPhysicalDeviceOpReason_ThresholdExceeded: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(2i32);
pub const ScmPhysicalDeviceOpReason_Unknown: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(0i32);
pub const ScmPhysicalDeviceOpReason_WritePersistenceLossImminent: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(18i32);
pub const ScmPhysicalDeviceOpStatus_HardwareError: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(4i32);
pub const ScmPhysicalDeviceOpStatus_InService: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(3i32);
pub const ScmPhysicalDeviceOpStatus_Max: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(8i32);
pub const ScmPhysicalDeviceOpStatus_Missing: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(7i32);
pub const ScmPhysicalDeviceOpStatus_NotUsable: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(5i32);
pub const ScmPhysicalDeviceOpStatus_Ok: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(1i32);
pub const ScmPhysicalDeviceOpStatus_PredictingFailure: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(2i32);
pub const ScmPhysicalDeviceOpStatus_TransientError: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(6i32);
pub const ScmPhysicalDeviceOpStatus_Unknown: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(0i32);
pub const ScmPhysicalDeviceProperty_DeviceHandle: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(5i32);
pub const ScmPhysicalDeviceProperty_DeviceInfo: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(0i32);
pub const ScmPhysicalDeviceProperty_DeviceSpecificInfo: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(4i32);
pub const ScmPhysicalDeviceProperty_FirmwareInfo: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(2i32);
pub const ScmPhysicalDeviceProperty_FruIdString: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(6i32);
pub const ScmPhysicalDeviceProperty_LocationString: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(3i32);
pub const ScmPhysicalDeviceProperty_ManagementStatus: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(1i32);
pub const ScmPhysicalDeviceProperty_Max: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(9i32);
pub const ScmPhysicalDeviceProperty_RuntimeFwActivationArmState: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(8i32);
pub const ScmPhysicalDeviceProperty_RuntimeFwActivationInfo: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(7i32);
pub const ScmPhysicalDeviceQuery_Descriptor: SCM_PD_QUERY_TYPE = SCM_PD_QUERY_TYPE(0i32);
pub const ScmPhysicalDeviceQuery_IsSupported: SCM_PD_QUERY_TYPE = SCM_PD_QUERY_TYPE(1i32);
pub const ScmPhysicalDeviceQuery_Max: SCM_PD_QUERY_TYPE = SCM_PD_QUERY_TYPE(2i32);
pub const ScmPhysicalDeviceReinit_ColdBootNeeded: SCM_PD_MEDIA_REINITIALIZATION_STATUS = SCM_PD_MEDIA_REINITIALIZATION_STATUS(2i32);
pub const ScmPhysicalDeviceReinit_Max: SCM_PD_MEDIA_REINITIALIZATION_STATUS = SCM_PD_MEDIA_REINITIALIZATION_STATUS(3i32);
pub const ScmPhysicalDeviceReinit_RebootNeeded: SCM_PD_MEDIA_REINITIALIZATION_STATUS = SCM_PD_MEDIA_REINITIALIZATION_STATUS(1i32);
pub const ScmPhysicalDeviceReinit_Success: SCM_PD_MEDIA_REINITIALIZATION_STATUS = SCM_PD_MEDIA_REINITIALIZATION_STATUS(0i32);
pub const ScmPhysicalDeviceSet_Descriptor: SCM_PD_SET_TYPE = SCM_PD_SET_TYPE(0i32);
pub const ScmPhysicalDeviceSet_IsSupported: SCM_PD_SET_TYPE = SCM_PD_SET_TYPE(1i32);
pub const ScmPhysicalDeviceSet_Max: SCM_PD_SET_TYPE = SCM_PD_SET_TYPE(2i32);
pub const ScmRegionFlagLabel: SCM_REGION_FLAG = SCM_REGION_FLAG(1i32);
pub const ScmRegionFlagNone: SCM_REGION_FLAG = SCM_REGION_FLAG(0i32);
pub const ShrinkAbort: SHRINK_VOLUME_REQUEST_TYPES = SHRINK_VOLUME_REQUEST_TYPES(3i32);
pub const ShrinkCommit: SHRINK_VOLUME_REQUEST_TYPES = SHRINK_VOLUME_REQUEST_TYPES(2i32);
pub const ShrinkPrepare: SHRINK_VOLUME_REQUEST_TYPES = SHRINK_VOLUME_REQUEST_TYPES(1i32);
pub const SmrGcActionPause: REFS_SMR_VOLUME_GC_ACTION = REFS_SMR_VOLUME_GC_ACTION(3i32);
pub const SmrGcActionStart: REFS_SMR_VOLUME_GC_ACTION = REFS_SMR_VOLUME_GC_ACTION(1i32);
pub const SmrGcActionStartFullSpeed: REFS_SMR_VOLUME_GC_ACTION = REFS_SMR_VOLUME_GC_ACTION(2i32);
pub const SmrGcActionStop: REFS_SMR_VOLUME_GC_ACTION = REFS_SMR_VOLUME_GC_ACTION(4i32);
pub const SmrGcMethodCompaction: REFS_SMR_VOLUME_GC_METHOD = REFS_SMR_VOLUME_GC_METHOD(1i32);
pub const SmrGcMethodCompression: REFS_SMR_VOLUME_GC_METHOD = REFS_SMR_VOLUME_GC_METHOD(2i32);
pub const SmrGcMethodRotation: REFS_SMR_VOLUME_GC_METHOD = REFS_SMR_VOLUME_GC_METHOD(3i32);
pub const SmrGcStateActive: REFS_SMR_VOLUME_GC_STATE = REFS_SMR_VOLUME_GC_STATE(2i32);
pub const SmrGcStateActiveFullSpeed: REFS_SMR_VOLUME_GC_STATE = REFS_SMR_VOLUME_GC_STATE(3i32);
pub const SmrGcStateInactive: REFS_SMR_VOLUME_GC_STATE = REFS_SMR_VOLUME_GC_STATE(0i32);
pub const SmrGcStatePaused: REFS_SMR_VOLUME_GC_STATE = REFS_SMR_VOLUME_GC_STATE(1i32);
pub const StorAttributeMgmt_ClearAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = STORAGE_ATTRIBUTE_MGMT_ACTION(0i32);
pub const StorAttributeMgmt_ResetAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = STORAGE_ATTRIBUTE_MGMT_ACTION(2i32);
pub const StorAttributeMgmt_SetAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = STORAGE_ATTRIBUTE_MGMT_ACTION(1i32);
pub const StorRpmbAuthenticatedDeviceConfigRead: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(7i32);
pub const StorRpmbAuthenticatedDeviceConfigWrite: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(6i32);
pub const StorRpmbAuthenticatedRead: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(4i32);
pub const StorRpmbAuthenticatedWrite: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(3i32);
pub const StorRpmbProgramAuthKey: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(1i32);
pub const StorRpmbQueryWriteCounter: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(2i32);
pub const StorRpmbReadResultRequest: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(5i32);
pub const StorageAccessAlignmentProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(6i32);
pub const StorageAdapterCryptoProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(17i32);
pub const StorageAdapterPhysicalTopologyProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(53i32);
pub const StorageAdapterProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(1i32);
pub const StorageAdapterProtocolSpecificProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(49i32);
pub const StorageAdapterRpmbProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(16i32);
pub const StorageAdapterSerialNumberProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(57i32);
pub const StorageAdapterTemperatureProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(51i32);
pub const StorageCounterTypeFlushLatency100NSMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(20i32);
pub const StorageCounterTypeLoadUnloadCycleCount: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(12i32);
pub const StorageCounterTypeLoadUnloadCycleCountMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(13i32);
pub const StorageCounterTypeManufactureDate: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(9i32);
pub const StorageCounterTypeMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(21i32);
pub const StorageCounterTypePowerOnHours: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(17i32);
pub const StorageCounterTypeReadErrorsCorrected: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(4i32);
pub const StorageCounterTypeReadErrorsTotal: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(3i32);
pub const StorageCounterTypeReadErrorsUncorrected: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(5i32);
pub const StorageCounterTypeReadLatency100NSMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(18i32);
pub const StorageCounterTypeStartStopCycleCount: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(10i32);
pub const StorageCounterTypeStartStopCycleCountMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(11i32);
pub const StorageCounterTypeTemperatureCelsius: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(1i32);
pub const StorageCounterTypeTemperatureCelsiusMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(2i32);
pub const StorageCounterTypeUnknown: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(0i32);
pub const StorageCounterTypeWearPercentage: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(14i32);
pub const StorageCounterTypeWearPercentageMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(16i32);
pub const StorageCounterTypeWearPercentageWarning: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(15i32);
pub const StorageCounterTypeWriteErrorsCorrected: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(7i32);
pub const StorageCounterTypeWriteErrorsTotal: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(6i32);
pub const StorageCounterTypeWriteErrorsUncorrected: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(8i32);
pub const StorageCounterTypeWriteLatency100NSMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(19i32);
pub const StorageCryptoAlgorithmAESECB: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(3i32);
pub const StorageCryptoAlgorithmBitlockerAESCBC: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(2i32);
pub const StorageCryptoAlgorithmESSIVAESCBC: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(4i32);
pub const StorageCryptoAlgorithmMax: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(5i32);
pub const StorageCryptoAlgorithmUnknown: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(0i32);
pub const StorageCryptoAlgorithmXTSAES: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(1i32);
pub const StorageCryptoKeySize128Bits: STORAGE_CRYPTO_KEY_SIZE = STORAGE_CRYPTO_KEY_SIZE(1i32);
pub const StorageCryptoKeySize192Bits: STORAGE_CRYPTO_KEY_SIZE = STORAGE_CRYPTO_KEY_SIZE(2i32);
pub const StorageCryptoKeySize256Bits: STORAGE_CRYPTO_KEY_SIZE = STORAGE_CRYPTO_KEY_SIZE(3i32);
pub const StorageCryptoKeySize512Bits: STORAGE_CRYPTO_KEY_SIZE = STORAGE_CRYPTO_KEY_SIZE(4i32);
pub const StorageCryptoKeySizeUnknown: STORAGE_CRYPTO_KEY_SIZE = STORAGE_CRYPTO_KEY_SIZE(0i32);
pub const StorageDeviceAttributesProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(55i32);
pub const StorageDeviceCopyOffloadProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(13i32);
pub const StorageDeviceDeviceTelemetryProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(10i32);
pub const StorageDeviceEnduranceProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(62i32);
pub const StorageDeviceIdProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(2i32);
pub const StorageDeviceIoCapabilityProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(48i32);
pub const StorageDeviceLBProvisioningProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(11i32);
pub const StorageDeviceLedStateProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(63i32);
pub const StorageDeviceLocationProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(58i32);
pub const StorageDeviceManagementStatus: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(56i32);
pub const StorageDeviceMediumProductType: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(15i32);
pub const StorageDeviceNumaProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(59i32);
pub const StorageDevicePhysicalTopologyProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(54i32);
pub const StorageDevicePowerCapUnitsMilliwatts: STORAGE_DEVICE_POWER_CAP_UNITS = STORAGE_DEVICE_POWER_CAP_UNITS(1i32);
pub const StorageDevicePowerCapUnitsPercent: STORAGE_DEVICE_POWER_CAP_UNITS = STORAGE_DEVICE_POWER_CAP_UNITS(0i32);
pub const StorageDevicePowerProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(12i32);
pub const StorageDeviceProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(0i32);
pub const StorageDeviceProtocolSpecificProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(50i32);
pub const StorageDeviceResiliencyProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(14i32);
pub const StorageDeviceSeekPenaltyProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(7i32);
pub const StorageDeviceSelfEncryptionProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(64i32);
pub const StorageDeviceTemperatureProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(52i32);
pub const StorageDeviceTrimProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(8i32);
pub const StorageDeviceUniqueIdProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(3i32);
pub const StorageDeviceUnsafeShutdownCount: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(61i32);
pub const StorageDeviceWriteAggregationProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(9i32);
pub const StorageDeviceWriteCacheProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(4i32);
pub const StorageDeviceZonedDeviceProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(60i32);
pub const StorageDiagnosticLevelDefault: STORAGE_DIAGNOSTIC_LEVEL = STORAGE_DIAGNOSTIC_LEVEL(0i32);
pub const StorageDiagnosticLevelMax: STORAGE_DIAGNOSTIC_LEVEL = STORAGE_DIAGNOSTIC_LEVEL(1i32);
pub const StorageDiagnosticTargetTypeHbaFirmware: STORAGE_DIAGNOSTIC_TARGET_TYPE = STORAGE_DIAGNOSTIC_TARGET_TYPE(3i32);
pub const StorageDiagnosticTargetTypeMax: STORAGE_DIAGNOSTIC_TARGET_TYPE = STORAGE_DIAGNOSTIC_TARGET_TYPE(4i32);
pub const StorageDiagnosticTargetTypeMiniport: STORAGE_DIAGNOSTIC_TARGET_TYPE = STORAGE_DIAGNOSTIC_TARGET_TYPE(2i32);
pub const StorageDiagnosticTargetTypePort: STORAGE_DIAGNOSTIC_TARGET_TYPE = STORAGE_DIAGNOSTIC_TARGET_TYPE(1i32);
pub const StorageDiagnosticTargetTypeUndefined: STORAGE_DIAGNOSTIC_TARGET_TYPE = STORAGE_DIAGNOSTIC_TARGET_TYPE(0i32);
pub const StorageEncryptionTypeEDrive: STORAGE_ENCRYPTION_TYPE = STORAGE_ENCRYPTION_TYPE(1i32);
pub const StorageEncryptionTypeTcgOpal: STORAGE_ENCRYPTION_TYPE = STORAGE_ENCRYPTION_TYPE(2i32);
pub const StorageEncryptionTypeUnknown: STORAGE_ENCRYPTION_TYPE = STORAGE_ENCRYPTION_TYPE(0i32);
pub const StorageFruIdProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(65i32);
pub const StorageIdAssocDevice: STORAGE_ASSOCIATION_TYPE = STORAGE_ASSOCIATION_TYPE(0i32);
pub const StorageIdAssocPort: STORAGE_ASSOCIATION_TYPE = STORAGE_ASSOCIATION_TYPE(1i32);
pub const StorageIdAssocTarget: STORAGE_ASSOCIATION_TYPE = STORAGE_ASSOCIATION_TYPE(2i32);
pub const StorageIdCodeSetAscii: STORAGE_IDENTIFIER_CODE_SET = STORAGE_IDENTIFIER_CODE_SET(2i32);
pub const StorageIdCodeSetBinary: STORAGE_IDENTIFIER_CODE_SET = STORAGE_IDENTIFIER_CODE_SET(1i32);
pub const StorageIdCodeSetReserved: STORAGE_IDENTIFIER_CODE_SET = STORAGE_IDENTIFIER_CODE_SET(0i32);
pub const StorageIdCodeSetUtf8: STORAGE_IDENTIFIER_CODE_SET = STORAGE_IDENTIFIER_CODE_SET(3i32);
pub const StorageIdNAAFormatIEEEERegisteredExtended: STORAGE_ID_NAA_FORMAT = STORAGE_ID_NAA_FORMAT(5i32);
pub const StorageIdNAAFormatIEEEExtended: STORAGE_ID_NAA_FORMAT = STORAGE_ID_NAA_FORMAT(2i32);
pub const StorageIdNAAFormatIEEERegistered: STORAGE_ID_NAA_FORMAT = STORAGE_ID_NAA_FORMAT(3i32);
pub const StorageIdTypeEUI64: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(2i32);
pub const StorageIdTypeFCPHName: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(3i32);
pub const StorageIdTypeLogicalUnitGroup: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(6i32);
pub const StorageIdTypeMD5LogicalUnitIdentifier: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(7i32);
pub const StorageIdTypePortRelative: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(4i32);
pub const StorageIdTypeScsiNameString: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(8i32);
pub const StorageIdTypeTargetPortGroup: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(5i32);
pub const StorageIdTypeVendorId: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(1i32);
pub const StorageIdTypeVendorSpecific: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(0i32);
pub const StorageMiniportProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(5i32);
pub const StoragePortCodeSetATAport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(4i32);
pub const StoragePortCodeSetReserved: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(0i32);
pub const StoragePortCodeSetSBP2port: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(6i32);
pub const StoragePortCodeSetSCSIport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(2i32);
pub const StoragePortCodeSetSDport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(7i32);
pub const StoragePortCodeSetSpaceport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(3i32);
pub const StoragePortCodeSetStorport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(1i32);
pub const StoragePortCodeSetUSBport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(5i32);
pub const StoragePowerupDeviceAttention: STORAGE_POWERUP_REASON_TYPE = STORAGE_POWERUP_REASON_TYPE(2i32);
pub const StoragePowerupIO: STORAGE_POWERUP_REASON_TYPE = STORAGE_POWERUP_REASON_TYPE(1i32);
pub const StoragePowerupUnknown: STORAGE_POWERUP_REASON_TYPE = STORAGE_POWERUP_REASON_TYPE(0i32);
pub const StorageReserveIdHard: STORAGE_RESERVE_ID = STORAGE_RESERVE_ID(1i32);
pub const StorageReserveIdMax: STORAGE_RESERVE_ID = STORAGE_RESERVE_ID(4i32);
pub const StorageReserveIdNone: STORAGE_RESERVE_ID = STORAGE_RESERVE_ID(0i32);
pub const StorageReserveIdSoft: STORAGE_RESERVE_ID = STORAGE_RESERVE_ID(2i32);
pub const StorageReserveIdUpdateScratch: STORAGE_RESERVE_ID = STORAGE_RESERVE_ID(3i32);
pub const StorageRpmbFrameTypeMax: STORAGE_RPMB_FRAME_TYPE = STORAGE_RPMB_FRAME_TYPE(2i32);
pub const StorageRpmbFrameTypeStandard: STORAGE_RPMB_FRAME_TYPE = STORAGE_RPMB_FRAME_TYPE(1i32);
pub const StorageRpmbFrameTypeUnknown: STORAGE_RPMB_FRAME_TYPE = STORAGE_RPMB_FRAME_TYPE(0i32);
pub const StorageSanitizeMethodBlockErase: STORAGE_SANITIZE_METHOD = STORAGE_SANITIZE_METHOD(1i32);
pub const StorageSanitizeMethodCryptoErase: STORAGE_SANITIZE_METHOD = STORAGE_SANITIZE_METHOD(2i32);
pub const StorageSanitizeMethodDefault: STORAGE_SANITIZE_METHOD = STORAGE_SANITIZE_METHOD(0i32);
pub const StorageTierClassCapacity: STORAGE_TIER_CLASS = STORAGE_TIER_CLASS(1i32);
pub const StorageTierClassMax: STORAGE_TIER_CLASS = STORAGE_TIER_CLASS(3i32);
pub const StorageTierClassPerformance: STORAGE_TIER_CLASS = STORAGE_TIER_CLASS(2i32);
pub const StorageTierClassUnspecified: STORAGE_TIER_CLASS = STORAGE_TIER_CLASS(0i32);
pub const StorageTierMediaTypeDisk: STORAGE_TIER_MEDIA_TYPE = STORAGE_TIER_MEDIA_TYPE(1i32);
pub const StorageTierMediaTypeMax: STORAGE_TIER_MEDIA_TYPE = STORAGE_TIER_MEDIA_TYPE(5i32);
pub const StorageTierMediaTypeScm: STORAGE_TIER_MEDIA_TYPE = STORAGE_TIER_MEDIA_TYPE(4i32);
pub const StorageTierMediaTypeSsd: STORAGE_TIER_MEDIA_TYPE = STORAGE_TIER_MEDIA_TYPE(2i32);
pub const StorageTierMediaTypeUnspecified: STORAGE_TIER_MEDIA_TYPE = STORAGE_TIER_MEDIA_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_GET_STATISTICS {
    pub Operation: u32,
}
pub const TAPE_RESET_STATISTICS: i32 = 2i32;
pub const TAPE_RETURN_ENV_INFO: i32 = 1i32;
pub const TAPE_RETURN_STATISTICS: i32 = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_STATISTICS {
    pub Version: u32,
    pub Flags: u32,
    pub RecoveredWrites: i64,
    pub UnrecoveredWrites: i64,
    pub RecoveredReads: i64,
    pub UnrecoveredReads: i64,
    pub CompressionRatioReads: u8,
    pub CompressionRatioWrites: u8,
}
pub const TCCollectionApplicationRequested: DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE = DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE(2i32);
pub const TCCollectionBugCheck: DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE = DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE(1i32);
pub const TCCollectionDeviceRequested: DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE = DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE(3i32);
pub const TC_DEVICEDUMP_SUBSECTION_DESC_LENGTH: u32 = 16u32;
pub const TC_PUBLIC_DATA_TYPE_ATAGP: windows_core::PCSTR = windows_core::s!("ATAGPLogPages");
pub const TC_PUBLIC_DATA_TYPE_ATASMART: windows_core::PCSTR = windows_core::s!("ATASMARTPages");
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG: u32 = 2u32;
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG_MAX: u32 = 16u32;
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_SMART: u32 = 1u32;
pub const TELEMETRY_COMMAND_SIZE: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_CREATE_MINIVERSION_INFO {
    pub StructureVersion: u16,
    pub StructureLength: u16,
    pub BaseVersion: u32,
    pub MiniVersion: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_GET_METADATA_INFO_OUT {
    pub TxfFileId: TXFS_GET_METADATA_INFO_OUT_0,
    pub LockingTransaction: windows_core::GUID,
    pub LastLsn: u64,
    pub TransactionState: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_GET_METADATA_INFO_OUT_0 {
    pub LowPart: i64,
    pub HighPart: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_GET_TRANSACTED_VERSION {
    pub ThisBaseVersion: u32,
    pub LatestVersion: u32,
    pub ThisMiniVersion: u16,
    pub FirstMiniVersion: u16,
    pub LatestMiniVersion: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_LIST_TRANSACTIONS {
    pub NumberOfTransactions: u64,
    pub BufferSizeRequired: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_LIST_TRANSACTIONS_ENTRY {
    pub TransactionId: windows_core::GUID,
    pub TransactionState: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES {
    pub KtmTransaction: windows_core::GUID,
    pub NumberOfFiles: u64,
    pub BufferSizeRequired: u64,
    pub Offset: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    pub Offset: u64,
    pub NameFlags: u32,
    pub FileId: i64,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: i64,
    pub FileName: [u16; 1],
}
impl Default for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_CREATED: u32 = 1u32;
pub const TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_DELETED: u32 = 2u32;
pub const TXFS_LOGGING_MODE_FULL: u32 = 2u32;
pub const TXFS_LOGGING_MODE_SIMPLE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_MODIFY_RM {
    pub Flags: TXFS_RMF_LAGS,
    pub LogContainerCountMax: u32,
    pub LogContainerCountMin: u32,
    pub LogContainerCount: u32,
    pub LogGrowthIncrement: u32,
    pub LogAutoShrinkPercentage: u32,
    pub Reserved: u64,
    pub LoggingMode: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_QUERY_RM_INFORMATION {
    pub BytesRequired: u32,
    pub TailLsn: u64,
    pub CurrentLsn: u64,
    pub ArchiveTailLsn: u64,
    pub LogContainerSize: u64,
    pub HighestVirtualClock: i64,
    pub LogContainerCount: u32,
    pub LogContainerCountMax: u32,
    pub LogContainerCountMin: u32,
    pub LogGrowthIncrement: u32,
    pub LogAutoShrinkPercentage: u32,
    pub Flags: TXFS_RMF_LAGS,
    pub LoggingMode: u16,
    pub Reserved: u16,
    pub RmState: u32,
    pub LogCapacity: u64,
    pub LogFree: u64,
    pub TopsSize: u64,
    pub TopsUsed: u64,
    pub TransactionCount: u64,
    pub OnePCCount: u64,
    pub TwoPCCount: u64,
    pub NumberLogFileFull: u64,
    pub OldestTransactionAge: u64,
    pub RMName: windows_core::GUID,
    pub TmLogPathOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TXFS_READ_BACKUP_INFORMATION_OUT {
    pub Anonymous: TXFS_READ_BACKUP_INFORMATION_OUT_0,
}
impl Default for TXFS_READ_BACKUP_INFORMATION_OUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    pub BufferLength: u32,
    pub Buffer: [u8; 1],
}
impl Default for TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_RMF_LAGS(pub u32);
impl TXFS_RMF_LAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TXFS_RMF_LAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TXFS_RMF_LAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TXFS_RMF_LAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TXFS_RMF_LAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TXFS_RMF_LAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TXFS_RM_FLAG_DO_NOT_RESET_RM_AT_NEXT_START: TXFS_RMF_LAGS = TXFS_RMF_LAGS(32768u32);
pub const TXFS_RM_FLAG_ENFORCE_MINIMUM_SIZE: TXFS_RMF_LAGS = TXFS_RMF_LAGS(4096u32);
pub const TXFS_RM_FLAG_GROW_LOG: TXFS_RMF_LAGS = TXFS_RMF_LAGS(1024u32);
pub const TXFS_RM_FLAG_LOGGING_MODE: TXFS_RMF_LAGS = TXFS_RMF_LAGS(1u32);
pub const TXFS_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE: TXFS_RMF_LAGS = TXFS_RMF_LAGS(64u32);
pub const TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MAX: TXFS_RMF_LAGS = TXFS_RMF_LAGS(4u32);
pub const TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MIN: TXFS_RMF_LAGS = TXFS_RMF_LAGS(8u32);
pub const TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS: TXFS_RMF_LAGS = TXFS_RMF_LAGS(16u32);
pub const TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT: TXFS_RMF_LAGS = TXFS_RMF_LAGS(32u32);
pub const TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX: TXFS_RMF_LAGS = TXFS_RMF_LAGS(128u32);
pub const TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN: TXFS_RMF_LAGS = TXFS_RMF_LAGS(256u32);
pub const TXFS_RM_FLAG_PREFER_AVAILABILITY: TXFS_RMF_LAGS = TXFS_RMF_LAGS(131072u32);
pub const TXFS_RM_FLAG_PREFER_CONSISTENCY: TXFS_RMF_LAGS = TXFS_RMF_LAGS(65536u32);
pub const TXFS_RM_FLAG_PRESERVE_CHANGES: TXFS_RMF_LAGS = TXFS_RMF_LAGS(8192u32);
pub const TXFS_RM_FLAG_RENAME_RM: TXFS_RMF_LAGS = TXFS_RMF_LAGS(2u32);
pub const TXFS_RM_FLAG_RESET_RM_AT_NEXT_START: TXFS_RMF_LAGS = TXFS_RMF_LAGS(16384u32);
pub const TXFS_RM_FLAG_SHRINK_LOG: TXFS_RMF_LAGS = TXFS_RMF_LAGS(2048u32);
pub const TXFS_RM_STATE_ACTIVE: u32 = 2u32;
pub const TXFS_RM_STATE_NOT_STARTED: u32 = 0u32;
pub const TXFS_RM_STATE_SHUTTING_DOWN: u32 = 3u32;
pub const TXFS_RM_STATE_STARTING: u32 = 1u32;
pub const TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_REDO_LSN: u32 = 1u32;
pub const TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_VIRTUAL_CLOCK: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_ROLLFORWARD_REDO_INFORMATION {
    pub LastVirtualClock: i64,
    pub LastRedoLsn: u64,
    pub HighestRecoveryLsn: u64,
    pub Flags: u32,
}
pub const TXFS_SAVEPOINT_CLEAR: u32 = 4u32;
pub const TXFS_SAVEPOINT_CLEAR_ALL: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_SAVEPOINT_INFORMATION {
    pub KtmTransaction: super::super::Foundation::HANDLE,
    pub ActionCode: u32,
    pub SavepointId: u32,
}
pub const TXFS_SAVEPOINT_ROLLBACK: u32 = 2u32;
pub const TXFS_SAVEPOINT_SET: u32 = 1u32;
pub const TXFS_START_RM_FLAG_LOGGING_MODE: u32 = 1024u32;
pub const TXFS_START_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE: u32 = 32u32;
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MAX: u32 = 1u32;
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MIN: u32 = 2u32;
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_SIZE: u32 = 4u32;
pub const TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS: u32 = 8u32;
pub const TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT: u32 = 16u32;
pub const TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX: u32 = 64u32;
pub const TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN: u32 = 128u32;
pub const TXFS_START_RM_FLAG_PREFER_AVAILABILITY: u32 = 8192u32;
pub const TXFS_START_RM_FLAG_PREFER_CONSISTENCY: u32 = 4096u32;
pub const TXFS_START_RM_FLAG_PRESERVE_CHANGES: u32 = 2048u32;
pub const TXFS_START_RM_FLAG_RECOVER_BEST_EFFORT: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TXFS_START_RM_INFORMATION {
    pub Flags: u32,
    pub LogContainerSize: u64,
    pub LogContainerCountMin: u32,
    pub LogContainerCountMax: u32,
    pub LogGrowthIncrement: u32,
    pub LogAutoShrinkPercentage: u32,
    pub TmLogPathOffset: u32,
    pub TmLogPathLength: u16,
    pub LoggingMode: u16,
    pub LogPathLength: u16,
    pub Reserved: u16,
    pub LogPath: [u16; 1],
}
impl Default for TXFS_START_RM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TXFS_TRANSACTED_VERSION_NONTRANSACTED: u32 = 4294967294u32;
pub const TXFS_TRANSACTED_VERSION_UNCOMMITTED: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TXFS_TRANSACTION_ACTIVE_INFO {
    pub TransactionsActiveAtSnapshot: bool,
}
pub const TXFS_TRANSACTION_STATE_ACTIVE: u32 = 1u32;
pub const TXFS_TRANSACTION_STATE_NONE: u32 = 0u32;
pub const TXFS_TRANSACTION_STATE_NOTACTIVE: u32 = 3u32;
pub const TXFS_TRANSACTION_STATE_PREPARED: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TXFS_WRITE_BACKUP_INFORMATION {
    pub Buffer: [u8; 1],
}
impl Default for TXFS_WRITE_BACKUP_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Travan: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(34i32);
pub const UNDEFINE_ALTERNATE: u32 = 13u32;
pub const UNDEFINE_PRIMARY: u32 = 12u32;
pub const UNLOCK_ELEMENT: u32 = 1u32;
pub const UNRECOVERED_READS_VALID: u32 = 8u32;
pub const UNRECOVERED_WRITES_VALID: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USN_DELETE_FLAGS(pub u32);
impl USN_DELETE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for USN_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for USN_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for USN_DELETE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for USN_DELETE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for USN_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const USN_DELETE_FLAG_DELETE: USN_DELETE_FLAGS = USN_DELETE_FLAGS(1u32);
pub const USN_DELETE_FLAG_NOTIFY: USN_DELETE_FLAGS = USN_DELETE_FLAGS(2u32);
pub const USN_DELETE_VALID_FLAGS: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USN_JOURNAL_DATA_V0 {
    pub UsnJournalID: u64,
    pub FirstUsn: i64,
    pub NextUsn: i64,
    pub LowestValidUsn: i64,
    pub MaxUsn: i64,
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USN_JOURNAL_DATA_V1 {
    pub UsnJournalID: u64,
    pub FirstUsn: i64,
    pub NextUsn: i64,
    pub LowestValidUsn: i64,
    pub MaxUsn: i64,
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
    pub MinSupportedMajorVersion: u16,
    pub MaxSupportedMajorVersion: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USN_JOURNAL_DATA_V2 {
    pub UsnJournalID: u64,
    pub FirstUsn: i64,
    pub NextUsn: i64,
    pub LowestValidUsn: i64,
    pub MaxUsn: i64,
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
    pub MinSupportedMajorVersion: u16,
    pub MaxSupportedMajorVersion: u16,
    pub Flags: u32,
    pub RangeTrackChunkSize: u64,
    pub RangeTrackFileSizeThreshold: i64,
}
pub const USN_PAGE_SIZE: u32 = 4096u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USN_RANGE_TRACK_OUTPUT {
    pub Usn: i64,
}
pub const USN_REASON_BASIC_INFO_CHANGE: u32 = 32768u32;
pub const USN_REASON_CLOSE: u32 = 2147483648u32;
pub const USN_REASON_COMPRESSION_CHANGE: u32 = 131072u32;
pub const USN_REASON_DATA_EXTEND: u32 = 2u32;
pub const USN_REASON_DATA_OVERWRITE: u32 = 1u32;
pub const USN_REASON_DATA_TRUNCATION: u32 = 4u32;
pub const USN_REASON_DESIRED_STORAGE_CLASS_CHANGE: u32 = 16777216u32;
pub const USN_REASON_EA_CHANGE: u32 = 1024u32;
pub const USN_REASON_ENCRYPTION_CHANGE: u32 = 262144u32;
pub const USN_REASON_FILE_CREATE: u32 = 256u32;
pub const USN_REASON_FILE_DELETE: u32 = 512u32;
pub const USN_REASON_HARD_LINK_CHANGE: u32 = 65536u32;
pub const USN_REASON_INDEXABLE_CHANGE: u32 = 16384u32;
pub const USN_REASON_INTEGRITY_CHANGE: u32 = 8388608u32;
pub const USN_REASON_NAMED_DATA_EXTEND: u32 = 32u32;
pub const USN_REASON_NAMED_DATA_OVERWRITE: u32 = 16u32;
pub const USN_REASON_NAMED_DATA_TRUNCATION: u32 = 64u32;
pub const USN_REASON_OBJECT_ID_CHANGE: u32 = 524288u32;
pub const USN_REASON_RENAME_NEW_NAME: u32 = 8192u32;
pub const USN_REASON_RENAME_OLD_NAME: u32 = 4096u32;
pub const USN_REASON_REPARSE_POINT_CHANGE: u32 = 1048576u32;
pub const USN_REASON_SECURITY_CHANGE: u32 = 2048u32;
pub const USN_REASON_STREAM_CHANGE: u32 = 2097152u32;
pub const USN_REASON_TRANSACTED_CHANGE: u32 = 4194304u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USN_RECORD_COMMON_HEADER {
    pub RecordLength: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USN_RECORD_EXTENT {
    pub Offset: i64,
    pub Length: i64,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy)]
pub union USN_RECORD_UNION {
    pub Header: USN_RECORD_COMMON_HEADER,
    pub V2: USN_RECORD_V2,
    pub V3: USN_RECORD_V3,
    pub V4: USN_RECORD_V4,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for USN_RECORD_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USN_RECORD_V2 {
    pub RecordLength: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub FileReferenceNumber: u64,
    pub ParentFileReferenceNumber: u64,
    pub Usn: i64,
    pub TimeStamp: i64,
    pub Reason: u32,
    pub SourceInfo: u32,
    pub SecurityId: u32,
    pub FileAttributes: u32,
    pub FileNameLength: u16,
    pub FileNameOffset: u16,
    pub FileName: [u16; 1],
}
impl Default for USN_RECORD_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USN_RECORD_V3 {
    pub RecordLength: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub FileReferenceNumber: super::super::Storage::FileSystem::FILE_ID_128,
    pub ParentFileReferenceNumber: super::super::Storage::FileSystem::FILE_ID_128,
    pub Usn: i64,
    pub TimeStamp: i64,
    pub Reason: u32,
    pub SourceInfo: u32,
    pub SecurityId: u32,
    pub FileAttributes: u32,
    pub FileNameLength: u16,
    pub FileNameOffset: u16,
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for USN_RECORD_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USN_RECORD_V4 {
    pub Header: USN_RECORD_COMMON_HEADER,
    pub FileReferenceNumber: super::super::Storage::FileSystem::FILE_ID_128,
    pub ParentFileReferenceNumber: super::super::Storage::FileSystem::FILE_ID_128,
    pub Usn: i64,
    pub Reason: u32,
    pub SourceInfo: USN_SOURCE_INFO_ID,
    pub RemainingExtents: u32,
    pub NumberOfExtents: u16,
    pub ExtentSize: u16,
    pub Extents: [USN_RECORD_EXTENT; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for USN_RECORD_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USN_SOURCE_AUXILIARY_DATA: USN_SOURCE_INFO_ID = USN_SOURCE_INFO_ID(2u32);
pub const USN_SOURCE_CLIENT_REPLICATION_MANAGEMENT: USN_SOURCE_INFO_ID = USN_SOURCE_INFO_ID(8u32);
pub const USN_SOURCE_DATA_MANAGEMENT: USN_SOURCE_INFO_ID = USN_SOURCE_INFO_ID(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USN_SOURCE_INFO_ID(pub u32);
pub const USN_SOURCE_REPLICATION_MANAGEMENT: USN_SOURCE_INFO_ID = USN_SOURCE_INFO_ID(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USN_TRACK_MODIFIED_RANGES {
    pub Flags: u32,
    pub Unused: u32,
    pub ChunkSize: u64,
    pub FileSizeThreshold: i64,
}
pub const UfsDataTypeMax: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(6i32);
pub const UfsDataTypeQueryAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(2i32);
pub const UfsDataTypeQueryDescriptor: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(1i32);
pub const UfsDataTypeQueryDmeAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(4i32);
pub const UfsDataTypeQueryDmePeerAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(5i32);
pub const UfsDataTypeQueryFlag: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(3i32);
pub const UfsDataTypeUnknown: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(0i32);
pub const Unknown: MEDIA_TYPE = MEDIA_TYPE(0i32);
pub const VALID_NTFT: u32 = 192u32;
pub const VENDOR_ID_LENGTH: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VERIFY_INFORMATION {
    pub StartingOffset: i64,
    pub Length: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUALIZATION_INSTANCE_INFO_INPUT {
    pub NumberOfWorkerThreads: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    pub HeaderSize: u16,
    pub Flags: u32,
    pub NotificationInfoSize: u32,
    pub NotificationInfoOffset: u16,
    pub ProviderMajorVersion: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    pub VirtualizationInstanceID: windows_core::GUID,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_STORAGE_BEHAVIOR_CODE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    pub Size: u32,
    pub BehaviorCode: VIRTUAL_STORAGE_BEHAVIOR_CODE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VOLUME_BITMAP_BUFFER {
    pub StartingLcn: i64,
    pub BitmapSize: i64,
    pub Buffer: [u8; 1],
}
impl Default for VOLUME_BITMAP_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VOLUME_DISK_EXTENTS {
    pub NumberOfDiskExtents: u32,
    pub Extents: [DISK_EXTENT; 1],
}
impl Default for VOLUME_DISK_EXTENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    pub GptAttributes: u64,
}
pub const VOLUME_IS_DIRTY: u32 = 1u32;
pub const VOLUME_SESSION_OPEN: u32 = 4u32;
pub const VOLUME_UPGRADE_SCHEDULED: u32 = 2u32;
pub const VXATape: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(94i32);
pub const VXATape_1: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(83i32);
pub const VXATape_2: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(84i32);
pub const VirtualStorageBehaviorCacheWriteBack: VIRTUAL_STORAGE_BEHAVIOR_CODE = VIRTUAL_STORAGE_BEHAVIOR_CODE(2i32);
pub const VirtualStorageBehaviorCacheWriteThrough: VIRTUAL_STORAGE_BEHAVIOR_CODE = VIRTUAL_STORAGE_BEHAVIOR_CODE(1i32);
pub const VirtualStorageBehaviorRestartIoProcessing: VIRTUAL_STORAGE_BEHAVIOR_CODE = VIRTUAL_STORAGE_BEHAVIOR_CODE(4i32);
pub const VirtualStorageBehaviorStopIoProcessing: VIRTUAL_STORAGE_BEHAVIOR_CODE = VIRTUAL_STORAGE_BEHAVIOR_CODE(3i32);
pub const VirtualStorageBehaviorUndefined: VIRTUAL_STORAGE_BEHAVIOR_CODE = VIRTUAL_STORAGE_BEHAVIOR_CODE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIM_PROVIDER_ADD_OVERLAY_INPUT {
    pub WimType: u32,
    pub WimIndex: u32,
    pub WimFileNameOffset: u32,
    pub WimFileNameLength: u32,
}
pub const WIM_PROVIDER_CURRENT_VERSION: u32 = 1u32;
pub const WIM_PROVIDER_EXTERNAL_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_PROVIDER_EXTERNAL_FLAG_SUSPENDED: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIM_PROVIDER_EXTERNAL_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub DataSourceId: i64,
    pub ResourceHash: [u8; 20],
}
impl Default for WIM_PROVIDER_EXTERNAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIM_PROVIDER_OVERLAY_ENTRY {
    pub NextEntryOffset: u32,
    pub DataSourceId: i64,
    pub WimGuid: windows_core::GUID,
    pub WimFileNameOffset: u32,
    pub WimType: u32,
    pub WimIndex: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    pub DataSourceId: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    pub DataSourceId: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    pub DataSourceId: i64,
    pub WimFileNameOffset: u32,
    pub WimFileNameLength: u32,
}
pub const WMI_DISK_GEOMETRY_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x25007f51_57c2_11d1_a528_00a0c9062910);
pub const WOF_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WOF_EXTERNAL_FILE_ID {
    pub FileId: super::super::Storage::FileSystem::FILE_ID_128,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WOF_EXTERNAL_INFO {
    pub Version: u32,
    pub Provider: u32,
}
pub const WOF_PROVIDER_CLOUD: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WOF_VERSION_INFO {
    pub WofVersion: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRITE_CACHE_CHANGE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRITE_CACHE_ENABLE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRITE_CACHE_TYPE(pub i32);
pub const WRITE_COMPRESSION_INFO_VALID: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRITE_THROUGH(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WRITE_USN_REASON_INPUT {
    pub Flags: u32,
    pub UsnReasonToWrite: u32,
}
pub const WriteCacheChangeUnknown: WRITE_CACHE_CHANGE = WRITE_CACHE_CHANGE(0i32);
pub const WriteCacheChangeable: WRITE_CACHE_CHANGE = WRITE_CACHE_CHANGE(2i32);
pub const WriteCacheDisabled: WRITE_CACHE_ENABLE = WRITE_CACHE_ENABLE(1i32);
pub const WriteCacheEnableUnknown: WRITE_CACHE_ENABLE = WRITE_CACHE_ENABLE(0i32);
pub const WriteCacheEnabled: WRITE_CACHE_ENABLE = WRITE_CACHE_ENABLE(2i32);
pub const WriteCacheNotChangeable: WRITE_CACHE_CHANGE = WRITE_CACHE_CHANGE(1i32);
pub const WriteCacheTypeNone: WRITE_CACHE_TYPE = WRITE_CACHE_TYPE(1i32);
pub const WriteCacheTypeUnknown: WRITE_CACHE_TYPE = WRITE_CACHE_TYPE(0i32);
pub const WriteCacheTypeWriteBack: WRITE_CACHE_TYPE = WRITE_CACHE_TYPE(2i32);
pub const WriteCacheTypeWriteThrough: WRITE_CACHE_TYPE = WRITE_CACHE_TYPE(3i32);
pub const WriteThroughNotSupported: WRITE_THROUGH = WRITE_THROUGH(1i32);
pub const WriteThroughSupported: WRITE_THROUGH = WRITE_THROUGH(2i32);
pub const WriteThroughUnknown: WRITE_THROUGH = WRITE_THROUGH(0i32);
pub const ZoneConditionClosed: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(4i32);
pub const ZoneConditionConventional: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(0i32);
pub const ZoneConditionEmpty: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(1i32);
pub const ZoneConditionExplicitlyOpened: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(3i32);
pub const ZoneConditionFull: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(14i32);
pub const ZoneConditionImplicitlyOpened: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(2i32);
pub const ZoneConditionOffline: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(15i32);
pub const ZoneConditionReadOnly: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(13i32);
pub const ZoneTypeConventional: STORAGE_ZONE_TYPES = STORAGE_ZONE_TYPES(1i32);
pub const ZoneTypeMax: STORAGE_ZONE_TYPES = STORAGE_ZONE_TYPES(4i32);
pub const ZoneTypeSequentialWritePreferred: STORAGE_ZONE_TYPES = STORAGE_ZONE_TYPES(3i32);
pub const ZoneTypeSequentialWriteRequired: STORAGE_ZONE_TYPES = STORAGE_ZONE_TYPES(2i32);
pub const ZoneTypeUnknown: STORAGE_ZONE_TYPES = STORAGE_ZONE_TYPES(0i32);
pub const ZonedDeviceTypeDeviceManaged: STORAGE_ZONED_DEVICE_TYPES = STORAGE_ZONED_DEVICE_TYPES(3i32);
pub const ZonedDeviceTypeHostAware: STORAGE_ZONED_DEVICE_TYPES = STORAGE_ZONED_DEVICE_TYPES(2i32);
pub const ZonedDeviceTypeHostManaged: STORAGE_ZONED_DEVICE_TYPES = STORAGE_ZONED_DEVICE_TYPES(1i32);
pub const ZonedDeviceTypeUnknown: STORAGE_ZONED_DEVICE_TYPES = STORAGE_ZONED_DEVICE_TYPES(0i32);
pub const ZonesAttributeTypeAndLengthMayDifferent: STORAGE_ZONES_ATTRIBUTES = STORAGE_ZONES_ATTRIBUTES(0i32);
pub const ZonesAttributeTypeMayDifferentLengthSame: STORAGE_ZONES_ATTRIBUTES = STORAGE_ZONES_ATTRIBUTES(3i32);
pub const ZonesAttributeTypeSameLastZoneLengthDifferent: STORAGE_ZONES_ATTRIBUTES = STORAGE_ZONES_ATTRIBUTES(2i32);
pub const ZonesAttributeTypeSameLengthSame: STORAGE_ZONES_ATTRIBUTES = STORAGE_ZONES_ATTRIBUTES(1i32);
