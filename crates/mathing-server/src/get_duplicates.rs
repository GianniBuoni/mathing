use std::{collections::HashSet, sync::Arc};

/// Function to get duplicates from request arguments and return them
/// as a single string to pass to a UniqueConstraint error.
pub fn get_duplicates(items: Arc<[String]>) -> Option<String> {
    let mut unique = HashSet::<&str>::new();
    let mut repeat = HashSet::<&str>::new();

    items.iter().for_each(|f| {
        if !unique.insert(f) {
            repeat.insert(f);
        }
    });
    if repeat.is_empty() {
        return None;
    }

    let repeat = repeat.into_iter().collect::<Vec<&str>>().join(", ");
    Some(repeat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Basic test expecting to get a string value
    fn test_get_duplicates() {
        let want = "jon".to_string();
        let names = vec![want.clone(); 3].into();
        let got = get_duplicates(names).unwrap();

        assert_eq!(want, got);
    }
    #[test]
    /// Basic test expecting a None values
    fn test_no_duplicate() {
        let names = vec!["jon".into(), "jonn".into(), "jonnn".into()].into();
        let got = get_duplicates(names);

        assert!(got.is_none())
    }
    #[test]
    /// Test with mulpitle duplicates, expecting a properly joined string
    fn test_duplicate_formatting() {
        let items = [
            "jon".into(),
            "jon".into(),
            "paul".into(),
            "paul".into(),
            "ringo".into(),
        ]
        .into();
        let got = get_duplicates(items).unwrap();

        assert!(got == "jon, paul" || got == "paul, jon");
    }
}
