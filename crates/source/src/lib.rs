use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

use dashmap::DashMap;
use serde::Deserialize;
use serde::Serialize;

use mago_interner::StringIdentifier;
use mago_interner::ThreadedInterner;

use crate::error::SourceError;

pub mod error;

/// A unique identifier for a source, consisting of a string identifier and a user-defined flag.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct SourceIdentifier(pub StringIdentifier, pub bool);

/// Represents a source file with an identifier, optional path, content, and line information.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Source {
    pub identifier: SourceIdentifier,
    pub path: Option<PathBuf>,
    pub content: StringIdentifier,
    pub size: usize,
    pub lines: Vec<usize>,
}

pub trait HasSource {
    fn source(&self) -> SourceIdentifier;
}

/// Internal structure to store source information before full loading.
#[derive(Debug)]
struct SourceEntry {
    /// The name of the source.
    name: String,
    /// The path to the source.
    path: Option<PathBuf>,
    /// The content of the source.
    content: Option<(StringIdentifier, usize, Vec<usize>)>,
}

/// A manager for sources, which stores sources and provides methods to insert and retrieve them.
#[derive(Clone, Debug)]
pub struct SourceManager {
    /// The interner used to store source names.
    interner: ThreadedInterner,
    /// The map of source identifiers to source entries.
    sources: Arc<DashMap<SourceIdentifier, SourceEntry>>,
}

impl SourceIdentifier {
    pub fn dummy() -> Self {
        Self(StringIdentifier::empty(), true)
    }

    /// Returns the string identifier of the source.
    #[inline(always)]
    pub const fn value(&self) -> StringIdentifier {
        self.0
    }

    /// Returns whether the source is external.
    #[inline(always)]
    pub const fn is_external(&self) -> bool {
        !self.1
    }

    /// Returns whether the source is user-defined.
    #[inline(always)]
    pub const fn is_user_defined(&self) -> bool {
        self.1
    }
}

impl Source {
    /// Retrieve the line number for the given byte offset.
    ///
    /// # Parameters
    ///
    /// - `offset`: The byte offset to retrieve the line number for.
    ///
    /// # Returns
    ///
    /// The line number for the given byte offset (0-based index).
    pub fn line_number(&self, offset: usize) -> usize {
        self.lines.binary_search(&offset).unwrap_or_else(|next_line| next_line - 1)
    }

    /// Retrieve the column number for the given byte offset.
    ///
    /// # Parameters
    ///
    /// - `offset`: The byte offset to retrieve the column number for.
    ///
    /// # Returns
    ///
    /// The column number for the given byte offset (0-based index).
    pub fn column_number(&self, offset: usize) -> usize {
        let line_start = self.lines.binary_search(&offset).unwrap_or_else(|next_line| self.lines[next_line - 1]);

        offset - line_start
    }
}

impl SourceManager {
    /// Creates a new source manager with the given interner.
    ///
    /// # Parameters
    ///
    /// - `interner`: The interner to use for source names.
    ///
    /// # Returns
    ///
    /// The new source manager.
    pub fn new(interner: ThreadedInterner) -> Self {
        Self { interner, sources: Arc::new(DashMap::new()) }
    }

    /// Inserts a source with the given name and path into the manager.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the source.
    /// - `path`: The path to the source.
    /// - `user_defined`: Whether the source is user-defined.
    ///
    /// # Returns
    ///
    /// The identifier of the inserted source.
    pub fn insert_path(&self, name: String, path: PathBuf, user_defined: bool) -> SourceIdentifier {
        let string_id = self.interner.intern(&name);
        let source_id = SourceIdentifier(string_id, user_defined);

        if self.sources.contains_key(&source_id) {
            return source_id;
        }

        self.sources.insert(source_id, SourceEntry { name, path: Some(path), content: None });

        source_id
    }

    /// Inserts a source with the given name and content into the manager.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the source.
    /// - `content`: The content of the source.
    /// - `user_defined`: Whether the source is user-defined.
    ///
    /// # Returns
    ///
    /// The identifier of the inserted source.
    pub fn insert_content(&mut self, name: String, content: String, user_defined: bool) -> SourceIdentifier {
        if let Some(entry) = self.sources.iter().find(|entry| entry.name == name) {
            return *entry.key();
        }

        let source_id = SourceIdentifier(self.interner.intern(&name), user_defined);
        let lines = line_starts(&content).collect();
        let size = content.len();
        let content = self.interner.intern(content);

        self.sources.insert(source_id, SourceEntry { name, path: None, content: Some((content, size, lines)) });

        source_id
    }

