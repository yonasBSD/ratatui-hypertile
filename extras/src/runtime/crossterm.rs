use ratatui_hypertile::{HypertileEvent, KeyChord, KeyCode, Modifiers};

pub fn keychord_from_crossterm(key: crossterm::event::KeyEvent) -> Option<KeyChord> {
    use crossterm::event::{KeyCode as CrosstermCode, KeyModifiers as CrosstermModifiers};

    let code = match key.code {
        CrosstermCode::Char(c) => KeyCode::Char(c),
        CrosstermCode::Enter => KeyCode::Enter,
        CrosstermCode::Esc => KeyCode::Escape,
        CrosstermCode::Tab => KeyCode::Tab,
        CrosstermCode::BackTab => KeyCode::BackTab,
        CrosstermCode::Backspace => KeyCode::Backspace,
        CrosstermCode::Up => KeyCode::Up,
        CrosstermCode::Down => KeyCode::Down,
        CrosstermCode::Left => KeyCode::Left,
        CrosstermCode::Right => KeyCode::Right,
        CrosstermCode::Home => KeyCode::Home,
        CrosstermCode::End => KeyCode::End,
        CrosstermCode::PageUp => KeyCode::PageUp,
        CrosstermCode::PageDown => KeyCode::PageDown,
        CrosstermCode::Delete => KeyCode::Delete,
        CrosstermCode::Insert => KeyCode::Insert,
        CrosstermCode::F(n) => KeyCode::F(n),
        _ => return None,
    };

    let mut modifiers = Modifiers::NONE;
    if key.modifiers.contains(CrosstermModifiers::SHIFT) {
        modifiers |= Modifiers::SHIFT;
    }
    if key.modifiers.contains(CrosstermModifiers::CONTROL) {
        modifiers |= Modifiers::CTRL;
    }
    if key.modifiers.contains(CrosstermModifiers::ALT) {
        modifiers |= Modifiers::ALT;
    }

    Some(KeyChord { code, modifiers })
}

pub fn event_from_crossterm(key: crossterm::event::KeyEvent) -> Option<HypertileEvent> {
    keychord_from_crossterm(key).map(HypertileEvent::Key)
}
