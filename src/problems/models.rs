use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StateTest {
    query: String,
    expected_result: String,
}

#[derive(Serialize, Deserialize)]
pub struct RowsTest {
    query: String,
    expected_result: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataRetrievalTest {
    query: String,
    expected_result: String,
}

#[derive(Serialize, Deserialize)]
pub enum EvaluationType {
    StateModification(StateTest),
    RowModification(RowsTest),
    DataRetrieval(DataRetrievalTest),
}

#[derive(sqlx::FromRow)]
pub struct Problem {
    pub id: u64,
    pub problem_name: String,
    pub description: String,
    pub markdown: String,
    pub eval_data: String,
    pub db_migration: String,
}
