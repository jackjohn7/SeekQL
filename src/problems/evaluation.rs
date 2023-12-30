use crate::problems::models::Problem;
use core::fmt;
use rusqlite::Connection;
use std::error::Error;

use super::models::{DataRetrievalTest, EvaluationType, RowsTest, StateTest};

#[derive(Debug)]
pub enum EvaluationErr {
    DatabaseFailure(String),
    EvaluationDataReadFailure(String),
    UserDatabaseFailure(String),
}

impl fmt::Display for EvaluationErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DatabaseFailure(e) => write!(
                f,
                "Evaluation Error:\nAn error occurred when creating database: {}",
                e
            ),
            Self::EvaluationDataReadFailure(e) => write!(
                f,
                "Evaluation Error:\nAn error occurred when creating database: {}",
                e
            ),
            Self::UserDatabaseFailure(e) => write!(
                f,
                "Evaluation Error:\nAn error occurred when creating database: {}",
                e
            ),
        }
    }
}

impl Error for EvaluationErr {}

impl From<rusqlite::Error> for EvaluationErr {
    fn from(error: rusqlite::Error) -> Self {
        Self::DatabaseFailure(error.to_string())
        //match error {
        //    rusqlite::Error::SqliteFailure(x, _) => Self::DatabaseFailure(x.to_string()),
        //}
    }
}

impl From<serde_json::Error> for EvaluationErr {
    fn from(error: serde_json::Error) -> Self {
        Self::EvaluationDataReadFailure(error.to_string())
    }
}

pub fn evaluate_input(user_input: String, problem: Problem) -> Result<(), EvaluationErr> {
    let conn = Connection::open_in_memory()?;

    // run the provided migration to create the testable conditions
    conn.execute_batch(&problem.db_migration)?;

    // parse the problem data into test data
    let eval_data: EvaluationType = serde_json::from_str(problem.eval_data.as_str())?;

    let evaluation = match eval_data {
        EvaluationType::StateModification(data) => evaluate_state_modification(&conn, data),
        EvaluationType::RowModification(data) => evaluate_row_modification(&conn, data),
        EvaluationType::DataRetrieval(data) => evaluate_data_retrieval(&conn, data),
    }?;

    if let Err(err) = conn.close() {
        return Err(EvaluationErr::DatabaseFailure(err.1.to_string()));
    }

    Ok(())
}

fn evaluate_state_modification(_conn: &Connection, _eval: StateTest) -> Result<(), EvaluationErr> {
    Ok(())
}

fn evaluate_row_modification(_conn: &Connection, _eval: RowsTest) -> Result<(), EvaluationErr> {
    Ok(())
}

fn evaluate_data_retrieval(
    _conn: &Connection,
    _eval: DataRetrievalTest,
) -> Result<(), EvaluationErr> {
    Ok(())
}
