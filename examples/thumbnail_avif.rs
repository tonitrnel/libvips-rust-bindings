fn main() {
    let app = libvips::VipsApp::new("Thumbnail Avif", false).unwrap();
    app.concurrency_set(2);


    let dir = std::env::current_dir().unwrap();
    let src_path = dir.join("examples/resources/test2.jpg").display().to_string();
    let dst_path = dir.join("examples/resources/test2_output.thumbnail.avif").display().to_string();

    let image = libvips::VipsImage::new_from_file(&src_path).unwrap();
    let thumbnail = libvips::ops::thumbnail_image(&image, 500).unwrap();
    let options = libvips::ops::HeifsaveOptions {
        keep: libvips::ops::ForeignKeep::None,
        compression: libvips::ops::ForeignHeifCompression::Av1,
        ..libvips::ops::HeifsaveOptions::default()
    };

    libvips::ops::heifsave_with_opts(&thumbnail, &dst_path, &options).unwrap_or_else(|_| panic!("error: {}", app.error_buffer().unwrap_or_default()));

    println!("width: {} > {}", image.get_width(), thumbnail.get_width());
    println!("height: {} > {}", image.get_height(), thumbnail.get_height());
    println!("dst path: {dst_path:?}");
}