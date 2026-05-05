# List of errors and how to fix them

1. 
```
PS D:\Development\nginx-infra-tutorial> wasm-pack build --target web
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
   Compiling wasm-bindgen-shared v0.2.120
   Compiling proc-macro2 v1.0.106
   Compiling quote v1.0.45
   Compiling rustversion v1.0.22
error: linking with `C:\Users\User\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin\rust-lld.exe` failed: exit code: 1
  |
  = note: "C:\\Users\\User\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\bin\\rust-lld.exe" "-flavor" "link" "/NOLOGO" "D:\\Development\\nginx-infra-tutorial\\target\\release\\build\\quote-01f9bb60f00190e6\\rustcKKcKvW\\symbols.o" "<2 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_link-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" "ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\Development\\nginx-infra-tutorial\\target\\release\\build\\quote-01f9bb60f00190e6\\build_script_build-01f9bb60f00190e6.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'ntdll.lib': no such file or directory
          rust-lld: error: could not open 'userenv.lib': no such file or directory
          rust-lld: error: could not open 'ws2_32.lib': no such file or directory
          rust-lld: error: could not open 'dbghelp.lib': no such file or directory
          

    Building
error: linking with `C:\Users\User\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin\rust-lld.exe` failed: exit code: 1
  |
  = note: "C:\\Users\\User\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\bin\\rust-lld.exe" "-flavor" "link" "/NOLOGO" "D:\\Development\\nginx-infra-tutorial\\target\\release\\build\\wasm-bindgen-shared-994e71948956b2b3\\rustcT8m1LB\\symbols.o" "<3 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_link-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" "ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\Development\\nginx-infra-tutorial\\target\\release\\build\\wasm-bindgen-shared-994e71948956b2b3\\build_script_build-994e71948956b2b3.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'ntdll.lib': no such file or directory
          rust-lld: error: could not open 'userenv.lib': no such file or directory
          rust-lld: error: could not open 'ws2_32.lib': no such file or directory
          rust-lld: error: could not open 'dbghelp.lib': no such file or directory
          


error: could not compile `quote` (build script) due to 1 previous error    
warning: build failed, waiting for other jobs to finish...
error: could not compile `wasm-bindgen-shared` (build script) due to 1 previous error
error: linking with `C:\Users\User\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin\rust-lld.exe` failed: exit code: 1
  |
  = note: "C:\\Users\\User\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\bin\\rust-lld.exe" "-flavor" "link" "/NOLOGO" "D:\\Development\\nginx-infra-tutorial\\target\\release\\build\\proc-macro2-7e9c9be0ebdfc812\\rustcgzKr7G\\symbols.o" "<2 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_link-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" "ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\Development\\nginx-infra-tutorial\\target\\release\\build\\proc-macro2-7e9c9be0ebdfc812\\build_script_build-7e9c9be0ebdfc812.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'ntdll.lib': no such file or directory
          rust-lld: error: could not open 'userenv.lib': no such file or directory
          rust-lld: error: could not open 'ws2_32.lib': no such file or directory
          rust-lld: error: could not open 'dbghelp.lib': no such file or directory
          

    Building
error: linking with `C:\Users\User\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin\rust-lld.exe` failed: exit code: 1
  |
  = note: "C:\\Users\\User\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\bin\\rust-lld.exe" "-flavor" "link" "/NOLOGO" "D:\\Development\\nginx-infra-tutorial\\target\\release\\build\\rustversion-ec557edddccc3121\\rustcZdDoHd\\symbols.o" "<3 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_link-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" "ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\Development\\nginx-infra-tutorial\\target\\release\\build\\rustversion-ec557edddccc3121\\build_script_build-ec557edddccc3121.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'kernel32.lib': no such file or directory
          rust-lld: error: could not open 'ntdll.lib': no such file or directory
          rust-lld: error: could not open 'userenv.lib': no such file or directory
          rust-lld: error: could not open 'ws2_32.lib': no such file or directory
          rust-lld: error: could not open 'dbghelp.lib': no such file or directory
          

    Building
error: could not compile `proc-macro2` (build script) due to 1 previous error
error: could not compile `rustversion` (build script) due to 1 previous error
Error: `cargo build` failed, see the output above for details
Caused by: `cargo build` failed, see the output above for details
PS D:\Development\nginx-infra-tutorial> 
```

Solution:

    Download Visual Studio Build Tools:
    Go to https://visualstudio.microsoft.com/downloads/
    Scroll down and click "Download" under "Tools for Visual Studio"
    Select Visual Studio Build Tools
    Run the installer and select:

    ✅ "Desktop development with C++"
    In the right panel, make sure "Windows 10/11 SDK" is checked
    Complete the installation and restart if prompted

    After installation, try building again:

    This should resolve the linker errors about missing kernel32.lib, ntdll.lib, etc.