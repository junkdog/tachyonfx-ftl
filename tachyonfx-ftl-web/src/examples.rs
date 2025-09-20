use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub code: String,
    pub canvas: String,
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
        basic_fade_in(),
        basic_fade_out(),
        basic_slide_left(),
        basic_slide_right(),
        basic_typewriter(),
        // TRANSITIONS - Effect combinations
        fade_slide_combo(),
        cross_fade_transition(),
        slide_sequence(),
        // ANIMATIONS - Complex movement and timing
        bouncing_text(),
        wave_effect(),
        matrix_rain(),
        rotating_text(),
        // ADVANCED - Complex DSL features
        parallel_effects(),
        conditional_timing(),
        custom_interpolations(),
        // SHOWCASE - Impressive demos
        terminal_boot_sequence(),
        glitch_effect(),
        demo_presentation(),
    ]
}

// Basic Examples
fn basic_fade_in() -> Example {
    Example {
        id: "basic_fade_in".to_string(),
        title: "Fade In".to_string(),
        description: "Smoothly fade text from transparent to visible".to_string(),
        category: Category::Basic,
        code: r#"fx::fade_from_fg(Color::Black, (1500, QuadOut))"#.to_string(),
        canvas: r#"Welcome to TachyonFX!

This text will fade in smoothly
using a quadratic easing curve.

Perfect for gentle introductions!"#
            .to_string(),
        tags: vec!["fade".to_string(), "basic".to_string(), "color".to_string()],
    }
}

fn basic_fade_out() -> Example {
    Example {
        id: "basic_fade_out".to_string(),
        title: "Fade Out".to_string(),
        description: "Gradually fade text to transparent".to_string(),
        category: Category::Basic,
        code: r#"fx::fade_to_fg(Color::Black, (2000, SineIn))"#.to_string(),
        canvas: r#"Goodbye, TachyonFX!

This text will fade out gently
using a sine wave easing.

Watch it disappear..."#
            .to_string(),
        tags: vec!["fade".to_string(), "basic".to_string(), "color".to_string()],
    }
}

fn basic_slide_left() -> Example {
    Example {
        id: "basic_slide_left".to_string(),
        title: "Slide from Left".to_string(),
        description: "Text slides in from the left side with gradient".to_string(),
        category: Category::Basic,
        code: r#"fx::slide_in(Motion::LeftToRight, 10, 0, Color::Black, (1000, BackOut))"#.to_string(),
        canvas: r#"▶ Sliding from the left!

Watch this content move smoothly
from off-screen to its position.

The BackOut easing adds a nice bounce!"#
            .to_string(),
        tags: vec![
            "slide".to_string(),
            "motion".to_string(),
            "basic".to_string(),
        ],
    }
}

fn basic_slide_right() -> Example {
    Example {
        id: "basic_slide_right".to_string(),
        title: "Slide from Right".to_string(),
        description: "Text slides in from the right side with elastic motion".to_string(),
        category: Category::Basic,
        code: r#"fx::slide_in(Motion::RightToLeft, 15, 5, Color::Gray, (1200, ElasticOut))"#.to_string(),
        canvas: r#"                    ◀ Sliding from the right!

           Content can slide from any direction.

           ElasticOut creates a springy effect
           that feels organic and playful!"#
            .to_string(),
        tags: vec![
            "slide".to_string(),
            "motion".to_string(),
            "elastic".to_string(),
        ],
    }
}

fn basic_typewriter() -> Example {
    Example {
        id: "basic_typewriter".to_string(),
        title: "Dissolve Effect".to_string(),
        description: "Text dissolves away and reforms randomly".to_string(),
        category: Category::Basic,
        code: r#"fx::dissolve((2500, Linear))"#.to_string(),
        canvas: r#"This text will dissolve away!

Characters disappear randomly,
creating a dynamic effect that
draws attention.

Perfect for transitions!"#
            .to_string(),
        tags: vec![
            "dissolve".to_string(),
            "text".to_string(),
            "random".to_string(),
        ],
    }
}

// Transition Examples
fn fade_slide_combo() -> Example {
    Example {
        id: "fade_slide_combo".to_string(),
        title: "Sweep + Color Transition".to_string(),
        description: "Combines sweeping motion with color transitions".to_string(),
        category: Category::Transitions,
        code: r#"fx::sweep_in(Motion::LeftToRight, 20, 10, Color::Blue, (800, CubicOut))"#.to_string(),
        canvas: r#"◆ Smooth Sweeping Motion

This text sweeps in with a color gradient,
creating a sophisticated entrance effect.

The sweep reveals content progressively
with smooth color transitions!"#
            .to_string(),
        tags: vec![
            "sweep".to_string(),
            "motion".to_string(),
            "color".to_string(),
        ],
    }
}

