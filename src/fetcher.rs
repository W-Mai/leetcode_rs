extern crate reqwest;
extern crate serde_json;

use std::fmt::{Display, Error, Formatter};
use reqwest::Response;

const PROBLEMS_URL: &str = "https://leetcode.cn/api/problems/algorithms";
const GRAPHQL_URL: &str = "https://leetcode.cn/graphql";

const QUESTION_QUERY_OPERATION: &str = "questionData";

pub async fn get_problems() -> Option<String> {
    let client = reqwest::Client::new();
    let res = client.get(PROBLEMS_URL)
        .send()
        .await.ok();
    match res {
        None => { None }
        Some(res) => {
            res.text().await.ok()
        }
    }
}
