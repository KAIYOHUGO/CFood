use std::{collections::BTreeMap, ops::Range};

use crate::cst::marked::Marked;

pub type Span = Range<CodeIndex>;
pub type SpanStore = BTreeMap<usize, Span>;

#[derive(Clone, Copy, Debug)]
pub struct CodeIndex {
    pub line: u32,
    pub col: u32,
    pub offset: usize,
}

pub trait Spanned {
    fn span(&self, store: &SpanStore) -> Option<Span>;
}

impl<T: Marked> Spanned for T {
    fn span(&self, store: &SpanStore) -> Option<Span> {
        store.get(&self.mark()).cloned()
    }
}

impl<T: Spanned> Spanned for Option<T> {
    fn span(&self, store: &SpanStore) -> Option<Span> {
        self.as_ref().map(|x| x.span(store)).flatten()
    }
}

impl<T: Spanned> Spanned for Vec<T> {
    fn span(&self, store: &SpanStore) -> Option<Span> {
        let start = self.first().map(|x| x.span(store)).flatten();
        let end = self.last().map(|x| x.span(store)).flatten();
        match (start, end) {
            (None, None) => None,
            (Some(start), Some(end)) => Some(start.start..end.end),
            _ => unreachable!(),
        }
    }
}
