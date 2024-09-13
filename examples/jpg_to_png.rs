extern crate libvips;

fn main() {
    let app = libvips::VipsApp::new("JPEG to PNG", false).unwrap();
    app.concurrency_set(2);

    let dir = std::env::current_dir().unwrap();

    let src_path = dir.join("examples/resources/test.jpg").display().to_string();
    let dst_path = dir.join("examples/resources/test_output.png").display().to_string();

    let image = libvips::VipsImage::new_from_file(&src_path).unwrap();

    let resized = libvips::ops::resize(&image, 0.5).unwrap();
    let options = libvips::ops::PngsaveOptions {
        q: 90,
        background: vec![255.0],
        ..libvips::ops::PngsaveOptions::default()
    };
    libvips::ops::pngsave_with_opts(&resized, &dst_path, &options).unwrap_or_else(|_| panic!("error: {}", app.error_buffer().unwrap_or_default()));
    println!("width: {} > {}", image.get_width(), resized.get_width());
    println!("height: {} > {}", image.get_height(), resized.get_height());
    println!("dst path: {dst_path:?}");
}