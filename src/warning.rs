use gir_parser::SourcePosition;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Severity {
    Error = 3,
    Warning = 2,
    Info = 1,
}

#[derive(PartialEq, Eq)]
struct Pos {
    filename: String,
    line: String,
}

#[derive(PartialEq, Eq)]
pub struct Warning {
    pos: Option<Pos>,
    message: String,
    severity: Severity,
}

impl Ord for Warning {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.severity.cmp(&other.severity)
    }
}

impl PartialOrd for Warning {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.severity.cmp(&other.severity))
    }
}

impl Warning {
    fn new_inner(pos: Option<&SourcePosition>, id: &str, msg: &str, severity: Severity) -> Self {
        let pos = pos.map(|pos| Pos {
            filename: pos.filename().to_owned(),
            line: pos.line().to_owned(),
        });
        Self {
            pos,
            message: format!("{id} {msg}"),
            severity,
        }
    }

    pub fn new(pos: Option<&SourcePosition>, id: &str, msg: &str) -> Self {
        Self::new_inner(pos, id, msg, Severity::Warning)
    }

    pub fn info(pos: Option<&SourcePosition>, id: &str, msg: &str) -> Self {
        Self::new_inner(pos, id, msg, Severity::Info)
    }

    pub fn error(pos: Option<&SourcePosition>, id: &str, msg: &str) -> Self {
        Self::new_inner(pos, id, msg, Severity::Error)
    }
}

impl fmt::Display for Warning {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let severity = match self.severity {
            Severity::Warning => "WARNING",
            Severity::Error => "ERROR",
            Severity::Info => "INFO",
        };
        let message = &self.message;
        if let Some(pos) = &self.pos {
            write!(f, "{severity} {}:{}: {message}", pos.filename, pos.line)
        } else {
            write!(f, "{severity}: {message}")
        }
    }
}
