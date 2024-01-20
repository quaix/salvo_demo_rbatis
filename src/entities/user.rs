use rbatis::{crud, impl_select_page};

pub const TABLE_USERS: &'static str = "users";

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Users {
    pub id: String,
    pub username: String,
    pub password: String,
}

crud!(Users {},TABLE_USERS);

impl_select_page!(Users{select_page() =>"
     if !sql.contains('count(1)'):
       `order by create_time desc`"});

impl_select_page!(Users{select_page_by_username(username:&str) =>"
     if username != null && username != '':
       `where username = #{username}`"});

/// postgres/mssql database not support `limit 0,10`,you should use limit_sql:&str and set `limit 10 offset 0`
impl_select_page!(Users{select_page_by_limit(username:&str,limit_sql:&str) => "`where username = #{username}`"});
