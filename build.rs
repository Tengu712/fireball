#[cfg(target_os = "windows")]
fn main() {
    println!(r"cargo:rustc-link-search=C:\VulkanSDK\1.3.216.0\Lib\");
}
