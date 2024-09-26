use super::Driver;
use sinepiac_diagnostics::{Error, Files};
use sinepiac_span::{get_newlines, SourceFile};
use std::borrow::Cow;

impl<'db> Files<'db> for Driver {
    type FileId = SourceFile;

    type Name = Cow<'db, str>;

    type Source = &'db str;

    fn name(&'db self, id: Self::FileId) -> Result<Self::Name, Error> {
        Ok(id.path(&self.db).to_string_lossy())
    }

    fn source(&'db self, id: Self::FileId) -> Result<Self::Source, Error> {
        Ok(id.content(&self.db))
    }

    fn line_index(&'db self, id: Self::FileId, byte_index: usize) -> Result<usize, Error> {
        let line_numbers = get_newlines(&self.db, id);
        match line_numbers.line_index(&self.db, byte_index as u32) {
            Some(idx) => Ok(idx as usize),
            None => Err(Error::IndexTooLarge {
                given: byte_index,
                max: line_numbers.total_bytes(&self.db) as usize,
            }),
        }
    }

    fn line_range(
        &'db self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<std::ops::Range<usize>, Error> {
        let line_numbers = get_newlines(&self.db, id);
        Ok(line_numbers.line_span(&self.db, line_index as u32).into())
    }
}
