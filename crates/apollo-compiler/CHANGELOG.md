# Changelog

All notable changes to `apollo-compiler` will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- # [x.x.x] (unreleased) - 2021-mm-dd

> Important: X breaking changes below, indicated by **BREAKING**

## BREAKING

## Features

## Fixes

## Maintenance

## Documentation -->
# [0.6.0](https://crates.io/crates/apollo-compiler/0.6.0) - 2023-01-18

This release has a few breaking changes as we try to standardise APIs across the
compiler. We appreciate your patience with these changes. If you run into trouble, please [open an issue].

## BREAKING
- Rename `compiler.create_*` methods to `compiler.add_*`, [SimonSapin] in [412]
- Rename `schema` to `type_system` for `compiler.add_` and `compiler.update_`
methods, [SimonSapin] in [413]
- Unify `ty`, `type_def` and `kind` namings in HIR, [lrlna] in [415]
  - in `Type` struct impl: `ty()` --> `type_def()`
  - in `TypeDefinition` struct impl: `ty()` --> `kind()`
  - in `FragmentDefinition` struct impl: `ty()` --> `type_def()`
  - in `RootOperationTypeDefinition` struct: `operation_type` field -->
  `operation_ty`

## Features
- `FileId`s are unique per process, [SimonSapin] in [405]
- Type alias `compiler.snapshot()` return type to `Snapshot`, [SimonSapin] in
[410]
- Introduce a type system high-level intermediate representation (HIR) as input
to the compiler, [SimonSapin] in [407]

## Fixes
- Use `#[salsa::transparent]` for `find_*` queries, i.e. not caching query results, [lrlna] in [403]

## Maintenance
- Add compiler benchmarks, [lrlna] in [404]

## Documentation
- Document `apollo-rs` runs on stable, [SimonSapin] in [402]

[open an issue]: https://github.com/apollographql/apollo-rs/issues/new/choose
[lrlna]: https://github.com/lrlna
[SimonSapin]: https://github.com/SimonSapin
[pull/402]: https://github.com/apollographql/apollo-rs/pull/402
[pull/403]: https://github.com/apollographql/apollo-rs/pull/403
[pull/404]: https://github.com/apollographql/apollo-rs/pull/404
[pull/405]: https://github.com/apollographql/apollo-rs/pull/405
[pull/407]: https://github.com/apollographql/apollo-rs/pull/407
[pull/410]: https://github.com/apollographql/apollo-rs/pull/410
[pull/412]: https://github.com/apollographql/apollo-rs/pull/412
[pull/413]: https://github.com/apollographql/apollo-rs/pull/413
[pull/415]: https://github.com/apollographql/apollo-rs/pull/415


# [0.5.0](https://crates.io/crates/apollo-compiler/0.5.0) - 2023-01-04

## Highlights
### Multi-file support
You can now build a compiler from multiple sources. This  is especially useful
when various parts of a GraphQL document are coming in at different times and
need to be analysed as a single context. Or, alternatively, you are looking to
lint or validate multiple GraphQL files part of the same context in a given directory or workspace.

The are three different kinds of sources:
- `document`: for when a source is composed of executable and type system
definitions, or you're uncertain of definitions types
- `schema`: for sources with type system definitions or extensions
- `executable`: for sources with executable definitions/GraphQL queries

You can add a source with `create_` and update it with `update_`, for example
`create_document` and `update_document`. Here is an example:

```rust
    let schema = r#"
type Query {
  dog: Dog
}

type Dog {
  name: String!
}
    "#;

    let query = r#"
query getDogName {
  dog {
    name
  }
}

# duplicate name, should show up in diagnostics
query getDogName {
  dog {
    owner {
      name
    }
  }
}
    "#;
    let updated_schema = r#"
type Query {
  dog: Dog
}

type Dog {
  name: String!
  owner: Human
}

type Human {
  name: String!
}
    "#;
    let mut compiler = ApolloCompiler::new();
    let schema_id = compiler.create_schema(schema, "schema.graphl");
    let executable_id = compiler.create_executable(query, "query.graphql");
    compiler.update_schema(updated_schema, schema_id);
```

For more elaborate examples, please refer to [`multi_source_validation`] and
[`file_watcher`] examples in the `examples` dir.

We look forward to your feedback on this feature, should you be using it.

