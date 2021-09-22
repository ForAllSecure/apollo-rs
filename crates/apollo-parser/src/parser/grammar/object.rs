use crate::parser::grammar::{directive, field, name};
use crate::{Parser, SyntaxKind, TokenKind, T};

/// See: https://spec.graphql.org/June2018/#ObjectTypeDefinition
///
/// ```txt
/// ObjectTypeDefinition
///     Description[opt] type Name ImplementsInterfaces[opt] Directives[Const][opt] FieldsDefinition[opt]
/// ```
pub(crate) fn object_type_definition(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::OBJECT_TYPE_DEFINITION);
    p.bump(SyntaxKind::type_KW);

    match p.peek() {
        Some(TokenKind::Name) => name::name(p),
        _ => p.err("expected a name"),
    }
    if let Some(TokenKind::Name) = p.peek() {
        if p.peek_data().unwrap() == "implements" {
            implements_interfaces(p, false);
        } else {
            p.err("unexpected Name");
        }
    }

    if let Some(T![@]) = p.peek() {
        directive::directives(p);
    }

    if let Some(T!['{']) = p.peek() {
        field::fields_definition(p);
    }
}

/// See: https://spec.graphql.org/June2018/#ObjectTypeExtension
///
/// ```txt
/// ObjectTypeExtension
///     extend type Name ImplementsInterfaces[opt] Directives[Const][opt] FieldsDefinition
///     extend type Name ImplementsInterfaces[opt] Directives[Const]
///     extend type Name ImplementsInterfaces
/// ```
pub(crate) fn object_type_extension(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::OBJECT_TYPE_EXTENSION);
    p.bump(SyntaxKind::extend_KW);
    p.bump(SyntaxKind::type_KW);

    // Use this variable to see if any of ImplementsInterfacs, Directives or
    // FieldsDefinitions is provided. If none are present, we push an error.
    let mut meets_requirements = false;

    match p.peek() {
        Some(TokenKind::Name) => name::name(p),
        _ => p.err("expected a Name"),
    }
    if let Some(TokenKind::Name) = p.peek() {
        if p.peek_data().unwrap() == "implements" {
            meets_requirements = true;
            implements_interfaces(p, false);
        } else {
            p.err("unexpected Name");
        }
    }
    if let Some(T![@]) = p.peek() {
        meets_requirements = true;
        directive::directives(p)
    }
    if let Some(T!['{']) = p.peek() {
        meets_requirements = true;
        field::fields_definition(p)
    }

    if !meets_requirements {
        p.err("expected an Implements Interface, Directives or a Fields Definition");
    }
}

