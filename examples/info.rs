#![allow(non_snake_case)]

use std::{
    ffi::{CStr, CString},
    process::ExitCode,
    ptr,
};

use anari_sys::{
    anariGetDeviceExtensions, anariGetObjectInfo, anariGetParameterInfo, anariNewDevice,
    ANARIDataType, ANARIDevice, ANARIObject, ANARIParameter, ANARIStatusCode, ANARIStatusSeverity,
    ANARI_BOOL, ANARI_PARAMETER_LIST, ANARI_RENDERER, ANARI_SEVERITY_DEBUG, ANARI_SEVERITY_ERROR,
    ANARI_SEVERITY_FATAL_ERROR, ANARI_SEVERITY_INFO, ANARI_SEVERITY_PERFORMANCE_WARNING,
    ANARI_SEVERITY_WARNING, ANARI_STRING,
};

unsafe extern "C" fn status_callback(
    _user_ptr: *const ::std::os::raw::c_void,
    _device: ANARIDevice,
    _source: ANARIObject,
    _source_type: ANARIDataType,
    severity: ANARIStatusSeverity,
    _code: ANARIStatusCode,
    message: *const ::std::os::raw::c_char,
) {
    let message = CStr::from_ptr(message).to_string_lossy();
    if severity == ANARI_SEVERITY_FATAL_ERROR {
        eprintln!("[FATAL] {message}");
    } else if severity == ANARI_SEVERITY_ERROR {
        eprintln!("[ERROR] {message}");
    } else if severity == ANARI_SEVERITY_WARNING {
        eprintln!("[WARNING] {message}");
    } else if severity == ANARI_SEVERITY_PERFORMANCE_WARNING {
        eprintln!("[PERFORMANCE WARNING] {message}");
    } else if severity == ANARI_SEVERITY_INFO {
        println!("[info] {message}");
    } else if severity == ANARI_SEVERITY_DEBUG {
        println!("[debug] {message}");
    }
}

fn main() -> ExitCode {
    // Load the library
    let library_name = std::env::args()
        .nth(1)
        .expect("expected a library name argument (e.g. helide)");
    println!("Loading {library_name} library");
    let library_name = CString::new(library_name.as_bytes()).expect("invalid library name?");
    let library = unsafe {
        anari_sys::anariLoadLibrary(library_name.as_ptr(), Some(status_callback), ptr::null())
    };
    if library.is_null() {
        return ExitCode::FAILURE;
    }

    // Query available devices
    let mut device_types = unsafe { anari_sys::anariGetDeviceSubtypes(library) };
    let mut device_type_names = Vec::new();
    if device_types.is_null() {
        println!("No devices anounced.");
        return ExitCode::FAILURE;
    } else {
        println!("Available devices:");
        loop {
            let device_type = unsafe { *device_types };
            if device_type.is_null() {
                break;
            }
            let device_type_name = unsafe { CStr::from_ptr(device_type) };
            device_type_names.push(device_type_name);
            println!("  - {}", device_type_name.to_string_lossy());
            device_types = unsafe { device_types.add(1) };
        }
    }

    // Use the first device
    let device_name = device_type_names.first().unwrap();

    // Query device extensions
    let mut extensions = unsafe { anariGetDeviceExtensions(library, device_name.as_ptr()) };
    let mut extension_names = Vec::new();
    if extensions.is_null() {
        println!("No extensions available!");
    } else {
        println!(
            "Available extensions for {} device:",
            device_name.to_string_lossy()
        );
        loop {
            let extension = unsafe { *extensions };
            if extension.is_null() {
                break;
            }
            let extension_name = unsafe { CStr::from_ptr(extension) };
            println!("  - {}", extension_name.to_string_lossy());
            extension_names.push(extension_name);
            extensions = unsafe { extensions.add(1) };
        }
    }
    // let extensions: *mut anari_sys::ANARIExtensions = ptr::null_mut();
    // assert_eq!(
    //     unsafe { anariGetDeviceExtensionStruct(extensions, library, device_name.as_ptr()) },
    //     0
    // );
    // if !extensions.is_null() {
    //     println!("{:?}", unsafe { *extensions });
    // }

    // Create a new device
    let dev = unsafe { anariNewDevice(library, device_name.as_ptr()) };

    // Query available renderers
    let mut renderers = unsafe { anari_sys::anariGetObjectSubtypes(dev, ANARI_RENDERER) };
    let mut renderer_names = Vec::new();
    if renderers.is_null() {
        println!("No renderers available!");
        return ExitCode::FAILURE;
    } else {
        println!(
            "Available renderers for {} device:",
            device_name.to_string_lossy()
        );
        loop {
            let renderer = unsafe { *renderers };
            if renderer.is_null() {
                break;
            }
            let renderer_name = unsafe { CStr::from_ptr(renderer) };
            println!("  - {}", renderer_name.to_string_lossy());
            renderer_names.push(renderer_name);
            renderers = unsafe { renderers.add(1) };
        }
    }

    // Inspect renderer parameters
    let info_name = CString::new("parameter").unwrap();
    let info_name_description = CString::new("description").unwrap();
    let info_name_required = CString::new("required").unwrap();
    let renderer_name = renderer_names.first().unwrap();
    let mut renderer_params = unsafe {
        anariGetObjectInfo(
            dev,
            ANARI_RENDERER,
            renderer_name.as_ptr(),
            info_name.as_ptr(),
            ANARI_PARAMETER_LIST,
        )
    }
    .cast::<ANARIParameter>();
    if renderer_params.is_null() {
        println!(
            "{} renderer has no parameters.",
            device_name.to_string_lossy()
        );
    } else {
        println!("Parameters of {} renderer:", device_name.to_string_lossy());
        loop {
            let p = unsafe { *renderer_params };
            if p.name.is_null() {
                break;
            }
            let desc = unsafe {
                anariGetParameterInfo(
                    dev,
                    ANARI_RENDERER,
                    renderer_name.as_ptr(),
                    p.name,
                    p.type_,
                    info_name_description.as_ptr(),
                    ANARI_STRING,
                )
            }
            .cast::<i8>();
            assert!(!desc.is_null());
            let required = unsafe {
                anariGetParameterInfo(
                    dev,
                    ANARI_RENDERER,
                    renderer_name.as_ptr(),
                    p.name,
                    p.type_,
                    info_name_required.as_ptr(),
                    ANARI_BOOL,
                )
            }
            .cast::<bool>();
            let name = unsafe { CStr::from_ptr(p.name) }.to_string_lossy();
            let desc = unsafe { CStr::from_ptr(desc) }.to_string_lossy();
            println!(
                "  - [{}] {}, {}: {}",
                p.type_,
                name,
                if !required.is_null() && unsafe { *required } {
                    "required"
                } else {
                    "optional"
                },
                desc
            );
            renderer_params = unsafe { renderer_params.add(1) };
        }
    }

    return ExitCode::SUCCESS;
}
