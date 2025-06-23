#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.set("FileDescription", "Steam Workshop Utility");
    res.set("ProductName", "s7forge");
    res.set("CompanyName", "Burak Kartal");
    res.set("FileVersion", "1.0.0");
    res.set("ProductVersion", "1.0.0");
    res.compile().unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // Do nothing on non-Windows platforms
}
