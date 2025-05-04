use regex::Regex;

pub fn extend_fields(input: &str, extra_fields: &str, required_attrs: Option<Vec<&str>>) -> String {
    // either extend the ` fields ` attribute in-place or just add it, if it was not set
    if input.is_empty() {
        assert!(required_attrs.is_none(), "expected {required_attrs:?} attributes");
        format!(r"fields({extra_fields})")
    } else if input.contains("fields(") {
        let (modified_input, extracted_fields) = extract_required_attributes(input, required_attrs);
        let (attr_left, fields, attr_right) = extract_fields(&modified_input);

        let separator = if extra_fields.is_empty() || extracted_fields.is_empty() {
            String::new()
        } else {
            ", ".to_string()
        };
        format!("{attr_left}fields({fields}, {extra_fields}{separator}{extracted_fields}){attr_right}")
    } else {
        let (mut modified_input, extracted_fields) = extract_required_attributes(input, required_attrs);
        if !modified_input.is_empty() {
            // in case we have an empty `modified_input` we don't need the trailing `,`
            modified_input += ", ";
        }
        let separator = if extra_fields.is_empty() || extracted_fields.is_empty() {
            String::new()
        } else {
            ", ".to_string()
        };

        format!("r{modified_input}fields({extra_fields}{separator}{extracted_fields})")
    }
}

fn extract_required_attributes(input: &str, required_attrs: Option<Vec<&str>>) -> (String, String) {
    if required_attrs.is_none() {
        return (input.to_string(), String::new());
    }

    let mut output = input.to_string();
    let mut fields = vec![];

    for attr in required_attrs.unwrap() {
        assert_eq!(attr, "service_name", "required attribute '{attr}' is not supported");

        let re = Regex::new(format!(r"{attr}\s=\s([^,\s]+),?\s?").as_str()).unwrap();
        if let Some(caps) = re.captures(&output) {
            fields.push(format!("service.name = {}", caps.get(1).unwrap().as_str()));
            output = output.replace(&caps[0], "");
        } else {
            panic!("expected '{attr}' attribute");
        }
    }

    (output, fields.join(", "))
}

fn extract_fields(input: &str) -> (&str, &str, &str) {
    let (attr_left, fields) = input.split_once("fields(").unwrap();

    let mut index = 0;
    let mut count = 1;

    for (i, c) in fields.chars().enumerate() {
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
            if count == 0 {
                index = i;
                break;
            }
        }
    }

    let (fields, attr_right) = fields.split_at(index + 1);

    (attr_left, &fields[0..index], attr_right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::empty("", r#"fields(span.type = "web")"#)]
    #[case::with_fields(
        r#"fields(service.name = "baz")"#,
        r#"fields(service.name = "baz", span.type = "web")"#
    )]
    #[case::no_fields("skip(self)", r#"skip(self), fields(span.type = "web")"#)]
    fn test_extend_fields(#[case] input: &str, #[case] expected: &str) {
        // given
        let extra_fields = r#"span.type = "web""#;

        // when
        let extended_attr = extend_fields(input, extra_fields, None);

        // then
        assert_eq!(extended_attr, expected);
    }

    #[rstest]
    #[case::with_fields(
        r#"service_name = ServiceName::Database.as_ref(), fields(db.name = "baz")"#,
        r#"fields(db.name = "baz", span.type = "sql", service.name = ServiceName::Database.as_ref())"#
    )]
    #[case::no_fields(
        r#"service_name = "database", skip(self)"#,
        r#"skip(self), fields(span.type = "sql", service.name = "database")"#
    )]
    fn test_extend_fields_with_required_attrs(#[case] input: &str, #[case] expected: &str) {
        // given
        let extra_fields = r#"span.type = "sql""#;
        let required_attrs = Some(vec!["service_name"]);

        // when
        let extended_attr = extend_fields(input, extra_fields, required_attrs);

        // then
        assert_eq!(extended_attr, expected);
    }

    #[test]
    fn test_extract_fields() {
        // given
        let input = r#"skip(some), fields(service.name = ServiceName::Database.as_ref(), db.operation = SqlOperation::Select.as_ref(), db.system = "postgresql"), ret(level = Level::WARN)"#.to_string();

        // when
        let (attr_left, fields, attr_right) = extract_fields(&input);

        // then
        assert_eq!(attr_left, r#"skip(some), "#);

        assert_eq!(
            fields,
            r#"service.name = ServiceName::Database.as_ref(), db.operation = SqlOperation::Select.as_ref(), db.system = "postgresql""#
        );

        assert_eq!(attr_right, r#", ret(level = Level::WARN)"#);
    }
}
