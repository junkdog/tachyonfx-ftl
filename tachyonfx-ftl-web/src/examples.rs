use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub code: &'static str,
    pub canvas: &'static str,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Basic,
    Transitions,
    Animations,
    Advanced,
    Showcase,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Basic => write!(f, "Basic"),
            Category::Transitions => write!(f, "Transitions"),
            Category::Animations => write!(f, "Animations"),
            Category::Advanced => write!(f, "Advanced"),
            Category::Showcase => write!(f, "Showcase"),
        }
    }
}

// Built-in examples serving as the single source of truth
pub fn get_examples() -> Vec<Example> {
    vec![
        // BASIC - Simple single effects
        basic::fade_in(),

    ]
}

// basic examples
mod basic {
    use super::*;

    pub fn fade_in() -> Example {
        Example {
            id: "basic_fade_in".to_string(),
            title: "Fade In".to_string(),
            description: "Smoothly fade text from transparent to visible".to_string(),
            category: Category::Basic,
            code: r#"fx::fade_from_fg(Color::Black, (1500, QuadOut))"#,
            canvas: canvas::DEFAULT,
            tags: vec!["fade".to_string(), "basic".to_string(), "color".to_string()],
        }
    }
}





mod canvas {
    pub const DEFAULT: &'static str = include_str!("../assets/default_canvas.ansi");
}