Completed in [pull/368] in collaboration with [goto-bus-stop], [SimonSapin] and
[lrlna].

## BREAKING
- Remove UUID helpers and related UUID APIs from database by [SimonSapin] in
[pull/391]
- Merge `DocumentDatabase` trait into `HIRDatabase` by [SimonSapin] in
[pull/394]
- Replace `hir::Definition` enum with `hir::TypeSystemDefinitions` struct by
[SimonSapin] in [pull/395]
- `db.type_system_definitions` returns a `TypeSystemDefinitions` by [SimonSapin]
in [pull/395]
- Remove `db.db_definitions`, `find_definition_by_name` and
`find_type_system_definition_by_name` by [SimonSapin] in [pull/395]
- Remove queries returning type extensions, instead type definitions in the HIR
contain extension information by [SimonSapin] in [pull/387]

## Features
- `db.fragments`, `db.object_types`, `db.scalars`, `db.enums`, `db.unions`,
`db.interfaces`, `db.input_objects`, and `db.directive_definitions` return
name-indexed maps by [SimonSapin] in [pull/387]

[`file_watcher`]: https://github.com/apollographql/apollo-rs/blob/eb9687fc64dfe0bf618f2025f633e52950940b8a/crates/apollo-compiler/examples/file_watcher.rs
[`multi_source_validation`]: https://github.com/apollographql/apollo-rs/blob/8c66c2c36053ff592682504276307a3fead0b3ad/crates/apollo-compiler/examples/multi_source_validation.rs
[goto-bus-stop]: https://github.com/goto-bus-stop
[lrlna]: https://github.com/lrlna
[SimonSapin]: https://github.com/SimonSapin
[pull/368]: https://github.com/apollographql/apollo-rs/pull/368
[pull/391]: https://github.com/apollographql/apollo-rs/pull/391
[pull/394]: https://github.com/apollographql/apollo-rs/pull/394
[pull/395]: https://github.com/apollographql/apollo-rs/pull/395
[pull/387]: https://github.com/apollographql/apollo-rs/pull/387

# [0.4.1](https://crates.io/crates/apollo-compiler/0.4.1) - 2022-12-13
## Features
- **add new APIs - [SimonSapin], [pull/382]**
  - [`db.find_enum_by_name()`] to look up an `EnumTypeDefinition`.
  - [`directive.argument_by_name()`] to look up the value of an argument to a directive call.
  - [`scalar_type.is_built_in()`] to check if a `ScalarTypeDefinition` is defined by the GraphQL spec rather than the schema text.
  - [`enum_value.directives()`] to access the directives used on an enum value.
  - `hir::Float` is now `Copy` so it can be passed around more easily; use [`hir_float.get()`] to access the underlying `f64` or [`hir_float.to_i32_checked()`] to convert to an `i32`.

  [SimonSapin]: https://github.com/SimonSapin
  [pull/382]: https://github.com/apollographql/apollo-rs/pull/382
  [`db.find_enum_by_name()`]: https://docs.rs/apollo-compiler/0.4.1/apollo_compiler/trait.DocumentDatabase.html#tymethod.find_union_by_name
  [`directive.argument_by_name()`]: https://docs.rs/apollo-compiler/0.4.1/apollo_compiler/database/hir/struct.Directive.html#method.argument_by_name
  [`scalar_type.is_built_in()`]: https://docs.rs/apollo-compiler/0.4.1/apollo_compiler/database/hir/struct.ScalarTypeDefinition.html#method.is_built_in
  [`enum_value.directives()`]: https://docs.rs/apollo-compiler/0.4.1/apollo_compiler/database/hir/struct.EnumValueDefinition.html#method.directives
  [`hir_float.get()`]: https://docs.rs/apollo-compiler/0.4.1/apollo_compiler/database/hir/struct.Float.html#method.get
  [`hir_float.to_i32_checked()`]: https://docs.rs/apollo-compiler/0.4.1/apollo_compiler/database/hir/struct.Float.html#method.to_i32_checked

## Fixes
- **do not panic when creating HIR from a parse tree with syntax errors - [goto-bus-stop], [pull/381]**

  When using the compiler, nodes with syntax errors in them are ignored. As syntax errors are returned
  from the parser, you can still tell that something is wrong. The compiler just won't crash the whole
  program anymore.

  [goto-bus-stop]: https://github.com/goto-bus-stop
  [pull/381]: https://github.com/apollographql/apollo-rs/pull/381

