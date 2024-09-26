use salsa::Database;
use tracing::trace;

use crate::{SourceFile, Span};

/// This data structure holds locations of of the starting bytes of
/// new lines note we start counting lines from zero
#[salsa::tracked]
pub struct NewlinesLocs<'db> {
    #[return_ref]
    locs: Vec<u32>,
}

impl<'db> NewlinesLocs<'db> {
    pub fn line_count(&self, db: &dyn Database) -> u32 {
        self.locs(db).len() as u32
    }
    pub fn line_span(&'db self, db: &dyn Database, line_idx: u32) -> Span {
        trace!("NewlinesLocs::line_span Req, line_idx = {line_idx}");
        let locs = self.locs(db);
        let idx = line_idx as usize;
        let span = Span::default().with_lo(locs[idx]).with_hi(locs[idx + 1]);
        trace!("NewlinesLocs::line_span Resp, line_idx = {line_idx}, span = {span:?}");
        span
    }
    pub fn line_index(&'db self, db: &dyn Database, byte_index: u32) -> Option<u32> {
        let locs = self.locs(db);
        // TODO binary search
        for (idx, loc) in locs.iter().enumerate() {
            if byte_index < *loc {
                return Some(idx as u32 - 1);
            }
        }
        None
    }
    pub fn total_bytes(&self, db: &dyn Database) -> u32 {
        let locs = self.locs(db);
        locs[locs.len() - 1]
    }
}

#[salsa::tracked]
pub fn get_newlines<'db>(db: &'db dyn Database, src: SourceFile) -> NewlinesLocs<'db> {
    let content = src.content(db).chars();
    let mut locs: Vec<_> = vec![0];
    let mut accumulator = 0;
    for c in content {
        accumulator += c.len_utf8() as u32;
        if c == '\n' {
            locs.push(accumulator)
        }
    }
    locs.push(accumulator);
    //debug!("get_new_lines = {locs:?}");
    NewlinesLocs::new(db, locs)
}
