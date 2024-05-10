mod fetcher;

use askama::Template;
use futures::executor::block_on;
use futures::FutureExt;

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
        for problem in problems.stat_status_pairs {
            fetcher::get_problem(&problem)
                .then(|problem| async {
                    if let Some(problem) = problem {
                        let question_id = problem.question_id;
                        let file_name = format!("{}-{}", problem.title_slug, problem.title)
                            .replace(" ", "_")
                            .replace(".", "_")
                            .replace("?", "")
                            .replace("!", "")
                            .replace("(", "_")
                            .replace(")", "_");

                        let file_name = format!("./tests/{}-{}.rs", question_id, file_name);
                        let code_definition =
                            problem.code_definition.iter().find(|x| x.value == "rust");
                        if code_definition.is_none() {
                            return;
                        }
                        println!("{}", file_name);

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
                            code_definition.unwrap().default_code.clone(),
                        )
                        .write(file_name.as_ref());
                    }
                })
                .await;
        }
    }
}
