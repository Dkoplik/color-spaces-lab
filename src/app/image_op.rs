/// Первый вариант преобразования rbg в оттенки серого.
/// r - красная составляющая;
/// g - зелёная составляющая;
/// b - синяя составляющая;
/// 
/// Результат тоже в rgb формате, но должнен уже представлять собой оттенок серого.
fn rgb_to_grayscale1(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r_f32 = r as f32;
    let g_f32 = g as f32;
    let b_f32 = b as f32;
    
    let y = (0.299 * r_f32 + 0.587 * g_f32 + 0.114 * b_f32) as u8;
    (y, y, y)
}

/// Второй вариант преобразования rbg в оттенки серого.
/// r - красная составляющая;
/// g - зелёная составляющая;
/// b - синяя составляющая;
/// 
/// Результат тоже в rgb формате, но должнен уже представлять собой оттенок серого.
fn rgb_to_grayscale2(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r_f32 = r as f32;
    let g_f32 = g as f32;
    let b_f32 = b as f32;

    let y = (0.2126 * r_f32 + 0.7152 * g_f32 + 0.0722 * b_f32) as u8;
    (y, y, y)
}

/// Первый вариант преобразования изображения в оттенки серого.
/// buf - тупо вектор с цифрами от 0 до 255, соответственно, каждая тройка чисел это один пиксель.
/// 
/// Результат должен быть записан в сам же buf.
pub fn rgb_buffer_to_grayscale1(buf: &mut Vec<u8>) {
    for i in (0..buf.len()).step_by(3) {
        if i + 2 < buf.len() {
            let (r, g, b) = (buf[i], buf[i + 1], buf[i + 2]);
            let (gray, _, _) = rgb_to_grayscale1(r, g, b);
            buf[i] = gray;
            buf[i + 1] = gray;
            buf[i + 2] = gray;
        }
    }
}

/// Второй вариант преобразования изображения в оттенки серого.
/// buf - тупо вектор с цифрами от 0 до 255, соответственно, каждая тройка чисел это один пиксель.
/// 
/// Результат должен быть записан в сам же buf.
pub fn rgb_buffer_to_grayscale2(buf: &mut Vec<u8>) {
    for i in (0..buf.len()).step_by(3) {
        if i + 2 < buf.len() {
            let (r, g, b) = (buf[i], buf[i + 1], buf[i + 2]);
            let (gray, _, _) = rgb_to_grayscale2(r, g, b);
            buf[i] = gray;
            buf[i + 1] = gray;
            buf[i + 2] = gray;
        }
    }
}

/// Функция для вычисления разности двух изображений
pub fn compute_difference(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(buf1.len());    
    for i in 0..buf1.len() {
        let diff = (buf1[i] as i16 - buf2[i] as i16).abs() as u8;
        res.push(diff);
    }
    res
}

pub fn rgb_buffer_to_red_channel(buf: &mut Vec<u8>) {
    for i in (0..buf.len()).step_by(3) {
        if i + 2 < buf.len() {
            buf[i + 1] = 0;
            buf[i + 2] = 0;
        }
    }
}

pub fn rgb_buffer_to_green_channel(buf: &mut Vec<u8>) {
    for i in (0..buf.len()).step_by(3) {
        if i + 2 < buf.len() {
            buf[i] = 0;
            buf[i + 2] = 0;
        }
    }
}

pub fn rgb_buffer_to_blue_channel(buf: &mut Vec<u8>) {
    for i in (0..buf.len()).step_by(3) {
        if i + 2 < buf.len() {
            buf[i] = 0;
            buf[i + 1] = 0;
        }
    }
}

/// Преобразование представления цвета из RGB в HSV.
/// r - красная составляющая [0, 255];
/// g - зелёная составляющая [0, 255];
/// b - синяя составляющая [0, 255];
/// 
/// На выходе должен быть HSV: H [0, 360], S [0, 100], V [0, 100].
fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (u16, u8, u8) {
    // TODO
    return (0, 0, 0);
}

/// Преобразование представления цвета из HSV в RGB.
/// h - hue [0, 360];
/// s - saturation [0, 100];
/// v - value (brightness) [0, 100];
/// 
/// На выходе должен быть RGB со значениями от 0 до 255.
fn hsv_to_rgb(h: u16, s: u8, v: u8) -> (u8, u8, u8) {
    // TODO
    return (0, 0, 0);
}

/// Прибавляет ко всей картинке buf указанное значение HSV.
/// buf - тупо вектор с цифрами от 0 до 255, соответственно, каждая тройка чисел это один пиксель.
/// h - hue [0, 360];
/// s - saturation [0, 100];
/// v - value (brightness) [0, 100];
/// 
/// Результат должен быть записан в сам же buf в rgb формате.
pub fn add_hsv_to_buffer(buf: &mut Vec<u8>, h: u16, s: u8, v: u8) {
    // TODO из буфера можно брать по одному пикселю, приводить к hsv, суммировать, возвращать обратно в rgb.
}