use bevy::prelude::*;

// Cyberpunk color palette
/// #00ffcc - Cyan for primary text
pub const LABEL_TEXT: Color = Color::srgb(0.0, 1.0, 0.8);

/// #ff00ff - Magenta for headers
pub const HEADER_TEXT: Color = Color::srgb(1.0, 0.0, 1.0);

/// #00ffcc - Cyan for button text
pub const BUTTON_TEXT: Color = Color::srgb(0.0, 1.0, 0.8);
/// #1a0033 - Dark purple for button background
pub const BUTTON_BACKGROUND: Color = Color::srgb(0.102, 0.0, 0.2);
/// #ff00ff - Magenta for hovered buttons
pub const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.4, 0.0, 0.4);
/// #00ff88 - Green for pressed buttons
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.0, 0.6, 0.4);
