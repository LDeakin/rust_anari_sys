/* automatically generated by rust-bindgen 0.71.1 */

pub const ANARI_INVALID_HANDLE: ::std::os::raw::c_int = 0;
pub const ANARI_SDK_VERSION_MAJOR: ::std::os::raw::c_int = 0;
pub const ANARI_SDK_VERSION_MINOR: ::std::os::raw::c_int = 13;
pub const ANARI_SDK_VERSION_PATCH: ::std::os::raw::c_int = 1;
pub const ANARI_LOG_DEBUG: ::std::os::raw::c_int = 1;
pub const ANARI_LOG_INFO: ::std::os::raw::c_int = 2;
pub const ANARI_LOG_WARNING: ::std::os::raw::c_int = 3;
pub const ANARI_LOG_ERROR: ::std::os::raw::c_int = 4;
pub const ANARI_LOG_NONE: ::std::os::raw::c_int = 5;
pub const ANARI_NO_WAIT: ::std::os::raw::c_uint = 0;
pub const ANARI_WAIT: ::std::os::raw::c_uint = 1;
pub const ANARI_STATUS_NO_ERROR: ::std::os::raw::c_int = 0;
pub const ANARI_STATUS_UNKNOWN_ERROR: ::std::os::raw::c_int = 1;
pub const ANARI_STATUS_INVALID_ARGUMENT: ::std::os::raw::c_int = 2;
pub const ANARI_STATUS_INVALID_OPERATION: ::std::os::raw::c_int = 3;
pub const ANARI_STATUS_OUT_OF_MEMORY: ::std::os::raw::c_int = 4;
pub const ANARI_STATUS_UNSUPPORTED_DEVICE: ::std::os::raw::c_int = 5;
pub const ANARI_STATUS_VERSION_MISMATCH: ::std::os::raw::c_int = 6;
pub const ANARI_SEVERITY_FATAL_ERROR: ::std::os::raw::c_int = 1;
pub const ANARI_SEVERITY_ERROR: ::std::os::raw::c_int = 2;
pub const ANARI_SEVERITY_WARNING: ::std::os::raw::c_int = 3;
pub const ANARI_SEVERITY_PERFORMANCE_WARNING: ::std::os::raw::c_int = 4;
pub const ANARI_SEVERITY_INFO: ::std::os::raw::c_int = 5;
pub const ANARI_SEVERITY_DEBUG: ::std::os::raw::c_int = 6;
pub const ANARI_UNKNOWN: ::std::os::raw::c_int = 0;
pub const ANARI_DATA_TYPE: ::std::os::raw::c_int = 100;
pub const ANARI_STRING: ::std::os::raw::c_int = 101;
pub const ANARI_VOID_POINTER: ::std::os::raw::c_int = 102;
pub const ANARI_BOOL: ::std::os::raw::c_int = 103;
pub const ANARI_STRING_LIST: ::std::os::raw::c_int = 150;
pub const ANARI_DATA_TYPE_LIST: ::std::os::raw::c_int = 151;
pub const ANARI_PARAMETER_LIST: ::std::os::raw::c_int = 152;
pub const ANARI_FUNCTION_POINTER: ::std::os::raw::c_int = 200;
pub const ANARI_MEMORY_DELETER: ::std::os::raw::c_int = 201;
pub const ANARI_STATUS_CALLBACK: ::std::os::raw::c_int = 202;
pub const ANARI_LIBRARY: ::std::os::raw::c_int = 500;
pub const ANARI_DEVICE: ::std::os::raw::c_int = 501;
pub const ANARI_OBJECT: ::std::os::raw::c_int = 502;
pub const ANARI_ARRAY: ::std::os::raw::c_int = 503;
pub const ANARI_ARRAY1D: ::std::os::raw::c_int = 504;
pub const ANARI_ARRAY2D: ::std::os::raw::c_int = 505;
pub const ANARI_ARRAY3D: ::std::os::raw::c_int = 506;
pub const ANARI_CAMERA: ::std::os::raw::c_int = 507;
pub const ANARI_FRAME: ::std::os::raw::c_int = 508;
pub const ANARI_GEOMETRY: ::std::os::raw::c_int = 509;
pub const ANARI_GROUP: ::std::os::raw::c_int = 510;
pub const ANARI_INSTANCE: ::std::os::raw::c_int = 511;
pub const ANARI_LIGHT: ::std::os::raw::c_int = 512;
pub const ANARI_MATERIAL: ::std::os::raw::c_int = 513;
pub const ANARI_RENDERER: ::std::os::raw::c_int = 514;
pub const ANARI_SURFACE: ::std::os::raw::c_int = 515;
pub const ANARI_SAMPLER: ::std::os::raw::c_int = 516;
pub const ANARI_SPATIAL_FIELD: ::std::os::raw::c_int = 517;
pub const ANARI_VOLUME: ::std::os::raw::c_int = 518;
pub const ANARI_WORLD: ::std::os::raw::c_int = 519;
pub const ANARI_INT8: ::std::os::raw::c_int = 1000;
pub const ANARI_INT8_VEC2: ::std::os::raw::c_int = 1001;
pub const ANARI_INT8_VEC3: ::std::os::raw::c_int = 1002;
pub const ANARI_INT8_VEC4: ::std::os::raw::c_int = 1003;
pub const ANARI_UINT8: ::std::os::raw::c_int = 1004;
pub const ANARI_UINT8_VEC2: ::std::os::raw::c_int = 1005;
pub const ANARI_UINT8_VEC3: ::std::os::raw::c_int = 1006;
pub const ANARI_UINT8_VEC4: ::std::os::raw::c_int = 1007;
pub const ANARI_INT16: ::std::os::raw::c_int = 1008;
pub const ANARI_INT16_VEC2: ::std::os::raw::c_int = 1009;
pub const ANARI_INT16_VEC3: ::std::os::raw::c_int = 1010;
pub const ANARI_INT16_VEC4: ::std::os::raw::c_int = 1011;
pub const ANARI_UINT16: ::std::os::raw::c_int = 1012;
pub const ANARI_UINT16_VEC2: ::std::os::raw::c_int = 1013;
pub const ANARI_UINT16_VEC3: ::std::os::raw::c_int = 1014;
pub const ANARI_UINT16_VEC4: ::std::os::raw::c_int = 1015;
pub const ANARI_INT32: ::std::os::raw::c_int = 1016;
pub const ANARI_INT32_VEC2: ::std::os::raw::c_int = 1017;
pub const ANARI_INT32_VEC3: ::std::os::raw::c_int = 1018;
pub const ANARI_INT32_VEC4: ::std::os::raw::c_int = 1019;
pub const ANARI_UINT32: ::std::os::raw::c_int = 1020;
pub const ANARI_UINT32_VEC2: ::std::os::raw::c_int = 1021;
pub const ANARI_UINT32_VEC3: ::std::os::raw::c_int = 1022;
pub const ANARI_UINT32_VEC4: ::std::os::raw::c_int = 1023;
pub const ANARI_INT64: ::std::os::raw::c_int = 1024;
pub const ANARI_INT64_VEC2: ::std::os::raw::c_int = 1025;
pub const ANARI_INT64_VEC3: ::std::os::raw::c_int = 1026;
pub const ANARI_INT64_VEC4: ::std::os::raw::c_int = 1027;
pub const ANARI_UINT64: ::std::os::raw::c_int = 1028;
pub const ANARI_UINT64_VEC2: ::std::os::raw::c_int = 1029;
pub const ANARI_UINT64_VEC3: ::std::os::raw::c_int = 1030;
pub const ANARI_UINT64_VEC4: ::std::os::raw::c_int = 1031;
pub const ANARI_FIXED8: ::std::os::raw::c_int = 1032;
pub const ANARI_FIXED8_VEC2: ::std::os::raw::c_int = 1033;
pub const ANARI_FIXED8_VEC3: ::std::os::raw::c_int = 1034;
pub const ANARI_FIXED8_VEC4: ::std::os::raw::c_int = 1035;
pub const ANARI_UFIXED8: ::std::os::raw::c_int = 1036;
pub const ANARI_UFIXED8_VEC2: ::std::os::raw::c_int = 1037;
pub const ANARI_UFIXED8_VEC3: ::std::os::raw::c_int = 1038;
pub const ANARI_UFIXED8_VEC4: ::std::os::raw::c_int = 1039;
pub const ANARI_FIXED16: ::std::os::raw::c_int = 1040;
pub const ANARI_FIXED16_VEC2: ::std::os::raw::c_int = 1041;
pub const ANARI_FIXED16_VEC3: ::std::os::raw::c_int = 1042;
pub const ANARI_FIXED16_VEC4: ::std::os::raw::c_int = 1043;
pub const ANARI_UFIXED16: ::std::os::raw::c_int = 1044;
pub const ANARI_UFIXED16_VEC2: ::std::os::raw::c_int = 1045;
pub const ANARI_UFIXED16_VEC3: ::std::os::raw::c_int = 1046;
pub const ANARI_UFIXED16_VEC4: ::std::os::raw::c_int = 1047;
pub const ANARI_FIXED32: ::std::os::raw::c_int = 1048;
pub const ANARI_FIXED32_VEC2: ::std::os::raw::c_int = 1049;
pub const ANARI_FIXED32_VEC3: ::std::os::raw::c_int = 1050;
pub const ANARI_FIXED32_VEC4: ::std::os::raw::c_int = 1051;
pub const ANARI_UFIXED32: ::std::os::raw::c_int = 1052;
pub const ANARI_UFIXED32_VEC2: ::std::os::raw::c_int = 1053;
pub const ANARI_UFIXED32_VEC3: ::std::os::raw::c_int = 1054;
pub const ANARI_UFIXED32_VEC4: ::std::os::raw::c_int = 1055;
pub const ANARI_FIXED64: ::std::os::raw::c_int = 1056;
pub const ANARI_FIXED64_VEC2: ::std::os::raw::c_int = 1057;
pub const ANARI_FIXED64_VEC3: ::std::os::raw::c_int = 1058;
pub const ANARI_FIXED64_VEC4: ::std::os::raw::c_int = 1059;
pub const ANARI_UFIXED64: ::std::os::raw::c_int = 1060;
pub const ANARI_UFIXED64_VEC2: ::std::os::raw::c_int = 1061;
pub const ANARI_UFIXED64_VEC3: ::std::os::raw::c_int = 1062;
pub const ANARI_UFIXED64_VEC4: ::std::os::raw::c_int = 1063;
pub const ANARI_FLOAT16: ::std::os::raw::c_int = 1064;
pub const ANARI_FLOAT16_VEC2: ::std::os::raw::c_int = 1065;
pub const ANARI_FLOAT16_VEC3: ::std::os::raw::c_int = 1066;
pub const ANARI_FLOAT16_VEC4: ::std::os::raw::c_int = 1067;
pub const ANARI_FLOAT32: ::std::os::raw::c_int = 1068;
pub const ANARI_FLOAT32_VEC2: ::std::os::raw::c_int = 1069;
pub const ANARI_FLOAT32_VEC3: ::std::os::raw::c_int = 1070;
pub const ANARI_FLOAT32_VEC4: ::std::os::raw::c_int = 1071;
pub const ANARI_FLOAT64: ::std::os::raw::c_int = 1072;
pub const ANARI_FLOAT64_VEC2: ::std::os::raw::c_int = 1073;
pub const ANARI_FLOAT64_VEC3: ::std::os::raw::c_int = 1074;
pub const ANARI_FLOAT64_VEC4: ::std::os::raw::c_int = 1075;
pub const ANARI_UFIXED8_RGBA_SRGB: ::std::os::raw::c_int = 2003;
pub const ANARI_UFIXED8_RGB_SRGB: ::std::os::raw::c_int = 2002;
pub const ANARI_UFIXED8_RA_SRGB: ::std::os::raw::c_int = 2001;
pub const ANARI_UFIXED8_R_SRGB: ::std::os::raw::c_int = 2000;
pub const ANARI_INT32_BOX1: ::std::os::raw::c_int = 2004;
pub const ANARI_INT32_BOX2: ::std::os::raw::c_int = 2005;
pub const ANARI_INT32_BOX3: ::std::os::raw::c_int = 2006;
pub const ANARI_INT32_BOX4: ::std::os::raw::c_int = 2007;
pub const ANARI_FLOAT32_BOX1: ::std::os::raw::c_int = 2008;
pub const ANARI_FLOAT32_BOX2: ::std::os::raw::c_int = 2009;
pub const ANARI_FLOAT32_BOX3: ::std::os::raw::c_int = 2010;
pub const ANARI_FLOAT32_BOX4: ::std::os::raw::c_int = 2011;
pub const ANARI_FLOAT64_BOX1: ::std::os::raw::c_int = 2208;
pub const ANARI_FLOAT64_BOX2: ::std::os::raw::c_int = 2209;
pub const ANARI_FLOAT64_BOX3: ::std::os::raw::c_int = 2210;
pub const ANARI_FLOAT64_BOX4: ::std::os::raw::c_int = 2211;
pub const ANARI_UINT64_REGION1: ::std::os::raw::c_int = 2104;
pub const ANARI_UINT64_REGION2: ::std::os::raw::c_int = 2105;
pub const ANARI_UINT64_REGION3: ::std::os::raw::c_int = 2106;
pub const ANARI_UINT64_REGION4: ::std::os::raw::c_int = 2107;
pub const ANARI_FLOAT32_MAT2: ::std::os::raw::c_int = 2012;
pub const ANARI_FLOAT32_MAT3: ::std::os::raw::c_int = 2013;
pub const ANARI_FLOAT32_MAT4: ::std::os::raw::c_int = 2014;
pub const ANARI_FLOAT32_MAT2x3: ::std::os::raw::c_int = 2015;
pub const ANARI_FLOAT32_MAT3x4: ::std::os::raw::c_int = 2016;
pub const ANARI_FLOAT32_QUAT_IJKW: ::std::os::raw::c_int = 2017;
pub const ANARI_FRAME_COMPLETION_CALLBACK: ::std::os::raw::c_int = 203;
pub type ANARIDataType = ::std::os::raw::c_int;
pub type ANARILogLevel = ::std::os::raw::c_int;
pub type ANARIWaitMask = ::std::os::raw::c_uint;
pub type ANARIStatusCode = ::std::os::raw::c_int;
pub type ANARIStatusSeverity = ::std::os::raw::c_int;
pub type ANARILibrary = *mut ::std::os::raw::c_void;
pub type ANARIObject = *mut ::std::os::raw::c_void;
pub type ANARIDevice = *mut ::std::os::raw::c_void;
pub type ANARICamera = *mut ::std::os::raw::c_void;
pub type ANARIArray = *mut ::std::os::raw::c_void;
pub type ANARIArray1D = *mut ::std::os::raw::c_void;
pub type ANARIArray2D = *mut ::std::os::raw::c_void;
pub type ANARIArray3D = *mut ::std::os::raw::c_void;
pub type ANARIFrame = *mut ::std::os::raw::c_void;
pub type ANARIFuture = *mut ::std::os::raw::c_void;
pub type ANARIGeometry = *mut ::std::os::raw::c_void;
pub type ANARIGroup = *mut ::std::os::raw::c_void;
pub type ANARIInstance = *mut ::std::os::raw::c_void;
pub type ANARILight = *mut ::std::os::raw::c_void;
pub type ANARIMaterial = *mut ::std::os::raw::c_void;
pub type ANARISampler = *mut ::std::os::raw::c_void;
pub type ANARISurface = *mut ::std::os::raw::c_void;
pub type ANARIRenderer = *mut ::std::os::raw::c_void;
pub type ANARISpatialField = *mut ::std::os::raw::c_void;
pub type ANARIVolume = *mut ::std::os::raw::c_void;
pub type ANARIWorld = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ANARIParameter {
    pub name: *const ::std::os::raw::c_char,
    pub type_: ANARIDataType,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ANARIParameter"][::std::mem::size_of::<ANARIParameter>() - 16usize];
    ["Alignment of ANARIParameter"][::std::mem::align_of::<ANARIParameter>() - 8usize];
    ["Offset of field: ANARIParameter::name"]
        [::std::mem::offset_of!(ANARIParameter, name) - 0usize];
    ["Offset of field: ANARIParameter::type_"]
        [::std::mem::offset_of!(ANARIParameter, type_) - 8usize];
};
pub type ANARIMemoryDeleter = ::std::option::Option<
    unsafe extern "C" fn(
        userPtr: *const ::std::os::raw::c_void,
        appMemory: *const ::std::os::raw::c_void,
    ),
