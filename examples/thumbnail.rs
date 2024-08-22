fn main() {
    let app = libvips::VipsApp::new("Thumbnail", true).unwrap();
    app.concurrency_set(2);
    app.cache_set_max(1024 * 1024 * 8);
    app.cache_set_max_mem(1024 * 1024 * 8);
    let start = std::time::Instant::now();
    let dir = std::env::current_dir().unwrap();
    let src_path = dir.join("examples/resources/test2.jpg").display().to_string();
    let dst_path = dir.join("examples/resources/test2_output.thumbnail.jpg").display().to_string();

    let image = libvips::VipsImage::new_from_file(&src_path).unwrap();
    let thumbnail = libvips::ops::thumbnail_image(&image, 500).unwrap();

    libvips::ops::vipssave(&thumbnail, &dst_path).unwrap_or_else(|_| panic!("error: {}", app.error_buffer().unwrap_or_default()));
    println!("memory: {}, elapsed: {}ms", format_bytes(load_memory_usage(std::process::id())), start.elapsed().as_millis())
    // println!("width: {} > {}", image.get_width(), thumbnail.get_width());
    // println!("height: {} > {}", image.get_height(), thumbnail.get_height());
    // println!("dst path: {dst_path:?}");
}

fn load_memory_usage(pid: u32) -> u64 {
    // read process working set memory
    let output = std::process::Command::new("powershell")
        .arg("Get-Process -Pid")
        .arg(pid.to_string())
        .arg("|")
        .arg("Select-Object -ExpandProperty WS")
        .output().unwrap();
    if !output.status.success() {
        panic!("Failed to exec Get-Process command, reason: {}",
               String::from_utf8(output.stderr).unwrap_or_default())
    }
    let value = String::from_utf8(output.stdout)
        .map(|it| it.trim().parse::<u64>()).unwrap().unwrap();
    value
}
fn format_bytes(bytes: u64) -> String {
    if bytes == 0 { return String::from("0 B"); }
    let units = ["B", "KB", "MB", "GB", "TB"];
    let base: u32 = 1024;
    let digit_groups = bytes.ilog(2) / base.ilog(2);
    format!("{:.2} {}", (bytes as f64) / (base.pow(digit_groups) as f64), units[digit_groups as usize])
}