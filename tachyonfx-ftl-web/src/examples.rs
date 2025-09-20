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
        code: r#"fade_in(Duration::from_ms(1500))
    .with_interpolation(QuadOut)"#
            .to_string(),
        canvas: r#"Welcome to TachyonFX!

This text will fade in smoothly
using a quadratic easing curve.

Perfect for gentle introductions!"#
            .to_string(),
        tags: vec!["fade".to_string(), "basic".to_string(), "alpha".to_string()],
    }
}

fn basic_fade_out() -> Example {
    Example {
        id: "basic_fade_out".to_string(),
        title: "Fade Out".to_string(),
        description: "Gradually fade text to transparent".to_string(),
        category: Category::Basic,
        code: r#"fade_out(Duration::from_ms(2000))
    .with_interpolation(SineIn)"#
            .to_string(),
        canvas: r#"Goodbye, TachyonFX!

This text will fade out gently
using a sine wave easing.

Watch it disappear..."#
            .to_string(),
        tags: vec!["fade".to_string(), "basic".to_string(), "alpha".to_string()],
    }
}

fn basic_slide_left() -> Example {
    Example {
        id: "basic_slide_left".to_string(),
        title: "Slide from Left".to_string(),
        description: "Text slides in from the left side".to_string(),
        category: Category::Basic,
        code: r#"slide_in_left(Duration::from_ms(1000))
    .with_interpolation(BackOut)"#
            .to_string(),
        canvas: r#"▶ Sliding from the left!

Watch this content move smoothly
from off-screen to its position.

The BackOut easing adds a nice bounce!"#
            .to_string(),
        tags: vec![
            "slide".to_string(),
            "movement".to_string(),
            "basic".to_string(),
        ],
    }
}

fn basic_slide_right() -> Example {
    Example {
        id: "basic_slide_right".to_string(),
        title: "Slide from Right".to_string(),
        description: "Text slides in from the right side".to_string(),
        category: Category::Basic,
        code: r#"slide_in_right(Duration::from_ms(1200))
    .with_interpolation(ElasticOut)"#
            .to_string(),
        canvas: r#"                    ◀ Sliding from the right!

           Content can slide from any direction.

           ElasticOut creates a springy effect
           that feels organic and playful!"#
            .to_string(),
        tags: vec![
            "slide".to_string(),
            "movement".to_string(),
            "elastic".to_string(),
        ],
    }
}

fn basic_typewriter() -> Example {
    Example {
        id: "basic_typewriter".to_string(),
        title: "Typewriter Effect".to_string(),
        description: "Text appears character by character like typing".to_string(),
        category: Category::Basic,
        code: r#"typewriter(Duration::from_ms(2500))
    .with_interpolation(Linear)"#
            .to_string(),
        canvas: r#"This is a typewriter effect.

Characters appear one by one,
just like someone is typing them
in real time.

Perfect for storytelling!"#
            .to_string(),
        tags: vec![
            "typewriter".to_string(),
            "text".to_string(),
            "reveal".to_string(),
        ],
    }
}

// Transition Examples
fn fade_slide_combo() -> Example {
    Example {
        id: "fade_slide_combo".to_string(),
        title: "Fade + Slide Combination".to_string(),
        description: "Combines fading with sliding for smooth transitions".to_string(),
        category: Category::Transitions,
        code: r#"fade_in(Duration::from_ms(800))
    .and(slide_in_left(Duration::from_ms(1000)))
    .with_interpolation(CubicOut)"#
            .to_string(),
        canvas: r#"◆ Smooth Combined Motion

This text both fades in AND slides,
creating a sophisticated entrance effect.

Multiple effects can work together
to create richer animations!"#
            .to_string(),
        tags: vec![
            "fade".to_string(),
            "slide".to_string(),
            "combination".to_string(),
        ],
    }
}

fn cross_fade_transition() -> Example {
    Example {
        id: "cross_fade_transition".to_string(),
        title: "Cross Fade Transition".to_string(),
        description: "One text fades out while another fades in".to_string(),
        category: Category::Transitions,
        code: r#"fade_out(Duration::from_ms(1000))
    .then(
        fade_in(Duration::from_ms(1000))
    )"#
        .to_string(),
        canvas: r#"Scene 1: Initial Content

This will fade out and be replaced
by new content in a smooth transition.

Great for scene changes!"#
            .to_string(),
        tags: vec![
            "fade".to_string(),
            "transition".to_string(),
            "sequence".to_string(),
        ],
    }
}

fn slide_sequence() -> Example {
    Example {
        id: "slide_sequence".to_string(),
        title: "Sequential Slides".to_string(),
        description: "Multiple slide effects in sequence".to_string(),
        category: Category::Transitions,
        code: r#"slide_in_left(Duration::from_ms(500))
    .then(slide_in_right(Duration::from_ms(500)))
    .then(slide_in_up(Duration::from_ms(500)))
    .with_interpolation(BounceOut)"#
            .to_string(),
        canvas: r#"▲ Multi-directional slides!

First from left → then right →
finally from bottom ↑

Each direction creates a different
visual rhythm and flow."#
            .to_string(),
        tags: vec![
            "slide".to_string(),
            "sequence".to_string(),
            "multi-direction".to_string(),
        ],
    }
}

