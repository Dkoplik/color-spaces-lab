use egui::*;
use egui_plot::{Bar, BarChart, Plot};

pub struct RGBHistogram {
    data: Vec<u8>, // RGB buffer
    red_bins: Vec<u32>,
    green_bins: Vec<u32>,
    blue_bins: Vec<u32>,
    max_count: u32,
    needs_update: bool,
}

impl Default for RGBHistogram {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            red_bins: vec![0; 256],
            green_bins: vec![0; 256],
            blue_bins: vec![0; 256],
            max_count: 0,
            needs_update: true,
        }
    }
}

impl RGBHistogram {
    pub fn new() -> Self {
        Self::default()
    }

    /// Обновить гистограмму под новую картинку
    pub fn update_data(&mut self, data: Vec<u8>) {
        if data.len() % 3 != 0 {
            panic!("Буфер картинки не кратен 3");
        }
        self.data = data;
        self.needs_update = true;
    }

    /// Очистить гистограмму
    pub fn clear(&mut self) {
        self.data.clear();
        self.red_bins.iter_mut().for_each(|x| *x = 0);
        self.green_bins.iter_mut().for_each(|x| *x = 0);
        self.blue_bins.iter_mut().for_each(|x| *x = 0);
        self.max_count = 0;
        self.needs_update = false;
    }

    /// Просмотреть буфер картинки для построения гистограммы
    fn calculate_histogram(&mut self) {
        // сбросить значения
        self.red_bins.iter_mut().for_each(|x| *x = 0);
        self.green_bins.iter_mut().for_each(|x| *x = 0);
        self.blue_bins.iter_mut().for_each(|x| *x = 0);
        self.max_count = 0;

        // обработать RGX буфер
        for chunk in self.data.chunks_exact(3) {
            let r = chunk[0] as usize;
            let g = chunk[1] as usize;
            let b = chunk[2] as usize;

            self.red_bins[r] += 1;
            self.green_bins[g] += 1;
            self.blue_bins[b] += 1;
        }

        // найти значение с наибольшим количеством
        self.max_count = self
            .red_bins
            .iter()
            .chain(self.green_bins.iter())
            .chain(self.blue_bins.iter())
            .max()
            .copied()
            .unwrap_or(1);

        self.needs_update = false;
    }

    /// Нарисовать гистограмму в указаном UI элементе
    pub fn show(&mut self, ui: &mut Ui, desired_size: Option<Vec2>) -> Response {
        if self.needs_update {
            self.calculate_histogram();
        }

        let size = desired_size.unwrap_or(vec2(400.0, 200.0));

        // Create plot area
        let plot = Plot::new("rgb_histogram")
            .view_aspect(2.0)
            .include_x(-0.5)
            .include_x(255.5)
            .include_y(0.0)
            .show_axes([false, false])
            .show_grid([false, false])
            .height(size.y)
            .width(size.x);

        plot.show(ui, |plot_ui| {
            self.draw_separate_bars(plot_ui);
        })
        .response
    }

    /// Нарисовать линии гистограммы для каждого канала
    fn draw_separate_bars(&self, plot_ui: &mut egui_plot::PlotUi) {
        let bar_width = 1.0 / 3.0; // Each bar takes 1/3 of the unit space

        // Create bars for each channel
        let red_bars = self.create_channel_bars(bar_width, 0.0, Color32::RED, &self.red_bins);
        let green_bars =
            self.create_channel_bars(bar_width, 1.0 / 3.0, Color32::GREEN, &self.green_bins);
        let blue_bars =
            self.create_channel_bars(bar_width, 2.0 / 3.0, Color32::BLUE, &self.blue_bins);

        // Draw each channel separately
        if !red_bars.is_empty() {
            let red_chart = BarChart::new("Red Channel", red_bars).color(Color32::RED);
            plot_ui.bar_chart(red_chart);
        }

        if !green_bars.is_empty() {
            let green_chart = BarChart::new("Green Channel", green_bars).color(Color32::GREEN);
            plot_ui.bar_chart(green_chart);
        }

        if !blue_bars.is_empty() {
            let blue_chart = BarChart::new("Blue Channel", blue_bars).color(Color32::BLUE);
            plot_ui.bar_chart(blue_chart);
        }
    }

    /// Обработать гистограмму для отдельного канала
    fn create_channel_bars(
        &self,
        bar_width: f64,
        offset: f64,
        color: Color32,
        bins: &[u32],
    ) -> Vec<Bar> {
        bins.iter()
            .enumerate()
            .map(|(intensity_value, &count)| {
                // Position the bar: intensity_value + offset within the unit
                let x_position = intensity_value as f64 + offset;
                Bar::new(x_position, count as f64)
                    .width(bar_width)
                    .fill(color)
                    .stroke(Stroke::new(0.5, color))
                    .name(format!("{}: {}", channel_name(color), intensity_value))
            })
            .collect()
    }

    /// Данные о гистограмме
    pub fn statistics(&self) -> HistogramStats {
        HistogramStats {
            total_pixels: self.data.len() / 3,
            red_max: self.red_bins.iter().max().copied().unwrap_or(0),
            green_max: self.green_bins.iter().max().copied().unwrap_or(0),
            blue_max: self.blue_bins.iter().max().copied().unwrap_or(0),
            red_mean: self.calculate_mean(&self.red_bins),
            green_mean: self.calculate_mean(&self.green_bins),
            blue_mean: self.calculate_mean(&self.blue_bins),
        }
    }

    fn calculate_mean(&self, bins: &[u32]) -> f64 {
        let sum: u64 = bins
            .iter()
            .enumerate()
            .map(|(value, &count)| (value as u64) * (count as u64))
            .sum();
        let total: u64 = bins.iter().map(|&x| x as u64).sum();

        if total > 0 {
            sum as f64 / total as f64
        } else {
            0.0
        }
    }
}

/// Данные о гистограмме
#[derive(Debug, Clone)]
pub struct HistogramStats {
    pub total_pixels: usize,
    pub red_max: u32,
    pub green_max: u32,
    pub blue_max: u32,
    pub red_mean: f64,
    pub green_mean: f64,
    pub blue_mean: f64,
}

/// Название канала
fn channel_name(color: Color32) -> &'static str {
    if color == Color32::RED {
        "Red"
    } else if color == Color32::GREEN {
        "Green"
    } else if color == Color32::BLUE {
        "Blue"
    } else {
        "Unknown"
    }
}
