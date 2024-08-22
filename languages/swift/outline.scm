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
    declaration_kind: "struct" @name
    name: (type_identifier) @name) @item

(class_declaration
    declaration_kind: "class" @name
    name: (type_identifier) @name) @item

(class_declaration
    declaration_kind: "extension" @name
    name: (user_type) @name) @item

(class_declaration
    declaration_kind: "actor" @name
    name: (type_identifier) @name) @item ;@context.actor

(init_declaration
    name: "init" @name) @item

(deinit_declaration
    "deinit" @name) @item

(function_declaration
    "func" @name
    name: (simple_identifier) @name
    (type_parameters)? @name
    "(" @name
        (
            (parameter) @name
            ","? @name
        )*
    ")" @name
    "->"? @name
    (user_type)? @name
    (tuple_type)? @name
    (dictionary_type)? @name
    (array_type)? @name
    (optional_type)? @name
) @item

(class_body
    (property_declaration
        (value_binding_pattern) @name
        name: (pattern) @name
        (type_annotation)? @name
    ) @item
)
