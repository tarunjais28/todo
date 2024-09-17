use crate::*;

pub fn run_line(line: &str, tl: &mut TodoList, cl: &mut Vec<TodoItem>) -> Option<String> {
    if let Ok((_, q)) = parser::query(line) {
        match run_query(q, tl, cl) {
            Ok(r) => Some(format!("{}", r)),
            Err(e) => Some(format!("Error: {}", e)),
        }
    } else {
        None
    }
}

fn run_query<'a>(
    q: Query,
    tl: &'a mut TodoList,
    cl: &'a mut Vec<TodoItem>,
) -> Result<QueryResult<'a>, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            let item = tl.push(desc, tags);
            Ok(QueryResult::Added(item))
        }
        Query::Done(idx) => {
            tl.done_with_index(idx, cl);
            Ok(QueryResult::Done)
        }
        Query::Search(params) => {
            let items = tl.search(params);
            Ok(QueryResult::Found(items))
        }
    }
}
