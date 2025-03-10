use std::sync::Arc;

use degen_sql::db::postgres::postgres_db::Database;

pub struct AppState {
    pub database: Arc<Database>,

    pub app_config: Arc< AppConfig >
}




pub struct AppConfig {

    pub service_name: String 
}
