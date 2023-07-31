use diesel::prelude::*;
use orb_runtime::Runtime;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::sites)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Site {
    pub id: Vec<u8>,
    pub name: String,
    pub module_id: Vec<u8>,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::sites)]
pub struct NewSite<'a> {
    pub id: Vec<u8>,
    pub name: &'a str,
    pub module_id: Vec<u8>,
}

impl<'a> NewSite<'a> {
    pub fn new(_runtime: &Runtime, name: &'a str) -> Self {
        let id = uuid::Uuid::new_v4();

        // TODO: Hook this up to something real
        let module_id = uuid::Uuid::new_v4();

        Self {
            id: id.as_bytes().to_vec(),
            module_id: module_id.as_bytes().to_vec(),
            name,
        }
    }
}

pub fn _get_site(_id: &[u8], conn: &mut SqliteConnection) -> Result<Site, diesel::result::Error> {
    use crate::schema::sites::dsl::*;

    let result = sites
        .find(id).select(Site::as_select()).load::<Site>(conn)?;

    Ok(result.get(0).unwrap().clone())
}
