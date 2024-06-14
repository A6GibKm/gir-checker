use clap::Parser;
use gir_parser::{prelude::*, Repository};

use crate::checker::CheckAll;
use crate::class::ClassChecker;
use crate::method::MethodChecker;
use crate::parameter::ParameterChecker;
use crate::property::PropertyChecker;

#[macro_use]
mod checker;
mod class;
mod method;
mod parameter;
mod property;
mod utils;
mod warning;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the perjon to greet
    file: std::path::PathBuf,
    #[arg(short, long)]
    ignore_deprecated: bool,
}

pub struct Options {
    pub ignore_deprecated: bool,
}

fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let options = Options {
        ignore_deprecated: args.ignore_deprecated,
    };

    let mut warnings = vec![];
    let repository = Repository::from_path(args.file).unwrap();
    for class in repository.namespace().classes().iter() {
        if options.ignore_deprecated && class.is_deprecated() {
            continue;
        }

        let class_checker = ClassChecker::new(class);

        class_checker.check_all(&mut warnings);

        for property in class.properties() {
            if options.ignore_deprecated && property.is_deprecated() {
                continue;
            }
            let property_checker = PropertyChecker::new(property, class);
            property_checker.check_all(&mut warnings);
        }

        for method in class.methods() {
            if options.ignore_deprecated && method.is_deprecated() {
                continue;
            }
            let method_checker = MethodChecker::new(method, class);
            method_checker.check_all(&mut warnings);

            for parameter in method.parameters().inner() {
                let parameter_checker = ParameterChecker::new(
                    parameter,
                    method,
                    method.c_identifier().unwrap_or_else(|| method.name()),
                    class,
                );
                parameter_checker.check_all(&mut warnings);
            }
        }
    }

    warnings.sort();
    for warning in warnings {
        println!("{warning}",);
    }
}
