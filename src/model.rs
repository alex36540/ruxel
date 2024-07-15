use crate::change_manager::CanvasState;
use egui::{epaint::RectShape, *};
use image::{ImageBuffer, Rgba};
use rayon::prelude::*;

#[derive(Default)]
pub struct Camera {
    square_size: f32,
    pixel_center: Pos2,
    screen_center: Pos2,
}

impl Camera {
    fn screen_cords_to_pixel_cords(&self, screen_cords: &Pos2) -> (isize, isize) {
        (
            (((screen_cords.x - self.screen_center.x) / self.square_size).floor()
                + self.pixel_center.x.round()) as isize,
            (((screen_cords.y - self.screen_center.y) / self.square_size).floor()
                + self.pixel_center.y.round()) as isize,
        )
    }

    fn pixel_cords_to_screen_cords(&self, w: isize, h: isize) -> Pos2 {
        Pos2::new(
            (w as f32 - self.pixel_center.x.round()) * self.square_size + self.screen_center.x,
            (h as f32 - self.pixel_center.y.round()) * self.square_size + self.screen_center.y,
        )
    }

    fn square_from_pixel_cords(&self, w: isize, h: isize, color: Color32) -> Shape {
        self.square_from_screen_cords(self.pixel_cords_to_screen_cords(w, h), color)
    }

    fn square_from_screen_cords(&self, screen_cords: Pos2, color: Color32) -> Shape {
        Shape::from(RectShape::new(
            Rect::from_min_size(
                screen_cords,
                Vec2 {
                    x: self.square_size,
                    y: self.square_size,
                },
            ),
            Rounding::default(),
            color,
            Stroke::NONE,
        ))
    }
}

pub struct Canvas {
    width: usize,
    height: usize,
    layers: usize,
    active_layer: usize,
    layer_names: Vec<String>,
    layer_name_cnt: usize,
    layers_to_show: Vec<bool>,
    alpha_ratio: usize,
    pixels: Vec<Color32>,
    squares: Vec<Shape>,
    camera: Camera,
    stroke: Stroke,
}

pub const DEFAULT_SIZE: usize = 32;
pub const SCROLL_SENSITIVITY: f32 = 0.8;
const HIGHLIGHT_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 127);