# [0.4.0](https://crates.io/crates/apollo-compiler/0.4.0) - 2022-11-29
## Features
- **add parser recursion limit API - [SimonSapin], [pull/353], [issue/296]**

  Calling `ApolloCompiler::with_recursion_limit` instead of `ApolloCompiler::new`
  makes the compiler configure [the corresponding parser limit][with].
  This limit protects against stack overflow and is enabled either way.
  Configuring it may be useful for example if you’re also configuring the stack size.

  [SimonSapin]: https://github.com/SimonSapin
  [pull/353]: https://github.com/apollographql/apollo-rs/pull/353
  [issue/296]: https://github.com/apollographql/apollo-rs/issues/296
  [with]: https://docs.rs/apollo-parser/0.3.1/apollo_parser/struct.Parser.html#method.recursion_limit

- **expose the repeatable attribute on `DirectiveDefinition` - [allancalix], [pull/367]**

  There was previously no way to access the `repeatable` field on the `DirectiveDefinition` type.
  This field is required for validation rules.

  [allancalix]: https://github.com/allancalix
  [pull/367]: https://github.com/apollographql/apollo-rs/pull/367

- **add type extensions - [SimonSapin], [pull/369]**

  apollo-compiler now partially supports GraphQL `extend` types. The `is_subtype` query takes
  extensions into account.

  Some other parts of the compiler, like validation, do not yet support extensions.

  [SimonSapin]: https://github.com/SimonSapin
  [pull/369]: https://github.com/apollographql/apollo-rs/pull/369

## Fixes
- **fix `@include` allowed directive locations - [allancalix], [pull/366]**

  The locations for the `@include` directive wrongly specified `FragmentDefinition` instead of `FragmentSpread`.
  It now matches the spec.

  [allancalix]: https://github.com/allancalix
  [pull/366]: https://github.com/apollographql/apollo-rs/pull/366

## Maintenance
- **avoid double lookup in `SchemaDefinition::{query,mutation,subscription}` - [SimonSapin], [pull/364]**

  [SimonSapin]: https://github.com/SimonSapin
  [pull/364]: https://github.com/apollographql/apollo-rs/pull/364

# [0.3.0](https://crates.io/crates/apollo-compiler/0.3.0) - 2022-11-02
## Breaking
- **compiler.parse is renamed to compiler.ast - [lrlna], [pull/290]**

  `compiler.ast()` returns the `SyntaxTree` produced by the parser and is much
  clearer method than `compiler.parse()`.

  [lrlna]: https://github.com/lrlna
  [pull/290]: https://github.com/apollographql/apollo-rs/pull/290

- **selection.ty(db) now expects a `db` parameter - [lrlna], [pull/290]**

  As byproduct of separating compiler's query_groups into individual components.
  Selection's type can now be accessed like so:

    ```rust
    let ctx = ApolloCompiler::new(input);
    let top_product_fields: Vec<String> = top_products
      .iter()
      .filter_map(|field| Some(field.ty(&ctx.db)?.name()))
      .collect();
    ```

  [lrlna]: https://github.com/lrlna
  [pull/290]: https://github.com/apollographql/apollo-rs/pull/290

- **removes db.definitions() API - [lrlna], [pull/295]**

  `db.definitions()` returned a `!Send` value that is no longer possible with
  the `ParallelDatabase` implementation.

  To access HIR definitions, use `db.db_definitions()` and `db.type_system_definitions`.

  [lrlna]: https://github.com/lrlna
  [pull/295]: https://github.com/apollographql/apollo-rs/pull/295

## Features
- **add subtype_map and is_subtype queries - [lrlna]/[SimonSapin], [pull/333]**

  This allows users to check whether a particular type is a subtype of another
  type. For example, in a UnionDefinition such as `union SearchResult = Photo |
  Person`, `Person` is a suptype of `SearchResult`. In an InterfaceDefinition such
  as `type Business implements NamedEntity & ValuedEntity { # fields }`,
  `Business` is a subtype of `NamedEntity`.

  [lrlna]: https://github.com/lrlna
  [SimonSapin]: https://github.com/SimonSapin
  [pull/333]: https://github.com/apollographql/apollo-rs/pull/333

