use askama::Template;

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
}

fn main() {
    let hello = QuestionTemplate::new(20,
                                      "Hello".to_string(),
                                      "World".to_string(),
                                      "".to_string(),
                                      "".to_string(),
                                      "".to_string(),
                                      "".to_string());

    std::fs::write("./template.rs", hello.render().unwrap()).unwrap();
}
