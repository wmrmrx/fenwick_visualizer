use eframe::{egui, epi};
use egui::{Pos2, Ui};

pub struct FenwickTree {
    len: usize,
    input_len: usize,
    arr: Vec<i16>,
    arr_marked: Vec<bool>,
    fenwick: Vec<i16>,
    fenwick_marked: Vec<bool>,
    input_query: String,
    input_update_ind: String,
    input_update_val: String,
    query_answer: Option<i16>,
}

impl FenwickTree {
    pub fn new(len: usize) -> FenwickTree {
        FenwickTree {
            len,
            input_len: len,
            arr: vec![0; len + 1],
            arr_marked: vec![false; len + 1],
            fenwick: vec![0; len + 1],
            fenwick_marked: vec![false; len + 1],
            input_query: String::new(),
            input_update_ind: String::new(),
            input_update_val: String::new(),
            query_answer: None,
        }
    }
    fn query(&mut self, mut ind: usize) {
        let mut relevant_indexes = Vec::new();
        let mut sum = 0;
        self.arr_marked = vec![false; self.len + 1];
        for i in self.arr_marked.iter_mut().take(ind+1) {
            *i = true;
        }
        self.fenwick_marked = vec![false; self.len + 1];
        while ind > 0 {
            let bit: usize = ind.trailing_zeros() as usize;
            self.fenwick_marked[ind] = true;
            relevant_indexes.push(ind);
            sum += self.fenwick[ind];
            ind -= 1 << bit;
        }
        self.query_answer = Some(sum);
    }
    fn update(&mut self, mut ind: usize, val: i16) {
        self.arr_marked = vec![false; self.len + 1];
        let diff = val - self.arr[ind];
        self.arr[ind] = val;
        self.arr_marked[ind] = true;
        self.fenwick_marked = vec![false; self.len + 1];
        while ind <= self.len {
            let bit: usize = ind.trailing_zeros() as usize;
            self.fenwick_marked[ind] = true;
            self.fenwick[ind] += diff;
            ind += 1 << bit;
        }
    }
    fn reset(&mut self) {
        *self = FenwickTree::new(self.input_len);
    }
    fn draw_indexes(&self, ui: &mut Ui) {
        let x = 30.0;
        let (by, uy) = (10.0, 590.0);
        for i in 1..=self.len {
            let y: f32 = uy - (uy - by) / (self.len as f32) * (i as f32 - 0.5);
            let mid_point = Pos2 { x, y };
            ui.painter().text(
                mid_point,
                egui::Align2::RIGHT_CENTER,
                i,
                egui::TextStyle::Heading,
                egui::Color32::WHITE,
            );
        }
    }
    fn draw_array(&self, ui: &mut Ui) {
        let (lx, rx) = (40.0, 100.0);
        let (by, uy) = (10.0, 590.0);
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
                self.arr[i],
                egui::TextStyle::Heading,
                if self.arr_marked[i] {
                    egui::Color32::GOLD
                } else {
                    egui::Color32::WHITE
                },
            );
        }
    }
    fn draw_fenwick(&self, ui: &mut Ui) {
        let (by, uy) = (10.0, 590.0);
        let (x_width, x_spacing) = (10.0, 45.0);
        let (mut x_intervals, mut y_intervals) = (
            vec![0.0; 64 - self.len.leading_zeros() as usize],
            vec![0.0; self.len + 1],
        );
        for (ind, y) in y_intervals.iter_mut().enumerate() {
            *y = uy - (uy - by) / (self.len as f32) * (ind as f32);
        }
        for (ind, x) in x_intervals.iter_mut().enumerate() {
            *x = 105.0 + (x_width + x_spacing) * (ind as f32);
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
                if self.fenwick_marked[i] {
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
                self.fenwick[i],
                egui::TextStyle::Monospace,
                if self.fenwick_marked[i] {
                    egui::Color32::GOLD
                } else {
                    egui::Color32::WHITE
                },
            );
        }
    }
}

impl epi::App for FenwickTree {
    fn name(&self) -> &str {
        "Fenwick Tree visualizer"
    }
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::SidePanel::right("Side Panel").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.input_len, 1..=24).text("Array length"));
            if self.len != self.input_len {
                self.reset();
            }
            ui.horizontal(|ui| {
                if ui.button("Query").clicked() {
                    if let Ok(x) = self.input_query.parse::<usize>() {
                        if 1 <= x && x <= self.len {
                            self.query(x);
                        }
                    }
                }
                ui.label("prefix sum of index X");
            });
            ui.horizontal(|ui| {
                ui.label("X: ");
                ui.text_edit_singleline(&mut self.input_query);
            });
            ui.horizontal(|ui| {
                ui.label("Query answer: ");
                if let Some(x) = self.query_answer {
                    ui.label(format!("{}", x));
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Update").clicked() {
                    if let Ok(x) = self.input_update_ind.parse::<usize>() {
                        if let Ok(y) = self.input_update_val.parse::<i16>() {
                            if 1 <= x && x <= self.len {
                                self.update(x, y);
                            }
                        }
                    }
                }
                ui.label("value in X to Y");
            });
            ui.horizontal(|ui| {
                ui.label("X: ");
                ui.text_edit_singleline(&mut self.input_update_ind);
            });
            ui.horizontal(|ui| {
                ui.label("Y: ");
                ui.text_edit_singleline(&mut self.input_update_val);
            });
            ui.horizontal(|ui| {
                if ui.button("Randomize").clicked() {
                    self.reset();
                    for i in 1..=self.len {
                        self.update(i, fastrand::i16(-10..=10));
                    }
                    self.arr_marked = vec![false; self.len + 1];
                    self.fenwick_marked = vec![false; self.len + 1];
                }
                ui.label("array");
            });
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    self.reset();
                }
                ui.label("array");
            });
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );
            });
        });
        egui::Area::new("Nim Game").show(ctx, |ui| {
            self.draw_indexes(ui);
            self.draw_array(ui);
            self.draw_fenwick(ui);
        });
    }
}
