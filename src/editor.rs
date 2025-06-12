use egui::{self, Label, RichText, ScrollArea, Sense};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct CodeEditor {
    pub text: String,
    scroll: f32,
    ps: SyntaxSet,
    theme: syntect::highlighting::Theme,
    syntax: syntect::parsing::SyntaxReference,
}

impl CodeEditor {
    pub fn new(text: String, language: &str) -> Self {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps
            .find_syntax_by_extension(language)
            .unwrap_or_else(|| ps.find_syntax_plain_text()).clone();

        Self {
            text,
            scroll: 0.0,
            ps,
            theme: ts.themes["InspiredGitHub"].clone(),
            syntax: syntax.clone(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let desired_height = ui.spacing().interact_size.y * 20.0;
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .max_height(desired_height)
            .show(ui, |ui| {
                let mut hl = HighlightLines::new(&self.syntax, &self.theme);

                for (i, line) in LinesWithEndings::from(&self.text).enumerate() {
                    let ranges = hl.highlight_line(line, &self.ps).unwrap_or_default();
                    let layout_job = Self::highlight_to_job(line, &ranges);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{:>3}", i + 1)).monospace().weak());
                        ui.add(Label::new(layout_job).sense(Sense::click()));
                    });
                }
            });
    }

    fn highlight_to_job<'a>(
        _line: &'a str,
        ranges: &[(syntect::highlighting::Style, &'a str)],
    ) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();
        for &(style, text) in ranges {
            let color =
                egui::Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b);
            let format = egui::TextFormat {
                font_id: egui::FontId::monospace(14.0),
                color,
                ..Default::default()
            };
            job.append(text, 0.0, format);
        }
        job
    }

    pub fn get_code(&self) -> &str {
        &self.text
    }

    pub fn set_code(&mut self, new_code: String) {
        self.text = new_code;
    }
}