/// See: https://spec.graphql.org/June2018/#ImplementsInterfaces
///
/// ```txt
/// ImplementsInterfaces
///     implements &[opt] NamedType
///     ImplementsInterfaces & NamedType
/// ```
pub(crate) fn implements_interfaces(p: &mut Parser, is_interfaces: bool) {
    let _g = p.start_node(SyntaxKind::IMPLEMENTS_INTERFACES);
    p.bump(SyntaxKind::implements_KW);

    match p.peek() {
        Some(TokenKind::Name) => {
            let node = p.peek_data().unwrap();
            match node.as_str() {
                "&" => {
                    p.bump(SyntaxKind::AMP);
                    implements_interfaces(p, is_interfaces)
                }
                _ => name::name(p),
            }
        }
        _ => {
            if !is_interfaces {
                p.err("expected an Object Type Definition");
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
            - DOCUMENT@0..134
                - WHITESPACE@0..13 "\n            "
                - OBJECT_TYPE_DEFINITION@13..134
                    - type_KW@13..17 "type"
                    - WHITESPACE@17..18 " "
                    - NAME@18..25
                        - IDENT@18..24 "Person"
                        - WHITESPACE@24..25 " "
                    - IMPLEMENTS_INTERFACES@25..42
                        - implements_KW@25..35 "implements"
                        - WHITESPACE@35..36 " "
                        - NAME@36..42
                            - IDENT@36..41 "Human"
                            - WHITESPACE@41..42 " "
                    - FIELDS_DEFINITION@42..134
                        - L_CURLY@42..43 "{"
                        - WHITESPACE@43..58 "\n              "
                        - FIELD_DEFINITION@58..85
                            - NAME@58..62
                                - IDENT@58..62 "name"
                            - COLON@62..63 ":"
                            - WHITESPACE@63..64 " "
                            - TYPE@64..85
                                - WHITESPACE@64..79 "\n              "
                                - NAMED_TYPE@79..85
                                    - NAME@79..85
                                        - IDENT@79..85 "String"
                        - FIELD_DEFINITION@85..108
                            - NAME@85..88
                                - IDENT@85..88 "age"
                            - COLON@88..89 ":"
                            - WHITESPACE@89..90 " "
                            - TYPE@90..108
                                - WHITESPACE@90..105 "\n              "
                                - NAMED_TYPE@105..108
                                    - NAME@105..108
                                        - IDENT@105..108 "Int"
                        - FIELD_DEFINITION@108..133
                            - NAME@108..115
                                - IDENT@108..115 "picture"
                            - COLON@115..116 ":"
                            - WHITESPACE@116..117 " "
                            - TYPE@117..133
                                - WHITESPACE@117..130 "\n            "
                                - NAMED_TYPE@130..133
                                    - NAME@130..133
                                        - IDENT@130..133 "Url"
                        - R_CURLY@133..134 "}"
            "#,
        )
    }

    #[test]
    fn it_parses_extension() {
        utils::check_ast(
            "
            extend type Person implements Human @deprecated {
              name: String
              age: Int
              picture: Url
            }",
            r#"
            - DOCUMENT@0..153
                - WHITESPACE@0..13 "\n            "
                - OBJECT_TYPE_EXTENSION@13..153
                    - extend_KW@13..19 "extend"
                    - WHITESPACE@19..20 " "
                    - type_KW@20..24 "type"
                    - WHITESPACE@24..25 " "
                    - NAME@25..32
                        - IDENT@25..31 "Person"
                        - WHITESPACE@31..32 " "
                    - IMPLEMENTS_INTERFACES@32..49
                        - implements_KW@32..42 "implements"
                        - WHITESPACE@42..43 " "
                        - NAME@43..49
                            - IDENT@43..48 "Human"
                            - WHITESPACE@48..49 " "
                    - DIRECTIVES@49..61
                        - DIRECTIVE@49..61
                            - AT@49..50 "@"
                            - NAME@50..61
                                - IDENT@50..60 "deprecated"
                                - WHITESPACE@60..61 " "
                    - FIELDS_DEFINITION@61..153
                        - L_CURLY@61..62 "{"
                        - WHITESPACE@62..77 "\n              "
                        - FIELD_DEFINITION@77..104
                            - NAME@77..81
                                - IDENT@77..81 "name"
                            - COLON@81..82 ":"
                            - WHITESPACE@82..83 " "
                            - TYPE@83..104
                                - WHITESPACE@83..98 "\n              "
                                - NAMED_TYPE@98..104
                                    - NAME@98..104
                                        - IDENT@98..104 "String"
                        - FIELD_DEFINITION@104..127
                            - NAME@104..107
                                - IDENT@104..107 "age"
                            - COLON@107..108 ":"
                            - WHITESPACE@108..109 " "
                            - TYPE@109..127
                                - WHITESPACE@109..124 "\n              "
                                - NAMED_TYPE@124..127
                                    - NAME@124..127
                                        - IDENT@124..127 "Int"
                        - FIELD_DEFINITION@127..152
                            - NAME@127..134
                                - IDENT@127..134 "picture"
                            - COLON@134..135 ":"
                            - WHITESPACE@135..136 " "
                            - TYPE@136..152
                                - WHITESPACE@136..149 "\n            "
                                - NAMED_TYPE@149..152
                                    - NAME@149..152
                                        - IDENT@149..152 "Url"
                        - R_CURLY@152..153 "}"
            "#,
        )
    }

    #[test]
    fn it_errors_when_extesion_name_is_missing() {
        utils::check_ast(
            "
            extend type {
              name: String
              age: Int
              picture: Url
            }",
            r#"
            - DOCUMENT@0..117
                - WHITESPACE@0..13 "\n            "
                - OBJECT_TYPE_EXTENSION@13..117
                    - extend_KW@13..19 "extend"
                    - WHITESPACE@19..20 " "
                    - type_KW@20..24 "type"
                    - WHITESPACE@24..25 " "
                    - FIELDS_DEFINITION@25..117
                        - L_CURLY@25..26 "{"
                        - WHITESPACE@26..41 "\n              "
                        - FIELD_DEFINITION@41..68
                            - NAME@41..45
                                - IDENT@41..45 "name"
                            - COLON@45..46 ":"
                            - WHITESPACE@46..47 " "
                            - TYPE@47..68
                                - WHITESPACE@47..62 "\n              "
                                - NAMED_TYPE@62..68
                                    - NAME@62..68
                                        - IDENT@62..68 "String"
                        - FIELD_DEFINITION@68..91
                            - NAME@68..71
                                - IDENT@68..71 "age"
                            - COLON@71..72 ":"
                            - WHITESPACE@72..73 " "
                            - TYPE@73..91
                                - WHITESPACE@73..88 "\n              "
                                - NAMED_TYPE@88..91
                                    - NAME@88..91
                                        - IDENT@88..91 "Int"
                        - FIELD_DEFINITION@91..116
                            - NAME@91..98
                                - IDENT@91..98 "picture"
                            - COLON@98..99 ":"
                            - WHITESPACE@99..100 " "
                            - TYPE@100..116
                                - WHITESPACE@100..113 "\n            "
                                - NAMED_TYPE@113..116
                                    - NAME@113..116
                                        - IDENT@113..116 "Url"
                        - R_CURLY@116..117 "}"
            - ERROR@0:1 "expected a Name"
            "#,
        )
    }

    #[test]
    fn it_errors_when_extesion_is_missing_required_syntax() {
        utils::check_ast(
            "extend type Person",
            r#"
            - DOCUMENT@0..18
                - OBJECT_TYPE_EXTENSION@0..18
                    - extend_KW@0..6 "extend"
                    - WHITESPACE@6..7 " "
                    - type_KW@7..11 "type"
                    - WHITESPACE@11..12 " "
                    - NAME@12..18
                        - IDENT@12..18 "Person"
            - ERROR@0:3 "expected an Implements Interface, Directives or a Fields Definition"
            "#,
        )
    }
}