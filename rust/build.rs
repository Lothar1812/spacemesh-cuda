use cmake::Config;

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let profile = match profile.as_str() {
        "debug" => "Debug",
        "release" => "Release",
        _ => "Release",
    };
    let mut dst = Config::new("..")
        .define("CMAKE_BUILD_TYPE", profile)
        .define("WITH_TEST", "OFF")
        .cxxflag("/O2") // Anpassung für Windows-Compiler-Flag
        .build();
    dst.push("lib");
    println!("cargo:rustc-link-search=native={}", dst.display());

    let default_cuda_lib_path = "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.2\\lib\\x64";
    let default_boost_path = "C:\\local\\boost_1_75_0\\lib64-msvc-14.2";
    println!("cargo:rustc-link-search=native={}", default_cuda_lib_path);
    println!("cargo:rustc-link-search=native={}", default_boost_path);
    println!("cargo:rustc-link-lib=static=cudart_static");
    println!("cargo:rustc-link-lib=msvcrt"); // Anpassung für Windows: msvcrt statt stdc++
    println!("cargo:rustc-link-lib=static=spacemesh-cuda");
}
