use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub code: &'static str,
    pub canvas: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
        basic::fade_from_fg(),
        basic::fade_to(),
        basic::fade_to_fg(),
        basic::fade_from(),
        basic::hsl_shift(),
        basic::hsl_shift_fg(),

        // TEXT/CHARACTER EFFECTS
        text::coalesce(),
        text::coalesce_from(),
        text::dissolve(),
        text::dissolve_to(),
        text::evolve(),
        text::evolve_into(),
        text::evolve_from(),
        text::sweep_in(),
        text::sweep_out(),
        text::slide_in(),
        text::slide_out(),
        text::stretch(),
        text::expand(),
        text::explode(),

        // TIMING AND CONTROL
        timing::delay(),
        timing::ping_pong(),
        timing::prolong_start(),
        timing::prolong_end(),
        timing::never_complete(),
        timing::repeat_times(),
        timing::repeat_forever(),
        timing::freeze_at(),
        timing::remap_alpha(),
        timing::with_duration(),
        timing::timed_never_complete(),

        // COMBINATION EFFECTS
        combination::parallel(),
        combination::sequence(),

        // GEOMETRY EFFECTS
        geometry::translate(),
        geometry::translate_buf(),
        geometry::resize_area(),

        // CUSTOM EFFECTS
        custom::effect_fn(),
        custom::effect_fn_buf(),
        custom::offscreen_buffer(),
        custom::dynamic_area(),
    ]
}

// basic examples
mod basic {
    use indoc::indoc;
    use super::*;

