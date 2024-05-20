///  // match args.entity_type {
    //     get_ast::EntityType::User(user_command) => user_command.delegate(),
    //     get_ast::EntityType::Video(video_command) => video_command.delegate(),
    //     get_ast::EntityType::View(view_command) => view_command.delegate(),
    // }
/// 

/// ```
/// use syn::{Item, Visibility, Signature, Generics, Ident, Member, Block, Stmt, Expr, Pat, Path, PathSegment, PathArguments, ReturnType, Fn, Paren, Match, Brace, Arm, FatArrow, Comma, PathSep, Dot};
/// 
/// const exp: Item::Fn = Item::Fn {
///     attrs: [],
///     vis: Visibility::Inherited,
///     sig: Signature {
///         constness: None,
///         asyncness: None,
///         unsafety: None,
///         abi: None,
///         fn_token: Fn,
///         ident: Ident {
///             ident: "main",
///             span: #0 bytes(1428..1432),
///         },
///         generics: Generics {
///             lt_token: None,
///             params: [],
///             gt_token: None,
///             where_clause: None,
///         },
///         paren_token: Paren,
///         inputs: [],
///         variadic: None,
///         output: ReturnType::Default,
///     },
///     block: Block {
///         brace_token: Brace,
///         stmts: [
///             Stmt::Expr(
///                 Expr::Match {
///                     attrs: [],
///                     match_token: Match,
///                     expr: Expr::Field {
///                         attrs: [],
///                         base: Expr::Path {
///                             attrs: [],
///                             qself: None,
///                             path: Path {
///                                 leading_colon: None,
///                                 segments: [
///                                     PathSegment {
///                                         ident: Ident {
///                                             ident: "args",
///                                             span: #0 bytes(1502..1506),
///                                         },
///                                         arguments: PathArguments::None,
///                                     },
///                                 ],
///                             },
///                         },
///                         dot_token: Dot,
///                         member: Member::Named(
///                             Ident {
///                                 ident: "entity_type",
///                                 span: #0 bytes(1507..1518),
///                             },
///                         ),
///                     },
///                     brace_token: Brace,
///                     arms: [
///                         Arm {
///                             attrs: [],
///                             pat: Pat::TupleStruct {
///                                 attrs: [],
///                                 qself: None,
///                                 path: Path {
///                                     leading_colon: None,
///                                     segments: [
///                                         PathSegment {
///                                             ident: Ident {
///                                                 ident: "get_ast",
///                                                 span: #0 bytes(1529..1536),
///                                             },
///                                             arguments: PathArguments::None,
///                                         },
///                                         PathSep,
///                                         PathSegment {
///                                             ident: Ident {
///                                                 ident: "EntityType",
///                                                 span: #0 bytes(1538..1548),
///                                             },
///                                             arguments: PathArguments::None,
///                                         },
///                                         PathSep,
///                                         PathSegment {
///                                             ident: Ident {
///                                                 ident: "User",
///                                                 span: #0 bytes(1550..1554),
///                                             },
///                                             arguments: PathArguments::None,
///                                         },
///                                     ],
///                                 },
///                                 paren_token: Paren,
///                                 elems: [
///                                     Pat::Ident {
///                                         attrs: [],
///                                         by_ref: None,
///                                         mutability: None,
///                                         ident: Ident {
///                                             ident: "user_command",
///                                             span: #0 bytes(1555..1567),
///                                         },
///                                         subpat: None,
///                                     },
///                                 ],
///                             },
///                             guard: None,
///                             fat_arrow_token: FatArrow,
///                             body: Expr::MethodCall {
///                                 attrs: [],
///                                 receiver: Expr::Path {
///                                     attrs: [],
///                                     qself: None,
///                                     path: Path {
///                                         leading_colon: None,
///                                         segments: [
///                                             PathSegment {
///                                                 ident: Ident {
///                                                     ident: "user_command",
///                                                     span: #0 bytes(1572..1584),
///                                                 },
///                                                 arguments: PathArguments::None,
///                                             },
///                                         ],
///                                     },
///                                 },
///                                 dot_token: Dot,
///                                 method: Ident {
///                                     ident: "delegate",
///                                     span: #0 bytes(1585..1593),
///                                 },
///                                 turbofish: None,
///                                 paren_token: Paren,
///                                 args: [],
///                             },
///                             comma: Some(
///                                 Comma,
///                             ),
///                         },
///                         Arm {
///                             attrs: [],
///                             pat: Pat::TupleStruct {
///                                 attrs: [],
///                                 qself: None,
///                                 path: Path {
///                                     leading_colon: None,
///                                     segments: [
///                                         PathSegment {
///                                             ident: Ident {
///                                                 ident: "get_ast",
///                                                 span: #0 bytes(1605..1612),
///                                             },
///                                             arguments: PathArguments::None,
///                                         },
///                                         PathSep,
///                                         PathSegment {
///                                             ident: Ident {
///                                                 ident: "EntityType",
///                                                 span: #0 bytes(1614..1624),
///                                             },
///                                             arguments: PathArguments::None,
///                                         },
///                                         PathSep,
///                                         PathSegment {
///                                             ident: Ident {
///                                                 ident: "Video",
///                                                 span: #0 bytes(1626..1631),
///                                             },
///                                             arguments: PathArguments::None,
///                                         },
///                                     ],
///                                 },
///                                 paren_token: Paren,
///                                 elems: [
///                                     Pat::Ident {
///                                         attrs: [],
///                                         by_ref: None,
///                                         mutability: None,
///                                         ident: Ident {
///                                             ident: "video_command",
///                                             span: #0 bytes(1632..1645),
///                                         },
///                                         subpat: None,
///                                     },
///                                 ],
///                             },
///                             guard: None,
///                             fat_arrow_token: FatArrow,
///                             body: Expr::MethodCall {
///                                 attrs: [],
///                                 receiver: Expr::Path {
///                                     attrs: [],
///                                     qself: None,
///                                     path: Path {
///                                         leading_colon: None,
///                                         segments: [
///                                             PathSegment {
///                                                 ident: Ident {
///                                                     ident: "video_command",
///                                                     span: #0 bytes(1650..1663),
///                                                 },
///                                                 arguments: PathArguments::None,
///                                             },
///                                         ],
///                                     },
///                                 },
///                                 dot_token: Dot,
///                                 method: Ident {
///                                     ident: "delegate",
///                                     span: #0 bytes(1664..1672),
///                                 },
///                                 turbofish: None,
///                                 paren_token: Paren,
///                                 args: [],
///                             },
///                             comma: Some(
///                                 Comma,
///                             ),
///                         },
///                         Arm {
///                             attrs: [],
///                             pat: Pat::TupleStruct {
///                                 attrs: [],
///                                 qself: None,
///                                 path: Path {
///                                     leading_colon: None,
///                                     segments: [
///                                         PathSegment {
///                                             ident: Ident {
///                                                 ident: "get_ast",
///                                                 span: #0 bytes(1684..1691),
///                                             },
///                                             arguments: PathArguments::None,
///                                         },
///                                         PathSep,
///                                         PathSegment {
///                                             ident: Ident {
///                                                 ident: "EntityType",
///                                                 span: #0 bytes(1693..1703),
///                                             },
///                                             arguments: PathArguments::None,
///                                         },
///                                         PathSep,
///                                         PathSegment {
///                                             ident: Ident {
///                                                 ident: "View",
///                                                 span: #0 bytes(1705..1709),
///                                             },
///                                             arguments: PathArguments::None,
///                                         },
///                                     ],
///                                 },
///                                 paren_token: Paren,
///                                 elems: [
///                                     Pat::Ident {
///                                         attrs: [],
///                                         by_ref: None,
///                                         mutability: None,
///                                         ident: Ident {
///                                             ident: "view_command",
///                                             span: #0 bytes(1710..1722),
///                                         },
///                                         subpat: None,
///                                     },
///                                 ],
///                             },
///                             guard: None,
///                             fat_arrow_token: FatArrow,
///                             body: Expr::MethodCall {
///                                 attrs: [],
///                                 receiver: Expr::Path {
///                                     attrs: [],
///                                     qself: None,
///                                     path: Path {
///                                         leading_colon: None,
///                                         segments: [
///                                             PathSegment {
///                                                 ident: Ident {
///                                                     ident: "view_command",
///                                                     span: #0 bytes(1727..1739),
///                                                 },
///                                                 arguments: PathArguments::None,
///                                             },
///                                         ],
///                                     },
///                                 },
///                                 dot_token: Dot,
///                                 method: Ident {
///                                     ident: "delegate",
///                                     span: #0 bytes(1740..1748),
///                                 },
///                                 turbofish: None,
///                                 paren_token: Paren,
///                                 args: [],
///                             },
///                             comma: Some(
///                                 Comma,
///                             ),
///                         },
///                     ],
///                 },
///                 None,
///             ),
///         ],
///     },
/// };
/// ```