fn cross_fade_transition() -> Example {
    Example {
        id: "cross_fade_transition".to_string(),
        title: "HSL Color Shift".to_string(),
        description: "Shifts hue, saturation, and lightness smoothly".to_string(),
        category: Category::Transitions,
        code: r#"fx::hsl_shift_fg([120.0, 25.0, -10.0], (1000, QuadInOut))"#.to_string(),
        canvas: r#"🌈 Color Shifting Magic

This text changes hue, saturation,
and lightness over time.

Watch the colors transform smoothly
using HSL color space!"#
            .to_string(),
        tags: vec![
            "hsl".to_string(),
            "color".to_string(),
            "shift".to_string(),
        ],
    }
}

fn slide_sequence() -> Example {
    Example {
        id: "slide_sequence".to_string(),
        title: "Stretch Effect".to_string(),
        description: "Stretches content using block characters".to_string(),
        category: Category::Transitions,
        code: r#"fx::stretch(Motion::LeftToRight, Style::default().bg(Color::Cyan), (500, BounceOut))"#.to_string(),
        canvas: r#"▲ Stretching Motion!

Content stretches horizontally
using block characters.

Creates a unique expanding effect
that feels dynamic and engaging."#
            .to_string(),
        tags: vec![
            "stretch".to_string(),
            "motion".to_string(),
            "blocks".to_string(),
        ],
    }
}

// Animation Examples
fn bouncing_text() -> Example {
    Example {
        id: "bouncing_text".to_string(),
        title: "Repeating Effect".to_string(),
        description: "Effect that repeats multiple times".to_string(),
        category: Category::Animations,
        code: r#"fx::repeat(fx::dissolve(800), RepeatMode::Times(3))"#.to_string(),
        canvas: r#"🎾 REPEAT! REPEAT! REPEAT!

This effect repeats 3 times!

The dissolve effect runs repeatedly
creating a mesmerizing animation.

Perfect for emphasis!"#
            .to_string(),
        tags: vec![
            "repeat".to_string(),
            "dissolve".to_string(),
            "loop".to_string(),
        ],
    }
}

fn wave_effect() -> Example {
    Example {
        id: "wave_effect".to_string(),
        title: "Infinite Loop".to_string(),
        description: "Effect that loops forever using repeating".to_string(),
        category: Category::Animations,
        code: r#"fx::repeating(fx::hsl_shift_fg([60.0, 0.0, 10.0], (2000, SineInOut)))"#.to_string(),
        canvas: r#"🌊 Infinite Color Flow...

∿∿∿ Colors shift in endless waves ∿∿∿

The HSL shift creates smooth color changes
that repeat forever.

Hypnotic and beautiful!"#
            .to_string(),
        tags: vec![
            "repeating".to_string(),
            "hsl".to_string(),
            "infinite".to_string(),
            "color".to_string(),
        ],
    }
}

fn matrix_rain() -> Example {
    Example {
        id: "matrix_rain".to_string(),
        title: "Symbol Evolution".to_string(),
        description: "Characters evolve through symbol sets".to_string(),
        category: Category::Animations,
        code: r#"fx::evolve(EvolveSymbolSet::Matrix, (3000, Linear))"#.to_string(),
        canvas: r#"01001000 01100101 01101100 01101100 01101111
11000001 10110111 11010011 10001101 11100010
01010100 01100001 01100011 01101000 01111001
11011101 11001110 11111000 10110001 10101010
01000110 01011000 00100000 01110010 01100001
11001011 10110101 11000111 11101001 11001100

Characters transform through Matrix symbols
Creating that classic digital rain effect!"#
            .to_string(),
        tags: vec![
            "evolve".to_string(),
            "matrix".to_string(),
            "symbols".to_string(),
            "digital".to_string(),
        ],
    }
}

fn rotating_text() -> Example {
    Example {
        id: "rotating_text".to_string(),
        title: "Explode Effect".to_string(),
        description: "Text explodes outward with physics".to_string(),
        category: Category::Animations,
        code: r#"fx::explode(15.0, 2.0, (2000, QuartInOut))"#.to_string(),
        canvas: r#"⟲ EXPLODING OUTWARD! ⟳

Characters fly outward from center
with realistic physics simulation.

Velocity and gravity parameters
create believable motion!"#
            .to_string(),
        tags: vec![
            "explode".to_string(),
            "physics".to_string(),
            "motion".to_string(),
        ],
    }
}

