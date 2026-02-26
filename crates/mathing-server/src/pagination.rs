use std::fmt::Display;

use crate::prelude::{
    mathing_proto::{PaginationRequest, PaginationResponse},
    *,
};

#[derive(Debug, Clone, Copy)]
pub struct PaginationBuilder {
    count: Option<u32>,
    limit: u32,
    page: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    count: u32,
    pub limit: u32,
    page: u32,
}

impl Default for PaginationBuilder {
    fn default() -> Self {
        Self {
            limit: 30,
            page: 1,
            count: Default::default(),
        }
    }
}

impl From<Option<PaginationRequest>> for PaginationBuilder {
    fn from(value: Option<PaginationRequest>) -> Self {
        if let Some(value) = value {
            Self {
                limit: value.limit,
                page: value.page,
                ..Default::default()
            }
        } else {
            Self::default()
        }
    }
}

impl PaginationBuilder {
    pub fn with_count(&mut self, count: u32) {
        self.count = Some(count);
    }
    pub fn try_build(self) -> Result<Pagination, ServerError> {
        let count = self.count.ok_or(ServerError::ConversionError(
            "OffsetBuilder",
            "Offset",
            "count: None".to_string(),
        ))?;

        Ok(Pagination {
            limit: self.limit,
            page: self.page,
            count,
        })
    }
}

impl Pagination {
    /// Provides the sql offset from the pagination request parameters
    pub fn get_offset(&self) -> u32 {
        self.limit * (self.page - 1)
    }
    pub fn get_total_pages(&self) -> u32 {
        self.count.div_ceil(self.limit)
    }
    pub fn try_validate(&self) -> Result<(), ClientError> {
        if (self.page == 0) || (self.get_offset() > self.count) {
            return Err(ClientError::OutOfBounds(*self));
        }
        Ok(())
    }
}

impl Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Limit: '{}', Page: '{}', Count: '{}'",
            self.limit, self.page, self.count
        )
    }
}

impl From<Pagination> for PaginationResponse {
    fn from(value: Pagination) -> Self {
        Self {
            total_rows: value.count,
            total_pages: value.get_total_pages(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::expected_error;

    use super::*;

    #[test]
    fn test_build_error() {
        let want =
            ServerError::ConversionError("OffsetBuilder", "Offset", "count: None".to_string());
        let offset = PaginationBuilder {
            limit: 20,
            page: 1,
            count: None,
        };
        let got = offset.try_build().map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[test]
    fn test_out_of_range_validate() {
        let offset = Pagination {
            count: 20,
            limit: 40,
            page: 3,
        };
        let want = ClientError::OutOfBounds(offset);
        let got = offset.try_validate().map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[test]
    /// pagination requests should not request a page 0
    /// the first page should be 1
    fn test_zero_out_of_range() {
        let offset = Pagination {
            count: 43,
            limit: 10,
            page: 0,
        };
        let want = ClientError::OutOfBounds(offset);
        let got = offset.try_validate().map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[test]
    fn test_valid_offset() -> anyhow::Result<()> {
        let offset = PaginationBuilder {
            limit: 40,
            page: 1,
            count: Some(20),
        };
        offset.try_build()?.try_validate()?;
        Ok(())
    }

    #[test]
    /// Total page count should round up when there are remainders
    /// in the division.
    fn test_page_count() {
        let want = 5;
        let offset = Pagination {
            count: 43,
            page: 1,
            limit: 10,
        };
        let got = offset.get_total_pages();

        assert_eq!(want, got)
    }

    #[test]
    /// getting a count of 0 from the db should still be valid
    /// and produce valid total_pages and offset calculations.
    fn test_zero_count() -> anyhow::Result<()> {
        let want_total_pages = 0;
        let want_offset = 0;

        let offset = Pagination {
            count: 0,
            page: 1,
            limit: 10,
        };
        offset.try_validate()?;
        let got_total_pages = offset.get_total_pages();
        let got_offset = offset.get_offset();

        assert_eq!(want_total_pages, got_total_pages);
        assert_eq!(want_offset, got_offset);

        Ok(())
    }
}
