mod llm_database;

use llm_database::{Database, Response};
use rusqlite::Result;

fn display_response(response: &Response) {
    println!("Response ID: {}", response.id);
    println!("DateTime: {}", response.datetime_utc);
    println!("Model: {}", response.model);
    println!("Prompt: {}", response.prompt);
    println!("Response: {}", response.response);
    println!(
        "Tokens: in={:?}, out={:?}\n",
        response.input_tokens, response.output_tokens
    );
}

fn main() -> Result<()> {
    let db_path = "";
    let db = Database::new(db_path)?;

    println!("Reading conversations...");
    let conversations = db.get_conversations()?;
    for conv in conversations {
        println!("\nConversation: {:?}", conv);

        if let Ok(responses) = db.get_responses_for_conversation(&conv.id) {
            for response in responses {
                display_response(&response);
            }
        }
    }

    Ok(())
}
