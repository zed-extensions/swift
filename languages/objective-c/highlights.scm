; ============================================================
; C base highlights (compatible with tree-sitter-objc)
; ============================================================

(identifier) @variable

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]*$"))

; C Keywords

[
  "break"
  "case"
  "const"
  "continue"
  "default"
  "do"
  "else"
  "enum"
  "extern"
  "for"
  "if"
  "inline"
  "return"
  "sizeof"
  "static"
  "struct"
  "switch"
  "typedef"
  "union"
  "volatile"
  "while"
] @keyword

; Preprocessor

(preproc_directive) @keyword

; #include / #import — highlight the whole node as keyword, path as string
(preproc_include) @keyword
(preproc_include path: (string_literal) @string)
(preproc_include path: (system_lib_string) @string)

; #define — keyword + macro name as constant + value as string
(preproc_def) @keyword
(preproc_def name: (identifier) @constant)
(preproc_def value: (preproc_arg) @string)

(preproc_function_def) @keyword
(preproc_function_def name: (identifier) @function.special)
(preproc_function_def value: (preproc_arg) @string)

; #if / #ifdef / #ifndef / #elif / #else / #endif
(preproc_if) @keyword
(preproc_ifdef) @keyword
(preproc_elif) @keyword
(preproc_else) @keyword
(preproc_call) @keyword

; C Operators

[
  "--"
  "-"
  "-="
  "->"
  "="
  "!="
  "!"
  "*"
  "&"
  "&&"
  "+"
  "++"
  "+="
  "<"
  "=="
  ">"
  ">="
  "<="
  "||"
  "|"
  "~"
  "%"
  "/"
  "?"
  ":"
] @operator

; Delimiters

[
  "."
  ";"
  ","
] @delimiter

; Strings and Literals

(string_literal) @string
(system_lib_string) @string
(concatenated_string) @string
(escape_sequence) @string.escape

(null) @constant
(number_literal) @number
(char_literal) @number
(true) @constant.builtin
(false) @constant.builtin

; Fields and Types

(field_identifier) @property
(statement_identifier) @label
(type_identifier) @type
(primitive_type) @type
(sized_type_specifier) @type

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function))
(function_declarator
  declarator: (identifier) @function)
(preproc_function_def
  name: (identifier) @function.special)

; Comments

(comment) @comment

; Brackets

["(" ")" "[" "]" "{" "}"] @punctuation.bracket

; ============================================================
; Objective-C specific highlights
; ============================================================

; Preprocs

(preproc_undef
  name: (_) @constant) @preproc

; @import module
(module_import "@import" @keyword path: (identifier) @namespace)

; Type Qualifiers

[
  "@optional"
  "@required"
  "__covariant"
  "__contravariant"
  (visibility_specification)
] @type.qualifier

; Storageclasses

[
  "@autoreleasepool"
  "@synthesize"
  "@dynamic"
  (protocol_qualifier)
] @storageclass

; ObjC Keywords

[
  "@protocol"
  "@interface"
  "@implementation"
  "@compatibility_alias"
  "@property"
  "@selector"
  "@defs"
  "availability"
  "@end"
] @keyword

(class_declaration "@" @keyword "class" @keyword)

(method_definition ["+" "-"] @keyword.function)
(method_declaration ["+" "-"] @keyword.function)

[
  "__typeof__"
  "__typeof"
  "typeof"
  "in"
] @keyword.operator

[
  "@synchronized"
  "oneway"
] @keyword.coroutine

; Exceptions

[
  "@try"
  "__try"
  "@catch"
  "__catch"
  "@finally"
  "__finally"
  "@throw"
] @exception

; Variables

((identifier) @variable.builtin
  (#any-of? @variable.builtin "self" "super"))

; Functions & Methods

[
  "objc_bridge_related"
  "@available"
  "__builtin_available"
  "va_arg"
  "asm"
] @function.builtin

(method_definition (identifier) @method)

(method_declaration (identifier) @method)

(method_identifier (identifier)? @method ":" @method (identifier)? @method)

(message_expression method: (identifier) @method.call)

; Constructors

((message_expression method: (identifier) @constructor)
  (#eq? @constructor "init"))

; Attributes

(availability_attribute_specifier
  [
    "CF_FORMAT_FUNCTION" "NS_AVAILABLE" "__IOS_AVAILABLE" "NS_AVAILABLE_IOS"
    "API_AVAILABLE" "API_UNAVAILABLE" "API_DEPRECATED" "NS_ENUM_AVAILABLE_IOS"
    "NS_DEPRECATED_IOS" "NS_ENUM_DEPRECATED_IOS" "NS_FORMAT_FUNCTION" "DEPRECATED_MSG_ATTRIBUTE"
    "__deprecated_msg" "__deprecated_enum_msg" "NS_SWIFT_NAME" "NS_SWIFT_UNAVAILABLE"
    "NS_EXTENSION_UNAVAILABLE_IOS" "NS_CLASS_AVAILABLE_IOS" "NS_CLASS_DEPRECATED_IOS" "__OSX_AVAILABLE_STARTING"
    "NS_ROOT_CLASS" "NS_UNAVAILABLE" "NS_REQUIRES_NIL_TERMINATION" "CF_RETURNS_RETAINED"
    "CF_RETURNS_NOT_RETAINED" "DEPRECATED_ATTRIBUTE" "UI_APPEARANCE_SELECTOR" "UNAVAILABLE_ATTRIBUTE"
  ]) @attribute

; ObjC Macros / Type Qualifiers

(type_qualifier
  [
    "_Complex"
    "_Nonnull"
    "_Nullable"
    "_Nullable_result"
    "_Null_unspecified"
    "__autoreleasing"
    "__block"
    "__bridge"
    "__bridge_retained"
    "__bridge_transfer"
    "__complex"
    "__kindof"
    "__nonnull"
    "__nullable"
    "__ptrauth_objc_class_ro"
    "__ptrauth_objc_isa_pointer"
    "__ptrauth_objc_super_pointer"
    "__strong"
    "__thread"
    "__unsafe_unretained"
    "__unused"
    "__weak"
  ]) @function.macro.builtin

[ "__real" "__imag" ] @function.macro.builtin

; ObjC Types

(class_declaration (identifier) @type)

(class_interface "@interface" . (identifier) @type superclass: _? @type category: _? @namespace)

(class_implementation "@implementation" . (identifier) @type superclass: _? @type category: _? @namespace)

(protocol_forward_declaration (identifier) @type)

(protocol_reference_list (identifier) @type)

[
  "BOOL"
  "IMP"
  "SEL"
  "Class"
  "id"
] @type.builtin

; Constants

(property_attribute (identifier) @constant "="?)

[ "__asm" "__asm__" ] @constant.macro

; Properties

(property_implementation "@synthesize" (identifier) @property)

((identifier) @property
  (#has-ancestor? @property struct_declaration))

; Parameters

(method_parameter ":" @method (identifier) @parameter)

(method_parameter declarator: (identifier) @parameter)

(parameter_declaration
  declarator: (function_declarator
                declarator: (parenthesized_declarator
                              (block_pointer_declarator
                                declarator: (identifier) @parameter))))

"..." @parameter.builtin

; ObjC Operators

"^" @operator

; ObjC Literals

(platform) @string.special

(version_number) @number

; ObjC Punctuation

"@" @punctuation.special
