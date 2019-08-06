use super::connector_loader::load_connector;
use crate::commands::{CommandError, CommandResult};
use crate::migration::datamodel_calculator::*;
use crate::migration::datamodel_migration_steps_inferrer::*;
use datamodel::dml::*;
use migration_connector::*;
use sql_migration_connector::{SqlFamily, SqlMigrationConnector};

pub struct MigrationEngine {
    config: String,
    datamodel_migration_steps_inferrer: Box<dyn DataModelMigrationStepsInferrer>,
    datamodel_calculator: Box<dyn DataModelCalculator>,
    connector: Box<dyn MigrationConnector>,
}

impl std::panic::RefUnwindSafe for MigrationEngine {}

impl MigrationEngine {
    pub fn new(config: &str) -> crate::Result<MigrationEngine> {
        let connector = Self::load_connector(config)?;

        let engine = MigrationEngine {
            config: config.to_string(),
            datamodel_migration_steps_inferrer: Box::new(DataModelMigrationStepsInferrerImplWrapper),
            datamodel_calculator: Box::new(DataModelCalculatorImpl),
            connector,
        };

        Box::new(engine)
    }

    pub fn init(&self) -> CommandResult<()> {
        self.connector().initialize()?;
        Ok(())
    }

    pub fn reset(&self) -> CommandResult<()> {
        self.connector().reset()?;
        Ok(())
    }

    pub fn datamodel_migration_steps_inferrer(&self) -> &DataModelMigrationStepsInferrer {
        &*self.datamodel_migration_steps_inferrer
    }

    pub fn datamodel_calculator(&self) -> &DataModelCalculator {
        &*self.datamodel_calculator
    }

    pub fn render_datamodel(&self, datamodel: &Datamodel) -> String {
        datamodel::render(&datamodel).expect("Rendering the Datamodel failed.")
    }

    fn load_connector(
        config: &str,
        must_exist: bool,
    ) -> crate::Result<Box<dyn MigrationConnector>> {
        let config = datamodel::load_configuration(config)?;

        let source = config.datasources.first().ok_or(CommandError::DataModelErrors {
            code: 1000,
            errors: vec!["There is no datasource in the configuration.".to_string()],
        })?;

        let connector = match source.connector_type().as_ref() {
            "sqlite" => SqlMigrationConnector::sqlite(&source.url())?,
            "postgresql" => SqlMigrationConnector::postgres(&source.url())?,
            "mysql" => SqlMigrationConnector::mysql(&source.url())?,
            x => unimplemented!("Connector {} is not supported yet", x),
        };

        Ok(Box::new(connector))
    }
}
