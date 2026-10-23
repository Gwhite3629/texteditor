#[derive(Debug, Default)]
// Mutable state of app
pub enum AppState {
    #[default]
    SIMPLECOMMAND,
    MULTICOMMAND,
    SELECT,
    MOVE,
    TEXT,
}

#[derive(Debug, Default)]
// Immutable state of app upon launch
pub struct AppProperties {
    rw: bool, // Is file open as read-only or write-able
    df: bool, // Generate diff file from initial state to final state
    ow: bool, // Overwrite original file with new file, if false then create new file
    wb: bool, // Write-back enable. If true then write to final file on every save. If false then write to temp file and write to final on exit
}
