use crate::*;

pub fn run_line(line: &str, tl: &mut TodoList) -> Option<String> {
    if let Ok((_, q)) = parser::query(line) {
        match run_query(q, tl) {
            Ok(r) => Some(format!("{}", r)),
            Err(e) => Some(format!("Error: {}", e)),
        }
    } else {
        None
    }
}

fn run_query(q: Query, tl: &mut TodoList) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            let idx = tl.push(desc, tags);
            Ok(QueryResult::Added(idx))
        }
        Query::Done(idx) => {
            tl.done_with_index(idx);
            Ok(QueryResult::Done)
        }
        Query::Search(params) => {
            let indices = tl.search(params);
            Ok(QueryResult::Found(indices))
        }
    }
}
