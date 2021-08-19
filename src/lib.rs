fn formatting_is_broken_too()
{
    if { true } { /* oh and trigger clippy */}

    use std::os::unix; // and this will only be caught when running on windows
}

#[cfg(feature = "myfeature")]
ThisWillOnlyFailToCompileIfMyfeatureIsIncluded;

#[cfg(test)]
mod tests {
    #[test]
    fn it_doesnt_work() {
        assert_eq!(2 + 2, 3);
    }
}
 