    pub fn fade_from_fg() -> Example {
        Example {
            id: "fade_from_fg",
            title: "Fade In",
            description: "Smoothly fade text from transparent to visible",
            category: Category::Basic,
            code: indoc! {"
                let bg = Color::from_u32(0x32302F);
                fx::fade_from_fg(bg, (1500, QuadOut))
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn fade_from() -> Example {
        Example {
            id: "fade_from",
            title: "Fade From Foreground",
            description: "Fades from specified foreground color",
            category: Category::Basic,
            code: indoc! {r#"
                // "screen area" bg, i.e. area outside the content area
                let bg = Color::from_u32(0x1D2021);

                // the default color space is HSL, which can look better
                // than RGB interpolation, usually produces artifacts when
                // dealing with grayish colors. (try changing to HSL or HSV)
                let linear_rgb = ColorSpace::Rgb;

                fx::fade_from(bg, bg, 1000)       // 1s, linear interpolation
                    .with_color_space(linear_rgb) // also the cheapest interpolation
            "#},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn fade_to() -> Example {
        Example {
            id: "fade_to",
            title: "Fade To Colors",
            description: "Fades to specified colors",
            category: Category::Basic,
            code: indoc! {"
                let screen_bg = Color::from_u32(0x1d2021);
                fx::fade_to(screen_bg, screen_bg, (1000, CircOut))
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn fade_to_fg() -> Example {
        Example {
            id: "fade_to_fg",
            title: "Fade To Foreground",
            description: "Fades to specified foreground color",
            category: Category::Basic,
            code: indoc! {"
                let target_color = Color::from_u32(0xfe8019);

                // limit effect to cells matching a specific color
                let filter = CellFilter::FgColor(Color::from_u32(0xebdbb2));
                let timer = (1000, Interpolation::CircOut);

                fx::fade_to_fg(target_color, timer)
                    .filter(filter)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn hsl_shift() -> Example {
        Example {
            id: "hsl_shift",
            title: "HSL Shift",
            description: "Changes hue, saturation, and lightness",
            category: Category::Basic,
            code: indoc! {"
                let timer = (1000, Interpolation::Linear);
                let fg_shift = [120.0, 25.0, 25.0];
                let bg_shift = [-40.0, -50.0, -50.0];
                fx::hsl_shift(Some(fg_shift), Some(bg_shift), timer)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn hsl_shift_fg() -> Example {
        Example {
            id: "hsl_shift_fg",
            title: "HSL Shift Foreground",
            description: "Changes foreground HSL values",
            category: Category::Basic,
            code: indoc! {"
                let timer = (1000, Interpolation::Linear);
                let fg_shift = [120.0, 25.0, 25.0];
                fx::hsl_shift(Some(fg_shift), None, timer)
            "},
            canvas: canvas::DEFAULT,
        }
    }
}

// text/character effects
mod text {
    use indoc::indoc;
    use super::*;

    pub fn coalesce() -> Example {
        Example {
            id: "coalesce",
            title: "Coalesce",
            description: "Reforms dissolved foreground",
            category: Category::Transitions,
            code: indoc! {"
                fx::coalesce((600, Interpolation::BounceInOut))
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn dissolve() -> Example {
        Example {
            id: "dissolve",
            title: "Dissolve",
            description: "Dissolves foreground content",
            category: Category::Transitions,
            code: indoc! {"
                fx::dissolve(1000) // linear interpolation
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn sweep_in() -> Example {
        Example {
            id: "sweep_in",
            title: "Sweep In",
            description: "Sweeps content with color",
            category: Category::Transitions,
            code: indoc! {"
                let c = Color::from_u32(0x1d2021);
                let timer = (1000, Interpolation::Linear);
                fx::sweep_in(Motion::LeftToRight, 10, 0, c, timer)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn sweep_out() -> Example {
        Example {
            id: "sweep_out",
            title: "Sweep Out",
            description: "Sweeps content with color",
            category: Category::Transitions,
            code: indoc! {"
                let c = Color::from_u32(0x1d2021);
                let timer = (1000, Interpolation::Linear);
                fx::sweep_out(Motion::RightToLeft, 10, 0, c, timer)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn slide_in() -> Example {
        Example {
            id: "slide_in",
            title: "Slide In",
            description: "Slides content with gradient",
            category: Category::Transitions,
            code: indoc! {"
                let c = Color::from_u32(0x1d2021);
                let timer = (1000, Interpolation::Linear);
                fx::slide_in(Motion::UpToDown, 10, 0, c, timer)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn slide_out() -> Example {
        Example {
            id: "slide_out",
            title: "Slide Out",
            description: "Slides content with gradient",
            category: Category::Transitions,
            code: indoc! {"
                let c = Color::from_u32(0x1d2021);
                let timer = (1000, Interpolation::Linear);
                fx::slide_out(Motion::LeftToRight, 24, 0, c, timer)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn stretch() -> Example {
        Example {
            id: "stretch",
            title: "Stretch",
            description: "Stretches unidirectionally using block chars",
            category: Category::Transitions,
            code: indoc! {"
                let content_area = Rect::new(12, 7, 80, 17);
                let style = Style::default()
                    .fg(Color::from_u32(0x32302F))
                    .bg(Color::from_u32(0x1D2021));

                fx::stretch(
                    Motion::LeftToRight,
                    style,
                    EffectTimer::from_ms(1000, Interpolation::BounceOut)
                ).with_area(content_area)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn explode() -> Example {
        Example {
            id: "explode",
            title: "Explode",
            description: "Explodes content outward",
            category: Category::Transitions,
            code: indoc! {"
                let timer = (1000, Linear);
                fx::parallel(&[
                    fx::fade_to_fg(Color::from_u32(0x404040), timer),
                    fx::explode(15.0, 2.0, timer),
                ])
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn coalesce_from() -> Example {
        Example {
            id: "coalesce_from",
            title: "Coalesce From",
            description: "Reforms from specified style",
            category: Category::Transitions,
            code: indoc! {"
                let c = Color::from_u32(0x1d2021);
                let style = Style::default().bg(c);
                fx::coalesce_from(style, (1000, Interpolation::ExpoInOut))
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn dissolve_to() -> Example {
        Example {
            id: "dissolve_to",
            title: "Dissolve To",
            description: "Dissolves to specified style",
            category: Category::Transitions,
            code: indoc! {"
                let c = Color::from_u32(0x1d2021);
                let style = Style::default().bg(c);
                fx::dissolve_to(style, (1000, Interpolation::CircOut))
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn evolve() -> Example {
        Example {
            id: "evolve",
            title: "Evolve",
            description: "Transforms characters through symbol sets",
            category: Category::Transitions,
            code: indoc! {"
                let content_area = Rect::new(12, 7, 80, 17);
                let timer = (600, CircIn);
                let style = Style::default()
                    .bg(Color::from_u32(0x32302F))  // content area bg
                    .fg(Color::from_u32(0x1D2021)); // screen area bg

                fx::evolve_from((EvolveSymbolSet::Quadrants, style), timer)
                    .with_pattern(DissolvePattern::new())
                    .with_area(content_area)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn evolve_into() -> Example {
        Example {
            id: "evolve_into",
            title: "Evolve Into",
            description: "Evolves into underlying content",
            category: Category::Transitions,
            code: indoc! {"
                let content_area = Rect::new(12, 7, 80, 17);

                let style = Style::default()
                    .fg(Color::from_u32(0x32302F))  // content area bg
                    .bg(Color::from_u32(0x1D2021)); // screen area bg

                let timer = EffectTimer::from_ms(1500, Interpolation::CubicOut);

                fx::evolve_into((EvolveSymbolSet::Shaded, style), timer)
                    .with_pattern(RadialPattern::center().with_transition_width(20.0))
                    .with_area(content_area)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn evolve_from() -> Example {
        Example {
            id: "evolve_from",
            title: "Evolve From",
            description: "Evolves from underlying content",
            category: Category::Transitions,
            code: indoc! {"
                let content_area = Rect::new(12, 7, 80, 17);

                fx::evolve_from(
                    (EvolveSymbolSet::BlocksHorizontal, Style::default().fg(Color::Green)),
                    EffectTimer::from_ms(1500, Interpolation::QuadOut)
                ).with_pattern(CheckerboardPattern::default())
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn expand() -> Example {
        Example {
            id: "expand",
            title: "Expand",
            description: "Expands bidirectionally from center",
            category: Category::Transitions,
            code: indoc! {"
                let content_area = Rect::new(12, 7, 80, 17);
                let style = Style::default()
                    .fg(Color::from_u32(0x32302F))
                    .bg(Color::from_u32(0x1D2021));

                fx::expand(
                    ExpandDirection::Vertical,
                    style,
                    EffectTimer::from_ms(1000, Interpolation::BounceOut)
                ).with_area(content_area)
            "},
            canvas: canvas::DEFAULT,
        }
    }
}

// timing and control effects
mod timing {
    use indoc::indoc;
    use super::*;

    pub fn delay() -> Example {
        Example {
            id: "delay",
            title: "Delay",
            description: "Delays effect by specified duration",
            category: Category::Advanced,
            code: indoc! {"
                // wait 800ms before dissolving the content
                fx::delay(800, fx::dissolve(200))
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn ping_pong() -> Example {
        Example {
            id: "ping_pong",
            title: "Ping Pong",
            description: "Plays effect forward then backward",
            category: Category::Advanced,
            code: indoc! {"
                let timer = (500, Interpolation::CircOut);
                fx::ping_pong(fx::coalesce(timer))
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn prolong_start() -> Example {
        Example {
            id: "prolong_start",
            title: "Prolong Start",
            description: "Extends effect duration at start",
            category: Category::Advanced,
            code: indoc! {"
                let c = Color::from_u32(0x504945);
                let timer = (500, Interpolation::CircOut);
                fx::prolong_start(timer, fx::fade_from_fg(c, timer))
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn prolong_end() -> Example {
        Example {
            id: "prolong_end",
            title: "Prolong End",
            description: "Extends effect duration at end",
            category: Category::Advanced,
            code: indoc! {"
                let c = Color::from_u32(0x504945);
                let timer = (500, Interpolation::CircOut);
                fx::prolong_end(timer, fx::fade_to_fg(c, timer))
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn never_complete() -> Example {
        Example {
            id: "never_complete",
            title: "Never Complete",
            description: "Makes effect run indefinitely",
            category: Category::Advanced,
            code: indoc! {"
                // Create a permanent color change over 1 second
                let fade = fx::fade_to_fg(Color::Red, EffectTimer::from_ms(1000, Interpolation::Linear));
                fx::never_complete(fade)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn repeat_times() -> Example {
        Example {
            id: "repeat_times",
            title: "Repeat Times",
            description: "Repeats effect by count",
            category: Category::Advanced,
            code: indoc! {"
                // Repeat a fade effect 3 times
                let fade = fx::fade_to_fg(Color::Red, EffectTimer::from_ms(1000, Interpolation::Linear));
                fx::repeat(fade, RepeatMode::Times(3))
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn repeat_forever() -> Example {
        Example {
            id: "repeat_forever",
            title: "Repeat Forever",
            description: "Repeats effect indefinitely",
            category: Category::Advanced,
            code: indoc! {"
                // Create an endless color cycling effect
                let fade = fx::fade_to_fg(Color::Red, EffectTimer::from_ms(1000, Interpolation::Linear));
                fx::repeating(fade)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn freeze_at() -> Example {
        Example {
            id: "freeze_at",
            title: "Freeze At",
            description: "Freezes effect at specific alpha value",
            category: Category::Advanced,
            code: indoc! {"
                let fade_effect = fx::fade_to_fg(Color::Red, 1000);
                fx::freeze_at(0.5, false, fade_effect)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn remap_alpha() -> Example {
        Example {
            id: "remap_alpha",
            title: "Remap Alpha",
            description: "Remaps effect's alpha progression",
            category: Category::Advanced,
            code: indoc! {"
                let fade_effect = fx::fade_to_fg(Color::Red, 1000);
                fx::remap_alpha(0.2, 0.8, fade_effect)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn with_duration() -> Example {
        Example {
            id: "with_duration",
            title: "With Duration",
            description: "Applies duration limit to effect",
            category: Category::Advanced,
            code: indoc! {"
                let effect = fx::fade_to_fg(Color::Red, 2000);
                fx::with_duration(Duration::from_secs(1), effect)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn timed_never_complete() -> Example {
        Example {
            id: "timed_never_complete",
            title: "Timed Never Complete",
            description: "Makes effect run indefinitely with time limit",
            category: Category::Advanced,
            code: indoc! {"
                let effect = fx::fade_to_fg(Color::Red, 1000);
                fx::timed_never_complete(Duration::from_secs(5), effect)
            "},
            canvas: canvas::DEFAULT,
        }
    }
}

// combination effects
mod combination {
    use indoc::indoc;
    use super::*;

    pub fn parallel() -> Example {
        Example {
            id: "parallel",
            title: "Parallel",
            description: "Runs effects simultaneously",
            category: Category::Showcase,
            code: indoc! {"
                let c = Color::from_u32(0x504945);
                let timer = (1000, Interpolation::CircOut);
                fx::parallel(&[
                    fx::fade_from_fg(c, timer),
                    fx::coalesce(timer),
                ])
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn sequence() -> Example {
        Example {
            id: "sequence",
            title: "Sequence",
            description: "Runs effects sequentially",
            category: Category::Showcase,
            code: indoc! {"
                let c = Color::from_u32(0x504945);
                let timer = (500, Interpolation::CircOut);
                fx::sequence(&[
                    fx::fade_from_fg(c, timer),
                    fx::dissolve(timer),
                ])
            "},
            canvas: canvas::DEFAULT,
        }
    }
}

// custom effects
mod custom {
    use indoc::indoc;
    use super::*;

    pub fn effect_fn() -> Example {
        Example {
            id: "effect_fn",
            title: "Custom Effect Function",
            description: "Custom effects with cell iterator",
            category: Category::Advanced,
            code: indoc! {"
                fx::effect_fn(Instant::now(), 1000, |state, _ctx, cell_iter| {
                    let cycle: f32 = (state.elapsed().as_millis() % 3600) as f32;
                    cell_iter
                        .filter(|(_, cell)| cell.symbol() != \" \")
                        .enumerate()
                        .for_each(|(i, (_pos, cell))| {
                            let hue = (2.0 * i as f32 + cycle * 0.2) % 360.0;
                            let color = color_from_hsl(hue, 100.0, 50.0);
                            cell.set_fg(color);
                    });
                })
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn effect_fn_buf() -> Example {
        Example {
            id: "effect_fn_buf",
            title: "Custom Effect Buffer",
            description: "Custom effects with buffer",
            category: Category::Advanced,
            code: indoc! {"
                let timer = EffectTimer::from_ms(1000, Interpolation::Linear);
                let no_state = ();

                fx::effect_fn_buf(no_state, timer, |_state, context, buf| {
                    let offset = context.timer.remaining().as_millis() as usize;

                    let filter = context.filter();
                    let cell_pred = filter.map(FilterProcessor::validator);

                    for (i, pos) in buf.area.positions().enumerate() {
                        let cell = &mut buf[pos];
                        if cell_pred.as_ref().is_some_and(|p| p.is_valid(pos, &cell)) {
                            cell.set_fg(Color::Indexed(((offset + i) % 256) as u8));
                        }
                    }
                }).filter(CellFilter::Text)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn offscreen_buffer() -> Example {
        Example {
            id: "offscreen_buffer",
            title: "Offscreen Buffer",
            description: "Renders to separate buffer",
            category: Category::Advanced,
            code: indoc! {"
                let area = Rect::new(0, 0, 80, 24);
                let offscreen_buffer = ref_count(Buffer::empty(area));
                let fade_effect = fx::fade_to_fg(Color::Red, EffectTimer::from_ms(1000, Interpolation::Linear));
                fx::offscreen_buffer(fade_effect, offscreen_buffer.clone())
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn dynamic_area() -> Example {
        Example {
            id: "dynamic_area",
            title: "Dynamic Area",
            description: "Wraps effects for responsive layouts",
            category: Category::Advanced,
            code: indoc! {"
                let area_ref = RefRect::new(Rect::new(0, 0, 20, 5));
                let fade_effect = fx::fade_to_fg(Color::Red, EffectTimer::from_ms(1000, Interpolation::Linear));
                fx::dynamic_area(area_ref.clone(), fade_effect)
            "},
            canvas: canvas::DEFAULT,
        }
    }
}

// geometry effects
mod geometry {
    use indoc::indoc;
    use super::*;

    pub fn translate() -> Example {
        Example {
            id: "translate",
            title: "Translate",
            description: "Moves effect area",
            category: Category::Advanced,
            code: indoc! {"
                let timer = EffectTimer::from_ms(1000, Interpolation::Linear);
                let effect = fx::fade_to_fg(Color::Red, timer);
                fx::translate(Some(effect), (5, 10), timer)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn translate_buf() -> Example {
        Example {
            id: "translate_buf",
            title: "Translate Buffer",
            description: "Moves buffer contents",
            category: Category::Advanced,
            code: indoc! {"
                let area = Rect::new(0, 0, 10, 10);
                let mut buf = Buffer::empty(area);
                Block::bordered()
                    .title(\"translated\")
                    .render(area, &mut buf);

                let timer = EffectTimer::from_ms(1000, Interpolation::Linear);
                fx::translate_buf(Offset { x: -30, y: 0 }, ref_count(buf), timer)
            "},
            canvas: canvas::DEFAULT,
        }
    }

    pub fn resize_area() -> Example {
        Example {
            id: "resize_area",
            title: "Resize Area",
            description: "Resizes effect area",
            category: Category::Advanced,
            code: indoc! {"
                let timer = EffectTimer::from_ms(2000, Interpolation::CubicInOut);
                let effect = fx::fade_to_fg(Color::Blue, timer);
                fx::resize_area(Some(effect), Size::new(20, 10), timer)
            "},
            canvas: canvas::DEFAULT,
        }
    }
}





mod canvas {
    pub const DEFAULT: &'static str = include_str!("../assets/default_canvas.ansi");
}