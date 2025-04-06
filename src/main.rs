use eframe::egui;
use eframe::NativeOptions;
use egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(egui::vec2(320.0, 240.0))
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Box::new(CalculatorApp::default())),
    )
}

struct CalculatorApp {
    input: String,
    result: String,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            result: String::new(),
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calculator");

            ui.horizontal(|ui| {
                let input_field = ui.text_edit_singleline(&mut self.input);
                if input_field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.calculate();
                }
            });

            if ui.button("Calculate").clicked() {
                self.calculate();
            }

            ui.label(&self.result);

            // Calculator buttons
            ui.horizontal(|ui| {
                if ui.button("7").clicked() { self.input.push('7'); }
                if ui.button("8").clicked() { self.input.push('8'); }
                if ui.button("9").clicked() { self.input.push('9'); }
                if ui.button("+").clicked() { self.input.push('+'); }
            });
            ui.horizontal(|ui| {
                if ui.button("4").clicked() { self.input.push('4'); }
                if ui.button("5").clicked() { self.input.push('5'); }
                if ui.button("6").clicked() { self.input.push('6'); }
                if ui.button("-").clicked() { self.input.push('-'); }
            });
            ui.horizontal(|ui| {
                if ui.button("*").clicked() { self.input.push('*'); }
            });
        });
    }
}

impl CalculatorApp {
    fn calculate(&mut self) {
        let parts: Vec<&str> = self.input.split_whitespace().collect();

        if parts.len() != 3 {
            self.result = "Invalid format. Use: number operator number".to_string();
            return;
        }

        let num1: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                self.result = "Invalid first number".to_string();
                return;
            }
        };

        let operator = parts[1];

        let num2: f64 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                self.result = "Invalid second number".to_string();
                return;
            }
        };

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    self.result = "Cannot divide by zero!".to_string();
                    return;
                }
                num1 / num2
            }
            _ => {
                self.result = "Invalid operator".to_string();
                return;
            }
        };

        self.result = format!("Result: {}", result);
    }
}