    /// Checks whether the manager contains a source with the given identifier.
    ///
    /// # Parameters
    ///
    /// - `source_id`: The identifier of the source to check for.
    ///
    /// # Returns
    ///
    /// Whether the manager contains a source with the given identifier.
    pub fn contains(&self, source_id: &SourceIdentifier) -> bool {
        self.sources.contains_key(source_id)
    }

    /// Retrieve an iterator over all source identifiers in the manager.
    pub fn source_ids(&self) -> impl Iterator<Item = SourceIdentifier> + '_ {
        self.sources.iter().map(|entry| *entry.key())
    }

    /// Retrieve an iterator over all user-defined source identifiers in the manager.
    pub fn user_defined_source_ids(&self) -> impl Iterator<Item = SourceIdentifier> + '_ {
        self.sources.iter().filter(|entry| entry.key().is_user_defined()).map(|entry| *entry.key())
    }

    /// Retrieve an iterator over all external source identifiers in the manager.
    pub fn external_source_ids(&self) -> impl Iterator<Item = SourceIdentifier> + '_ {
        self.sources.iter().filter(|entry| entry.key().is_external()).map(|entry| *entry.key())
    }

    /// Retrieve the source with the given identifier from the manager.
    ///
    /// # Parameters
    ///
    /// - `source_id`: The identifier of the source to retrieve.
    ///
    /// # Returns
    ///
    /// The source with the given identifier, or an error if the source does not exist, or could not be loaded.
    pub fn load(&self, source_id: &SourceIdentifier) -> Result<Source, SourceError> {
        let Some(mut entry) = self.sources.get_mut(source_id) else {
            return Err(SourceError::UnavailableSource(*source_id));
        };

        match &entry.content {
            Some((content, size, lines)) => Ok(Source {
                identifier: *source_id,
                path: entry.path.clone(),
                content: *content,
                size: *size,
                lines: lines.clone(),
            }),
            None => {
                let path = entry.path.clone().expect("source entry must contain either content or path");
                let content = std::fs::read(&path)
                    .map(|bytes| match String::from_utf8_lossy(&bytes) {
                        Cow::Borrowed(str) => str.to_string(),
                        Cow::Owned(string) => {
                            tracing::warn!(
                                "encountered invalid utf-8 sequence in file {:?}. behavior with non-utf-8 files is undefined and may lead to unexpected results.",
                                path,
                            );

                            string
                        }
                    })?;

                let (_, v) = entry.pair_mut();

                let lines: Vec<_> = line_starts(&content).collect();
                let size = content.len();
                let content = self.interner.intern(content);

                let source = Source { identifier: *source_id, path: Some(path), content, size, lines: lines.clone() };

                v.content = Some((content, size, lines));

                Ok(source)
            }
        }
    }

    pub fn write(&self, source_id: SourceIdentifier, content: String) -> Result<(), SourceError> {
        let mut entry = self.sources.get_mut(&source_id).ok_or(SourceError::UnavailableSource(source_id))?;

        // Update the content of the source entry.
        let lines = line_starts(&content).collect();
        let size = content.len();
        let content = self.interner.intern(content);

        let (_, v) = entry.pair_mut();
        if let Some((old_content, _, _)) = v.content.as_mut() {
            if *old_content == content {
                return Ok(());
            }
        }

        v.content = Some((content, size, lines));
        if let Some(path) = entry.value().path.as_ref() {
            std::fs::write(path, self.interner.lookup(&content)).map_err(SourceError::IOError)?;
        }

        Ok(())
    }

    /// Retrieve the number of sources in the manager.
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Check whether the manager is empty.
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }
}

unsafe impl Send for SourceManager {}
unsafe impl Sync for SourceManager {}

impl<T: HasSource> HasSource for Box<T> {
    fn source(&self) -> SourceIdentifier {
        self.as_ref().source()
    }
}

fn line_starts(source: &str) -> impl '_ + Iterator<Item = usize> {
    std::iter::once(0).chain(source.match_indices('\n').map(|(i, _)| i + 1))
}
