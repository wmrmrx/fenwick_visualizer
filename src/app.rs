use eframe::egui;

struct FontSizes {
    pub index: f32,
    pub array: f32,
    pub fenwick: f32,
}

impl Default for FontSizes {
    fn default() -> Self {
        Self {
            index: 14.0,
            array: 16.0,
            fenwick: 14.0,
        }
    }
}

pub struct FenwickTree {
    len: usize,
    input_len: usize,
    arr: Vec<i64>,
    arr_highlighted: Vec<bool>,
    fenwick: Vec<i64>,
    fenwick_highlighted: Vec<bool>,
    input_query: String,
    input_update_ind: String,
    input_update_val: String,
    font_sizes: FontSizes,
    query_answer: Option<i64>,
    error: Option<&'static str>,
}

impl FenwickTree {
    pub fn new(len: usize, _cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            len,
            font_sizes: Default::default(),
            input_len: len,
            arr: vec![0; len],
            arr_highlighted: vec![false; len],
            fenwick: vec![0; len],
            fenwick_highlighted: vec![false; len],
            input_query: String::new(),
            input_update_ind: String::new(),
            input_update_val: String::new(),
            query_answer: None,
            error: None,
        }
    }

    fn resize(&mut self, len: usize) {
        if len != self.len {
            self.len = len;
            self.arr.resize(len, 0);
            self.arr_highlighted.resize(len, false);
            self.fenwick.resize(len, 0);
            self.fenwick_highlighted.resize(len, false);
        }
    }

    fn query(&mut self, mut ind: usize) {
        let mut sum = 0;
        self.arr_highlighted
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x = i < ind);
        self.fenwick_highlighted.iter_mut().for_each(|x| *x = false);
        while ind > 0 {
            let bit: usize = ind.trailing_zeros() as usize;
            self.fenwick_highlighted[ind - 1] = true;
            sum += self.fenwick[ind - 1];
            ind -= 1 << bit;
        }
        self.query_answer = Some(sum);
    }

    fn update(&mut self, mut ind: usize, val: i64) {
        self.arr_highlighted.iter_mut().for_each(|x| *x = false);
        self.fenwick_highlighted.iter_mut().for_each(|x| *x = false);
        let diff = val - self.arr[ind - 1];
        self.arr[ind - 1] = val;
        self.arr_highlighted[ind - 1] = true;
        while ind <= self.len {
            let bit: usize = ind.trailing_zeros() as usize;
            self.fenwick_highlighted[ind - 1] = true;
            self.fenwick[ind - 1] += diff;
            ind += 1 << bit;
        }
    }

    fn reset(&mut self) {
        for i in 0..self.len {
            self.arr[i] = 0;
            self.arr_highlighted[i] = false;
            self.fenwick[i] = 0;
            self.fenwick_highlighted[i] = false;
        }
        self.input_query.clear();
        self.input_update_ind.clear();
        self.input_update_val.clear();
        self.query_answer = None;
        self.error = None;
    }

    fn draw(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        use egui::{Pos2, Vec2};
        let Vec2 { x: _, y: h } = frame.info().window_info.size;
        const K: f32 = 1.6;
        let (by, uy) = (10.0, h - 10.0);

        let x = 15.0;
        let draw_indexes = || {
            for i in 1..=self.len {
                let y: f32 = uy - (uy - by) / (self.len as f32) * (i as f32 - 0.5);
                let point = Pos2 { x, y };
                ui.painter().text(
                    point,
                    egui::Align2::LEFT_CENTER,
                    i,
                    egui::FontId {
                        size: self.font_sizes.index,
                        family: egui::FontFamily::Monospace,
                    },
                    egui::Color32::WHITE,
                );
            }
        };
        draw_indexes();

        let lx = x + K * self.font_sizes.index;
        let rx = lx + 20.0 + K * self.font_sizes.array;
        let draw_array = || {
            let stroke = egui::Stroke {
                width: 3.5,
                color: egui::Color32::WHITE,
            };
            let rect = egui::Rect {
                min: Pos2 { x: lx, y: by },
                max: Pos2 { x: rx, y: uy },
            };
            ui.painter().rect_stroke(rect, 3.0, stroke);
            for i in 1..=self.len {
                let y: f32 = uy - (uy - by) / (self.len as f32) * (i as f32);
                let line = [Pos2 { x: lx, y }, Pos2 { x: rx, y }];
                if i < self.len {
                    ui.painter().line_segment(line, stroke);
                }
                let mid_point = Pos2 {
                    x: (lx + rx) / 2.0,
                    y: y + (uy - by) / (self.len as f32) / 2.0,
                };
                ui.painter().text(
                    mid_point,
                    egui::Align2::CENTER_CENTER,
                    self.arr[i - 1],
                    egui::FontId {
                        size: self.font_sizes.array,
                        family: egui::FontFamily::Monospace,
                    },
                    if self.arr_highlighted[i - 1] {
                        egui::Color32::GOLD
                    } else {
                        egui::Color32::WHITE
                    },
                );
            }
        };
        draw_array();

        let draw_fenwick = || {
            let (x_width, x_spacing) = (10.0, 30.0 + K * self.font_sizes.fenwick);
            let (mut x_intervals, mut y_intervals) = (
                vec![0.0; 64 - self.len.leading_zeros() as usize].into_boxed_slice(),
                vec![0.0; self.len + 1].into_boxed_slice(),
            );
            for (ind, y) in y_intervals.iter_mut().enumerate() {
                *y = uy - (uy - by) / (self.len as f32) * (ind as f32);
            }
            for (ind, x) in x_intervals.iter_mut().enumerate() {
                *x = rx + 10.0 + (x_width + x_spacing) * (ind as f32);
            }
            for i in 1..=self.len {
                let bit = i.trailing_zeros() as usize;
                let rect = egui::Rect {
                    min: Pos2 {
                        x: x_intervals[bit],
                        y: y_intervals[i],
                    },
                    max: Pos2 {
                        x: x_intervals[bit] + x_width,
                        y: y_intervals[i - (1 << bit)],
                    },
                };
                ui.painter().rect_filled(
                    rect,
                    2.0,
                    if self.fenwick_highlighted[i - 1] {
                        egui::Color32::GOLD
                    } else {
                        egui::Color32::WHITE
                    },
                );
                ui.painter().text(
                    Pos2 {
                        x: x_intervals[bit] + x_width + 5.0,
                        y: (y_intervals[i] + y_intervals[i - (1 << bit)]) / 2.0,
                    },
                    egui::Align2::LEFT_CENTER,
                    self.fenwick[i - 1],
                    egui::FontId {
                        size: self.font_sizes.fenwick,
                        family: egui::FontFamily::Monospace,
                    },
                    if self.fenwick_highlighted[i - 1] {
                        egui::Color32::GOLD
                    } else {
                        egui::Color32::WHITE
                    },
                );
            }
        };
        draw_fenwick();
    }
}

