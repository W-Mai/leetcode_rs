extern crate reqwest;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::{Display, Error, Formatter};

const PROBLEMS_URL: &str = "https://leetcode.cn/api/problems/algorithms";
const GRAPHQL_URL: &str = "https://leetcode.cn/graphql";

const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        metaData
    }
}"#;

const QUESTION_QUERY_OPERATION: &str = "questionData";

pub async fn get_problems() -> Option<Problems> {
    let client = reqwest::Client::new();
    client
        .get(PROBLEMS_URL)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?
}

pub async fn get_problem(problem_stat: &StatWithStatus) -> Option<Problem> {
    if problem_stat.paid_only {
        println!(
            "Problem {} is paid-only",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }
    let resp = surf::post(GRAPHQL_URL).body_json(&Query::question_query(
        problem_stat.stat.question_title_slug.as_ref().unwrap(),
    ));
    if resp.is_err() {
        println!(
            "Problem {} not initialized due to some error",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }
    let resp = resp.unwrap().recv_json().await;
    if resp.is_err() {
        println!(
            "Problem {} not initialized due to some error",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }

    let translation_resp = surf::post(GRAPHQL_URL).body_json(&Query::translation_query(
        problem_stat.stat.question_title_slug.as_ref().unwrap(),
    ));

    let translation_resp = translation_resp.unwrap().recv_json().await;
    if translation_resp.is_err() {
        println!(
            "Problem {} not initialized due to some error",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }

    let resp: RawProblem = resp.unwrap();
    let translation_resp: TranslationResponse = translation_resp.unwrap();
    return Some(Problem {
        title: problem_stat.stat.question_title.clone().unwrap(),
        title_slug: problem_stat.stat.question_title_slug.clone().unwrap(),
        code_definition: serde_json::from_str(&resp.data.question.code_definition).unwrap(),
        content: translation_resp.data.question.translated_content,
        sample_test_case: resp.data.question.sample_test_case,
        difficulty: problem_stat.difficulty.to_string(),
        question_id: problem_stat.stat.frontend_question_id.clone(),
        return_type: {
            let v: Value = serde_json::from_str(&resp.data.question.meta_data).unwrap();
            v["return"]["type"].to_string().replace("\"", "")
        },
    });
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problems {
    user_name: String,
    num_solved: u32,
    num_total: u32,
    ac_easy: u32,
    ac_medium: u32,
    ac_hard: u32,
    pub stat_status_pairs: Vec<StatWithStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatWithStatus {
    pub stat: Stat,
    difficulty: Difficulty,
    paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    pub frontend_question_id: String,
    is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Difficulty {
    level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.level {
            1 => f.write_str("Easy"),
            2 => f.write_str("Medium"),
            3 => f.write_str("Hard"),
            _ => f.write_str("Unknown"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub title: String,
    pub title_slug: String,
    pub content: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: Vec<CodeDefinition>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    pub difficulty: String,
    pub question_id: String,
    pub return_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawProblem {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslationResponse {
    data: TranslationData,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslationData {
    question: Translation,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    content: String,
    stats: String,
    #[serde(rename = "codeDefinition")]
    code_definition: String,
    #[serde(rename = "sampleTestCase")]
    sample_test_case: String,
    #[serde(rename = "metaData")]
    meta_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Translation {
    #[serde(rename = "translatedTitle")]
    translated_title: String,
    #[serde(rename = "translatedContent")]
    translated_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

impl Query {
    fn question_query(title_slug: &str) -> Query {
        Query {
            operation_name: QUESTION_QUERY_OPERATION.to_owned(),
            variables: json!({ "titleSlug": title_slug }),
            query: QUESTION_QUERY_STRING.to_owned(),
        }
    }

    fn translation_query(title_slug: &str) -> Query {
        Query {
            operation_name: "questionTranslation".to_owned(),
            variables: json!({ "titleSlug": title_slug }),
            query: r#"
            query questionTranslation($titleSlug: String!) {
                question(titleSlug: $titleSlug) {
                    translatedTitle
                    translatedContent
                }
            }"#
            .to_owned(),
        }
    }
}