// Animation Examples
fn bouncing_text() -> Example {
    Example {
        id: "bouncing_text".to_string(),
        title: "Bouncing Animation".to_string(),
        description: "Text with a bouncy, playful animation".to_string(),
        category: Category::Animations,
        code: r#"slide_in_up(Duration::from_ms(800))
    .with_interpolation(BounceOut)
    .repeat(3)
    .with_delay(Duration::from_ms(200))"#
            .to_string(),
        canvas: r#"🎾 BOUNCE! BOUNCE! BOUNCE!

This text has serious spring in its step!

The BounceOut interpolation creates
a physics-like bouncing effect.

Try adjusting the repeat count!"#
            .to_string(),
        tags: vec![
            "bounce".to_string(),
            "repeat".to_string(),
            "physics".to_string(),
        ],
    }
}

fn wave_effect() -> Example {
    Example {
        id: "wave_effect".to_string(),
        title: "Wave Motion".to_string(),
        description: "Text flows like a wave with sine interpolation".to_string(),
        category: Category::Animations,
        code: r#"slide_in_left(Duration::from_ms(2000))
    .with_interpolation(SineInOut)
    .repeat(Float::INFINITY)"#
            .to_string(),
        canvas: r#"🌊 Flowing like water...

∿∿∿ Wave motion creates organic movement ∿∿∿

The sine wave interpolation feels natural,
like breathing or ocean waves.

This pattern repeats infinitely!"#
            .to_string(),
        tags: vec![
            "wave".to_string(),
            "sine".to_string(),
            "infinite".to_string(),
            "organic".to_string(),
        ],
    }
}

fn matrix_rain() -> Example {
    Example {
        id: "matrix_rain".to_string(),
        title: "Matrix Rain Effect".to_string(),
        description: "Classic falling text effect like The Matrix".to_string(),
        category: Category::Animations,
        code: r#"slide_in_up(Duration::from_ms(3000))
    .with_interpolation(Linear)
    .repeat(Float::INFINITY)
    .with_delay(Duration::from_ms(100))"#
            .to_string(),
        canvas: r#"01001000 01100101 01101100 01101100 01101111
11000001 10110111 11010011 10001101 11100010
01010100 01100001 01100011 01101000 01111001
11011101 11001110 11111000 10110001 10101010
01000110 01011000 00100000 01110010 01100001
11001011 10110101 11000111 11101001 11001100

Wake up, Neo... Follow the white rabbit.
The Matrix has you..."#
            .to_string(),
        tags: vec![
            "matrix".to_string(),
            "binary".to_string(),
            "rain".to_string(),
            "linear".to_string(),
        ],
    }
}

fn rotating_text() -> Example {
    Example {
        id: "rotating_text".to_string(),
        title: "Rotating Animation".to_string(),
        description: "Text that rotates with custom timing".to_string(),
        category: Category::Animations,
        code: r#"rotate(Duration::from_ms(2000))
    .with_interpolation(QuartInOut)
    .repeat(2)"#
            .to_string(),
        canvas: r#"⟲ ROTATION IN MOTION ⟳

This text spins around its center
with a QuartInOut easing curve.

Watch how it starts slow,
speeds up in the middle,
then slows down again!"#
            .to_string(),
        tags: vec![
            "rotate".to_string(),
            "spin".to_string(),
            "custom-timing".to_string(),
        ],
    }
}

// Advanced Examples
fn parallel_effects() -> Example {
    Example {
        id: "parallel_effects".to_string(),
        title: "Parallel Effect Chains".to_string(),
        description: "Multiple effects running simultaneously".to_string(),
        category: Category::Advanced,
        code: r#"parallel([
    fade_in(Duration::from_ms(1000)),
    slide_in_left(Duration::from_ms(1200)),
    scale_in(Duration::from_ms(800))
])
.with_interpolation(BackOut)"#
            .to_string(),
        canvas: r#"⚡ PARALLEL PROCESSING ⚡

This text experiences multiple effects
happening at the same time:

• Fading in (alpha change)
• Sliding from left (position change)
• Scaling up (size change)

All synchronized but with different durations!"#
            .to_string(),
        tags: vec![
            "parallel".to_string(),
            "multi-effect".to_string(),
            "synchronization".to_string(),
        ],
    }
}

fn conditional_timing() -> Example {
    Example {
        id: "conditional_timing".to_string(),
        title: "Conditional Timing".to_string(),
        description: "Effects that adapt based on conditions".to_string(),
        category: Category::Advanced,
        code: r#"if_condition(content.len() > 100,
    typewriter(Duration::from_ms(3000)),
    fade_in(Duration::from_ms(1000))
)
.with_delay(Duration::from_ms(500))"#
            .to_string(),
        canvas: r#"📊 ADAPTIVE EFFECTS 📊

This effect changes behavior based on
the content length:

• Long content → Typewriter effect
• Short content → Simple fade in

The system can make decisions about
which animation to use!"#
            .to_string(),
        tags: vec![
            "conditional".to_string(),
            "adaptive".to_string(),
            "logic".to_string(),
        ],
    }
}

