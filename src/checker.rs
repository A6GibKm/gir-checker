use crate::warning::Warning;

use gir_parser::prelude::*;

pub trait Checker {
    fn check(&self, warnings: &mut Vec<Warning>);
    fn identifier(&self) -> &str;
}

pub trait DocChecker: Checker {
    type Doc: Documentable;

    fn documentable(&self) -> &Self::Doc;

    fn check_docs(&self, warnings: &mut Vec<Warning>) {
        let id = self.identifier();
        let pos = self.documentable().source_position();

        if self.documentable().doc().is_none() {
            warnings.push(Warning::new(pos, id, "missing a docstring"));
        }
    }
}

pub trait CheckAll {
    fn check_all(&self, warnings: &mut Vec<Warning>);
}

impl<C: DocChecker> CheckAll for C {
    fn check_all(&self, warnings: &mut Vec<Warning>) {
        self.check(warnings);
        self.check_docs(warnings);
    }
}

macro_rules! impl_doc_checker {
    ($rust_type:ty, $parser_type:ident, $field:ident) => {
        impl DocChecker for $rust_type {
            type Doc = $parser_type;

            fn documentable(&self) -> &Self::Doc {
                &self.$field
            }
        }
    };
}
