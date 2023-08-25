use std::{net::SocketAddr, str::FromStr};

use axum::Server;
use common::config::{AppConfig, APP_CONFIG};
use infrastructure::db::model::common::{init_db, DB_LOCAL};
use interaction::router;

const LOCAL_HOSE: &str = "0.0.0.0";

pub async fn server_api(config: AppConfig) {
    APP_CONFIG.get_or_init(|| config.clone());

    let _guard = common::log::init_log(&config.log);
    tracing::info!("api server srart:{}:{}", LOCAL_HOSE, config.port.clone());
    tracing::info!("port:{} \nworkspace:{} \n", config.port, config.workspace,);

    let db = match init_db(common::i18n::Locale::En, &config.db_local_path()).await {
        Ok(r) => r,
        Err(e) => {
            panic!("{}", e)
        }
    };
    DB_LOCAL.get_or_init(|| db);

    let router = router::router::create_router();
    let addr = SocketAddr::from_str(&format!("{}:{}", LOCAL_HOSE, config.port)).unwrap();
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
