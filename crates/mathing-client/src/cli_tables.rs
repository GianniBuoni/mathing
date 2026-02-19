use tabled::{
    Table,
    builder::Builder,
    settings::{Color, Style, Theme, object::FirstRow, style::HorizontalLine},
};

use crate::prelude::mathing_proto::{RowsAffected, UserRow};

struct TableStyle(Theme);

impl Default for TableStyle {
    fn default() -> Self {
        let mut theme = Theme::from_style(
            Style::blank().horizontals([(1, HorizontalLine::empty().horizontal('═'))]),
        );
        theme.set_borders_top(' ');
        Self(theme)
    }
}

impl FromIterator<UserRow> for Table {
    fn from_iter<T: IntoIterator<Item = UserRow>>(iter: T) -> Self {
        let mut table = Builder::new();
        table.push_record(["NO.", "USERS"]);
        iter.into_iter()
            .enumerate()
            .for_each(|(i, u)| table.push_record([format!("#{i}"), u.name]));

        let mut table = table.build();
        table
            .with(TableStyle::default().0)
            .modify(FirstRow, Color::FG_BRIGHT_GREEN);

        table
    }
}

impl From<RowsAffected> for Table {
    fn from(value: RowsAffected) -> Self {
        let mut table = Builder::with_capacity(2, 1);
        table.push_record(["ROWS AFFECTED"]);
        table.push_record([value.rows_affected.to_string()]);

        let mut table = table.build();
        table
            .with(TableStyle::default().0)
            .modify(FirstRow, Color::FG_BRIGHT_MAGENTA);

        table
    }
}
