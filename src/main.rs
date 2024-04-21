use askama::Template;

#[derive(Template)]
#[template(path = "template")]
struct QuestionTemplate<'a> {
    problem_id: &'a str,
    problem_title: &'a str,
    problem_desc: &'a str,

    problem_link: &'a str,
    discuss_link: &'a str,

    extra_use: &'a str,
    problem_default_code: &'a str,
}

fn main() {
    let hello = QuestionTemplate {
        problem_id: "world",
        problem_title: "",
        problem_desc: "",
        problem_link: "",
        discuss_link: "",
        extra_use: "",
        problem_default_code: "",
    };
    println!("{}", hello.render().unwrap());
}