fn custom_interpolations() -> Example {
    Example {
        id: "custom_interpolations".to_string(),
        title: "Custom Interpolation Curves".to_string(),
        description: "Using advanced easing functions for unique motion".to_string(),
        category: Category::Advanced,
        code: r#"slide_in_left(Duration::from_ms(1500))
    .with_interpolation(
        bezier(0.68, -0.55, 0.265, 1.55)
    )
    .then(
        bounce_in_place(Duration::from_ms(500))
    )"#
        .to_string(),
        canvas: r#"🎨 CUSTOM CURVES 🎨

This uses a custom Bézier curve
for a unique motion feel:

The curve creates anticipation
(pulls back first)
then overshoots the target
before settling in place.

Fine-tune the motion to match your vision!"#
            .to_string(),
        tags: vec![
            "bezier".to_string(),
            "custom".to_string(),
            "curves".to_string(),
            "advanced".to_string(),
        ],
    }
}

// Showcase Examples
fn terminal_boot_sequence() -> Example {
    Example {
        id: "terminal_boot_sequence".to_string(),
        title: "Terminal Boot Sequence".to_string(),
        description: "Simulates a computer boot-up with multiple stages".to_string(),
        category: Category::Showcase,
        code: r#"sequence([
    typewriter(Duration::from_ms(1000)),
    pause(Duration::from_ms(500)),
    typewriter(Duration::from_ms(800)),
    pause(Duration::from_ms(300)),
    fade_in(Duration::from_ms(400))
])
.with_interpolation(Linear)"#
            .to_string(),
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
            "boot".to_string(),
            "sequence".to_string(),
            "terminal".to_string(),
            "typewriter".to_string(),
        ],
    }
}

fn glitch_effect() -> Example {
    Example {
        id: "glitch_effect".to_string(),
        title: "Digital Glitch".to_string(),
        description: "Cyberpunk-style glitch effect with distortion".to_string(),
        category: Category::Showcase,
        code: r#"glitch(Duration::from_ms(2000))
    .with_intensity(0.7)
    .with_color_shift(true)
    .repeat(3)
    .with_delay(Duration::from_ms(800))"#
            .to_string(),
        canvas: r#"G̴̰̈L̵̄I̶̓T̷̽C̸̈́H̴̾ ̵̎E̶̽F̷̈F̴̽E̵̐C̸̾T̷̏

Ḑ̷i̸g̶i̷t̴a̸l̵ ̶c̷o̶r̸r̶u̸p̵t̸i̷o̶n̴ ̸i̶n̵ ̶p̸r̶o̸g̷r̶e̶s̸s̷.̴.̷.̶

T̷h̴e̸ ̶m̵a̸t̷r̶i̵x̴ ̸i̶s̷ ̵b̶r̸e̷a̶k̸i̵n̷g̶ ̴d̵o̶w̸n̴
S̸y̴s̵t̶e̸m̶ ̷i̴n̸s̵t̶a̵b̸i̷l̸i̶t̵y̴ ̶d̸e̷t̸e̸c̴t̸e̷d̵
R̷e̸a̶l̸i̷t̸y̴.̵e̶x̸e̵ ̶h̸a̴s̶ ̷s̸t̴o̶p̸p̸e̵d̴ ̷w̸o̶r̶k̸i̸n̶g̵

ERROR: REALITY_BUFFER_OVERFLOW"#
            .to_string(),
        tags: vec![
            "glitch".to_string(),
            "cyberpunk".to_string(),
            "corruption".to_string(),
            "digital".to_string(),
        ],
    }
}

fn demo_presentation() -> Example {
    Example {
        id: "demo_presentation".to_string(),
        title: "Feature Showcase".to_string(),
        description: "Comprehensive demo showing multiple TachyonFX capabilities".to_string(),
        category: Category::Showcase,
        code: r#"sequence([
    fade_in(Duration::from_ms(1000)),
    pause(Duration::from_ms(800)),
    slide_in_left(Duration::from_ms(1200))
        .with_interpolation(BackOut),
    pause(Duration::from_ms(600)),
    typewriter(Duration::from_ms(2000)),
    pause(Duration::from_ms(400)),
    parallel([
        rotate(Duration::from_ms(1000)),
        scale_pulse(Duration::from_ms(1000))
    ]),
    fade_out(Duration::from_ms(800))
])"#
        .to_string(),
        canvas: r#"🎭 TACHYONFX SHOWCASE 🎭

╔══════════════════════════════════════╗
║        FEATURE DEMONSTRATION         ║
╚══════════════════════════════════════╝

This example combines multiple effects:

1. Fade In    → Gentle introduction
2. Slide      → Dynamic movement
3. Typewriter → Character-by-character reveal
4. Rotation   → Spinning transformation
5. Scaling    → Size pulsing
6. Fade Out   → Elegant exit

Each effect can be combined, sequenced,
and customized to create rich animations!"#
            .to_string(),
        tags: vec![
            "showcase".to_string(),
            "demo".to_string(),
            "comprehensive".to_string(),
            "multi-effect".to_string(),
        ],
    }
}
