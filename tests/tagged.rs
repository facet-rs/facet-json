use facet::Facet;

#[derive(Facet, Debug, PartialEq)]
#[facet(type_tag = "type")]
#[repr(u8)]
enum Rule {
    #[facet(rename = "REPEAT")]
    Repeat { content: Box<Rule> },

    #[facet(rename = "REPEAT1")]
    Repeat1 { content: Box<Rule> },

    #[facet(rename = "SYMBOL")]
    Symbol { name: String },
}

#[test]
fn deserialize_tree_sitter_rules() {
    let json1 = r#"{"type": "REPEAT", "content": {"type": "SYMBOL", "name": "x"}}"#;
    let rule1: Rule = facet_json::from_str(json1).unwrap();
    match rule1 {
        Rule::Repeat { content } => match *content {
            Rule::Symbol { name } => assert_eq!(name, "x"),
            _ => panic!("Expected Symbol variant"),
        },
        _ => panic!("Expected Repeat variant"),
    }

    let json2 = r#"{"type": "REPEAT1", "content": {"type": "SYMBOL", "name": "y"}}"#;
    let rule2: Rule = facet_json::from_str(json2).unwrap();
    match rule2 {
        Rule::Repeat1 { content } => match *content {
            Rule::Symbol { name } => assert_eq!(name, "y"),
            _ => panic!("Expected Symbol variant"),
        },
        _ => panic!("Expected Repeat1 variant"),
    }

    let json3 = r#"{"type": "SYMBOL", "name": "z"}"#;
    let rule3: Rule = facet_json::from_str(json3).unwrap();
    match rule3 {
        Rule::Symbol { name } => assert_eq!(name, "z"),
        _ => panic!("Expected Symbol variant"),
    }
}

#[test]
fn serialize_tree_sitter_rules() {
    let rule1 = Rule::Repeat {
        content: Box::new(Rule::Symbol { name: "x".into() }),
    };
    let json1 = facet_json::to_string(&rule1);
    let parsed1: Rule = facet_json::from_str(&json1).unwrap();
    assert_eq!(parsed1, rule1);

    let rule2 = Rule::Symbol { name: "z".into() };
    let json2 = facet_json::to_string(&rule2);
    let parsed2: Rule = facet_json::from_str(&json2).unwrap();
    assert_eq!(parsed2, rule2);
}
