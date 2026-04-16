use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{Context, Result, anyhow, bail};


#[allow(warnings)]
mod generated {
    #[allow(warnings)]
    pub mod types {
        include!(concat!(env!("OUT_DIR"), "/types.rs"));
    }

    #[allow(warnings)]
    pub mod llama {
        use super::types::*;
        include!(concat!(env!("OUT_DIR"), "/llama_loader.rs"));
    }

    #[allow(warnings)]
    pub mod ggml {
        use super::types::*;
        include!(concat!(env!("OUT_DIR"), "/ggml_loader.rs"));
    }

    #[allow(warnings)]
    pub mod ggml_base {
        use super::types::*;
        include!(concat!(env!("OUT_DIR"), "/ggml_base_loader.rs"));
    }
}

pub use generated::types::*;

struct LoadedLibraries {
    path: PathBuf,
    llama: generated::llama::llama,
    ggml: generated::ggml::ggml,
    ggml_base: generated::ggml_base::ggml_base,
}

#[cfg(target_os = "windows")]
const LIB_NAMES: [&str; 3] = ["ggml-base.dll", "ggml.dll", "llama.dll"];

#[cfg(target_os = "linux")]
const LIB_NAMES: [&str; 3] = ["libggml-base.so", "libggml.so", "libllama.so"];

#[cfg(target_os = "macos")]
const LIB_NAMES: [&str; 3] = [
    "libggml-base.dylib",
    "libggml.dylib",
    "libllama.dylib",
];

static LIBRARIES: OnceLock<LoadedLibraries> = OnceLock::new();

pub fn initialize(runtime_dir: &Path) -> Result<()> {
    if !runtime_dir.exists() {
        bail!(
            "runtime directory `{}` does not exist",
            runtime_dir.display()
        );
    }

    let dir = runtime_dir
        .canonicalize()
        .with_context(|| format!("failed to canonicalize `{}`", runtime_dir.display()))?;

    if let Some(existing) = LIBRARIES.get() {
        if existing.path == dir {
            return Ok(());
        }
        bail!(
            "koharu-llm is already initialized with `{}` and cannot be reinitialized with `{}`",
            existing.path.display(),
            dir.display()
        );
    }

    let libraries = load_libraries(&dir)?;
    register_backends(&libraries.ggml, &dir)?;

    LIBRARIES
        .set(libraries)
        .map_err(|_| anyhow!("koharu-llm runtime libraries were initialized concurrently"))?;

    Ok(())
}

fn load_libraries(dir: &Path) -> Result<LoadedLibraries> {
    let [ggml_base_name, ggml_name, llama_name] = LIB_NAMES;

    let ggml_base = load_and_bind(&dir.join(ggml_base_name), ggml_base_name, |lib| unsafe {
        generated::ggml_base::ggml_base::from_library(lib)
    })?;
    let ggml = load_and_bind(&dir.join(ggml_name), ggml_name, |lib| unsafe {
        generated::ggml::ggml::from_library(lib)
    })?;
    let llama = load_and_bind(&dir.join(llama_name), llama_name, |lib| unsafe {
        generated::llama::llama::from_library(lib)
    })?;

    Ok(LoadedLibraries {
        path: dir.to_path_buf(),
        llama,
        ggml,
        ggml_base,
    })
}

fn load_and_bind<T>(
    path: &Path,
    name: &str,
    bind: impl FnOnce(libloading::Library) -> std::result::Result<T, libloading::Error>,
) -> Result<T> {
    // On Windows, DLL search paths are configured via add_runtime_search_path,
    // On Linux and macOS, we load libraries by path to ensure we get the correct ones.
    #[cfg(target_os = "windows")]
    let _ = path;
    #[cfg(target_os = "windows")]
    let library = unsafe { libloading::Library::new(name) }
        .with_context(|| format!("failed to load `{name}`"))?;
    #[cfg(not(target_os = "windows"))]
    let library = unsafe { libloading::Library::new(path) }
        .with_context(|| format!("failed to load `{name}` from `{}`", path.display()))?;
    bind(library).with_context(|| format!("failed to bind `{name}`"))
}

fn register_backends(ggml: &generated::ggml::ggml, dir: &Path) -> Result<()> {
    let dir = dir
        .to_str()
        .ok_or_else(|| anyhow!("runtime directory is not valid UTF-8"))?;
    let dir = CString::new(dir).context("runtime directory contains an interior null byte")?;

    unsafe {
        ggml.ggml_backend_load_all_from_path(dir.as_ptr());
    }

    Ok(())
}

fn libraries() -> &'static LoadedLibraries {
    LIBRARIES.get().expect(
        "koharu-llm runtime libraries are not initialized",
    )
}

fn llama_lib() -> &'static generated::llama::llama {
    &libraries().llama
}

fn ggml_lib() -> &'static generated::ggml::ggml {
    &libraries().ggml
}

fn ggml_base_lib() -> &'static generated::ggml_base::ggml_base {
    &libraries().ggml_base
}

#[allow(warnings)]
mod wrappers {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/wrappers.rs"));
}

pub use wrappers::*;
