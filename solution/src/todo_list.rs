use std::fmt;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index(u64);

impl Index {
    pub fn new(i: u64) -> Index {
        Index(i)
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn increement(&mut self) {
        self.0 += 1;
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(s: &str) -> Description {
        Description(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag {
        Tag(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn from_strings(ss: Vec<&str>) -> Vec<Tag> {
        ss.into_iter().map(Tag::new).collect()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub index: Index,
    pub description: Description,
    pub tags: Vec<Tag>,
    pub done: bool,
}

impl TodoItem {
    pub fn new(index: Index, description: Description, tags: Vec<Tag>) -> TodoItem {
        TodoItem {
            index,
            description,
            tags,
            done: false,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}, {:?}", self.index, self.description, self.tags)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoList {
    top_index: Index,
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            top_index: Index::new(0),
            items: vec![],
        }
    }

    pub fn push(&mut self, description: Description, tags: Vec<Tag>) -> TodoItem {
        let item = TodoItem::new(self.top_index, description, tags);
        self.items.push(item.to_owned());
        self.top_index.increement();
        item
    }

    pub fn done_with_index(&mut self, idx: Index, cl: &mut Vec<TodoItem>) -> Option<Index> {
        if self.items.get(idx.value() as usize).is_some() {
            // Moving item to completed list
            let mut item = self.items.remove(idx.value() as usize);
            item.done = true;
            cl.push(item);
            Some(idx)
        } else {
            None
        }
    }

    pub fn search(&self, sp: SearchParams) -> Vec<&TodoItem> {
        let mut items: Vec<&TodoItem> = Vec::new();
        
        if sp.words.is_empty() && sp.tags.is_empty() {
            self.items.iter().all(|item| {
                items.push(item);
                true
            });
        }

        for words in sp.words {
            items = self
                .items
                .iter()
                .filter(|item| item.description.value().contains(words.value()))
                .collect();
        }

        for tag in sp.tags {
            items.extend(
                self.items
                    .iter()
                    .filter(|item| item.tags.contains(&tag) && !items.contains(item))
                    .collect::<Vec<&TodoItem>>(),
            );
        }

        items.reverse();

        items
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}
