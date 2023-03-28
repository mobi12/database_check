pub use std::vec;

use super::columninfo::ColumnInfo;

pub struct TableInfo {
    pub table_name: String,
    pub table_columns: Vec<ColumnInfo>
}

impl TableInfo {
    fn new(table_name: String) -> TableInfo {
        TableInfo {
            table_name: table_name,
            table_columns: vec![]
        }
    }

    fn add_column(&mut self, column: &mut ColumnInfo) {
        self.table_columns.append(column);
    }

    fn diff(other_table: TableInfo) -> Vec<ColumnInfo> {

    }
}