impl eframe::App for FenwickTree {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::right("Side Panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.add(egui::Slider::new(&mut self.input_len, 1..=64).text("Array length"));
            self.resize(self.input_len);
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Query").clicked() {
                    self.error = if let Ok(x) = self.input_query.parse::<usize>() {
                        if 1 <= x && x <= self.len {
                            self.query(x);
                            None
                        } else {
                            Some("Invalid query index range (must be between 1 and array length)")
                        }
                    } else {
                        Some("Invalid query index")
                    }
                }
                ui.label("prefix sum of index X");
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("X: ");
                ui.text_edit_singleline(&mut self.input_query);
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("Query answer: ");
                if let Some(x) = self.query_answer {
                    ui.label(format!("{}", x));
                }
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Update").clicked() {
                    self.error = if let Ok(x) = self.input_update_ind.parse::<usize>() {
                        if 1 <= x && x <= self.len {
                            if let Ok(y) = self.input_update_val.parse::<i64>() {
                                self.update(x, y);
                                None
                            } else {
                                Some("Invalid update value")
                            }
                        } else {
                            Some("Invalid update index range (must be between 1 and array length)")
                        }
                    } else {
                        Some("Invalid update index")
                    }
                }
                ui.label("value in X to Y");
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("X: ");
                ui.text_edit_singleline(&mut self.input_update_ind);
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("Y: ");
                ui.text_edit_singleline(&mut self.input_update_val);
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Randomize").clicked() {
                    self.reset();
                    for i in 1..=self.len {
                        self.update(i, fastrand::i64(-100..=100));
                    }
                    self.arr_highlighted.iter_mut().for_each(|x| *x = false);
                    self.fenwick_highlighted.iter_mut().for_each(|x| *x = false);
                }
                ui.label("array");
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    self.reset();
                }
                ui.label("array");
            });

            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.font_sizes.index, 4.0..=64.0).text("Index font size"),
            );
            ui.add(
                egui::Slider::new(&mut self.font_sizes.array, 4.0..=64.0)
                    .text("Array elements font size"),
            );
            ui.add(
                egui::Slider::new(&mut self.font_sizes.fenwick, 4.0..=64.0)
                    .text("Fenwick tree font size"),
            );

            ui.add_space(20.0);
            if let Some(err) = self.error {
                ui.label(egui::RichText::new(err).color(egui::Color32::RED));
            }
        });

        egui::Area::new("Area").show(ctx, |ui| {
            self.draw(ui, frame);
        });
    }
}
