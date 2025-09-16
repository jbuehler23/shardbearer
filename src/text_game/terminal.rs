use crate::screens::Screen;
use crate::text_game::{
    game::{Game, GameUI, InputEvent},
    story::{BANNER, Story},
};
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::text::LineHeight;

#[derive(Component)]
pub struct GameTextDisplay;

#[derive(Component)]
pub struct InputPrompt;

#[derive(Component)]
pub struct ContentSection {
    pub section_type: ContentType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Banner,
    StoryText,
    Choices,
    Messages,
}

#[derive(Component)]
pub struct ChoiceItem {
    pub index: usize,
}

#[derive(Component)]
pub struct ScrollContainer;

#[derive(Component)]
pub struct TerminalContainer;

#[derive(Component)]
pub struct CursorBlink {
    timer: Timer,
    visible: bool,
}

#[derive(Component)]
pub struct TypewriterEffect {
    full_text: String,
    current_index: usize,
    timer: Timer,
    complete: bool,
}

#[derive(Component)]
pub struct ScanlineEffect;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup_terminal);
    app.add_systems(
        Update,
        (
            ui_render_system,
            terminal_input_system,
            cursor_blink_system,
            typewriter_effect_system,
            scanline_animation_system,
        )
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(OnExit(Screen::Gameplay), cleanup_terminal);
}

fn setup_terminal(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn terminal container with CRT-style background
    commands
        .spawn((
            Name::new("Terminal Container"),
            TerminalContainer,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.01, 0.01, 0.05)), // Very dark blue-black
        ))
        .with_children(|parent| {
            // Scrollable game content area
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(85.0),
                        max_width: Val::Px(1200.0),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.02, 0.05, 0.9)),
                    ScrollContainer,
                ))
                .with_children(|scroll| {
                    // Content will be dynamically added here
                    scroll.spawn((
                        Text("Initializing SHARDBEARER...".to_string()),
                        TextFont {
                            font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                            font_size: 14.0,
                            font_smoothing: default(),
                            line_height: LineHeight::RelativeToFont(1.2),
                        },
                        TextColor(Color::srgb(0.0, 1.0, 0.8)),
                        Node {
                            padding: UiRect::all(Val::Px(15.0)),
                            min_height: Val::Px(600.0), // Force minimum height for overflow
                            ..default()
                        },
                        GameTextDisplay,
                        TypewriterEffect {
                            full_text: "Initializing SHARDBEARER...".to_string(),
                            current_index: 0,
                            timer: Timer::from_seconds(0.001, TimerMode::Repeating), // Fast but visible
                            complete: false,
                        },
                    ));
                });

            // Input area at bottom
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        max_width: Val::Px(1200.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        padding: UiRect::axes(Val::Px(15.0), Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.02, 0.05, 0.95)),
                    BorderColor::all(Color::srgb(0.0, 1.0, 0.8)),
                    BorderRadius::all(Val::Px(2.0)),
                ))
                .with_children(|input_area| {
                    // Input prompt with blinking cursor
                    input_area.spawn((
                        Text("> _".to_string()),
                        TextFont {
                            font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                            font_size: 16.0,
                            font_smoothing: default(),
                            line_height: LineHeight::RelativeToFont(1.2),
                        },
                        TextColor(Color::srgb(0.0, 1.0, 0.8)),
                        InputPrompt,
                        CursorBlink {
                            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                            visible: true,
                        },
                    ));
                });
        });

    // Add scanline effect overlay
    commands.spawn((
        Name::new("Scanline Effect"),
        ScanlineEffect,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Px(2.0),
            top: Val::Px(0.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 1.0, 0.8, 0.05)),
    ));
}

