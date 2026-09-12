use std::sync::Arc;

use arrow_array::{RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use once_cell::sync::Lazy;

use crate::{error::Result, sql::generated::CommandGetCatalogs};

/// A builder for a [`CommandGetCatalogs`] response.
///
/// Builds rows like this:
///
/// * catalog_name: utf8,
pub struct GetCatalogsBuilder {
    catalogs: Vec<String>,
}

impl CommandGetCatalogs {
    /// Create a builder suitable for constructing a response
    pub fn into_builder(self) -> GetCatalogsBuilder {
        self.into()
    }
}

impl From<CommandGetCatalogs> for GetCatalogsBuilder {
    fn from(_: CommandGetCatalogs) -> Self {
        Self::new()
    }
}

impl Default for GetCatalogsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl GetCatalogsBuilder {
    /// Create a new instance of [`GetCatalogsBuilder`]
    pub fn new() -> Self {
        Self {
            catalogs: Vec::new(),
        }
    }

    /// Append a row
    pub fn append(&mut self, catalog_name: impl Into<String>) {
        self.catalogs.push(catalog_name.into());
    }

    /// builds a `RecordBatch` with the correct schema for a
    /// [`CommandGetCatalogs`] response
    pub fn build(self) -> Result<RecordBatch> {
        let Self { mut catalogs } = self;
        catalogs.sort_unstable();

        let batch = RecordBatch::try_new(
            Arc::clone(&GET_CATALOG_SCHEMA),
            vec![Arc::new(StringArray::from_iter_values(catalogs)) as _],
        )?;

        Ok(batch)
    }

    /// Returns the schema that will result from [`CommandGetCatalogs`]
    ///
    /// [`CommandGetCatalogs`]: crate::sql::CommandGetCatalogs
    pub fn schema(&self) -> SchemaRef {
        get_catalogs_schema()
    }
}

fn get_catalogs_schema() -> SchemaRef {
    Arc::clone(&GET_CATALOG_SCHEMA)
}

/// The schema for GetCatalogs
static GET_CATALOG_SCHEMA: Lazy<SchemaRef> = Lazy::new(|| {
    Arc::new(Schema::new(vec![Field::new(
        "catalog_name",
        DataType::Utf8,
        false,
    )]))
});
