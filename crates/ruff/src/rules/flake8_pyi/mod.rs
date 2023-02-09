//! Rules from [flake8-pyi](https://pypi.org/project/flake8-pyi/).
pub(crate) mod rules;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::registry::Rule;
    use crate::test::test_path;
    use crate::{assert_yaml_snapshot, settings};

    // TODO: figure out how to have another test case for Y001 that is a .py file.
    #[test_case(Rule::PrefixPrivateTypes, Path::new("Y001.pyi"))]
    #[test_case(Rule::PrefixPrivateTypes, Path::new("Y001.py"))]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.code(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("flake8_pyi").join(path).as_path(),
            &settings::Settings::for_rule(rule_code),
        )?;
        assert_yaml_snapshot!(snapshot, diagnostics);
        Ok(())
    }
}
