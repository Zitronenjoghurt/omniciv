use crate::content::store::Key;
use crate::defs::building::BuildingDef;

pub enum Command {
    BuildBuilding(Key<BuildingDef>),
}
