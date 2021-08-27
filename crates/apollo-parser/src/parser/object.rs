use crate::parser::{directive, field, name};
use crate::{create_err, Parser, SyntaxKind, TokenKind};

/// See: https://spec.graphql.org/June2018/#ObjectTypeDefinition
///
/// ```txt
/// ObjectTypeDefinition
///     Description[opt] type Name ImplementsInterfaces[opt] Directives[Const][opt] FieldsDefinition[opt]
/// ```
pub(crate) fn object_type_definition(parser: &mut Parser) {
    let _guard = parser.start_node(SyntaxKind::OBJECT_TYPE_DEFINITION);
    parser.bump(SyntaxKind::type_KW);

    match parser.peek() {
        Some(TokenKind::Node) => name::name(parser),
        _ => {
            parser.push_err(create_err!(
                parser
                    .peek_data()
                    .unwrap_or_else(|| String::from("no further data")),
                "Expected Object Type Definition to have a Name, got {}",
                parser
                    .peek_data()
                    .unwrap_or_else(|| String::from("no further data")),
            ));
        }
    }
    if let Some(TokenKind::Node) = parser.peek() {
        if parser.peek_data().unwrap() == "implements" {
            implements_interfaces(parser, false);
        } else {
            parser.push_err(create_err!(
                parser
                    .peek_data()
                    .unwrap_or_else(|| String::from("no further data")),
                "Unexpected Name in Object Type Definition, {}",
                parser
                    .peek_data()
                    .unwrap_or_else(|| String::from("no further data")),
            ));
        }
    }

    if let Some(TokenKind::At) = parser.peek() {
        directive::directives(parser);
    }

    if let Some(TokenKind::LCurly) = parser.peek() {
        field::fields_definition(parser);
    }
}

/// See: https://spec.graphql.org/June2018/#ImplementsInterfaces
///
/// ```txt
/// ImplementsInterfaces
///     implements &[opt] NamedType
///     ImplementsInterfaces & NamedType
/// ```
pub(crate) fn implements_interfaces(parser: &mut Parser, is_interfaces: bool) {
    let _guard = parser.start_node(SyntaxKind::IMPLEMENTS_INTERFACES);
    parser.bump(SyntaxKind::implements_KW);

    match parser.peek() {
        Some(TokenKind::Node) => {
            let node = parser.peek_data().unwrap();
            match node.as_str() {
                "&" => {
                    parser.bump(SyntaxKind::AMP);
                    implements_interfaces(parser, is_interfaces)
                }
                _ => name::name(parser),
            }
        }
        _ => {
            if !is_interfaces {
                parser.push_err(create_err!(
                    parser
                        .peek_data()
                        .unwrap_or_else(|| String::from("no further data")),
                    "Expected to have Implements Interfaces in a Object Type Definition, got {}",
                    parser
                        .peek_data()
                        .unwrap_or_else(|| String::from("no further data"))
                ));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::utils;

    #[test]
    fn it_parses_object_type_definition() {
        utils::check_ast(
            "
            type Person implements Human {
              name: String
              age: Int
              picture: Url
            }",
            r#"
            - DOCUMENT@0..44
                - OBJECT_TYPE_DEFINITION@0..44
                    - type_KW@0..4 "type"
                    - NAME@4..10
                        - IDENT@4..10 "Person"
                    - IMPLEMENTS_INTERFACES@10..25
                        - implements_KW@10..20 "implements"
                        - NAME@20..25
                            - IDENT@20..25 "Human"
                    - FIELDS_DEFINITION@25..44
                        - L_CURLY@25..26 "{"
                        - FIELD_DEFINITION@26..31
                            - NAME@26..30
                                - IDENT@26..30 "name"
                            - COLON@30..31 ":"
                            - TYPE@31..31
                                - NAMED_TYPE@31..31
                        - FIELD_DEFINITION@31..35
                            - NAME@31..34
                                - IDENT@31..34 "age"
                            - COLON@34..35 ":"
                            - TYPE@35..35
                                - NAMED_TYPE@35..35
                        - FIELD_DEFINITION@35..43
                            - NAME@35..42
                                - IDENT@35..42 "picture"
                            - COLON@42..43 ":"
                            - TYPE@43..43
                                - NAMED_TYPE@43..43
                        - R_CURLY@43..44 "}"
            "#,
        )
    }
}