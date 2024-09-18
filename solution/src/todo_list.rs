use crate::*;
use rayon::prelude::*;
use std::{collections::HashSet, fmt};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
pub struct TodoList {
    top_index: Arc<Mutex<Index>>,
    items: Arc<Mutex<Vec<TodoItem>>>,
}

impl TodoList {
    pub fn new(capacity: usize) -> TodoList {
        TodoList {
            top_index: Arc::new(Mutex::new(Index::new(0))),
            items: Arc::new(Mutex::new(Vec::with_capacity(capacity))),
        }
    }

    pub fn push(&self, description: Description, tags: Vec<Tag>) -> Index {
        let mut top_index = self.top_index.lock().unwrap();
        let idx = top_index.clone(); // Assuming Index implements Clone
        let item = TodoItem::new(idx.clone(), description, tags);

        // Lock items vector and push the new item
        let mut items = self.items.lock().unwrap();
        items.push(item);

        top_index.increement(); // Update the top_index
        idx
    }   

    pub fn done_with_index(&self, idx: Index) -> Option<Index> {
        let item_idx = idx.value() as usize;

        // Lock items vector to safely access and modify it
        let mut items = self.items.lock().unwrap();

        if item_idx < items.len() {
            let item = &mut items[item_idx];
            item.done = true;
            Some(idx)
        } else {
            None
        }
    }

    pub fn search(&self, sp: SearchParams) -> Vec<Index> {
        let mut indices: HashSet<Index> = HashSet::new();

        // Case 1: When both search words and search tags are missing
        if sp.words.is_empty() && sp.tags.is_empty() {
            self.items.lock().unwrap().iter().for_each(|item| {
        indices.insert(item.index);
            });
            return indices.into_iter().collect();
        }

        self.items.lock().unwrap()
            .par_iter() 
            .filter_map(|item| {
                let mut cw = 0;
                let mut fw = 0;

                // Case 2: When only searching with words
                if !sp.words.is_empty() && sp.tags.is_empty() {
                    cw = sp.words.len();
                    sp.words.iter().all(|word| {
                        find_words(item.description.value(), word.value(), &mut fw, item.done)
                    });
                }

                // Case 3: When only searching with tags
                if sp.words.is_empty() && !sp.tags.is_empty() {
                    cw = sp.tags.len();
                    sp.tags
                        .iter()
                        .all(|tag| find_tags(&item.tags, tag, &mut fw, item.done));
                }

                // Case 4: When searching with both words and tags
                if !sp.words.is_empty() && !sp.tags.is_empty() {
                    sp.words.iter().all(|word| {
                        find_words(item.description.value(), word.value(), &mut cw, item.done)
                    });
                    sp.tags
                        .iter()
                        .all(|tag| find_tags(&item.tags, tag, &mut fw, item.done));
                }

                if fw == cw && fw > 0 {
                    Some(item.index)
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Notes:
/// This two functions `find_word()` and `find_tags()` takes can be merged
/// to single function, but due to performance constraint they are being
/// splitted to multiple funtions.
///
/// fn find_subsequence_in_items<F, T>(
///     items: &[T],
///     target: &T,
///     extract_fn: F,
///     counter: &mut usize,
///     is_done: bool,
/// ) -> bool
/// where
///     F: Fn(&T) -> &str,
/// {
///     if is_done {
///         return false;
///     }
///
///     let target_chars: Vec<char> = extract_fn(target).chars().collect();
///     let mut target_iter = target_chars.iter();
///     let mut target_item = target_iter.next();
///
///     for item in items {
///         let item_str = extract_fn(item);
///
///         // Try to match each character in the target
///         for ch in item_str.chars() {
///             if let Some(&c) = target_item {
///                 if c == ch {
///                     target_item = target_iter.next();
///                 }
///             }
///
///             // If we've iterated through all items in the subsequence, it means the subsequence was found
///             if target_item.is_none() {
///                 *counter += 1;
///                 return true;
///             }
///         }
///
///         // Reset target iteration for the next item
///         target_iter = target_chars.iter();
///         target_item = target_iter.next();
///     }
///
///     false
/// }
///
/// fn main() {
///     // Iterating over words
///     sp.words.iter().all(|word| {
///     find_subsequence_in_items(
///         &item
///             .description
///             .value()
///             .split_whitespace()
///             .collect::<Vec<&str>>(),
///         &word.value(),
///         |s| s,
///         &mut fw,
///         item.done,
///     )
///   });
///
/// // Iterating over tags
///     sp.tags.iter().all(|tag| {
///         find_subsequence_in_items(&item.tags, tag, |t| t.value(), &mut fw, item.done)
///     });
/// }
fn find_words(sentence: &str, word: &str, counter: &mut usize, is_done: bool) -> bool {
    if is_done {
        return false;
    }

    let mut word_iter = word.chars();
    let mut word_item = word_iter.next();
    for sen in sentence.split_whitespace() {
        // Try to match each character in word
        for ch in sen.chars() {
            if let Some(c) = word_item {
                if c == ch {
                    word_item = word_iter.next();
                }
            }

            // If we've iterated through all items in the subsequence, it means the subsequence was found
            if word_item.is_none() {
                *counter += 1;
                return true;
            }
        }
        // Reset iteration for the next item
        word_iter = word.chars();
        word_item = word_iter.next();
    }
    false
}

fn find_tags(tags: &[Tag], sh_tag: &Tag, counter: &mut usize, is_done: bool) -> bool {
    if is_done {
        return false;
    }
    let mut word_iter = sh_tag.value().chars();
    let mut word_item = word_iter.next();

    for tag in tags {
        // Try to match each character in word
        for ch in tag.value().chars() {
        if let Some(c) = word_item {
        if c == ch {
                    word_item = word_iter.next();
                }
            }

            // If we've iterated through all items in the subsequence, it means the subsequence was found
            if word_item.is_none() {
                *counter += 1;
                return true;
            }
        }
        // Reset iteration for the next item
        word_iter = sh_tag.value().chars();
        word_item = word_iter.next();
    }
    false
}
