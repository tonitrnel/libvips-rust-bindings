// (c) Copyright 2019-2024 OLX
fn main() {
    #[cfg(target_os = "windows")]
    {
        use std::path::PathBuf;
        // Get the binaries for Windows from the link below.
        // https://github.com/libvips/build-win64-mxe/releases/
        //
        // Use Windows binaries with the suffix `-ffi.zip` .
        //
        let path_var = std::env::var("PATH").unwrap_or_default();
        let path_entries = path_var.split(';')
            .collect::<Vec<&str>>();
        let dll_path = match path_entries.iter().find(|it| it.contains("libvips\\bin")) {
            #[cfg(target_env = "gnu")]
            Some(dll_path) => {
                PathBuf::from(dll_path)
            }
            #[cfg(target_env = "msvc")]
            Some(dll_path) => {
                PathBuf::from(dll_path).parent().unwrap().join("lib")
            }
            None => {
                panic!(
                    "`PATH` environment variable does not include the required directory.\n\
        \nhelp: It appears that the necessary binaries for `vips` are missing from your system.\n\
        To resolve this issue:\n\
        1. Download the Windows binaries with the suffix `-ffi.zip` from the official repository:\n\
           https://github.com/libvips/build-win64-mxe/releases/\n\
        2. Extract the downloaded files.\n\
        3. Locate the `bin` directory within the extracted files.\n\
        4. Add this `bin` directory to your `PATH` environment variable.\n\
        \nAfter completing these steps, try running your program again."
                );
            }
        };
        if !dll_path.exists() {
            panic!("The specified directory {dll_path:?} does not exist or could not be found.")
        }
        #[cfg(target_env = "msvc")]
        {
            println!("cargo:rustc-link-search={}", dll_path.display());
            println!("cargo:rustc-link-lib=dylib=libvips");
            println!("cargo:rustc-link-lib=dylib=libglib-2.0");
            println!("cargo:rustc-link-lib=dylib=libgobject-2.0");
        }
        #[cfg(target_env = "gnu")]
        {
            println!("cargo:rustc-link-search={}", dll_path.display());
            println!("cargo:rustc-link-lib=dylib=vips-42");
            println!("cargo:rustc-link-lib=dylib=glib-2.0-0");
            println!("cargo:rustc-link-lib=dylib=gobject-2.0-0");
        }
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=vips");
        println!("cargo:rustc-link-lib=glib-2.0");
        println!("cargo:rustc-link-lib=gobject-2.0");
    }
}