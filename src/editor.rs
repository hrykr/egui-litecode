use egui::{Color32, FontId, Galley, TextEdit, TextFormat, Ui};
use egui::text::LayoutJob;
use std::fmt;
use std::sync::Arc;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet, Style};
use syntect::parsing::{SyntaxSet, SyntaxReference};

/// Basical code editor widget for [egui](https://crates.io/crates/egui), supporting syntax highlighting and themes.
/// 
/// # Implement
/// 
/// Use `CodeEditor::new(syntax_ext, color_theme)` to create a new instance.\
/// Then use call `ui` method to integrate it into your egui application.
/// 
/// # Usage
/// 
/// Use `mycodeeditor.code` to access the code.\
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
    pub fn new(syntax_ext: &str, color_theme: &str) -> Self {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = Arc::new(ts.themes[color_theme].clone());
        let syntax = ps.find_syntax_by_extension(syntax_ext).unwrap(); // force unwrap safe here

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
        let syntax_set = self.syntax_set.clone();
        let theme = self.theme.clone();
        let syntax = self.syntax;

        let mut layouter = {
            let font = font.clone();
            Box::new(move |ui: &Ui, text_buffer: &dyn egui::TextBuffer, wrap_width: f32| {
                let mut job = LayoutJob::default();
                let mut highlighter = HighlightLines::new(syntax, &theme);
                let text = text_buffer.as_str();

                for (i, line) in text.lines().enumerate() {
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

                    if i + 1 < text.lines().count() {
                        job.append(
                            "\n",
                            0.0,
                            TextFormat {
                                font_id: font.clone(),
                                color: Color32::WHITE,
                                ..Default::default()
                            },
                        );
                    }
                }

                job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(job))
            }) as Box<dyn FnMut(&Ui, &dyn egui::TextBuffer, f32) -> Arc<Galley>>
        };

        ui.add(
            TextEdit::multiline(&mut self.code)
                .font(font)
                .desired_width(f32::INFINITY)
                .code_editor()
                .layouter(&mut layouter),
        )
    }
}

impl Default for CodeEditor {
    fn default() -> Self {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = Arc::new(ts.themes["base16-ocean.dark"].clone());
        let syntax= ps.find_syntax_by_extension("rs").unwrap(); // force unwrap safe here
        Self {
            code: "".into(),
            syntax_set: ps.clone(),
            theme,
            syntax: Box::leak(Box::new(syntax.clone())), // static lifetime workaround
            highlighter: None,
        }
    }
}