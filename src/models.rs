use super::schema::memos;

#[derive(Queryable)]
pub struct Memo {
    pub id: i32,
    pub name: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "memos"]
pub struct NewMemo<'a> {
    pub name: &'a str,
    pub body: &'a str,
}
