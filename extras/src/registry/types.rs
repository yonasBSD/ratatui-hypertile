use ratatui_hypertile::PaneId;

/// Passed to plugin mount and unmount callbacks.
#[derive(Debug, Clone, Copy)]
pub struct PluginContext {
    pub pane_id: PaneId,
}

/// Registry errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    UnknownPluginType(String),
    DuplicatePane(PaneId),
    MissingPane(PaneId),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPluginType(name) => write!(f, "unknown plugin type: {name}"),
            Self::DuplicatePane(id) => write!(f, "plugin already mounted for pane {}", id.get()),
            Self::MissingPane(id) => write!(f, "no plugin mounted for pane {}", id.get()),
        }
    }
}

impl std::error::Error for RegistryError {}
