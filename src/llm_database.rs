use rusqlite::{Connection, Result, Row};

#[derive(Debug)]
pub struct Conversation {
    pub id: String,
    pub name: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug)]
pub struct Response {
    pub id: String,
    pub model: String,
    pub prompt: String,
    pub system: Option<String>,
    pub response: String,
    pub conversation_id: Option<String>,
    pub duration_ms: Option<i64>,
    pub datetime_utc: String,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
}

impl Response {
    fn from_row(row: &Row) -> Result<Response> {
        Ok(Response {
            id: row.get("id")?,
            model: row.get("model")?,
            prompt: row.get("prompt")?,
            system: row.get("system")?,
            response: row.get("response")?,
            conversation_id: row.get("conversation_id")?,
            duration_ms: row.get("duration_ms")?,
            datetime_utc: row.get("datetime_utc")?,
            input_tokens: row.get("input_tokens")?,
            output_tokens: row.get("output_tokens")?,
        })
    }
}

impl Conversation {
    fn from_row(row: &Row) -> Result<Conversation> {
        Ok(Conversation {
            id: row.get("id")?,
            name: row.get("name")?,
            model: row.get("model")?,
        })
    }
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Database { conn })
    }

    pub fn get_conversations(&self) -> Result<Vec<Conversation>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, model FROM conversations")?;
        let conversations = stmt
            .query_map([], |row| Conversation::from_row(row))?
            .collect::<Result<Vec<_>>>()?;
        Ok(conversations)
    }

    pub fn get_responses_for_conversation(&self, conversation_id: &str) -> Result<Vec<Response>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, model, prompt, system, response, conversation_id,
                    duration_ms, datetime_utc, input_tokens, output_tokens
             FROM responses
             WHERE conversation_id = ?1
             ORDER BY datetime_utc DESC",
        )?;

        let responses = stmt
            .query_map([conversation_id], |row| Response::from_row(row))?
            .collect::<Result<Vec<_>>>()?;
        Ok(responses)
    }

    pub fn get_recent_responses(&self, limit: i64) -> Result<Vec<Response>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, model, prompt, system, response, conversation_id,
                    duration_ms, datetime_utc, input_tokens, output_tokens
             FROM responses
             ORDER BY datetime_utc DESC
             LIMIT ?1",
        )?;

        let responses = stmt
            .query_map([limit], |row| Response::from_row(row))?
            .collect::<Result<Vec<_>>>()?;
        Ok(responses)
    }
}
