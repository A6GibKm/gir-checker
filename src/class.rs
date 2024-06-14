use crate::checker::{Checker, DocChecker};
use crate::warning::Warning;
use gir_parser::{prelude::*, Class};
use std::sync::OnceLock;

pub struct ClassChecker<'a> {
    class: &'a Class,
    id: OnceLock<String>,
}

impl<'a> ClassChecker<'a> {
    pub fn new(class: &'a Class) -> Self {
        Self {
            class,
            id: OnceLock::new(),
        }
    }

    pub fn has_docs(&self) -> bool {
        self.class.doc().is_some()
    }
}

impl_doc_checker!(ClassChecker<'_>, Class, class);

impl<'a> Checker for ClassChecker<'a> {
    fn identifier(&self) -> &str {
        self.id.get_or_init(|| {
            let class_name = self.class.name();
            format!("Class {class_name}")
        })
    }

    fn check(&self, warnings: &mut Vec<Warning>) {
        let pos = self.class.source_position();
        let id = self.identifier();

        if !self.has_docs() {
            warnings.push(Warning::error(pos, id, "is missing docs"));
        }
    }
}
