(protocol_declaration
    declaration_kind: "protocol" @name
    name: (type_identifier) @name) @item

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
    "(" @name
    (
        (parameter
            external_name: (simple_identifier)? @name
            name: (simple_identifier) @name ; should be captured only if external_name isn't presented.
            ":" @name
            (user_type
                (type_identifier)  @name
            )
        )
    ","?
    )*
    ")" @name
    body: (function_body) @body
) @item

(property_declaration
    (value_binding_pattern) @name
    name: (pattern) @name
) @item
