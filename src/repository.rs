use crate::database;
use crate::domain::Perl;
use rusqlite::{Row, ToSql, NO_PARAMS};

impl database::DatabaseEntity<Perl> for Perl {
    fn as_sql(&self) -> (&'static str, Vec<&dyn ToSql>) {
        match &self.context {
            Some(c) => {
                let sql = "INSERT INTO perl (author, content, context, created_at) VALUES (?1, ?2, ?3, ?4);";
                let params: Vec<&dyn ToSql> = vec![&self.author, &self.content, c, &self.created_at];
                return (sql, params);
            }
            None => {
                let sql = "INSERT INTO perl (author, content, created_at) VALUES (?1, ?2, ?3);";
                let params: Vec<&dyn ToSql> = vec![&self.author, &self.content, &self.created_at];
                return (sql, params);
            }
        }
    }

    fn from_data(row: &Row<'_>) -> Perl {
        Perl {
            author: row.get(1).unwrap(),
            content: row.get(2).unwrap(),
            context: row.get(3).unwrap(),
            created_at: row.get(4).unwrap(),
        }
    }
}

pub fn add(perl: &Perl) {
    let db = database::setup();
    db.insert(perl);
}

pub fn list() -> Vec<Perl> {
    let db = database::setup();
    db.select::<Perl>("SELECT * FROM perl ORDER BY created_at DESC;", NO_PARAMS)
}
