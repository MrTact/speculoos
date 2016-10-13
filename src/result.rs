use super::{AssertionFailure, Spec};

use std::fmt::Debug;

pub trait ResultSpec<T, E>
    where T: Debug,
          E: Debug
{
    fn is_ok(&mut self) -> &mut Self;
    fn is_error(&mut self) -> &mut Self;
}

pub trait ContainingResultSpec<T, E>
    where T: Debug + PartialEq,
          E: Debug + PartialEq
{
    fn is_ok_containing(&mut self, expected_value: &T) -> &mut Self;
    fn is_err_containing(&mut self, expected_value: &E) -> &mut Self;
}

impl<'s, T, E> ContainingResultSpec<T, E> for Spec<'s, Result<T, E>>
    where T: Debug + PartialEq,
          E: Debug + PartialEq
{
    /// Asserts that the subject is an `Ok` Result containing the expected value.
    /// The subject type must be a `Result`.
    ///
    /// ```rust,ignore
    /// assert_that(&Result::Ok::<usize, usize>(1)).is_ok_containing(&1);
    /// ```
    fn is_ok_containing(&mut self, expected_value: &T) -> &mut Self {
        match self.subject {
            &Ok(ref val) => {
                if !val.eq(expected_value) {
                    AssertionFailure::from_spec(self)
                        .with_expected(build_detail_message("ok", expected_value))
                        .with_actual(build_detail_message("ok", val))
                        .fail();
                }
            }
            &Err(ref val) => {
                AssertionFailure::from_spec(self)
                    .with_expected(build_detail_message("ok", expected_value))
                    .with_actual(build_detail_message("err", val))
                    .fail();
            }
        }

        self
    }

    /// Asserts that the subject is an `Err` Result containing the expected value.
    /// The subject type must be a `Result`.
    ///
    /// ```rust,ignore
    /// assert_that(&Result::Err::<usize, usize>(1)).is_err_containing(&1);
    /// ```
    fn is_err_containing(&mut self, expected_value: &E) -> &mut Self {
        match self.subject {
            &Err(ref val) => {
                if !val.eq(expected_value) {
                    AssertionFailure::from_spec(self)
                        .with_expected(build_detail_message("err", expected_value))
                        .with_actual(build_detail_message("err", val))
                        .fail();
                }
            }
            &Ok(ref val) => {
                AssertionFailure::from_spec(self)
                    .with_expected(build_detail_message("err", expected_value))
                    .with_actual(build_detail_message("ok", val))
                    .fail();
            }
        }

        self
    }
}

fn build_detail_message<T: Debug>(variant: &'static str, value: T) -> String {
    format!("Result[{}] containing <{:?}>", variant, value)
}

impl<'s, T, E> ResultSpec<T, E> for Spec<'s, Result<T, E>>
    where T: Debug,
          E: Debug
{
    /// Asserts that the subject is `Ok`. The value type must be a `Result`.
    ///
    /// ```rust,ignore
    /// assert_that(&Result::Ok::<usize, usize>(1)).is_ok();
    /// ```
    fn is_ok(&mut self) -> &mut Self {
        match self.subject {
            &Ok(_) => (),
            &Err(ref err) => {
                AssertionFailure::from_spec(self)
                    .with_expected(format!("result[ok]"))
                    .with_actual(format!("result[error]<{:?}>", err))
                    .fail();
            }
        };

        self
    }

    /// Asserts that the subject is `Err`. The value type must be a `Result`.
    ///
    /// ```rust,ignore
    /// assert_that(&Result::Err::<usize, usize>(1)).is_error();
    /// ```
    fn is_error(&mut self) -> &mut Self {
        match self.subject {
            &Err(_) => (),
            &Ok(ref val) => {
                AssertionFailure::from_spec(self)
                    .with_expected(format!("result[error]"))
                    .with_actual(format!("result[ok]<{:?}>", val))
                    .fail();
            }
        };

        self
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    #[test]
    fn should_not_panic_if_result_is_expected_to_be_ok_and_is() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_ok();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: result[ok]\n\t but was: result[error]<\"Oh no\">")]
    fn should_panic_if_result_is_expected_to_be_ok_and_is_not() {
        let result: Result<&str, &str> = Err("Oh no");
        assert_that(&result).is_ok();
    }

    #[test]
    fn should_not_panic_if_result_is_expected_to_be_error_and_is() {
        let result: Result<&str, &str> = Err("Oh no");
        assert_that(&result).is_error();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: result[error]\n\t but was: result[ok]<\"Hello\">")]
    fn should_panic_if_result_is_expected_to_be_error_and_is_not() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_error();
    }

    #[test]
    fn should_not_panic_if_result_is_ok_with_expected_value() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_ok_containing(&"Hello");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: Result[ok] containing <\"Hi\">\
                   \n\t but was: Result[ok] containing <\"Hello\">")]
    fn should_panic_if_result_is_ok_without_expected_value() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_ok_containing(&"Hi");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: Result[ok] containing <\"Hi\">\
                   \n\t but was: Result[err] containing <\"Hi\">")]
    fn should_panic_if_result_is_err_if_ok_with_value_expected() {
        let result: Result<&str, &str> = Err("Hi");
        assert_that(&result).is_ok_containing(&"Hi");
    }

    #[test]
    fn should_not_panic_if_result_is_err_with_expected_value() {
        let result: Result<&str, &str> = Err("Oh no");
        assert_that(&result).is_err_containing(&"Oh no");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: Result[err] containing <\"Oh no\">\
                   \n\t but was: Result[err] containing <\"Whoops\">")]
    fn should_panic_if_result_is_err_without_expected_value() {
        let result: Result<&str, &str> = Err("Whoops");
        assert_that(&result).is_err_containing(&"Oh no");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: Result[err] containing <\"Oh no\">\
                   \n\t but was: Result[ok] containing <\"Oh no\">")]
    fn should_panic_if_result_is_ok_if_err_with_value_expected() {
        let result: Result<&str, &str> = Ok("Oh no");
        assert_that(&result).is_err_containing(&"Oh no");
    }

}
