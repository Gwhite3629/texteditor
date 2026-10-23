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
    rw: bool, // Is file open as read-only or write-able (1 is read only)
    df: bool, // Generate diff file from initial state to final state (1 is generate diff)
    ow: bool, // Overwrite original file with new file, if false then create new file (1 is create new file)
    wb: bool, // Write-back enable. If true then write to final file on every save. If false then write to temp file and write to final on exit
}

impl AppProperties {
    pub fn default() -> Self {
        Self {
            rw: false,
            df: false,
            ow: false,
            wb: false,
        }
    }
    pub fn enable_read_only(&mut self) {
        self.rw = true;
    }
    pub fn enable_diff(&mut self) {
        self.df = true;
    }
    pub fn enable_new_file(&mut self) {
        self.ow = true;
    }
    pub fn enable_write_back(&mut self) {
        self.wb = true;
    }
}
