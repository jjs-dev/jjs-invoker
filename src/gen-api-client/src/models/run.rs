/*
 * JJS main API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Run : Represents a run.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Run {
    #[serde(rename = "contest_id")]
    pub contest_id: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "problem_name")]
    pub problem_name: String,
    #[serde(rename = "score")]
    pub score: Option<i32>,
    #[serde(rename = "status")]
    pub status: Option<crate::models::InvokeStatus>,
    #[serde(rename = "toolchain_name")]
    pub toolchain_name: String,
}

impl Run {
    /// Represents a run.
    pub fn new(
        contest_id: String,
        id: i32,
        problem_name: String,
        score: Option<i32>,
        status: Option<crate::models::InvokeStatus>,
        toolchain_name: String,
    ) -> Run {
        Run {
            contest_id,
            id,
            problem_name,
            score,
            status,
            toolchain_name,
        }
    }
}
