use core::panic;

fn extract_version(line: &str, prefix: &str) -> Option<u32> {
    if line.starts_with(prefix) {
        Some(
            line.strip_prefix(prefix)
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .parse::<u32>()
                .expect(&format!("{} is not an integer", prefix)),
        )
    } else {
        None
    }
}

fn get_anari_version_cmake() -> String {
    let cmake_version = std::fs::read_to_string("ANARI-SDK/CMakeLists.txt")
        .expect("Could not find ANARI-SDK/CMakeLists.txt. Update submodules?");
    let mut major_version = None;
    let mut minor_version = None;
    let mut patch_version = None;
    for line in cmake_version.lines() {
        if line.starts_with("set(ANARI_SDK_VERSION") {
            if let Some(version) = extract_version(&line, "set(ANARI_SDK_VERSION_MAJOR ") {
                major_version = Some(version);
            } else if let Some(version) = extract_version(&line, "set(ANARI_SDK_VERSION_MINOR ") {
                minor_version = Some(version);
            } else if let Some(version) = extract_version(&line, "set(ANARI_SDK_VERSION_PATCH ") {
                patch_version = Some(version);
            }
        }
    }

    if let (Some(major), Some(minor), Some(patch)) = (major_version, minor_version, patch_version) {
        return format!("{}.{}.{}", major, minor, patch);
    } else {
        panic!("Could not find anari version in ANARI-SDK/CMakeLists.txt");
    }
}

fn get_anari_version() -> (u32, u32, u32) {
    let version = env!("CARGO_PKG_VERSION");
    let versions = version.split("+anari.").collect::<Vec<_>>();
    let anari_version = match &versions[..] {
        &[_crate_version, anari_version] => anari_version,
        _ => panic!("Could not identify anari version from crate version"),
    };
    if get_anari_version_cmake() != anari_version {
        panic!("Crate version does not match cmake version of ANARI-SDK: got:{anari_version} expected:{}", get_anari_version_cmake());
    }
    let anari_versions = anari_version.split('.').collect::<Vec<_>>();
    match &anari_versions[..] {
        &[major, minor, patch] => (
            major
                .parse::<u32>()
                .expect("Anari major version is not an integer"),
            minor
                .parse::<u32>()
                .expect("Anari minor version is not an integer"),
            patch
                .parse::<u32>()
                .expect("Anari patch version is not an integer"),
        ),
        _ => panic!("Could not identify anari version from crate version"),
    }
}

fn create_version_header(out_path: &std::path::Path) {
    let (anari_ver_major, anari_ver_minor, anari_ver_patch) = get_anari_version();
    let anari_sdk_version = std::fs::read_to_string("ANARI-SDK/src/anari/anari_sdk_version.h.in")
        .expect("Could not find ANARI-SDK/src/anari/anari_sdk_version.h.in. Update submodules?")
        .replace("@ANARI_SDK_VERSION_MAJOR@", &anari_ver_major.to_string())
        .replace("@ANARI_SDK_VERSION_MINOR@", &anari_ver_minor.to_string())
        .replace("@ANARI_SDK_VERSION_PATCH@", &anari_ver_patch.to_string());
    let out_path = out_path.join("include/anari/frontend");
    std::fs::create_dir_all(&out_path).unwrap_or_else(|_| panic!("Unable to write {out_path:?}"));
    let out_path = out_path.join("anari_sdk_version.h");
    std::fs::write(&out_path, anari_sdk_version)
        .unwrap_or_else(|_| panic!("Unable to write {out_path:?}"));
}

fn compile_anari_cc(dst: &std::path::Path) {
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .include("ANARI-SDK/src/anari/include")
        .include(dst.join("include/anari"))
        .define("ANARI_STATIC_DEFINE", None)
        .file("ANARI-SDK/src/anari/anari_cpp_linalg_defs.cpp")
        .file("ANARI-SDK/src/anari/anari_cpp_std_defs.cpp")
        .file("ANARI-SDK/src/anari/API.cpp")
        .file("ANARI-SDK/src/anari/DeviceImpl.cpp")
        .file("ANARI-SDK/src/anari/LibraryImpl.cpp")
        .file("src/anari_extension_utility.cpp")
        // .file("src/type_utility.cpp")
    ;

    if cfg!(windows) {
        build.define("_USE_MATH_DEFINES", None);
    }

    build.compile("anari");
}

