use crate::{commands::*, migration_engine::MigrationEngine};
use datamodel::{dmmf, Configuration};
use prisma_query::connector::*;

pub struct MigrationApi {
    engine: MigrationEngine,
}

impl MigrationApi {
    pub fn new(config: &str) -> crate::Result<Self> {
        let engine = MigrationEngine::new(config)?;
        engine.init()?;
        Ok(Self { engine })
    }

    pub fn handle_command<C>(&self, input: C::Input) -> crate::Result<C::Output>
    where
        C: MigrationCommand,
    {
        Ok(C::new(input).execute(&self.engine)?)
    }

    pub fn dmmf_to_dml(&self, dmmf: &str, config: &Configuration) -> crate::Result<String> {
        let datamodel = dmmf::parse_from_dmmf(dmmf);

        Ok(datamodel::render_with_config(&datamodel, config)?)
    }

    pub fn get_config(&self, datamodel: &str) -> crate::Result<Configuration> {
        Ok(datamodel::load_configuration(datamodel)?)
    }
}
