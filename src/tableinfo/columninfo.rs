use std::cmp::PartialEq;

pub struct ColumnInfo {
    pub column_name: String,
    pub table_id: i32,
    pub column_type: String
}

impl ColumnInfo {
    fn new(column_name: String, table_id: i32, column_type: String) -> ColumnInfo {
        ColumnInfo {
            column_name,
            table_id,
            column_type
        }
    }
}

impl PartialEq for ColumnInfo {
    fn eq(&self, other: &Self) -> bool {
        self.column_name == other.column_name
        && self.table_id == other.table_id
        && self.column_type == other.column_type
    }
}