use once_cell::sync::Lazy;
use rbatis::RBatis;

pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

#[cfg(test)]
mod tests {
    use rbatis::plugin::page::PageRequest;
    use rbdc_mysql::MysqlDriver;
    use serde_json::json;

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

        let data = Users::select_page_by_username(&*RB, &PageRequest::new(1, 10), "string").await;
        println!("------select_page_by_username = {}", json!(data));

        let data = Users::select_page_by_limit(&*RB, &PageRequest::new(1, 100), "test", " limit 0,10 ").await;
        println!("------select_page_by_limit = {}", json!(data));

        let data = Users::select_page_by_username_like(&*RB, &PageRequest::new(1, 10), "s").await;
        println!("------select_page_by_username_like = {}", json!(data));

    }
}
