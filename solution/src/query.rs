use std::fmt;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Query {
    Add(Description, Vec<Tag>),
    Done(Index),
    Search(SearchParams),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchParams {
    pub words: Vec<SearchWord>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchWord(pub String);
impl SearchWord {
    pub fn new(s: &str) -> SearchWord {
        SearchWord(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryResult<'a> {
    Added(TodoItem),
    Done,
    Found(Vec<&'a todo_list::TodoItem>),
}

impl<'a> fmt::Display for QueryResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            QueryResult::Added(ti) => write!(f, "{}", ti.index),
            QueryResult::Done => write!(f, "done"),
            QueryResult::Found(rs) => {
                let mut buff: Vec<String> = vec![];
                buff.push(format!("{} item(s) found", rs.len()));
                let mut tags = String::new();
                for items in rs {
                    items.tags.iter().all(|tag| {
                        tags.push('#');
                        tags.push_str(&tag.to_string());
                        tags.push(' ');
                        true
                    });
                    buff.push(format!("{} {} {}", items.index, items.description, tags));
                    tags.clear();
                }
                write!(f, "{}", buff.join("\n"))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryError(pub String);

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An error occurred while processing the query: {}.",
            self.0
        )
    }
}
