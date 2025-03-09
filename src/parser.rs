use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "spec.pest"]
pub struct TemplateParser;

#[derive(Debug)]
pub struct Template {
    pub variables: Vec<String>,
    pub expressions: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Text(String),
    Variable(String),
    TextGroup(TextGroup),
}

#[derive(Debug)]
pub struct TextGroup {
    pub format: Vec<Expression>,
    pub style: String,
}

pub fn parse_template(template: &str) -> Template {
    let pairs = TemplateParser::parse(Rule::file, template).unwrap();
    let mut variables: Vec<String> = vec![];

    fn parse_expressions(
        pairs: pest::iterators::Pairs<Rule>,
        _variables: &mut Vec<String>,
    ) -> Vec<Expression> {
        let mut expressions: Vec<Expression> = vec![];

        for pair in pairs {
            match pair.as_rule() {
                Rule::text => expressions.push(Expression::Text(pair.as_str().to_string())),
                Rule::variable => {
                    let variable = &pair.as_str()[1..]; // remove $ sign at start
                    _variables.push(variable.to_string());
                    expressions.push(Expression::Variable(variable.to_string()))
                }
                Rule::text_group => {
                    let mut pair = pair.into_inner();
                    let format = parse_expressions(pair.next().unwrap().into_inner(), _variables);
                    let style = pair.next().unwrap().as_str().trim();

                    expressions.push(Expression::TextGroup(TextGroup {
                        format,
                        style: style.to_string(),
                    }));
                }
                _ => {}
            }
        }

        expressions
    }

    let expressions = parse_expressions(pairs, &mut variables);

    Template {
        variables,
        expressions,
    }
}
