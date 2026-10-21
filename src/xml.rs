use roxmltree::Node;
use serde_json::{Map, Value};

pub fn parse_event_xml(xml: &str) -> Result<Value, String> {
    let document = roxmltree::Document::parse(xml).map_err(|error| error.to_string())?;
    let root = document.root_element();
    let mut event = Map::new();
    event.insert(root.tag_name().name().to_owned(), node_to_value(root));
    Ok(Value::Object(event))
}

fn node_to_value(node: Node<'_, '_>) -> Value {
    let mut object = Map::new();
    for attribute in node.attributes() {
        object.insert(
            format!("@{}", attribute.name()),
            Value::String(attribute.value().to_owned()),
        );
    }

    let children: Vec<_> = node.children().filter(Node::is_element).collect();
    let text = node.text().filter(|text| !text.trim().is_empty());

    if children.is_empty() && object.is_empty() {
        return text
            .map(|value| Value::String(value.to_owned()))
            .unwrap_or(Value::Null);
    }

    for child in children {
        let name = child.tag_name().name().to_owned();
        let value = node_to_value(child);
        match object.get_mut(&name) {
            Some(Value::Array(values)) => values.push(value),
            Some(previous) => {
                let first = std::mem::replace(previous, Value::Null);
                *previous = Value::Array(vec![first, value]);
            }
            None => {
                object.insert(name, value);
            }
        }
    }

    if let Some(text) = text {
        object.insert("#text".to_owned(), Value::String(text.to_owned()));
    }
    Value::Object(object)
}

#[cfg(test)]
mod tests {
    use super::parse_event_xml;
    use serde_json::json;

    #[test]
    fn parses_attributes_text_and_repeated_elements() {
        let xml = "<Event><System><Provider Name=\"Provider\"/><EventID>7</EventID></System><Data>x</Data><Data>y</Data></Event>";
        let parsed = parse_event_xml(xml).expect("valid XML should parse");
        assert_eq!(
            parsed,
            json!({
                "Event": {
                    "System": {
                        "Provider": {"@Name": "Provider"},
                        "EventID": "7"
                    },
                    "Data": ["x", "y"]
                }
            })
        );
    }

    #[test]
    fn rejects_malformed_xml() {
        assert!(parse_event_xml("<Event>").is_err());
    }

    #[test]
    fn preserves_significant_text_whitespace() {
        let parsed =
            parse_event_xml("<Event><Data> value </Data></Event>").expect("valid XML should parse");
        assert_eq!(parsed["Event"]["Data"], " value ");
    }
}