fn ui_render_system(
    story: Res<Story>,
    mut game: ResMut<Game>,
    mut ui: ResMut<GameUI>,
    mut text_query: Query<
        (&mut Text, Option<&mut TypewriterEffect>),
        (With<GameTextDisplay>, Without<InputPrompt>),
    >,
) {
    if !game.should_redraw {
        return;
    }

    // Clear selection state when starting a new render
    let mut display_text = String::new();

    // Only show banner on the start screen
    if game.current == "start" {
        display_text.push_str(&BANNER);
        display_text.push_str("\n\n");
    }

    if let Some(node) = story.nodes.get(game.current) {
        if let Some(ascii) = node.ascii {
            display_text.push_str(ascii);
            display_text.push_str("\n");
        }
        display_text.push_str(&format!("═══ {} ═══\n\n", node.title.to_uppercase()));

        // Format body text with proper line wrapping
        let body_lines: Vec<&str> = node.body.split('\n').collect();
        for line in body_lines {
            display_text.push_str(line);
            display_text.push_str("\n");
        }
        display_text.push_str("\n");

        if let Some(end) = &node.ending {
            display_text.push_str("╔════════════════════════════════════════╗\n");
            display_text.push_str(&format!(
                "║  ENDING UNLOCKED: {}  ║\n",
                format!("{:^20}", end.name).to_uppercase()
            ));
            display_text.push_str("╚════════════════════════════════════════╝\n\n");
            display_text.push_str(&format!("{}\n\n", end.summary));
            display_text.push_str("┌─────────────────────────────┐\n");
            display_text.push_str("│  What now?                  │\n");
            display_text.push_str("│  1) Reboot from the alley   │\n");
            display_text.push_str("│  2) Show unlocked endings   │\n");
            display_text.push_str("│  3) Quit                    │\n");
            display_text.push_str("└─────────────────────────────┘\n");
        } else {
            // Find the max width needed for choices
            let max_width = node.choices.iter()
                .map(|c| c.text.len())
                .max()
                .unwrap_or(0)
                .min(65); // Cap at 65 chars wide for better readability

            let box_width = max_width + 6; // Add padding for number and spaces
            let top_border = format!("┌─ CHOICES {}┐\n", "─".repeat(box_width.saturating_sub(11)));
            let bottom_border = format!("└{}┘\n", "─".repeat(box_width));

            // Update UI state for choices
            ui.num_choices = node.choices.len();

            display_text.push_str(&top_border);
            for (i, choice) in node.choices.iter().enumerate() {
                // Wrap long text properly
                let text = if choice.text.len() > max_width {
                    let mut wrapped = choice.text.chars().take(max_width - 3).collect::<String>();
                    wrapped.push_str("...");
                    wrapped
                } else {
                    choice.text.to_string()
                };

                // Highlight selected choice
                let is_selected = ui.selected_choice == Some(i);
                let prefix = if is_selected { "► " } else { "  " };
                let choice_text = if is_selected {
                    format!("{}[{}] {}", prefix, i + 1, text)
                } else {
                    format!("{}{}) {}", prefix, i + 1, text)
                };

                display_text.push_str(&format!(
                    "│{:<width$}│\n",
                    choice_text,
                    width = max_width + 4
                ));
            }
            display_text.push_str(&bottom_border);

            // Add selection hint if a choice is selected
            if ui.selected_choice.is_some() {
                display_text.push_str("\n💡 Press ENTER to confirm selection, ESC to cancel\n");
            }
        }

        let unlocked = game.endings_unlocked.len();
        if unlocked > 0 {
            display_text.push_str(&format!("\n[ENDINGS UNLOCKED: {}/7]\n", unlocked));
        }

        if let Some(msg) = &game.message {
            display_text.push_str(&format!("\n▶ SYSTEM: {}\n", msg.to_uppercase()));
        }
    } else {
        display_text.push_str(&format!("[ERROR: MISSING NODE '{}']\n", game.current));
    }

    // Update display text with typewriter effect
    if let Ok((mut text, typewriter)) = text_query.single_mut() {
        if let Some(mut effect) = typewriter {
            // For selection changes, update the full text but keep the animation state
            // Only reset animation for actual content changes (not selection highlights)
            let content_changed = !effect.full_text.contains(&display_text[..display_text.len().min(50)]);

            if content_changed {
                // Content actually changed - reset typewriter
                effect.full_text = display_text.clone();
                effect.current_index = 0;
                effect.complete = false;
                effect.timer.reset();
            } else if effect.complete {
                // Selection change only - update immediately without animation
                text.0 = display_text.clone();
                effect.full_text = display_text.clone();
            }
        } else {
            text.0 = display_text;
        }
    }

    // Input prompt is now handled by cursor_blink_system

    game.message = None;
    game.should_redraw = false;
}