>;
pub type ANARIStatusCallback = ::std::option::Option<
    unsafe extern "C" fn(
        userPtr: *const ::std::os::raw::c_void,
        device: ANARIDevice,
        source: ANARIObject,
        sourceType: ANARIDataType,
        severity: ANARIStatusSeverity,
        code: ANARIStatusCode,
        message: *const ::std::os::raw::c_char,
    ),
>;
pub type ANARIFrameCompletionCallback = ::std::option::Option<
    unsafe extern "C" fn(
        userPtr: *const ::std::os::raw::c_void,
        device: ANARIDevice,
        frame: ANARIFrame,
    ),
>;
extern "C" {
    pub fn anariLoadLibrary(
        name: *const ::std::os::raw::c_char,
        statusCallback: ANARIStatusCallback,
        statusCallbackUserData: *const ::std::os::raw::c_void,
    ) -> ANARILibrary;
}
extern "C" {
    pub fn anariUnloadLibrary(module: ANARILibrary);
}
extern "C" {
    pub fn anariLoadModule(library: ANARILibrary, name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn anariUnloadModule(library: ANARILibrary, name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn anariNewDevice(
        library: ANARILibrary,
        type_: *const ::std::os::raw::c_char,
    ) -> ANARIDevice;
}
extern "C" {
    pub fn anariNewArray1D(
        device: ANARIDevice,
        appMemory: *const ::std::os::raw::c_void,
        deleter: ANARIMemoryDeleter,
        userData: *const ::std::os::raw::c_void,
        dataType: ANARIDataType,
        numElements1: u64,
    ) -> ANARIArray1D;
}
extern "C" {
    pub fn anariNewArray2D(
        device: ANARIDevice,
        appMemory: *const ::std::os::raw::c_void,
        deleter: ANARIMemoryDeleter,
        userData: *const ::std::os::raw::c_void,
        dataType: ANARIDataType,
        numElements1: u64,
        numElements2: u64,
    ) -> ANARIArray2D;
}
extern "C" {
    pub fn anariNewArray3D(
        device: ANARIDevice,
        appMemory: *const ::std::os::raw::c_void,
        deleter: ANARIMemoryDeleter,
        userData: *const ::std::os::raw::c_void,
        dataType: ANARIDataType,
        numElements1: u64,
        numElements2: u64,
        numElements3: u64,
    ) -> ANARIArray3D;
}
extern "C" {
    pub fn anariMapArray(device: ANARIDevice, array: ANARIArray) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn anariUnmapArray(device: ANARIDevice, array: ANARIArray);
}
extern "C" {
    pub fn anariNewLight(device: ANARIDevice, type_: *const ::std::os::raw::c_char) -> ANARILight;
}
extern "C" {
    pub fn anariNewCamera(device: ANARIDevice, type_: *const ::std::os::raw::c_char)
        -> ANARICamera;
}
extern "C" {
    pub fn anariNewGeometry(
        device: ANARIDevice,
        type_: *const ::std::os::raw::c_char,
    ) -> ANARIGeometry;
}
extern "C" {
    pub fn anariNewSpatialField(
        device: ANARIDevice,
        type_: *const ::std::os::raw::c_char,
    ) -> ANARISpatialField;
}
extern "C" {
    pub fn anariNewVolume(device: ANARIDevice, type_: *const ::std::os::raw::c_char)
        -> ANARIVolume;
}
extern "C" {
    pub fn anariNewSurface(device: ANARIDevice) -> ANARISurface;
}
extern "C" {
    pub fn anariNewMaterial(
        device: ANARIDevice,
        type_: *const ::std::os::raw::c_char,
    ) -> ANARIMaterial;
}
extern "C" {
    pub fn anariNewSampler(
        device: ANARIDevice,
        type_: *const ::std::os::raw::c_char,
    ) -> ANARISampler;
}
extern "C" {
    pub fn anariNewGroup(device: ANARIDevice) -> ANARIGroup;
}
extern "C" {
    pub fn anariNewInstance(
        device: ANARIDevice,
        type_: *const ::std::os::raw::c_char,
    ) -> ANARIInstance;
}
extern "C" {
    pub fn anariNewWorld(device: ANARIDevice) -> ANARIWorld;
}
extern "C" {
    pub fn anariNewObject(
        device: ANARIDevice,
        objectType: *const ::std::os::raw::c_char,
        type_: *const ::std::os::raw::c_char,
    ) -> ANARIObject;
}
extern "C" {
    pub fn anariSetParameter(
        device: ANARIDevice,
        object: ANARIObject,
        name: *const ::std::os::raw::c_char,
        dataType: ANARIDataType,
        mem: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn anariUnsetParameter(
        device: ANARIDevice,
        object: ANARIObject,
        name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn anariUnsetAllParameters(device: ANARIDevice, object: ANARIObject);
}
extern "C" {
    pub fn anariMapParameterArray1D(
        device: ANARIDevice,
        object: ANARIObject,
        name: *const ::std::os::raw::c_char,
        dataType: ANARIDataType,
        numElements1: u64,
        elementStride: *mut u64,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn anariMapParameterArray2D(
        device: ANARIDevice,
        object: ANARIObject,
        name: *const ::std::os::raw::c_char,
        dataType: ANARIDataType,
        numElements1: u64,
        numElements2: u64,
        elementStride: *mut u64,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn anariMapParameterArray3D(
        device: ANARIDevice,
        object: ANARIObject,
        name: *const ::std::os::raw::c_char,
        dataType: ANARIDataType,
        numElements1: u64,
        numElements2: u64,
        numElements3: u64,
        elementStride: *mut u64,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn anariUnmapParameterArray(
        device: ANARIDevice,
        object: ANARIObject,
        name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn anariCommitParameters(device: ANARIDevice, object: ANARIObject);
}
extern "C" {
    pub fn anariRelease(device: ANARIDevice, object: ANARIObject);
}
extern "C" {
    pub fn anariRetain(device: ANARIDevice, object: ANARIObject);
}
extern "C" {
    pub fn anariGetDeviceSubtypes(library: ANARILibrary) -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn anariGetDeviceExtensions(
        library: ANARILibrary,
        deviceSubtype: *const ::std::os::raw::c_char,
    ) -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn anariGetObjectSubtypes(
        device: ANARIDevice,
        objectType: ANARIDataType,
    ) -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn anariGetObjectInfo(
        device: ANARIDevice,
        objectType: ANARIDataType,
        objectSubtype: *const ::std::os::raw::c_char,
        infoName: *const ::std::os::raw::c_char,
        infoType: ANARIDataType,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn anariGetParameterInfo(
        device: ANARIDevice,
        objectType: ANARIDataType,
        objectSubtype: *const ::std::os::raw::c_char,
        parameterName: *const ::std::os::raw::c_char,
        parameterType: ANARIDataType,
        infoName: *const ::std::os::raw::c_char,
        infoType: ANARIDataType,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn anariGetProperty(
        device: ANARIDevice,
        object: ANARIObject,
        name: *const ::std::os::raw::c_char,
        type_: ANARIDataType,
        mem: *mut ::std::os::raw::c_void,
        size: u64,
        mask: ANARIWaitMask,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn anariNewFrame(device: ANARIDevice) -> ANARIFrame;
}
extern "C" {
    pub fn anariMapFrame(
        device: ANARIDevice,
        frame: ANARIFrame,
        channel: *const ::std::os::raw::c_char,
        width: *mut u32,
        height: *mut u32,
        pixelType: *mut ANARIDataType,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn anariUnmapFrame(
        device: ANARIDevice,
        frame: ANARIFrame,
        channel: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn anariNewRenderer(
        device: ANARIDevice,
        type_: *const ::std::os::raw::c_char,
    ) -> ANARIRenderer;
}
extern "C" {
    pub fn anariRenderFrame(device: ANARIDevice, frame: ANARIFrame);
}
extern "C" {
    pub fn anariFrameReady(
        device: ANARIDevice,
        frame: ANARIFrame,
        mask: ANARIWaitMask,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn anariDiscardFrame(device: ANARIDevice, frame: ANARIFrame);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ANARIExtensions {
    pub ANARI_KHR_LIGHT_DIRECTIONAL: ::std::os::raw::c_int,
    pub ANARI_KHR_LIGHT_POINT: ::std::os::raw::c_int,
    pub ANARI_KHR_LIGHT_RING: ::std::os::raw::c_int,
    pub ANARI_KHR_LIGHT_QUAD: ::std::os::raw::c_int,
    pub ANARI_KHR_LIGHT_HDRI: ::std::os::raw::c_int,
    pub ANARI_KHR_CAMERA_SHUTTER: ::std::os::raw::c_int,
    pub ANARI_KHR_INSTANCE_MOTION_SCALE_ROTATION_TRANSLATION: ::std::os::raw::c_int,
    pub ANARI_KHR_INSTANCE_MOTION_TRANSFORM: ::std::os::raw::c_int,
    pub ANARI_KHR_INSTANCE_TRANSFORM: ::std::os::raw::c_int,
    pub ANARI_KHR_VOLUME_TRANSFER_FUNCTION1D: ::std::os::raw::c_int,
    pub ANARI_KHR_AREA_LIGHTS: ::std::os::raw::c_int,
    pub ANARI_KHR_ARRAY1D_REGION: ::std::os::raw::c_int,
    pub ANARI_KHR_CAMERA_DEPTH_OF_FIELD: ::std::os::raw::c_int,
    pub ANARI_KHR_CAMERA_MOTION_TRANSFORMATION: ::std::os::raw::c_int,
    pub ANARI_KHR_CAMERA_OMNIDIRECTIONAL: ::std::os::raw::c_int,
    pub ANARI_KHR_CAMERA_ORTHOGRAPHIC: ::std::os::raw::c_int,
    pub ANARI_KHR_CAMERA_PERSPECTIVE: ::std::os::raw::c_int,
    pub ANARI_KHR_CAMERA_STEREO: ::std::os::raw::c_int,
    pub ANARI_KHR_DEVICE_SYNCHRONIZATION: ::std::os::raw::c_int,
    pub ANARI_KHR_FRAME_ACCUMULATION: ::std::os::raw::c_int,
    pub ANARI_KHR_FRAME_CHANNEL_ALBEDO: ::std::os::raw::c_int,
    pub ANARI_KHR_FRAME_CHANNEL_INSTANCE_ID: ::std::os::raw::c_int,
    pub ANARI_KHR_FRAME_CHANNEL_NORMAL: ::std::os::raw::c_int,
    pub ANARI_KHR_FRAME_CHANNEL_OBJECT_ID: ::std::os::raw::c_int,
    pub ANARI_KHR_FRAME_CHANNEL_PRIMITIVE_ID: ::std::os::raw::c_int,
    pub ANARI_KHR_FRAME_COMPLETION_CALLBACK: ::std::os::raw::c_int,
    pub ANARI_KHR_GEOMETRY_CONE: ::std::os::raw::c_int,
    pub ANARI_KHR_GEOMETRY_CURVE: ::std::os::raw::c_int,
    pub ANARI_KHR_GEOMETRY_CYLINDER: ::std::os::raw::c_int,
    pub ANARI_KHR_GEOMETRY_QUAD: ::std::os::raw::c_int,
    pub ANARI_KHR_GEOMETRY_QUAD_MOTION_DEFORMATION: ::std::os::raw::c_int,
    pub ANARI_KHR_GEOMETRY_SPHERE: ::std::os::raw::c_int,
    pub ANARI_KHR_GEOMETRY_TRIANGLE: ::std::os::raw::c_int,
    pub ANARI_KHR_GEOMETRY_TRIANGLE_MOTION_DEFORMATION: ::std::os::raw::c_int,
    pub ANARI_KHR_LIGHT_SPOT: ::std::os::raw::c_int,
    pub ANARI_KHR_MATERIAL_MATTE: ::std::os::raw::c_int,
    pub ANARI_KHR_MATERIAL_PHYSICALLY_BASED: ::std::os::raw::c_int,
    pub ANARI_KHR_RENDERER_AMBIENT_LIGHT: ::std::os::raw::c_int,
    pub ANARI_KHR_RENDERER_BACKGROUND_COLOR: ::std::os::raw::c_int,
    pub ANARI_KHR_RENDERER_BACKGROUND_IMAGE: ::std::os::raw::c_int,
    pub ANARI_KHR_SAMPLER_IMAGE1D: ::std::os::raw::c_int,
    pub ANARI_KHR_SAMPLER_IMAGE2D: ::std::os::raw::c_int,
    pub ANARI_KHR_SAMPLER_IMAGE3D: ::std::os::raw::c_int,
    pub ANARI_KHR_SAMPLER_PRIMITIVE: ::std::os::raw::c_int,
    pub ANARI_KHR_SAMPLER_TRANSFORM: ::std::os::raw::c_int,
    pub ANARI_KHR_SPATIAL_FIELD_STRUCTURED_REGULAR: ::std::os::raw::c_int,
    pub ANARI_EXP_VOLUME_SAMPLE_RATE: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ANARIExtensions"][::std::mem::size_of::<ANARIExtensions>() - 188usize];
    ["Alignment of ANARIExtensions"][::std::mem::align_of::<ANARIExtensions>() - 4usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_LIGHT_DIRECTIONAL"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_LIGHT_DIRECTIONAL) - 0usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_LIGHT_POINT"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_LIGHT_POINT) - 4usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_LIGHT_RING"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_LIGHT_RING) - 8usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_LIGHT_QUAD"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_LIGHT_QUAD) - 12usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_LIGHT_HDRI"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_LIGHT_HDRI) - 16usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_CAMERA_SHUTTER"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_CAMERA_SHUTTER) - 20usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_INSTANCE_MOTION_SCALE_ROTATION_TRANSLATION"][::std::mem::offset_of!(
        ANARIExtensions,
        ANARI_KHR_INSTANCE_MOTION_SCALE_ROTATION_TRANSLATION
    )
        - 24usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_INSTANCE_MOTION_TRANSFORM"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_INSTANCE_MOTION_TRANSFORM) - 28usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_INSTANCE_TRANSFORM"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_INSTANCE_TRANSFORM) - 32usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_VOLUME_TRANSFER_FUNCTION1D"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_VOLUME_TRANSFER_FUNCTION1D) - 36usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_AREA_LIGHTS"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_AREA_LIGHTS) - 40usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_ARRAY1D_REGION"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_ARRAY1D_REGION) - 44usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_CAMERA_DEPTH_OF_FIELD"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_CAMERA_DEPTH_OF_FIELD) - 48usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_CAMERA_MOTION_TRANSFORMATION"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_CAMERA_MOTION_TRANSFORMATION) - 52usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_CAMERA_OMNIDIRECTIONAL"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_CAMERA_OMNIDIRECTIONAL) - 56usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_CAMERA_ORTHOGRAPHIC"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_CAMERA_ORTHOGRAPHIC) - 60usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_CAMERA_PERSPECTIVE"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_CAMERA_PERSPECTIVE) - 64usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_CAMERA_STEREO"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_CAMERA_STEREO) - 68usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_DEVICE_SYNCHRONIZATION"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_DEVICE_SYNCHRONIZATION) - 72usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_FRAME_ACCUMULATION"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_FRAME_ACCUMULATION) - 76usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_FRAME_CHANNEL_ALBEDO"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_FRAME_CHANNEL_ALBEDO) - 80usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_FRAME_CHANNEL_INSTANCE_ID"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_FRAME_CHANNEL_INSTANCE_ID) - 84usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_FRAME_CHANNEL_NORMAL"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_FRAME_CHANNEL_NORMAL) - 88usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_FRAME_CHANNEL_OBJECT_ID"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_FRAME_CHANNEL_OBJECT_ID) - 92usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_FRAME_CHANNEL_PRIMITIVE_ID"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_FRAME_CHANNEL_PRIMITIVE_ID) - 96usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_FRAME_COMPLETION_CALLBACK"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_FRAME_COMPLETION_CALLBACK) - 100usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_GEOMETRY_CONE"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_GEOMETRY_CONE) - 104usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_GEOMETRY_CURVE"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_GEOMETRY_CURVE) - 108usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_GEOMETRY_CYLINDER"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_GEOMETRY_CYLINDER) - 112usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_GEOMETRY_QUAD"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_GEOMETRY_QUAD) - 116usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_GEOMETRY_QUAD_MOTION_DEFORMATION"][::std::mem::offset_of!(
        ANARIExtensions,
        ANARI_KHR_GEOMETRY_QUAD_MOTION_DEFORMATION
    ) - 120usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_GEOMETRY_SPHERE"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_GEOMETRY_SPHERE) - 124usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_GEOMETRY_TRIANGLE"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_GEOMETRY_TRIANGLE) - 128usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_GEOMETRY_TRIANGLE_MOTION_DEFORMATION"][::std::mem::offset_of!(
        ANARIExtensions,
        ANARI_KHR_GEOMETRY_TRIANGLE_MOTION_DEFORMATION
    )
        - 132usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_LIGHT_SPOT"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_LIGHT_SPOT) - 136usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_MATERIAL_MATTE"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_MATERIAL_MATTE) - 140usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_MATERIAL_PHYSICALLY_BASED"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_MATERIAL_PHYSICALLY_BASED) - 144usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_RENDERER_AMBIENT_LIGHT"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_RENDERER_AMBIENT_LIGHT) - 148usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_RENDERER_BACKGROUND_COLOR"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_RENDERER_BACKGROUND_COLOR) - 152usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_RENDERER_BACKGROUND_IMAGE"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_RENDERER_BACKGROUND_IMAGE) - 156usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_SAMPLER_IMAGE1D"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_SAMPLER_IMAGE1D) - 160usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_SAMPLER_IMAGE2D"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_SAMPLER_IMAGE2D) - 164usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_SAMPLER_IMAGE3D"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_SAMPLER_IMAGE3D) - 168usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_SAMPLER_PRIMITIVE"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_SAMPLER_PRIMITIVE) - 172usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_SAMPLER_TRANSFORM"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_KHR_SAMPLER_TRANSFORM) - 176usize];
    ["Offset of field: ANARIExtensions::ANARI_KHR_SPATIAL_FIELD_STRUCTURED_REGULAR"][::std::mem::offset_of!(
        ANARIExtensions,
        ANARI_KHR_SPATIAL_FIELD_STRUCTURED_REGULAR
    ) - 180usize];
    ["Offset of field: ANARIExtensions::ANARI_EXP_VOLUME_SAMPLE_RATE"]
        [::std::mem::offset_of!(ANARIExtensions, ANARI_EXP_VOLUME_SAMPLE_RATE) - 184usize];
};
extern "C" {
    pub fn anariGetDeviceExtensionStruct(
        extensions: *mut ANARIExtensions,
        library: ANARILibrary,
        deviceName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn anariGetObjectExtensionStruct(
        extensions: *mut ANARIExtensions,
        device: ANARIDevice,
        objectType: ANARIDataType,
        objectName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn anariGetInstanceExtensionStruct(
        extensions: *mut ANARIExtensions,
        device: ANARIDevice,
        object: ANARIObject,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn anariDeviceGetProcAddress(
        arg1: ANARIDevice,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::option::Option<
        unsafe extern "C" fn(arg1: ANARIDevice, name: *const ::std::os::raw::c_char),
    >;
}
pub type PFNANARIINSERTSTATUSMESSAGE = ::std::option::Option<
    unsafe extern "C" fn(arg1: ANARIDevice, arg2: *const ::std::os::raw::c_char),
>;
pub type PFNANARINAMEOBJECT = ::std::option::Option<
    unsafe extern "C" fn(arg1: ANARIDevice, arg2: ANARIObject, arg3: *const ::std::os::raw::c_char),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ANARI_EXT_debug_interface_s {
    pub anariInsertStatusMessage: PFNANARIINSERTSTATUSMESSAGE,
    pub anariNameObject: PFNANARINAMEOBJECT,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ANARI_EXT_debug_interface_s"]
        [::std::mem::size_of::<ANARI_EXT_debug_interface_s>() - 16usize];
    ["Alignment of ANARI_EXT_debug_interface_s"]
        [::std::mem::align_of::<ANARI_EXT_debug_interface_s>() - 8usize];
    ["Offset of field: ANARI_EXT_debug_interface_s::anariInsertStatusMessage"]
        [::std::mem::offset_of!(ANARI_EXT_debug_interface_s, anariInsertStatusMessage) - 0usize];
    ["Offset of field: ANARI_EXT_debug_interface_s::anariNameObject"]
        [::std::mem::offset_of!(ANARI_EXT_debug_interface_s, anariNameObject) - 8usize];
};
pub type ANARI_EXT_debug_interface = ANARI_EXT_debug_interface_s;
extern "C" {
    pub static mut ANARI_EXT_debug_interface_impl: ANARI_EXT_debug_interface;
}