- **pub compiler storage - allow database composition - [lrlna], [pull/328]**

  This allows for internal query_groups to be exported, and allows users to
  compose various databases from compiler's existing dbs and their queries.

  This is how you'd create a database with storage from apollo-compiler:
    ```rust
    use apollo_compiler::{database::{AstStorage, DocumentStorage}};

    #[salsa::database(AstStorage, DoumentStorage)]
    pub struct AnotherDatabase {
        pub storage: salsa::Storage<AnotherDatabase>,
    }
    ```

  You can also see a more detailed linting example in [examples] dir.

  [lrlna]: https://github.com/lrlna
  [pull/328]: https://github.com/apollographql/apollo-rs/pull/328
  [examples]: ./examples/extend_db.rs

- **validate argument name uniqueness - [goto-bus-stop], [pull/317]**

  It's an error to declare or provide multiple arguments by the same name, eg:

    ```graphql
    type Query {
      things(offset: Int!, offset: Int!): [Thing]
      # ERR: duplicate argument definition: offset
    }
    ```

    ```graphql
    query GetThings {
      things(offset: 10, offset: 20) { id }
      # ERR: duplicate argument values: offset
    }
    ```

  This adds `UniqueArgument` diagnostics and checks for argument duplications in:
  field definitions, fields, directives, interfaces and directive definitions.

  [goto-bus-stop]: https://github.com/goto-bus-stop
  [pull/317]: https://github.com/apollographql/apollo-rs/pull/317

- **getter for directives in HIR FragmentSpread - [allancalix], [pull/315]**

  Allow accessing directives in a given FragmentSpread node in a high-level
  intermediate representation of the compiler.

  [allancalix]: https://github.com/allancalix
  [pull/315]: https://github.com/apollographql/apollo-rs/pull/315

- **create validation database - [lrlna], [pull/303]**

  All validation now happens in its own database, which can be accessed with
  `ValidationDatabase` and `ValidationStorage`.

  [lrlna]: https://github.com/lrlna
  [pull/303]: https://github.com/apollographql/apollo-rs/pull/303

- **thread-safe compiler: introduce snapshots - [lrlna], [pull/295] + [pull/332]**

  Implements `ParallelDatabase` for `RootDatabase` of the compiler. This allows
  us to create snapshots that can allow users to query the database from
  multiple threads. For example:

    ```rust
    let input = r#"
    type Query {
      website: URL,
      amount: Int
    }

    scalar URL @specifiedBy(url: "https://tools.ietf.org/html/rfc3986")
    "#;

    let ctx = ApolloCompiler::new(input);
    let diagnostics = ctx.validate();
    for diagnostic in &diagnostics {
        println!("{}", diagnostic);
    }

    assert!(diagnostics.is_empty());

    let snapshot = ctx.snapshot();
    let snapshot2 = ctx.snapshot();

    let thread1 = std::thread::spawn(move || snapshot.find_object_type_by_name("Query".into()));
    let thread2 = std::thread::spawn(move || snapshot2.scalars());

    thread1.join().expect("object_type_by_name panicked");
    thread2.join().expect("scalars failed");
    ```

  [lrlna]: https://github.com/lrlna
  [pull/295]: https://github.com/apollographql/apollo-rs/pull/295
  [pull/332]: https://github.com/apollographql/apollo-rs/pull/332

- **add description getters to compiler's HIR nodes - [aschaeffer], [pull/289]**

  Expose getters for descriptions that can be accessed for any definitions that
  support them. For example:
    ```rust
    let input = r#"
    "Books in a given libary"
    type Book {
      id: ID!
    }
    "#;

    let ctx = ApolloCompiler::new(input);

    let desc = ctx.db.find_object_type_by_name("Book".to_string()).unwrap().description();
    ```
  [aschaeffer]: https://github.com/aschaeffer
  [pull/289]: https://github.com/apollographql/apollo-rs/pull/289

## Fixes
- **update parser version - [goto-bus-stop], [pull/331]**

  [goto-bus-stop]: https://github.com/goto-bus-stop
  [pull/331]: https://github.com/apollographql/apollo-rs/pull/331

- **unused variables return an error diagnostic - [lrlna], [pull/314]**

  We were previously returning a warning for any unused variables, it is now
  reported as an error.

  [lrlna]: https://github.com/lrlna
  [pull/314]: https://github.com/apollographql/apollo-rs/pull/314
