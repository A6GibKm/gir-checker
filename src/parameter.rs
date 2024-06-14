use gir_parser::{prelude::*, Class, FunctionScope, Parameter};

use crate::checker::{Checker, DocChecker};
use crate::warning::Warning;
use std::sync::OnceLock;

trait NewTrait: gir_parser::prelude::FunctionLike + gir_parser::prelude::Info {}

pub struct ParameterChecker<'a> {
    class: &'a Class,
    parameter: &'a Parameter,
    parent: &'a dyn FunctionLike,
    parent_name: &'a str,
    id: OnceLock<String>,
}

impl<'a> ParameterChecker<'a> {
    pub fn new(
        parameter: &'a Parameter,
        function: &'a impl FunctionLike,
        parent_name: &'a str,
        class: &'a Class,
    ) -> Self {
        Self {
            class,
            parameter,
            parent: function,
            parent_name,
            id: OnceLock::new(),
        }
    }

    fn is_notified(&self) -> bool {
        self.parameter
            .scope()
            .is_some_and(|s| matches!(s, FunctionScope::Notified))
    }
}

impl_doc_checker!(ParameterChecker<'_>, Parameter, parameter);

impl<'a> Checker for ParameterChecker<'a> {
    fn identifier(&self) -> &str {
        self.id.get_or_init(|| {
            let parameter_name = self.parameter.name();
            let fn_name = self.parent_name;
            format!("Parameter {parameter_name} of {fn_name}")
        })
    }

    fn check(&self, warnings: &mut Vec<Warning>) {
        let pos = self.parameter.source_position();
        let id = self.identifier();

        if self.is_notified() && self.parameter.destroy().is_none() {
            warnings.push(Warning::error(pos, id, "is missing destroy"))
        }

        if self.is_notified() && self.parameter.closure().is_none() {
            warnings.push(Warning::error(pos, id, "is missing closure"))
        }
    }
}
