use rusqlite::{params, Connection, Result};

/// 电影结构体，表示一部电影的记录。
#[derive(Debug, Clone)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub director: String,
    pub release_date: String,
}

impl Movie {
    /// 创建一个新的电影实例。
    pub fn new(id: u32, title: String, director: String, release_date: String) -> Self {
        Self {
            id,
            title,
            director,
            release_date,
        }
    }
}

/// 电影管理器，用于管理电影的CRUD操作。
pub struct MovieManager {
    conn: Connection,
}

impl MovieManager {
    /// 创建一个新的电影管理器，并初始化数据库连接。
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let manager = Self { conn };
        manager.initialize_database()?;
        Ok(manager)
    }

    /// 初始化数据库，创建movies表（如果不存在）。
    fn initialize_database(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS movies (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                director TEXT NOT NULL,
                release_date TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    /// 添加一部新的电影到数据库。
    pub fn create_movie(&self, id: u32, title: String, director: String, release_date: String) -> Result<()> {
        self.conn.execute(
            "INSERT INTO movies (id, title, director, release_date) VALUES (?1, ?2, ?3, ?4)",
            params![id, title, director, release_date],
        )?;
        Ok(())
    }

    /// 根据ID读取一部电影的信息。
    pub fn read_movie(&self, id: u32) -> Result<Option<Movie>> {
        let mut stmt = self.conn.prepare("SELECT id, title, director, release_date FROM movies WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let movie = Movie {
                id: row.get(0)?,
                title: row.get(1)?,
                director: row.get(2)?,
                release_date: row.get(3)?,
            };
            Ok(Some(movie))
        } else {
            Ok(None)
        }
    }

    /// 更新一部电影的信息。
    pub fn update_movie(
        &self,
        id: u32,
        title: Option<String>,
        director: Option<String>,
        release_date: Option<String>,
    ) -> Result<bool> {
        let exists = self.read_movie(id)?.is_some();
        if !exists {
            return Ok(false);
        }

        if let Some(title) = title {
            self.conn.execute("UPDATE movies SET title = ?1 WHERE id = ?2", params![title, id])?;
        }
        if let Some(director) = director {
            self.conn.execute("UPDATE movies SET director = ?1 WHERE id = ?2", params![director, id])?;
        }
        if let Some(release_date) = release_date {
            self.conn.execute("UPDATE movies SET release_date = ?1 WHERE id = ?2", params![release_date, id])?;
        }
        Ok(true)
    }

    /// 删除一部电影。
    pub fn delete_movie(&self, id: u32) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM movies WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    /// 列出所有电影。
    pub fn list_movies(&self) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT id, title, director, release_date FROM movies")?;
        let movie_iter = stmt.query_map([], |row| {
            Ok(Movie {
                id: row.get(0)?,
                title: row.get(1)?,
                director: row.get(2)?,
                release_date: row.get(3)?,
            })
        })?;

        for movie in movie_iter {
            let m = movie?;
            println!(
                "ID: {}, Title: {}, Director: {}, Release Date: {}",
                m.id, m.title, m.director, m.release_date
            );
        }
        Ok(())
    }
}
