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
pub fn add_hsv_to_buffer(buf: &mut Vec<u8>, h_add: u16, s_add: u8, v_add: u8) {
    for i in (0..buf.len()).step_by(3) {
        if i + 2 < buf.len() {
            let r = buf[i];
            let g = buf[i + 1];
            let b = buf[i + 2];
            
            let (mut h, mut s, mut v) = rgb_to_hsv(r, g, b);
            
            h = ((h as u32 + h_add as u32) % 360) as u16;
            s = (s as u16 + s_add as u16).min(100) as u8;
            v = (v as u16 + v_add as u16).min(100) as u8;
            
            let (r_new, g_new, b_new) = hsv_to_rgb(h, s, v);
            
            buf[i] = r_new;
            buf[i + 1] = g_new;
            buf[i + 2] = b_new;
        }
    }
}
