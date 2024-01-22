use once_cell::sync::Lazy;
use rbatis::RBatis;

pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

#[cfg(test)]
mod tests {
    use rbatis::plugin::page::PageRequest;
    use rbdc_mysql::MysqlDriver;
    use serde_json::json;
    use tracing::log;
    use tracing_test::traced_test;
    use crate::config::Configs;

    use crate::entities::user::Users;
    use crate::unit_tests::unit_tests::RB;


    #[tokio::test]
    async fn test_select_page() {
        dotenv::dotenv().ok();

        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }

        // mysql connect info
        let mysql_uri = "mysql://root:zjf1qaz!QAZ@localhost/test";
        RB.init(MysqlDriver {}, mysql_uri).expect("init RBatis failed");

        let data = Users::select_page(&*RB, &PageRequest::new(1, 10)).await;
        println!("------select_page = {}", json!(data));

        let data = Users::select_page_by_username(&*RB, &PageRequest::new(1, 100), "string").await;
        println!("------select_page_by_username = {}", json!(data));

        let data = Users::select_page_by_limit(&*RB, &PageRequest::new(1, 100), "test", " limit 0,10 ").await;
        println!("------select_page_by_limit = {}", json!(data));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_config_and_env() {
        Configs::init_by_config_and_environment();
        log::debug!("test_config_and_env end")
    }
}