fn terminal_input_system(
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut input_events: MessageWriter<InputEvent>,
    mut ui: ResMut<GameUI>,
    mut game: ResMut<Game>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match event.key_code {
            KeyCode::Enter => {
                // Check if we have a selected choice
                if let Some(choice_index) = ui.selected_choice {
                    input_events.write(InputEvent((choice_index + 1).to_string()));
                    ui.selected_choice = None;
                } else if !ui.input_buffer.trim().is_empty() {
                    let input = ui.input_buffer.trim().to_string();
                    input_events.write(InputEvent(input));
                    ui.input_buffer.clear();
                }
            }
            KeyCode::Escape => {
                ui.selected_choice = None;
                game.should_redraw = true;
            }
            KeyCode::ArrowUp => {
                if ui.num_choices > 0 {
                    ui.selected_choice = Some(match ui.selected_choice {
                        Some(i) if i > 0 => i - 1,
                        Some(_) => ui.num_choices - 1, // Wrap to last (covers i == 0 and any other case)
                        None => ui.num_choices - 1,
                    });
                    game.should_redraw = true;
                }
            }
            KeyCode::ArrowDown => {
                if ui.num_choices > 0 {
                    ui.selected_choice = Some(match ui.selected_choice {
                        Some(i) if i < ui.num_choices - 1 => i + 1,
                        Some(_) => 0, // Wrap to first
                        None => 0,
                    });
                    game.should_redraw = true;
                }
            }
            KeyCode::Backspace => {
                ui.input_buffer.pop();
            }
            KeyCode::Digit1 | KeyCode::Numpad1 => {
                if ui.num_choices > 0 {
                    ui.selected_choice = Some(0);
                    game.should_redraw = true;
                } else {
                    input_events.write(InputEvent("1".to_string()));
                }
            }
            KeyCode::Digit2 | KeyCode::Numpad2 => {
                if ui.num_choices > 1 {
                    ui.selected_choice = Some(1);
                    game.should_redraw = true;
                } else {
                    input_events.write(InputEvent("2".to_string()));
                }
            }
            KeyCode::Digit3 | KeyCode::Numpad3 => {
                if ui.num_choices > 2 {
                    ui.selected_choice = Some(2);
                    game.should_redraw = true;
                } else {
                    input_events.write(InputEvent("3".to_string()));
                }
            }
            KeyCode::Digit4 | KeyCode::Numpad4 => {
                if ui.num_choices > 3 {
                    ui.selected_choice = Some(3);
                    game.should_redraw = true;
                } else {
                    input_events.write(InputEvent("4".to_string()));
                }
            }
            KeyCode::Space => {
                ui.input_buffer.push(' ');
            }
            KeyCode::KeyA => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'A' } else { 'a' });
            }
            KeyCode::KeyB => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'B' } else { 'b' });
            }
            KeyCode::KeyC => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'C' } else { 'c' });
            }
            KeyCode::KeyD => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'D' } else { 'd' });
            }
            KeyCode::KeyE => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'E' } else { 'e' });
            }
            KeyCode::KeyF => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'F' } else { 'f' });
            }
            KeyCode::KeyG => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'G' } else { 'g' });
            }
            KeyCode::KeyH => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'H' } else { 'h' });
            }
            KeyCode::KeyI => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'I' } else { 'i' });
            }
            KeyCode::KeyJ => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'J' } else { 'j' });
            }
            KeyCode::KeyK => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'K' } else { 'k' });
            }
            KeyCode::KeyL => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'L' } else { 'l' });
            }
            KeyCode::KeyM => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'M' } else { 'm' });
            }
            KeyCode::KeyN => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'N' } else { 'n' });
            }
            KeyCode::KeyO => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'O' } else { 'o' });
            }
            KeyCode::KeyP => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'P' } else { 'p' });
            }
            KeyCode::KeyQ => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'Q' } else { 'q' });
            }
            KeyCode::KeyR => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'R' } else { 'r' });
            }
            KeyCode::KeyS => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'S' } else { 's' });
            }
            KeyCode::KeyT => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'T' } else { 't' });
            }
            KeyCode::KeyU => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'U' } else { 'u' });
            }
            KeyCode::KeyV => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'V' } else { 'v' });
            }
            KeyCode::KeyW => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'W' } else { 'w' });
            }
            KeyCode::KeyX => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'X' } else { 'x' });
            }
            KeyCode::KeyY => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'Y' } else { 'y' });
            }
            KeyCode::KeyZ => {
                let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
                ui.input_buffer.push(if shift { 'Z' } else { 'z' });
            }
            _ => {}
        }
    }
}

fn cleanup_terminal(
    mut commands: Commands,
    terminal_query: Query<Entity, With<TerminalContainer>>,
    scanline_query: Query<Entity, With<ScanlineEffect>>,
) {
    for entity in terminal_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in scanline_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn cursor_blink_system(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut CursorBlink), With<InputPrompt>>,
    ui: Res<GameUI>,
) {
    for (mut text, mut cursor) in query.iter_mut() {
        cursor.timer.tick(time.delta());
        if cursor.timer.just_finished() {
            cursor.visible = !cursor.visible;
        }
        // Always update the input display immediately (not tied to cursor blink)
        let cursor_char = if cursor.visible { "█" } else { " " };
        text.0 = format!("> {}{}", ui.input_buffer, cursor_char);
    }
}

fn typewriter_effect_system(time: Res<Time>, mut query: Query<(&mut Text, &mut TypewriterEffect)>) {
    for (mut text, mut effect) in query.iter_mut() {
        if !effect.complete {
            effect.timer.tick(time.delta());
            if effect.timer.just_finished() {
                // Use char_indices to handle Unicode properly
                let chars: Vec<_> = effect.full_text.char_indices().collect();
                effect.current_index = (effect.current_index + 1).min(chars.len());

                let visible_text = if effect.current_index >= chars.len() {
                    effect.full_text.clone()
                } else {
                    let end_byte_index = chars[effect.current_index].0;
                    effect.full_text[..end_byte_index].to_string()
                };

                text.0 = visible_text;
                if effect.current_index >= chars.len() {
                    effect.complete = true;
                }
            }
        }
    }
}

fn scanline_animation_system(time: Res<Time>, mut query: Query<&mut Node, With<ScanlineEffect>>) {
    for mut style in query.iter_mut() {
        let elapsed = time.elapsed_secs();
        let screen_height = 800.0; // Approximate screen height
        let scanline_speed = 2.0; // Seconds to complete one pass
        let position = (elapsed % scanline_speed) / scanline_speed * screen_height;
        style.top = Val::Px(position);
    }
}