impl Default for Canvas {
    fn default() -> Self {
        Self {
            width: DEFAULT_SIZE,
            height: DEFAULT_SIZE,
            layers: 1,
            active_layer: 0,
            layer_names: vec![String::from("0"); 1],
            layer_name_cnt: 1,
            layers_to_show: vec![true; 1],
            alpha_ratio: 8,
            pixels: vec![Color32::TRANSPARENT; DEFAULT_SIZE * DEFAULT_SIZE],
            squares: vec![
                Shape::from(RectShape::new(
                    Rect::from_min_size(Pos2::default(), Vec2::default()),
                    Rounding::default(),
                    Color32::TRANSPARENT,
                    Stroke::NONE,
                ));
                DEFAULT_SIZE * DEFAULT_SIZE * 2
            ],
            camera: Camera {
                square_size: 10.0,
                pixel_center: Pos2 {
                    x: (DEFAULT_SIZE / 2) as f32,
                    y: (DEFAULT_SIZE / 2) as f32,
                },
                screen_center: Pos2::default(),
            },
            stroke: Stroke::NONE,
        }
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            layers: 1,
            active_layer: 0,
            layer_names: vec![String::from("0"); 1],
            layer_name_cnt: 1,
            layers_to_show: vec![true; 1],
            alpha_ratio: 8,
            pixels: vec![Color32::TRANSPARENT; width * height],
            squares: vec![
                Shape::from(RectShape::new(
                    Rect::from_min_size(Pos2::default(), Vec2::default()),
                    Rounding::default(),
                    Color32::TRANSPARENT,
                    Stroke::NONE,
                ));
                width * height * 2
            ],
            camera: Camera {
                square_size: 10.0,
                pixel_center: Pos2 {
                    x: (width / 2) as f32,
                    y: (height / 2) as f32,
                },
                screen_center: Pos2::default(),
            },
            stroke: Stroke::NONE,
        }
    }

    #[allow(dead_code)]
    fn get_layer(&self, layer_idx: usize) -> Option<Vec<Color32>> {
        if layer_idx >= self.layers {
            let layer_size = self.width * self.height;
            Some(self.pixels[(layer_size * layer_idx)..(layer_size * (layer_idx + 1))].to_vec())
        } else {
            None
        }
    }

    fn get_pixel(&self, x: usize, y: usize, layer_idx: usize) -> Option<&Color32> {
        if layer_idx < self.layers && x < self.width && y < self.height {
            let layer_size = self.width * self.height;
            self.pixels
                .get(x + (y * self.width) + (layer_size * layer_idx))
        } else {
            None
        }
    }

    fn get_pixel_mut(&mut self, x: usize, y: usize, layer_idx: usize) -> Option<&mut Color32> {
        if layer_idx < self.layers && x < self.width && y < self.height {
            let layer_size = self.width * self.height;
            self.pixels
                .get_mut(x + (y * self.width) + (layer_size * layer_idx))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn get_square(&self, x: usize, y: usize, layer_idx: usize) -> Option<&Shape> {
        if layer_idx < self.layers && x < self.width && y < self.height {
            let layer_size = self.width * self.height;
            self.squares
                .get(x + (y * self.width) + (layer_size * (layer_idx + 1)))
        } else {
            None
        }
    }

    fn get_square_mut(&mut self, x: usize, y: usize, layer_idx: usize) -> Option<&mut Shape> {
        if layer_idx < self.layers && x < self.width && y < self.height {
            let layer_size = self.width * self.height;
            self.squares
                .get_mut(x + (y * self.width) + (layer_size * (layer_idx + 1)))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn get_alpha_square(&self, x: usize, y: usize) -> Option<&Shape> {
        if x < self.width && y < self.height {
            self.squares.get(x + (y * self.width))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn get_alpha_square_mut(&mut self, x: usize, y: usize) -> Option<&mut Shape> {
        if x < self.width && y < self.height {
            self.squares.get_mut(x + (y * self.width))
        } else {
            None
        }
    }

    pub fn get_squares(&self) -> Vec<Shape> {
        self.squares.clone()
    }

    pub fn get_squares_unhidden(&self) -> Vec<Shape> {
        let layer_size = self.width * self.height;
        let mut squares_unhidden: Vec<Shape> = self.squares.clone();
        let mut start_idx = layer_size;
        let mut end_idx = layer_size * 2;

        for i in 0..self.layers {
            if self.layers_to_show.get(i) == Some(&false) {
                squares_unhidden.drain(start_idx..end_idx);
                continue;
            }

            start_idx += layer_size;
            end_idx += layer_size;
        }

        squares_unhidden
    }

    /// Offers Performance Benefit :)
    fn get_pixel_unchecked(&self, x: usize, y: usize, layer_idx: usize) -> &Color32 {
        &self.pixels[x + (y * self.width) + (self.width * self.height * layer_idx)]
    }

    pub fn change_stroke(&mut self, new_stroke: Stroke) {
        self.stroke = new_stroke;
    }

    pub fn new_stroke_color(&mut self, color: Color32) {
        self.stroke.color = color;
    }

    pub fn set_pixel_from_screen_cords(
        &mut self,
        screen_cords: &Pos2,
        color: Color32,
    ) -> Result<(), String> {
        let (x, y) = self.camera.screen_cords_to_pixel_cords(screen_cords);
        if x.is_negative() || y.is_negative() {
            return Err("Not on canvas".into());
        }
        let (x, y) = (x as usize, y as usize);
        if let Some(pixel) = self.get_pixel_mut(x, y, self.active_layer) {
            *pixel = color;
        } else {
            return Err("Failed to get pixel".into());
        }
        if let Some(Shape::Rect(RectShape { fill, .. })) =
            self.get_square_mut(x, y, self.active_layer)
        {
            *fill = color;
            Ok(())
        } else {
            Err("Failed to get rectangle".into())
        }
    }

    pub fn set_pixel_from_pixel_coords(
        &mut self,
        pixel_coords: (usize, usize),
        color: Color32,
    ) -> Result<(), String> {
        let (x, y) = pixel_coords;

        if let Some(pixel) = self.get_pixel_mut(x, y, self.active_layer) {
            *pixel = color;
        } else {
            return Err("Failed to get pixel".into());
        }
        if let Some(Shape::Rect(RectShape { fill, .. })) =
            self.get_square_mut(x, y, self.active_layer)
        {
            *fill = color;
            Ok(())
        } else {
            Err("Failed to get rectangle".into())
        }
    }

    pub fn set_pixels_from_brush(&mut self, screen_cords: &Pos2, radius: usize, color: Color32) {
        let radius = if radius < 1 { 1 } else { radius as isize };

        let (center_w, center_h) = self.camera.screen_cords_to_pixel_cords(screen_cords);
        for y in -radius..=radius {
            for x in -radius..=radius {
                if x * x + y * y < radius * radius {
                    let (w, h) = (center_w + x, center_h + y);
                    if w.is_negative() || h.is_negative() {
                        continue;
                    }
                    let _ = self.set_pixel_from_pixel_coords((w as usize, h as usize), color);
                }
            }
        }
    }

    pub fn set_pixels_from_rect_brush(
        &mut self,
        start_screen_cords: &Pos2,
        end_screen_cords: &Pos2,
        color: Color32,
    ) {
        let (x1, y1) = self.camera.screen_cords_to_pixel_cords(start_screen_cords);
        let (x2, y2) = self.camera.screen_cords_to_pixel_cords(end_screen_cords);
        let (x1, y1, x2, y2) = (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2));
        for y in y1..=y2 {
            for x in x1..=x2 {
                if x.is_negative() || y.is_negative() {
                    continue;
                }
                let _ = self.set_pixel_from_pixel_coords((x as usize, y as usize), color);
            }
        }
    }

    pub fn get_pixel_from_screen_cords(
        &mut self,
        screen_cords: Pos2,
        active_layer: usize,
    ) -> Option<&Color32> {
        let x = ((screen_cords.x - self.camera.screen_center.x) / self.camera.square_size).floor()
            + self.camera.pixel_center.x.round();
        let y = ((screen_cords.y - self.camera.screen_center.y) / self.camera.square_size).floor()
            + self.camera.pixel_center.y.round();
        self.get_pixel(x as usize, y as usize, active_layer)
    }

    pub fn get_screen_center(&self) -> &Pos2 {
        &self.camera.screen_center
    }

    pub fn get_screen_center_mut(&mut self) -> &mut Pos2 {
        &mut self.camera.screen_center
    }

    pub fn update_squares(&mut self) {
        self.squares
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, sq)| {
                let l = (idx / (self.width * self.height)) as isize - 1; // -1 for the alpha layer.
                let h = (idx % (self.width * self.height)) / self.width;
                let w = (idx % (self.width * self.height)) % self.width;

                if l < 0 {
                    // Alpha layer
                    let screen_cords = self
                        .camera
                        .pixel_cords_to_screen_cords(w as isize, h as isize);
                    let color = if (h / self.alpha_ratio) % 2 == (w / self.alpha_ratio) % 2 {
                        Color32::DARK_GRAY
                    } else {
                        Color32::LIGHT_GRAY
                    };
                    *sq = self.camera.square_from_screen_cords(screen_cords, color);
                } else {
                    // Rest of the layers
                    let l = l as usize;

                    let screen_cords = self
                        .camera
                        .pixel_cords_to_screen_cords(w as isize, h as isize);
                    let color = self.pixels[w + (h * self.width) + (self.width * self.height * l)];
                    *sq = self.camera.square_from_screen_cords(screen_cords, color);
                }
            });
    }

    pub fn get_circle_brush(&self, screen_cords: &Pos2, radius: usize) -> Vec<Shape> {
        let radius = radius as isize;

        let (w, h) = self.camera.screen_cords_to_pixel_cords(screen_cords);
        let mut brush = Vec::new();
        for y in -radius..=radius {
            for x in -radius..=radius {
                if x * x + y * y < radius * radius {
                    brush.push(
                        self.camera
                            .square_from_pixel_cords(w + x, h + y, HIGHLIGHT_COLOR),
                    )
                }
            }
        }
        brush
    }

    pub fn get_rect_brush(&self, start_screen_cords: &Pos2, end_screen_cords: &Pos2) -> Shape {
        let (x1, y1) = self.camera.screen_cords_to_pixel_cords(start_screen_cords);
        let (x2, y2) = self.camera.screen_cords_to_pixel_cords(end_screen_cords);
        let (x1, y1, x2, y2) = (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2));
        Shape::from(RectShape::new(
            Rect::from_min_max(
                self.camera.pixel_cords_to_screen_cords(x1, y1),
                self.camera.pixel_cords_to_screen_cords(x2 + 1, y2 + 1),
            ),
            Rounding::default(),
            HIGHLIGHT_COLOR,
            Stroke::NONE,
        ))
    }

    pub fn zoom(&mut self, amount: &f32) {
        self.camera.square_size = (self.camera.square_size * amount).clamp(2.0, 100.0).ceil();
        self.update_squares();
    }

    pub fn scroll(&mut self, amount: &Vec2) {
        self.camera.pixel_center = Pos2 {
            x: (self.camera.pixel_center.x
                - amount.x * SCROLL_SENSITIVITY / self.camera.square_size)
                .clamp(0.0, self.width as f32),
            y: (self.camera.pixel_center.y
                - amount.y * SCROLL_SENSITIVITY / self.camera.square_size)
                .clamp(0.0, self.height as f32),
        };
        self.update_squares();
    }

    pub fn get_stroke(&self) -> &Stroke {
        &self.stroke
    }

    pub fn round_coords(coord: Pos2, interval: usize) -> Pos2 {
        let interval_f32 = interval as f32;
        let rounded_x = (coord.x / interval_f32).round() * interval_f32;
        let rounded_y = (coord.y / interval_f32).round() * interval_f32;

        Pos2::new(rounded_x, rounded_y)
    }

    pub fn get_rgba_buffer(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut rgba_buffer = ImageBuffer::new(self.width as u32, self.height as u32);
        for i in 0..self.layers {
            for y in 0..self.height {
                for x in 0..self.width {
                    let color32 = self.get_pixel_unchecked(x, y, i);
                    let rgba = color32.to_srgba_unmultiplied();
                    if i > 0 && rgba[3] == 0 {
                        continue;
                    }
                    rgba_buffer.put_pixel(x as u32, y as u32, Rgba(rgba));
                }
            }
        }
        rgba_buffer
    }

    pub fn get_num_layers(&self) -> usize {
        self.layers
    }

    pub fn add_layer(&mut self) {
        self.layers += 1;
        self.layer_names.push(self.layer_name_cnt.to_string());
        self.layer_name_cnt += 1;
        self.layers_to_show.push(true);

        // Reallocate pixels vec and shapes vec
        let layer_size = self.width * self.height;
        self.pixels
            .resize(self.pixels.len() + layer_size, Color32::TRANSPARENT);
        self.squares.resize(
            self.squares.len() + layer_size,
            Shape::from(RectShape::new(
                Rect::from_min_size(Pos2::default(), Vec2::default()),
                Rounding::default(),
                Color32::TRANSPARENT,
                Stroke::NONE,
            )),
        );
        self.update_squares();
    }

    pub fn get_active_layer(&self) -> usize {
        self.active_layer
    }

    pub fn set_active_layer(&mut self, layer_idx: usize) {
        self.active_layer = layer_idx;
    }

    pub fn delete_layer(&mut self, layer_idx: usize) {
        let layer_size = self.width * self.height;

        let start_idx = layer_size * layer_idx;
        let end_idx = layer_size * (layer_idx + 1);

        // check range, then delete layer from pixels and resize
        if start_idx < self.pixels.len() && end_idx <= self.pixels.len() && start_idx < end_idx {
            self.pixels.drain(start_idx..end_idx);
        } else {
            panic!(
                "Range {}-{} is out of bounds for pixels",
                start_idx, end_idx
            );
        }

        // for pixels, need to update start and end
        let start_idx = start_idx + layer_size;
        let end_idx = end_idx + layer_size;

        // delete from squares in the same way
        if start_idx < self.squares.len() && end_idx <= self.squares.len() && start_idx < end_idx {
            self.squares.drain(start_idx..end_idx);
        } else {
            panic!(
                "Range {}-{} is out of bounds for squares",
                start_idx, end_idx
            );
        }

        // update other variables
        if self.layers > 1 && self.active_layer == self.layers - 1 {
            self.active_layer -= 1
        }

        self.layers -= 1;
        self.layer_names.remove(layer_idx);
        self.layers_to_show.remove(layer_idx);

        self.update_squares();
    }

    pub fn get_layer_name(&self, layer_idx: usize) -> &String {
        self.layer_names.get(layer_idx).unwrap()
    }

    pub fn get_layers_to_show_mut(&mut self) -> &mut Vec<bool> {
        &mut self.layers_to_show
    }

    pub fn fill(
        &mut self,
        screen_coord: &Pos2,
        target_color: &Color32,
        fill_color: &Color32,
    ) -> Result<(), String> {
        // Check if trying to fill a color with itself
        if target_color == fill_color {
            return Err(String::from("Tried to fill with the same color"));
        }

        let pixel_coord = self.camera.screen_cords_to_pixel_cords(screen_coord);
        if self
            .get_pixel(
                pixel_coord.0 as usize,
                pixel_coord.1 as usize,
                self.active_layer,
            )
            .is_none()
        {
            return Err(String::from("Tried to fill outside of canvas"));
        }

        self.flood_fill(pixel_coord, target_color, fill_color);
        Ok(())
    }

    fn flood_fill(
        &mut self,
        pixel_coord: (isize, isize),
        target_color: &Color32,
        fill_color: &Color32,
    ) {
        let (x, y) = pixel_coord;

        // Checking for out of bounds, and then casting as usize for compatibility with other functions
        if x.is_negative() || y.is_negative() {
            return;
        }
        let x_usize = x as usize;
        let y_usize = y as usize;

        // check for target color, set if it is correct color
        match self.get_pixel_mut(x_usize, y_usize, self.active_layer) {
            Some(color) => {
                if color == target_color {
                    // Setting the color here
                    if self
                        .set_pixel_from_pixel_coords((x_usize, y_usize), *fill_color)
                        .is_err()
                    {
                        return;
                    }
                } else {
                    return;
                }
            }
            None => return,
        }

        // Call recursively on surrounding squares (4 directions)
        self.flood_fill((x + 1, y), target_color, fill_color);
        self.flood_fill((x - 1, y), target_color, fill_color);
        self.flood_fill((x, y + 1), target_color, fill_color);
        self.flood_fill((x, y - 1), target_color, fill_color);
    }

    pub fn create_state(&self) -> CanvasState {
        CanvasState::new(
            self.layers,
            self.active_layer,
            self.layer_names.clone(),
            self.layer_name_cnt,
            self.layers_to_show.clone(),
            self.pixels.clone(),
            self.squares.clone(),
        )
    }

    pub fn load_state(&mut self, state: &CanvasState) {
        self.layers = state.layers;
        self.active_layer = state.active_layer;
        self.layer_names = state.layer_names.clone();
        self.layer_name_cnt = state.layer_name_cnt;
        self.layers_to_show = state.layers_to_show.clone();
        self.pixels = state.pixels.clone();
        self.squares = state.squares.clone();

        self.update_squares();
    }
}