#[cfg(feature = "bindgen")]
#[derive(Debug)]
struct ReplaceDefs;

#[cfg(feature = "bindgen")]
impl bindgen::callbacks::ParseCallbacks for ReplaceDefs {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        println!("original_item_name: {original_item_name}");
        if original_item_name == "ANARI_DATA_TYPE_DEFINE" {
            panic!();
        }
        None
    }

    fn func_macro(&self, _name: &str, _value: &[&[u8]]) {
        panic!("func macro");
    }

    fn str_macro(&self, _name: &str, _value: &[u8]) {
        panic!("str macro");
    }

    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if name == "ANARI_UNKNOWN" {
            return bindgen::callbacks::MacroParsingBehavior::Default;
        }
        bindgen::callbacks::MacroParsingBehavior::Default
    }

    fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        if name.starts_with("ANARI") {
            if name.ends_with("_WAIT") {
                Some(bindgen::callbacks::IntKind::UInt)
            } else {
                Some(bindgen::callbacks::IntKind::Int)
            }
        } else {
            None
        }
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings(out_path: &std::path::Path) {
    // Fixup anari enums for bindgen (replace ANARI_DATA_TYPE_DEFINE(XXXX) with XXXX )
    let anari_enums =
        std::fs::read_to_string("ANARI-SDK/src/anari/include/anari/frontend/anari_enums.h")
            .expect("Could not read ANARI-SDK/src/anari/include/anari/frontend/anari_enums.h");
    let mut anari_enums_fix = String::new();
    for line in anari_enums.lines() {
        if line == "#define ANARI_DATA_TYPE_DEFINE(v) ANARIDataType(v)" {
            continue;
        } else if line == "#define ANARI_DATA_TYPE_DEFINE(v) v" {
            continue;
        } else if line.contains(" ANARI_DATA_TYPE_DEFINE(") {
            println!("line: {line}");
            let line = line.replace("ANARI_DATA_TYPE_DEFINE(", "");
            let line = line.strip_suffix(')').unwrap();
            println!("line fix: {line}");
            anari_enums_fix.push_str(line);
            anari_enums_fix.push('\n');
        }
    }

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header("ANARI-SDK/src/anari/include/anari/anari.h")
        .header_contents("frontend/anari_enums.h", &anari_enums_fix)
        .header("ANARI-SDK/src/anari/include/anari/frontend/anari_extension_utility.h")
        // .header("ANARI-SDK/src/anari/include/anari/frontend/type_utility.h")
        .header("ANARI-SDK/src/anari/include/anari/ext/anari_ext_interface.h")
        .header("ANARI-SDK/src/anari/include/anari/ext/anari_debug.h")
        .clang_arg("-IANARI-SDK/src/anari/include")
        .clang_arg(format!(
            "-I{}",
            out_path.join("include/anari").to_str().unwrap()
        ))
        .clang_arg("-Dinline=")
        .allowlist_item(".*[Aa]nari[^_]*") // skip anari_ functions in type_utility.h
        .allowlist_item(".*ANARI.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(ReplaceDefs))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = manifest_dir.join("src/bindings.rs");
    bindings
        .write_to_file(&out_path)
        .unwrap_or_else(|_| panic!("Couldn't write bindings to {out_path:?}!"))
}

fn main() {
    println!("cargo:rustc-link-lib=static=anari");
    println!("cargo:rerun-if-changed=ANARI-SDK/");

    let dst = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    let _ = std::fs::remove_dir_all(dst.join("include/anari"));

    create_version_header(&dst);

    compile_anari_cc(&dst);

    #[cfg(feature = "bindgen")]
    generate_bindings(&dst);

    // std::fs::create_dir_all(dst.join("include/anari/frontend")).unwrap();
    // std::fs::create_dir_all(dst.join("include/anari/ext")).unwrap();
    // for f in [
    //     "anari.h",
    //     "frontend/anari_enums.h",
    //     "frontend/anari_extension_utility.h",
    //     "ext/anari_ext_interface.h",
    //     "ext/anari_debug.h",
    // ]
    // .iter()
    // {
    //     std::fs::copy(
    //         format!("ANARI-SDK/src/anari/include/anari/{}", f),
    //         dst.join(format!("include/anari/{}", f)),
    //     )
    //     .unwrap();
    // }

    println!("cargo:root={}", dst.to_str().unwrap());
    println!("cargo:include={}/include", dst.to_str().unwrap());
}
