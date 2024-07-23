// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

// First we need to implement a custom error for the
// failed status conversions
#[derive(Debug, thiserror::Error)]
#[error("{invalid_status} is not a valid status")]
pub struct ParseStatusError {
    invalid_status: String,
}

// comversion from &str
// this is the effective conversion implementation
impl TryFrom<&str> for Status {
    type Error = ParseStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // converts the imputs to lowercase and returns the corresponding enum variants
        match value.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(ParseStatusError {
                invalid_status: value.to_string(),
            }),
        }
    }
}

// comversion from String (use the previous implementation)
impl TryFrom<String> for Status {
    type Error = ParseStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
