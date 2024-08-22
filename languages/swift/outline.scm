(protocol_declaration
  declaration_kind: (_) @context
  name: (type_identifier) @name) @item

(class_declaration
    declaration_kind: "struct" @context
    name: (type_identifier) @name) @item

(class_declaration
    declaration_kind: "class" @context
    name: (type_identifier) @name) @item

(class_declaration
    declaration_kind: "extension" @context
    name: (user_type) @name) @item

(class_declaration
    declaration_kind: "actor" @context
    name: (type_identifier) @name) @item @context.actor

(init_declaration
  name: "init" @name) @item

(deinit_declaration
  "deinit" @name) @item

(function_declaration
  name: (simple_identifier) @name) @item

(property_declaration
  name: (simple_identifier) @name) @item
