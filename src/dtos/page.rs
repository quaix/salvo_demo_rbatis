use salvo::oapi::{Components, RefOr, Schema};
use salvo::prelude::ToSchema;
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PageRequestDto {
    /// current page index default 1,range=1...MAX
    pub page_no: u64,
    /// page page_size default 10
    pub page_size: u64,
    /// Control whether to execute count statements to count the total number
    pub do_count: bool,
}




