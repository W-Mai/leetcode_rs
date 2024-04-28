mod problems;
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
        problem_id: u32,
        problem_title: String,
        problem_desc: String,
        problem_link: String,
        discuss_link: String,
        extra_use: String,
        problem_default_code: String,
    ) -> Self {
        QuestionTemplate {
            problem_id: format!("{:06}", problem_id),
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
                println!("{:?}", problem);
            }
        }
    }
}
