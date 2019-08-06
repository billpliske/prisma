#[macro_use]
extern crate neon;

pub mod commands;
pub mod connector_loader;
pub mod migration;
pub mod migration_engine;
pub mod migration_api;
mod error;

#[macro_use]
extern crate serde_derive;

use commands::{CommandError, CommandResult};
use datamodel::{self, Datamodel, Configuration};
use migration_api::MigrationApi;

pub use migration_engine::*;
pub use error::Error;

pub fn parse_datamodel(datamodel: &str) -> CommandResult<Datamodel> {
    let result = datamodel::parse_with_formatted_error(&datamodel, "datamodel file, line");
    result.map_err(|e| CommandError::Generic { code: 1001, error: e })
}

pub type Result<T> = std::result::Result<T, Error>;

declare_types! {
    pub class JsMigrationApi for MigrationApi {
        init(mut cx) {
            let config: String = cx.argument::<JsString>(0)?.value();
            Ok(MigrationApi::new(&config)?)
        }

        method get_config(mut cx) {
            let this = cx.this();
            let datamodel: String = cx.argument::<JsString>(0)?.value();

            let config = {
                let guard = cx.lock();
                let api = this.borrow(&guard);

                api.get_config(&datamodel)?
            };

            datamodel::to_serializeable(&config)
        }

        method dmmf_to_dml(mut cx)
    }
}
