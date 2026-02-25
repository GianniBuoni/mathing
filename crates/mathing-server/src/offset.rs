use crate::prelude::{mathing_proto::PaginationRequest, *};

#[derive(Debug, Clone, Copy)]
pub struct OffsetBuilder {
    count: Option<u32>,
    limit: u32,
    page: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    count: u32,
    pub limit: u32,
    page: u32,
}

impl Default for OffsetBuilder {
    fn default() -> Self {
        Self {
            limit: 30,
            page: 1,
            count: Default::default(),
        }
    }
}

impl From<Option<PaginationRequest>> for OffsetBuilder {
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

impl OffsetBuilder {
    pub fn with_count(&mut self, count: u32) {
        self.count = Some(count);
    }
    pub fn try_build(self) -> Result<Offset, ServerError> {
        let count = self.count.ok_or(ServerError::ConversionError(
            "OffsetBuilder",
            "Offset",
            "count: None".to_string(),
        ))?;

        Ok(Offset {
            limit: self.limit,
            page: self.page,
            count,
        })
    }
}

impl Offset {
    /// Provides the sql offset from the pagination request parameters
    pub fn get_offset(&self) -> u32 {
        self.limit * (self.page - 1)
    }
    pub fn validate(&self) -> Result<(), ClientError> {
        if self.get_offset() > self.count {
            return Err(ClientError::OutOfBounds(*self));
        }
        Ok(())
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
        let offset = OffsetBuilder {
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
    fn test_out_of_range_validate() -> anyhow::Result<()> {
        let offset = OffsetBuilder {
            count: Some(20),
            limit: 40,
            page: 3,
        };
        let offset = offset.try_build()?;
        let want = ClientError::OutOfBounds(offset);
        let got = offset.validate().map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }

    #[test]
    fn test_valid_offset() -> anyhow::Result<()> {
        let offset = OffsetBuilder {
            limit: 40,
            page: 1,
            count: Some(20),
        };
        offset.try_build()?.validate()?;
        Ok(())
    }
}