// Advanced Examples
fn parallel_effects() -> Example {
    Example {
        id: "parallel_effects".to_string(),
        title: "Effect with Filters".to_string(),
        description: "Using filters to target specific cells".to_string(),
        category: Category::Advanced,
        code: r#"fx::fade_to_fg(Color::Red, (1000, BackOut))
    .with_filter(CellFilter::Text)"#.to_string(),
        canvas: r#"⚡ FILTERED EFFECTS ⚡

This effect only targets text characters,
leaving other elements unchanged.

🔥 Emojis and symbols ignored 🔥
Only regular text gets colored!

Filters provide precise control
over which cells are affected."#
            .to_string(),
        tags: vec![
            "filter".to_string(),
            "targeting".to_string(),
            "precise".to_string(),
        ],
    }
}

fn conditional_timing() -> Example {
    Example {
        id: "conditional_timing".to_string(),
        title: "Delayed Effect".to_string(),
        description: "Effect with delay before starting".to_string(),
        category: Category::Advanced,
        code: r#"fx::delay(500, fx::fade_to_fg(Color::Green, (1000, QuadOut)))"#.to_string(),
        canvas: r#"📊 DELAYED EFFECT 📊

This effect waits 500ms before starting
the color transition.

Delays allow for:
• Sequencing multiple effects
• Creating pause between actions
• Building anticipation

Timing is everything in animation!"#
            .to_string(),
        tags: vec![
            "delay".to_string(),
            "timing".to_string(),
            "sequence".to_string(),
        ],
    }
}

fn custom_interpolations() -> Example {
    Example {
        id: "custom_interpolations".to_string(),
        title: "Extended Duration".to_string(),
        description: "Extending effect start and end times".to_string(),
        category: Category::Advanced,
        code: r#"fx::prolong_end(300, fx::dissolve((1500, BounceInOut)))"#.to_string(),
        canvas: r#"🎨 EXTENDED DURATION 🎨

This effect prolongs the end by 300ms,
making it stay visible longer.

Use prolong_start() and prolong_end()
to extend effect timing:

• prolong_start() - delay before starting
• prolong_end() - hold after completion

Perfect for creating smooth transitions!"#
            .to_string(),
        tags: vec![
            "prolong".to_string(),
            "duration".to_string(),
            "timing".to_string(),
            "advanced".to_string(),
        ],
    }
}

// Showcase Examples
fn terminal_boot_sequence() -> Example {
    Example {
        id: "terminal_boot_sequence".to_string(),
        title: "Coalesce Formation".to_string(),
        description: "Text reforms from dissolution with style".to_string(),
        category: Category::Showcase,
        code: r#"fx::coalesce_from(Style::default().fg(Color::Green).bg(Color::Black), (1000, BackOut))"#.to_string(),
        canvas: r#"[BOOT] Initializing TachyonFX System...
[BOOT] Loading effect modules... OK
[BOOT] Compiling DSL engine... OK
[BOOT] Starting animation pipeline... OK
[BOOT] Initializing WebGL renderer... OK

SYSTEM READY.

Welcome to TachyonFX Terminal.
All systems operational."#
            .to_string(),
        tags: vec![
            "coalesce".to_string(),
            "reform".to_string(),
            "style".to_string(),
            "terminal".to_string(),
        ],
    }
}

fn glitch_effect() -> Example {
    Example {
        id: "glitch_effect".to_string(),
        title: "Never Complete".to_string(),
        description: "Effect that runs indefinitely".to_string(),
        category: Category::Showcase,
        code: r#"fx::never_complete(fx::hsl_shift_fg([0.0, 50.0, 0.0], (2000, SineInOut)))"#.to_string(),
        canvas: r#"G̴̰̈L̵̄I̶̓T̷̽C̸̈́H̴̾ ̵̎E̶̽F̷̈F̴̽E̵̐C̸̾T̷̏

Digital corruption in progress...

This effect never completes
Running infinitely with never_complete()

System instability detected
Reality.exe has stopped working

ERROR: REALITY_BUFFER_OVERFLOW"#
            .to_string(),
        tags: vec![
            "never_complete".to_string(),
            "infinite".to_string(),
            "hsl".to_string(),
            "digital".to_string(),
        ],
    }
}

fn demo_presentation() -> Example {
    Example {
        id: "demo_presentation".to_string(),
        title: "Expand Bidirectional".to_string(),
        description: "Expansive effect growing in multiple directions".to_string(),
        category: Category::Showcase,
        code: r#"fx::expand(ExpandDirection::Horizontal, Style::default().bg(Color::Magenta), (1500, ExpoOut))"#.to_string(),
        canvas: r#"🎭 TACHYONFX SHOWCASE 🎭

╔══════════════════════════════════════╗
║        FEATURE DEMONSTRATION         ║
╚══════════════════════════════════════╝

This example shows expansion effect:

Expands horizontally from center
Using a magenta background style
With exponential easing for drama

Creates a powerful reveal effect
Perfect for highlighting content!"#
            .to_string(),
        tags: vec![
            "expand".to_string(),
            "bidirectional".to_string(),
            "background".to_string(),
            "reveal".to_string(),
        ],
    }
}
