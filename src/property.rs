use gir_parser::{prelude::*, Class, Property};

use crate::checker::{Checker, DocChecker};
use crate::warning::Warning;
use std::sync::OnceLock;

pub struct PropertyChecker<'a> {
    class: &'a Class,
    property: &'a Property,
    id: OnceLock<String>,
}

impl<'a> PropertyChecker<'a> {
    pub fn new(property: &'a Property, class: &'a Class) -> Self {
        Self {
            class,
            property,
            id: OnceLock::new(),
        }
    }
}

impl_doc_checker!(PropertyChecker<'_>, Property, property);

impl<'a> Checker for PropertyChecker<'a> {
    fn identifier(&self) -> &str {
        self.id.get_or_init(|| {
            let class_name = self.class.name();
            let prop_name = self.property.name();
            format!("Property {class_name}:{prop_name}")
        })
    }

    fn check(&self, warnings: &mut Vec<Warning>) {
        let class = self.class;
        let property = self.property;
        let prop_name = self.property.name();
        let pos = self.property.source_position();
        let id = self.identifier();

        if property.doc().is_none() {
            warnings.push(Warning::error(pos, id, "is missing a docstring"));
        }

        if property.is_readable() {
            if let Some(getter) = property.getter() {
                if let Some(method) = class.methods().iter().find(|m| m.name() == property.name()) {
                    if !method.get_property().is_some_and(|g| g == getter) {
                        warnings.push(Warning::error(
                            pos,
                            id,
                            "has a getter but the getter does not have get-property",
                        ));
                    }
                }
            } else {
                warnings.push(Warning::info(pos, id, "is missing a getter"));
            }
        }

        if !property.is_readable() && property.getter().is_some() {
            warnings.push(Warning::error(
                pos,
                id,
                "has a getter but it is not readable",
            ));
        }

        if property.is_writable() && !property.is_construct_only() {
            if let Some(setter) = property.setter() {
                if let Some(method) = class.methods().iter().find(|m| m.name() == prop_name) {
                    if !method.set_property().is_some_and(|s| s == setter) {
                        warnings.push(Warning::error(
                            pos,
                            id,
                            "has a setter but the setter does not have set-property",
                        ));
                    }
                }
            } else {
                warnings.push(Warning::info(pos, id, "is missing a setter"));
            }
        }

        if !property.is_writable() && property.setter().is_some() {
            warnings.push(Warning::error(
                pos,
                id,
                "has a setter but it is not writable",
            ));
        }

        if property.is_construct_only() && property.setter().is_some() {
            warnings.push(Warning::error(
                pos,
                id,
                "has a setter but it is construct-only",
            ));
        }
    }
}
