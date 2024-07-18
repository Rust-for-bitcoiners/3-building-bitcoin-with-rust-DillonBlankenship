#![allow(unused)]
pub enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    pub fn ok(value: T) -> Self {
        MResult::Ok(value)
    }
    
    pub fn err(error: E) -> Self {
        MResult::Err(error)
    }

    pub fn is_ok(&self) -> bool {
        match self{
            MResult::Ok(_) => true,
            MResult::Err(_) => false,
        } 

    }

    pub fn is_err(&self) -> bool {
        matches!(self, MResult::Err(_))
    }

    pub fn unwrap(self) -> T {
        match self {
            MResult::Ok(value) => value,
            MResult::Err(_) => panic!("Called unwrap on an Err value"),
        }
    }

    pub fn unwrap_err(self) -> E {
        match self {
            MResult::Err(error) => error,
            MResult::Ok(_) => panic!("Called unwrap_err on an Ok value"),
        }
    }         
}

#[cfg(test)]
mod tests {
    use super::MResult;

    #[test]
    fn test_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert!(result.is_ok());
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert!(result.is_err());
        assert!(!result.is_ok());
        assert_eq!(result.unwrap_err(), "error");
    }

    #[test]
    #[should_panic(expected = "Called unwrap on an Err value")]
    fn test_unwrap_panic_on_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "Called unwrap_err on an Ok value")]
    fn test_unwrap_err_panic_on_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        result.unwrap_err();
    }

    #[test]
    fn test_is_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert!(result.is_ok());
        assert!(!result.is_err());
    }

    #[test]
    fn test_is_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert!(result.is_err());
        assert!(!result.is_ok());
    }
}
