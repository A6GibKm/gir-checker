use gir_parser::{prelude::*, Class, FunctionScope, Method, Parameter};

use crate::checker::{Checker, DocChecker};
use crate::warning::Warning;
use std::sync::OnceLock;

pub struct MethodChecker<'a> {
    class: &'a Class,
    method: &'a Method,
    id: OnceLock<String>,
}

impl<'a> MethodChecker<'a> {
    pub fn new(method: &'a Method, class: &'a Class) -> Self {
        Self {
            class,
            method,
            id: OnceLock::new(),
        }
    }

    // Destroy cbs are async so we need to check the async callback is not a
    // destroy callback.
    fn is_async(&self) -> bool {
        self.method.finish_func().is_some()
            || self
                .method
                .parameters()
                .inner()
                .iter()
                .filter_map(Parameter::scope)
                .any(|s| matches!(s, FunctionScope::Async))
    }
}

impl_doc_checker!(MethodChecker<'_>, Method, method);

impl<'a> Checker for MethodChecker<'a> {
    fn identifier(&self) -> &str {
        self.id.get_or_init(|| {
            if let Some(c_id) = self.method.c_identifier() {
                format!("Method {c_id}")
            } else {
                let class_name = self.class.name();
                let method_name = self.method.name();
                format!("Method {class_name}.{method_name}")
            }
        })
    }
    fn check(&self, warnings: &mut Vec<Warning>) {
        let pos = self.method.source_position();
        let id = self.identifier();
        if self.is_async() && self.method.finish_func().is_none() {
            warnings.push(Warning::info(pos, id, "is missing a finish-func"));
        }
    }
}
