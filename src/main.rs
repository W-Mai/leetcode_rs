mod fetcher;

use askama::Template;
use futures::executor::block_on;

#[derive(Template)]
#[template(path = "template")]
struct QuestionTemplate {
    problem_id: String,
    problem_title: String,
    problem_desc: String,

    problem_link: String,
    discuss_link: String,

    extra_use: String,
    problem_default_code: String,
}

impl QuestionTemplate {
    fn new(
        problem_id: String,
        problem_title: String,
        problem_desc: String,
        problem_link: String,
        discuss_link: String,
        extra_use: String,
        problem_default_code: String,
    ) -> Self {
        QuestionTemplate {
            problem_id,
            problem_title,
            problem_desc,
            problem_link,
            discuss_link,
            extra_use,
            problem_default_code,
        }
    }

    fn write(&self, path: &str) -> bool {
        if let Ok(rendered) = self.render() {
            return std::fs::write(path, rendered).is_ok();
        }
        false
    }
}

#[tokio::main]
async fn main() {
    if let Some(problems) = block_on(fetcher::get_problems()) {
        let first = problems.stat_status_pairs.first();
        if let Some(first) = first {
            let problem = block_on(fetcher::get_problem(first));
            if let Some(problem) = problem {
                println!("{}", problem.title);
                let file_name = format!("{}-{}", problem.title_slug, problem.title);
                let question_id = problem.question_id.replace(" ", "_");
                QuestionTemplate::new(
                    question_id.clone(),
                    problem.title,
                    problem.content,
                    format!("https://leetcode.com/problems/{}/", problem.title_slug),
                    format!(
                        "https://leetcode.com/problems/{}/discuss/",
                        problem.title_slug
                    ),
                    "".to_string(),
                    problem
                        .code_definition
                        .iter()
                        .find(|x| x.value == "rust")
                        .unwrap()
                        .default_code
                        .clone(),
                )
                .write(format!("./tests/{}-{}.rs", question_id, file_name).as_ref());
            }
        }
    }
}
