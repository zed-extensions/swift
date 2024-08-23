(protocol_declaration
    declaration_kind: "protocol" @name
    name: (type_identifier) @name
    (
        (":")
        .
        (
            (inheritance_specifier)
            .
            (",")? @name
        )* @name
    )? @name
) @item

(class_declaration
    declaration_kind: (
        [
            "actor"
            "class"
            "extension"
            "enum"
            "struct"
        ]
    ) @name
    name: [
        (user_type)
        (type_identifier)
    ] @name
    (
        (":")
        .
        (
            (inheritance_specifier)
            .
            (",")? @name
        )* @name
    )? @name
) @item

(init_declaration
    name: "init" @name) @item

(deinit_declaration
    "deinit" @name) @item

(function_declaration
    "func" @name
    .
    _ * @name
    .
    body: (function_body)
) @item

(class_body
    (property_declaration
        (value_binding_pattern) @name
        name: (pattern) @name
        (type_annotation)? @name
    ) @item
)
