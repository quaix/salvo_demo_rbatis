use crate::config::CFG;
use tokio::sync::OnceCell;
use rbatis::rbatis::RBatis;
pub static DB: OnceCell<RBatis> = OnceCell::const_new();

pub async fn init_db_conn() {
    DB.get_or_init(|| async {
        let rb = RBatis::new();

        // MySQL
        rb.init(rbdc_mysql::driver::MysqlDriver {}, &CFG.database.database_url).unwrap();

        let sql_file = match rb.driver_type().unwrap() {
            "sqlite" => "./data/table_sqlite.sql",
            "postgres" => "./data/table_postgres.sql",
            "mysql" => "./data/table_mysql.sql",
            "mssql" => "./data/table_mssql.sql",
            _ => { "" }
        };
        if sql_file != "" {
            let sql = std::fs::read_to_string(sql_file).unwrap();
            let _ = rb.exec(&sql, vec![]).await;
        }
        return rb;    
    })
    .await;
}
