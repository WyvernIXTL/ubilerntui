
def --wrapped "main build" [...args: string] {
    with-env {
        CARGO_PROFILE_DEV_CODEGEN_BACKEND: cranelift
        CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER: lld-link.exe
        RUSTFLAGS: "-Zthreads=8"
    } {
        cargo +nightly build -Zcodegen-backend ...$args
    }
}

def --wrapped "main b" [...args: string] {
    main build ...$args
}

def --wrapped "main run" [...args: string] {
    with-env {
        CARGO_PROFILE_DEV_CODEGEN_BACKEND: cranelift
        CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER: lld-link.exe
        RUSTFLAGS: "-Zthreads=8"
    } {
        cargo +nightly run -Zcodegen-backend ...$args
    }
}

def --wrapped "main r" [...args: string] {
    main run ...$args
}

def main [] {}
