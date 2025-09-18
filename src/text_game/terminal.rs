use crate::screens::Screen;
use crate::text_game::{
    game::{Game, GameUI, InputEvent},
    story::{BANNER, Story},
};
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::text::LineHeight;

#[derive(Component)]
pub struct GameTextDisplay;

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
pub struct TypewriterEffect {
    full_text: String,
    current_index: usize,
    timer: Timer,
    complete: bool,
}

#[derive(Component)]
pub struct ScanlineEffect;

#[derive(Component)]
pub struct ScrollbarTrack;

#[derive(Component)]
pub struct ScrollbarThumb;

#[derive(Component)]
pub struct ScrollIndicator;

#[derive(Component, Default)]
pub struct ContentMetrics {
    pub content_height: f32,
    pub viewport_height: f32,
    pub is_scrollable: bool,
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup_terminal);
    app.add_systems(
        Update,
        (
            ui_render_system,
            terminal_input_system,
            handle_scroll_input,
            update_content_metrics,
            update_scrollbar_visibility,
            update_scrollbar_system,
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
            // Main content container
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(85.0),
                        max_width: Val::Px(1200.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                ))
                .with_children(|content_parent| {
                    // Scrollable game content area
                    content_parent
                        .spawn((
                            Node {
                                width: Val::Percent(95.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                overflow: Overflow::scroll_y(),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.0, 0.02, 0.05, 0.9)),
                            ScrollContainer,
                            ScrollPosition::default(),
                            ContentMetrics::default(),
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
                            min_height: Val::Px(800.0), // Force sufficient height for scrolling
                            width: Val::Percent(100.0),
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

                    // Custom scrollbar (initially hidden)
                    content_parent
                        .spawn((
                            Node {
                                width: Val::Percent(5.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexStart,
                                padding: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.0, 0.3, 0.5, 0.3)),
                            ScrollbarTrack,
                            Visibility::Hidden,
                        ))
                        .with_children(|track| {
                            // Scrollbar thumb
                            track.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.0, 1.0, 0.8)), // Cyan thumb
                                ScrollbarThumb,
                            ));
                        });
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
    mut text_query: Query<(&mut Text, Option<&mut TypewriterEffect>), With<GameTextDisplay>>,
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
            // Set number of choices for ending screen
            ui.num_choices = 3;

            display_text.push_str("╔════════════════════════════════════════╗\n");
            display_text.push_str(&format!(
                "║  ENDING UNLOCKED: {}  ║\n",
                format!("{:^20}", end.name).to_uppercase()
            ));
            display_text.push_str("╚════════════════════════════════════════╝\n\n");
            display_text.push_str(&format!("{}\n\n", end.summary));

            // Build ending choices with selection highlighting
            let ending_choices = vec!["Reboot from the alley", "Show unlocked endings", "Quit"];

            // Calculate dynamic width for ending choices
            let max_ending_text_width = ending_choices
                .iter()
                .map(|text| text.len())
                .max()
                .unwrap_or(0);

            // Account for selection formatting: "► [1] text" vs "  1) text"
            let ending_box_width = max_ending_text_width + 8; // "► [1] " = 6 chars + padding
            let content_width = ending_box_width - 2; // Width inside the box borders

            let top_border = format!("┌─{}─┐\n", "─".repeat(ending_box_width - 2));
            let what_now_line =
                format!("│{:^width$}│\n", "What now?", width = ending_box_width - 2);
            let bottom_border = format!("└─{}─┘\n", "─".repeat(ending_box_width - 2));

            display_text.push_str(&top_border);
            display_text.push_str(&what_now_line);

            for (i, choice_text) in ending_choices.iter().enumerate() {
                let is_selected = ui.selected_choice == Some(i);
                let prefix = if is_selected { "► " } else { "  " };
                let choice_line = if is_selected {
                    format!("{}[{}] {}", prefix, i + 1, choice_text)
                } else {
                    format!("{}{}) {}", prefix, i + 1, choice_text)
                };
                display_text.push_str(&format!(
                    "│{:<width$}│\n",
                    choice_line,
                    width = content_width
                ));
            }

            display_text.push_str(&bottom_border);

            // Add selection hint if a choice is selected
            if ui.selected_choice.is_some() {
                display_text.push_str("\n▶ Press ENTER to confirm selection, ESC to cancel\n");
            }
        } else {
            // Find the max width needed for choices
            let max_text_width = node
                .choices
                .iter()
                .map(|c| c.text.len())
                .max()
                .unwrap_or(0)
                .min(65); // Cap at 65 chars wide for better readability

            // Calculate box width accounting for selection formatting
            // Unselected: "  1) text" = 5 chars + text
            // Selected: "► [1] text" = 6 chars + text (for single digit) or 7 chars (for double digit)
            let max_choice_num_width = if node.choices.len() >= 10 { 2 } else { 1 };
            let max_prefix_width = 2 + max_choice_num_width + 2; // "► [" + num + "] "
            let box_width = max_text_width + max_prefix_width + 2; // +2 for left/right padding
            let top_border = format!("┌─ CHOICES {}┐\n", "─".repeat(box_width.saturating_sub(11)));
            let bottom_border = format!("└{}┘\n", "─".repeat(box_width));

            // Update UI state for choices
            ui.num_choices = node.choices.len();

            display_text.push_str(&top_border);
            for (i, choice) in node.choices.iter().enumerate() {
                // Wrap long text properly
                let text = if choice.text.len() > max_text_width {
                    let mut wrapped = choice
                        .text
                        .chars()
                        .take(max_text_width - 3)
                        .collect::<String>();
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
                    width = box_width - 2
                ));
            }
            display_text.push_str(&bottom_border);

            // Add selection hint if a choice is selected
            if ui.selected_choice.is_some() {
                display_text.push_str("\n▶ Press ENTER to confirm selection, ESC to cancel\n");
            } else {
                display_text.push_str("\n▶ Hold SPACE to skip text animation\n");
            }
        }

        let unlocked = game.endings_unlocked.len();
        if unlocked > 0 {
            display_text.push_str(&format!("\n[ENDINGS UNLOCKED: {}/7]\n", unlocked));
        }

        if let Some(msg) = &game.message {
            display_text.push_str(&format!("\n▶ SYSTEM: {}\n", msg.to_uppercase()));
        }

        // Scroll hints will be added dynamically by update_content_metrics
    } else {
        display_text.push_str(&format!("[ERROR: MISSING NODE '{}']\n", game.current));
    }

    // Update display text with typewriter effect
    if let Ok((mut text, typewriter)) = text_query.single_mut() {
        if let Some(mut effect) = typewriter {
            // For selection changes, update the full text but keep the animation state
            // Only reset animation for actual content changes (not selection highlights)
            let content_changed = !effect
                .full_text
                .contains(&display_text[..display_text.len().min(50)]);

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
    scrollbar_query: Query<Entity, With<ScrollbarTrack>>,
) {
    for entity in terminal_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in scanline_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in scrollbar_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_scroll_input(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut scroll_query: Query<&mut ScrollPosition, With<ScrollContainer>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut scroll_position in scroll_query.iter_mut() {
        // Handle mouse wheel scrolling
        for ev in mouse_wheel_events.read() {
            let scroll_amount = match ev.unit {
                bevy::input::mouse::MouseScrollUnit::Line => ev.y * 25.0, // 25 pixels per line
                bevy::input::mouse::MouseScrollUnit::Pixel => ev.y,
            };

            scroll_position.y -= scroll_amount;
            scroll_position.y = scroll_position.y.max(0.0);
        }

        // Handle keyboard scrolling
        if keys.just_pressed(KeyCode::PageUp) {
            scroll_position.y -= 200.0;
            scroll_position.y = scroll_position.y.max(0.0);
        }
        if keys.just_pressed(KeyCode::PageDown) {
            scroll_position.y += 200.0;
        }
        if keys.just_pressed(KeyCode::Home) {
            scroll_position.y = 0.0;
        }
        if keys.just_pressed(KeyCode::End) {
            scroll_position.y = 10000.0; // Large value, will be clamped by Bevy
        }
    }
}

fn update_scrollbar_system(
    scroll_query: Query<(&ScrollPosition, &ContentMetrics), With<ScrollContainer>>,
    mut scrollbar_query: Query<&mut Node, With<ScrollbarThumb>>,
    scrollbar_track_query: Query<&Node, (With<ScrollbarTrack>, Without<ScrollbarThumb>)>,
) {
    if let Ok((scroll_position, metrics)) = scroll_query.single() {
        if !metrics.is_scrollable {
            return;
        }

        let Ok(track_node) = scrollbar_track_query.single() else { return; };

        for mut thumb_style in scrollbar_query.iter_mut() {
            // Calculate thumb size based on viewport/content ratio
            let viewport_ratio = metrics.viewport_height / metrics.content_height;
            let track_height = match track_node.height {
                Val::Percent(p) => metrics.viewport_height * p / 100.0,
                Val::Px(h) => h,
                _ => metrics.viewport_height,
            };

            let thumb_height = (track_height * viewport_ratio).max(30.0); // Min 30px
            thumb_style.height = Val::Px(thumb_height);

            // Calculate scrollable range
            let scrollable_content = metrics.content_height - metrics.viewport_height;
            let scroll_percentage = if scrollable_content > 0.0 {
                (scroll_position.y / scrollable_content).clamp(0.0, 1.0)
            } else {
                0.0
            };

            // Calculate thumb position
            let track_space = track_height - thumb_height;
            let thumb_position = scroll_percentage * track_space;

            thumb_style.top = Val::Px(thumb_position);
        }
    }
}

fn typewriter_effect_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Text, &mut TypewriterEffect)>,
) {
    for (mut text, mut effect) in query.iter_mut() {
        if !effect.complete {
            // Check if Space is held for speed-up
            let speed_up = keys.pressed(KeyCode::Space);

            // If Space is held, skip to the end instantly
            if speed_up {
                text.0 = effect.full_text.clone();
                effect.complete = true;
                effect.current_index = effect.full_text.len();
            } else {
                // Normal typewriter effect
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

fn update_content_metrics(
    mut metrics_query: Query<(&mut ContentMetrics, &Node, &Children), With<ScrollContainer>>,
    text_query: Query<&Node, With<GameTextDisplay>>,
    window: Query<&Window>,
) {
    let Ok(window) = window.single() else { return; };

    for (mut metrics, container_node, children) in metrics_query.iter_mut() {
        // Get viewport height from container node
        metrics.viewport_height = match container_node.height {
            Val::Px(h) => h,
            Val::Percent(p) => window.height() * p / 100.0 * 0.85, // Account for 85% height
            _ => 600.0, // fallback
        };

        // Get content height from text node
        for child in children.iter() {
            if let Ok(text_node) = text_query.get(child) {
                metrics.content_height = match text_node.min_height {
                    Val::Px(h) => h,
                    _ => 0.0,
                };

                // Add padding to content height
                if let Val::Px(padding) = text_node.padding.top {
                    metrics.content_height += padding * 2.0; // top + bottom
                }
            }
        }

        // Determine if scrollable
        metrics.is_scrollable = metrics.content_height > metrics.viewport_height;
    }
}

fn update_scrollbar_visibility(
    metrics_query: Query<&ContentMetrics, (With<ScrollContainer>, Changed<ContentMetrics>)>,
    mut scrollbar_query: Query<&mut Visibility, With<ScrollbarTrack>>,
    mut text_query: Query<(&mut Text, Option<&mut TypewriterEffect>), With<GameTextDisplay>>,
) {
    for metrics in metrics_query.iter() {
        // Update scrollbar visibility
        for mut visibility in scrollbar_query.iter_mut() {
            *visibility = if metrics.is_scrollable {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }

        // Add/remove scroll hints from text
        if let Ok((mut text, typewriter)) = text_query.single_mut() {
            let current_text = if let Some(ref effect) = typewriter {
                effect.full_text.clone()
            } else {
                text.0.clone()
            };

            // Check if we need to add scroll hints
            if metrics.is_scrollable && !current_text.contains("▲ Mouse wheel") {
                let mut updated_text = current_text.clone();
                updated_text.push_str("\n\n");
                updated_text.push_str("═══════════════════════════════════════\n");
                updated_text.push_str("▲ Mouse wheel, Page Up/Down to scroll\n");
                updated_text.push_str("▼ Home/End for top/bottom navigation\n");

                text.0 = updated_text.clone();
                if let Some(mut effect) = typewriter {
                    effect.full_text = updated_text.clone();
                }
            } else if !metrics.is_scrollable && current_text.contains("▲ Mouse wheel") {
                // Remove scroll hints if not scrollable
                let lines: Vec<&str> = current_text.lines().collect();
                let filtered: Vec<&str> = lines.into_iter()
                    .filter(|line| !line.contains("═══════") &&
                            !line.contains("▲ Mouse wheel") &&
                            !line.contains("▼ Home/End"))
                    .collect();
                let updated_text = filtered.join("\n");

                text.0 = updated_text.clone();
                if let Some(mut effect) = typewriter {
                    effect.full_text = updated_text.clone();
                }
            }
        }
    }
}

// CRT time update function removed - keeping for future reference
// fn update_crt_time(
//     time: Res<Time>,
//     mut crt_query: Query<&mut crate::text_game::crt_effect::CrtSettings>,
// ) {
//     for mut settings in crt_query.iter_mut() {
//         settings.time = time.elapsed_secs();
//     }
// }
