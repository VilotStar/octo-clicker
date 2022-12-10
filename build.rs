fn main() {
    // // let dir = r"D:\Repos\Autoclicker\bug-free-octo-memory";//std::env::current_dir().unwrap();
    println!("cargo:rustc-link-lib=dylib=FakerInputDll");
    println!(r"cargo:rustc-link-search=native=D:\Repos\Autoclicker\bug-free-octo-memory");
}