## Maintenance
- **split up db into several components - [lrlna], [pull/290]**

  We are splitting up the single `query_group` we had in our db into several
  `query_group`s that currently just build upon each other, and eventually could
  support more complex relationships between one another. The current structure:

  `Inputs` --> `DocumentParser` --> `Definitions` --> `Document`

  All of these `query_group`s make up the `RootDatabase`, i.e. `salsa::database`.

  This also allows external users to build out their own databases with
  compiler's query groups.

  [lrlna]: https://github.com/lrlna
  [pull/290]: https://github.com/apollographql/apollo-rs/pull/290

- **support wasm compiler target - [allancalix], [pull/287], [issue/288]**

  `apollo-compiler` can now compile to a Wasm target with `cargo check --target wasm32-unknown-unknown`.

  [allancalix]: https://github.com/allancalix
  [pull/287]: https://github.com/apollographql/apollo-rs/pull/287
  [issue/288]: https://github.com/apollographql/apollo-rs/issues/288

# [0.2.0](https://crates.io/crates/apollo-compiler/0.2.0) - 2022-08-16

## Breaking
- **inline_fragment().type_condition() returns Option<&str> - [lrlna], [pull/282]**

  Instead of returning `Option<&String>`, we now return `Option<&str>`

  [lrlna]: https://github.com/lrlna
  [pull/272]: https://github.com/apollographql/apollo-rs/pull/282

- **Value -> DefaultValue for default_value fields - [lrlna], [pull/276]**

  default_value getters in Input Value Definitions and Variable Definitions now
  return `DefaultValue` type. This is a type alias to Value, and makes it
  consistent with the GraphQL spec.

  [lrlna]: https://github.com/lrlna
  [pull/276]: https://github.com/apollographql/apollo-rs/pull/276

## Fixes
- **add type information to inline fragments  - [lrlna], [pull/282], [issue/280]**

  Inline fragments were missing type correct type information. We now search for
  applicable type's fields if a type condition exists, otherwise infer
  information from the current type in scope ([as per spec](https://spec.graphql.org/October2021/#sel-GAFbfJABAB_F3kG ))
  .Inline fragments

  [lrlna]: https://github.com/lrlna
  [pull/282]: https://github.com/apollographql/apollo-rs/pull/282
  [issue/280]: https://github.com/apollographql/apollo-rs/issues/280

- **fix cycle error in fragment spreads referencing fragments - [lrlna], [pull/283], [issue/281]**

  Because fragment definitions can have fragment spreads, we are running into a
  self-referential cycle when searching for a fragment definition to get its id.
  Instead, search for the fragment definition when getting a fragment in a
  fragment spread in the wrapper API.

  [lrlna]: https://github.com/lrlna
  [pull/283]: https://github.com/apollographql/apollo-rs/pull/283
  [issue/281]: https://github.com/apollographql/apollo-rs/issues/281

## Features
- **pub use ApolloDiagnostic - [EverlastingBugstopper], [pull/268]**

  Exports ApolloDiagnostic to allow users to use it in other contexts.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/268]: https://github.com/apollographql/apollo-rs/pull/268

- **default_value getter in input_value_definition - [lrlna], [pull/273]**

  A getter for default values in input value definitions.

  [lrlna]: https://github.com/lrlna
  [pull/273]: https://github.com/apollographql/apollo-rs/pull/273

- **feat(compiler): add db.find_union_by_name() - [lrlna], [pull/272]**

  Allows to query unions in the database.

  [lrlna]: https://github.com/lrlna
  [pull/272]: https://github.com/apollographql/apollo-rs/pull/272

- **adds inline_fragments getter to SelectionSet - [lrlna], [pull/282]**

  Convenience method to get all inline fragments in the current selection set.

  [lrlna]: https://github.com/lrlna
  [pull/282]: https://github.com/apollographql/apollo-rs/pull/282

# [0.1.0](https://crates.io/crates/apollo-compiler/0.1.0) - 2022-07-27

Introducing `apollo-compiler`!

A query-based compiler for the GraphQL language.

The compiler provides validation and context for GraphQL documents with a comprehensive API.

This is still a work in progress, for outstanding issues, checkout out the
[apollo-compiler label] in our issue tracker.

[apollo-compiler label]: https://github.com/apollographql/apollo-rs/labels/apollo-compiler
