use eframe::egui;
use image::RgbImage;

pub mod image_op;

#[derive(Default)]
enum Task {
    #[default]
    Grayscale,
    RGBChannels,
    HSV,
}

#[derive(Default)]
pub struct ColorsApp {
    loaded_image: Option<image::RgbImage>,
    cur_image: Option<Vec<u8>>,
    cur_image_size: Option<(usize, usize)>,
    cur_image_texture: Option<egui::TextureHandle>,
    image_path: Option<std::path::PathBuf>,
    task: Task,
}

impl ColorsApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Light);
        Self::default()
    }

    /// Загрузить файл с картинкой из файловой системы
    fn load_image(&mut self, ctx: &egui::Context) {
        let path = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "tga", "tiff"])
            .pick_file();

        if let Some(path) = path {
            if let Ok(img) = image::open(&path) {
                self.loaded_image = Some(img.to_rgb8());
                self.cur_image = Some(img.to_rgb8().into_raw());
                self.cur_image_size = Some((img.width() as usize, img.height() as usize));
                self.image_path = Some(path);
                self.update_texture(ctx);
            }
        }
    }

    /// Обновить выводимую картинку
    fn update_texture(&mut self, ctx: &egui::Context) {
        if let Some(raw_data) = &self.cur_image {
            let size = [
                self.cur_image_size.unwrap().0,
                self.cur_image_size.unwrap().1,
            ];
            let texture = ctx.load_texture(
                "cur_image",
                egui::ColorImage::from_rgb(size, raw_data),
                Default::default(),
            );
            self.cur_image_texture = Some(texture);
        }
    }

    /// Сохранить текущую выводимую картинку в файл
    fn save_image(&mut self) {
        if self.cur_image == None {
            return;
        }

        // File dialog
        let mut dialog = rfd::FileDialog::new()
            .add_filter("PNG Image", &["png"])
            .add_filter("JPEG Image", &["jpg", "jpeg"])
            .add_filter("BMP Image", &["bmp"])
            .add_filter("All Files", &["*"]);

        // Set default path
        if let Some(path) = &self.image_path {
            if let Some(parent) = path.parent() {
                dialog = dialog.set_directory(parent);
            }
            if let Some(file_name) = path.file_name() {
                dialog = dialog.set_file_name(file_name.to_string_lossy().as_ref());
            }
        }

        // Show save dialog
        let size = self.cur_image_size.unwrap();
        let buf = self.cur_image.clone().unwrap();
        let image = RgbImage::from_raw(size.0 as u32, size.1 as u32, buf).unwrap();
        if let Some(new_path) = dialog.save_file() {
            match image.save(&new_path) {
                Ok(_) => {
                    self.image_path = Some(new_path);
                }
                Err(e) => {
                    eprintln!("Failed to save image: {}", e);
                }
            }
        }
    }

    /// UI левой панели для задания с оттенками серого
    fn left_buttons_grayscale(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Вывести оригинальную картинку
            if ui.button("Original").clicked() {
                if let Some(orig_image) = &self.loaded_image {
                    self.cur_image = Some(orig_image.clone().into_raw());
                    self.update_texture(ctx);
                }
            }

            // Оттенки серого 1-ым методом
            if ui.button("Grayscale1").clicked() {
                if let Some(orig_image) = &self.loaded_image {
                    let mut buf = orig_image.clone().into_raw();
                    image_op::rgb_buffer_to_grayscale1(&mut buf);
                    self.cur_image = Some(buf);
                    self.update_texture(ctx);
                }
            }

            // Оттенки серого 2-ым методом
            if ui.button("Grayscale2").clicked() {
                if let Some(orig_image) = &self.loaded_image {
                    let mut buf = orig_image.clone().into_raw();
                    image_op::rgb_buffer_to_grayscale2(&mut buf);
                    self.cur_image = Some(buf);
                    self.update_texture(ctx);
                }
            }
        });
    }

    /// UI левой панели для задания с каналами RGB
    fn left_buttons_rgb_channels(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Вывести оригинальную картинку
            if ui.button("Original").clicked() {
                if let Some(orig_image) = &self.loaded_image {
                    self.cur_image = Some(orig_image.clone().into_raw());
                    self.update_texture(ctx);
                }
            }

            // Вывести красный канал
            if ui.button("Red channel").clicked() {
                if let Some(orig_image) = &self.loaded_image {
                    let mut buf = orig_image.clone().into_raw();
                    image_op::rgb_buffer_to_red_channel(&mut buf);
                    self.cur_image = Some(buf);
                    self.update_texture(ctx);
                }
            }

            // Вывести зелёный канал
            if ui.button("Green channel").clicked() {
                if let Some(orig_image) = &self.loaded_image {
                    let mut buf = orig_image.clone().into_raw();
                    image_op::rgb_buffer_to_green_channel(&mut buf);
                    self.cur_image = Some(buf);
                    self.update_texture(ctx);
                }
            }

            // Вывести синий канал
            if ui.button("Blue channel").clicked() {
                if let Some(orig_image) = &self.loaded_image {
                    let mut buf = orig_image.clone().into_raw();
                    image_op::rgb_buffer_to_blue_channel(&mut buf);
                    self.cur_image = Some(buf);
                    self.update_texture(ctx);
                }
            }
        });
    }
}

impl eframe::App for ColorsApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Top menu buttons
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::default().ui(ui, |ui| {
                // File dialog
                ui.menu_button("File", |ui| {
                    // Load Image
                    if ui.button("Load Image").clicked() {
                        self.load_image(ctx);
                    }

                    // Save Image
                    if ui.button("Save Image").clicked() {
                        self.save_image();
                    }

                    // Close app
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                // Task dialog
                ui.menu_button("Task", |ui| {
                    // Grayscale
                    if ui.button("Grayscale").clicked() {
                        self.task = Task::Grayscale;
                    }

                    // RGB channels
                    if ui.button("RGB Channels").clicked() {
                        self.task = Task::RGBChannels;
                    }

                    // HSV
                    if ui.button("HSV").clicked() {
                        self.task = Task::HSV;
                    }
                });
            });
        });

        // Image view
        egui::CentralPanel::default().show(ctx, |ui| {
            // Side buttons
            egui::SidePanel::left("left_panel")
                .resizable(false)
                .show_inside(ui, |ui| match self.task {
                    Task::Grayscale => self.left_buttons_grayscale(ctx, ui),
                    Task::RGBChannels => self.left_buttons_rgb_channels(ctx, ui),
                    Task::HSV => panic!("Надо сделать левую панель для HSV"), // TODO
                });

            // Image display
            if let Some(texture) = &self.cur_image_texture {
                ui.add(egui::Image::new(texture));
            } else {
                ui.label("Необходимо загрузить картинку.");
            }
        });
    }
}
