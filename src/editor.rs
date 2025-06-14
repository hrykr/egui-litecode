use egui::{Color32, FontId, TextEdit, TextFormat, Ui};
use egui::text::LayoutJob;
use std::fmt;
use std::sync::Arc;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet, Style};
use syntect::parsing::{SyntaxSet, SyntaxReference};

pub struct CodeEditor {
    pub code: String,
    syntax_set: SyntaxSet,
    theme: Arc<Theme>,
    syntax: &'static SyntaxReference,
    highlighter: Option<HighlightLines<'static>>,
}

impl Clone for CodeEditor {
    fn clone(&self) -> Self {
        CodeEditor {
            code: self.code.clone(),
            syntax_set: self.syntax_set.clone(),
            theme: self.theme.clone(),
            syntax: self.syntax, // static reference, just copy
            highlighter: None,   // do not clone highlighter
        }
    }
}

impl fmt::Debug for CodeEditor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CodeEditor")
            .field("code", &self.code)
            .field("syntax_set", &"...")
            .field("theme", &"...")
            .field("syntax", &self.syntax.name)
            .field("highlighter", &self.highlighter.is_some())
            .finish()
    }
}

impl PartialEq for CodeEditor {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.theme == other.theme
            && std::ptr::eq(self.syntax, other.syntax)
    }
}

impl CodeEditor {
    pub fn new() -> Self {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = Arc::new(ts.themes["base16-ocean.dark"].clone());
        let syntax = ps.find_syntax_by_extension("rs").unwrap(); // force unwrap safe here

        Self {
            code: "".into(),
            syntax_set: ps.clone(),
            theme,
            syntax: Box::leak(Box::new(syntax.clone())), // static lifetime workaround
            highlighter: None,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> egui::Response {
        let font = FontId::monospace(14.0);

        // Reset highlighter for fresh state
        let mut highlighter = HighlightLines::new(self.syntax, &self.theme);

        let mut layouter = {
            let font = font.clone();
            let syntax_set = self.syntax_set.clone();
            let mut highlighter = HighlightLines::new(self.syntax, &self.theme);

            Box::new(move |ui: &Ui, line: &str, wrap_width: f32| {
                let mut job = LayoutJob::default();

                if let Ok(ranges) = highlighter.highlight_line(line, &syntax_set) {
                    for (style, text) in ranges {
                        let color = Color32::from_rgb(
                            style.foreground.r,
                            style.foreground.g,
                            style.foreground.b,
                        );

                        job.append(
                            text,
                            0.0,
                            TextFormat {
                                font_id: font.clone(),
                                color,
                                ..Default::default()
                            },
                        );
                    }
                }

                job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(job))
            }) as Box<dyn FnMut(&Ui, &str, f32) -> Arc<egui::Galley>>
        };

        ui.add(
            TextEdit::multiline(&mut self.code)
                .font(font)
                .desired_width(f32::INFINITY)
                .layouter(&mut layouter),
        )
    }
}
