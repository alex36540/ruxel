use image::{DynamicImage, ImageBuffer, Rgba};
use rfd::FileDialog;

pub struct FileInteractions {
    pub show_save_dialog: bool,
}

impl Default for FileInteractions {
    fn default() -> Self {
        Self::new()
    }
}

impl FileInteractions {
    pub fn new() -> FileInteractions {
        FileInteractions {
            show_save_dialog: false,
        }
    }

    pub fn save_file(&mut self, rgba_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let img = DynamicImage::from(rgba_buffer);
        let scaled_img = img.resize(
            img.width() * 10,
            img.height() * 10,
            image::imageops::FilterType::Nearest,
        );

        if let Some(mut path) = FileDialog::new().save_file() {
            path.set_extension("png");

            scaled_img
                .save(path)
                .unwrap();
        }
        self.show_save_dialog = false;
    }
}


