[
  "."
  ";"
  ":"
  ","
] @punctuation.delimiter

[
  "\\("
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket ; TODO: "\\(" ")" in interpolations should be @punctuation.special

; Identifiers
(attribute) @variable

(type_identifier) @type

(self_expression) @variable.builtin

(user_type
  (type_identifier) @variable.builtin
  (#eq? @variable.builtin "Self"))

; Declarations
"func" @keyword.function

[
  (visibility_modifier)
  (member_modifier)
  (function_modifier)
  (property_modifier)
  (parameter_modifier)
  (inheritance_modifier)
  (mutation_modifier)
] @keyword

(function_declaration
  (simple_identifier) @function.method)

(init_declaration
  "init" @constructor)

(deinit_declaration
  "deinit" @constructor)

(throws) @keyword

"async" @keyword

"await" @keyword

(where_keyword) @keyword

(parameter
  external_name: (simple_identifier) @variable.parameter)

(parameter
  name: (simple_identifier) @variable.parameter)

(type_parameter
  (type_identifier) @property)

(inheritance_constraint
  (identifier
    (simple_identifier) @property))

(equality_constraint
  (identifier
    (simple_identifier) @property))

(pattern
  bound_identifier: (simple_identifier)) @variable

[
  "typealias"
  "struct"
  "class"
  "actor"
  "enum"
  "protocol"
  "extension"
  "indirect"
  "nonisolated"
  "override"
  "convenience"
  "required"
  "mutating"
  "nonmutating"
  "associatedtype"
] @keyword

(opaque_type
  "some" @keyword)

(existential_type
  "any" @keyword)

(precedence_group_declaration
  "precedencegroup" @keyword
  (simple_identifier) @type)

(precedence_group_attribute
  (simple_identifier) @keyword
  [
    (simple_identifier) @type
    (boolean_literal) @boolean
  ])

[
  (getter_specifier)
  (setter_specifier)
  (modify_specifier)
] @keyword

(class_body
  (property_declaration
    (pattern
      (simple_identifier) @variable.parameter)))

(protocol_property_declaration
  (pattern
    (simple_identifier) @variable.parameter))

(value_argument
  name: (value_argument_label) @label)

(import_declaration
  "import" @keyword.import)

(enum_entry
  "case" @keyword)

; Function calls
(call_expression
  (simple_identifier) @function.call) ; foo()

(call_expression ; foo.bar.baz(): highlight the baz()
  (navigation_expression
    (navigation_suffix
      (simple_identifier) @function.call)))

((navigation_expression
  (simple_identifier) @type) ; SomeType.method(): highlight SomeType as a type
  (#match? @property "^[A-Z]"))

(call_expression
  (simple_identifier) @keyword
  (#eq? @keyword "defer")) ; defer { ... }

(try_operator) @operator

(try_operator
  "try" @keyword)

(directive) @function.macro

(diagnostic) @function.macro

; Statements
(for_statement
  "for" @keyword.repeat)

(for_statement
  "in" @keyword.repeat)

(for_statement
  (pattern) @variable)

(else) @keyword

(as_operator) @keyword

[
  "while"
  "repeat"
  "continue"
  "break"
] @keyword.repeat

[
  "let"
  "var"
] @keyword

(guard_statement
  "guard" @keyword.conditional)

(if_statement
  "if" @keyword.conditional)

(switch_statement
  "switch" @keyword.conditional)

(switch_entry
  "case" @keyword)

(switch_entry
  "fallthrough" @keyword)

(switch_entry
  (default_keyword) @keyword)

"return" @keyword.return

(ternary_expression
  [
    "?"
    ":"
  ] @keyword.conditional)

[
  "do"
  (throw_keyword)
  (catch_keyword)
] @keyword

(statement_label) @label

; Comments
[
  (comment)
  (multiline_comment)
] @comment @spell

((comment) @comment.documentation
  (#lua-match? @comment.documentation "^///[^/]"))

((comment) @comment.documentation
  (#lua-match? @comment.documentation "^///$"))

((multiline_comment) @comment.documentation
  (#lua-match? @comment.documentation "^/[*][*][^*].*[*]/$"))

; String literals
(line_str_text) @string

(str_escaped_char) @string

(multi_line_str_text) @string

(raw_str_part) @string

(raw_str_end_part) @string

(raw_str_interpolation_start) @punctuation.special

[
  "\""
  "\"\"\""
] @string

; Lambda literals
(lambda_literal
  "in" @keyword.operator)

; Basic literals
[
  (integer_literal)
  (hex_literal)
  (oct_literal)
  (bin_literal)
] @number

(real_literal) @number.float

(boolean_literal) @boolean

"nil" @keyword

; Regex literals
(regex_literal) @string.regexp

; Operators
(custom_operator) @operator

[
  "!"
  "?"
  "+"
  "-"
  "*"
  "/"
  "%"
  "="
  "+="
  "-="
  "*="
  "/="
  "<"
  ">"
  "<="
  ">="
  "++"
  "--"
  "&"
  "~"
  "%="
  "!="
  "!=="
  "=="
  "==="
  "??"
  "->"
  "..<"
  "..."
  (bang)
] @operator

(value_parameter_pack
  "each" @keyword)

(value_pack_expansion
  "repeat" @keyword)

(type_parameter_pack
  "each" @keyword)

(type_pack_expansion
  "repeat" @keyword)

; The patterns below refine captures made earlier in this file. Later patterns take
; precedence for the same node, so their order matters.

; Declarations: color a declared name distinctly from its later uses, as Xcode does.
(class_declaration
  name: (type_identifier) @type.definition)

(protocol_declaration
  name: (type_identifier) @type.definition)

(typealias_declaration
  name: (type_identifier) @type.definition)

(function_declaration
  (simple_identifier) @function.definition)

(enum_entry
  name: (simple_identifier) @variant)

; Attributes: the "@" and the name, leaving any arguments to their own captures.
; Property wrappers and macros are macros, and global actors and result builders are
; system types, following Xcode's coloring.
(attribute
  "@" @attribute
  (user_type
    (type_identifier) @attribute))

((attribute
  "@" @function.macro
  (user_type
    (type_identifier) @function.macro @_attribute_name))
  (#any-of? @_attribute_name
    "Observable"
    "Model"
    "State"
    "Binding"
    "Bindable"
    "Environment"
    "EnvironmentObject"
    "StateObject"
    "ObservedObject"
    "Published"
    "AppStorage"
    "SceneStorage"
    "FocusState"
    "FocusedValue"
    "FocusedBinding"
    "GestureState"
    "Namespace"
    "ScaledMetric"
    "Query"
    "Attribute"
    "Relationship"
    "Transient"
    "Test"
    "Suite"
    "Entry"
    "Previewable"
    "Animatable"
    "DebugDescription"
    "Dependency"
    "Reducer"
    "ObservableState"
    "Presents"))

((attribute
  "@" @type.builtin
  (user_type
    (type_identifier) @type.builtin @_attribute_name))
  (#any-of? @_attribute_name
    "MainActor"
    "ViewBuilder"
    "SceneBuilder"
    "ToolbarContentBuilder"
    "CommandsBuilder"
    "TableColumnBuilder"
    "TableRowBuilder"
    "AccessibilityRotorContentBuilder"
    "ResultBuilder"
    "globalActor"
    "resultBuilder"
    "Sendable"
    "preconcurrency"
    "retroactive"))

; Constructor calls of capitalized names are types: project types by default, system
; types when the name is in the list below. The grammar parses `UUID()` as a call of a
; simple_identifier rather than as a constructor_expression, so both node kinds are covered.
((call_expression
  (simple_identifier) @type)
  (#match? @type "^[A-Z]"))

; System types: the Swift standard library, common Foundation and SwiftUI types, and
; Apple's framework prefixes.
([
  (type_identifier) @type.builtin
  (call_expression
    (simple_identifier) @type.builtin)
]
  (#any-of? @type.builtin
    "Any"
    "AnyObject"
    "AnyHashable"
    "Array"
    "ArraySlice"
    "AsyncSequence"
    "AsyncStream"
    "AsyncThrowingStream"
    "BinaryFloatingPoint"
    "BinaryInteger"
    "Bool"
    "Character"
    "ClosedRange"
    "Codable"
    "Collection"
    "Comparable"
    "ContiguousArray"
    "CustomDebugStringConvertible"
    "CustomStringConvertible"
    "Decodable"
    "Dictionary"
    "Double"
    "Duration"
    "Encodable"
    "Equatable"
    "Error"
    "ExpressibleByArrayLiteral"
    "ExpressibleByBooleanLiteral"
    "ExpressibleByDictionaryLiteral"
    "ExpressibleByFloatLiteral"
    "ExpressibleByIntegerLiteral"
    "ExpressibleByNilLiteral"
    "ExpressibleByStringLiteral"
    "Float"
    "Float16"
    "Float32"
    "Float64"
    "Float80"
    "FloatingPoint"
    "Hashable"
    "Hasher"
    "Identifiable"
    "Int"
    "Int8"
    "Int16"
    "Int32"
    "Int64"
    "Iterator"
    "IteratorProtocol"
    "KeyPath"
    "LocalizedError"
    "Never"
    "Numeric"
    "ObjectIdentifier"
    "Optional"
    "OptionSet"
    "PartialKeyPath"
    "RandomAccessCollection"
    "RandomNumberGenerator"
    "Range"
    "RawRepresentable"
    "Result"
    "Sendable"
    "Sequence"
    "Set"
    "SignedInteger"
    "SignedNumeric"
    "StaticString"
    "StringProtocol"
    "String"
    "Strideable"
    "Substring"
    "Task"
    "TaskGroup"
    "ThrowingTaskGroup"
    "UInt"
    "UInt8"
    "UInt16"
    "UInt32"
    "UInt64"
    "Unicode"
    "UnsafeMutablePointer"
    "UnsafeMutableRawPointer"
    "UnsafePointer"
    "UnsafeRawPointer"
    "UnsignedInteger"
    "Void"
    "Actor"
    "MainActor"
    "Bundle"
    "Calendar"
    "Data"
    "Date"
    "DateComponents"
    "DateFormatter"
    "DateInterval"
    "Decimal"
    "FileManager"
    "IndexPath"
    "IndexSet"
    "JSONDecoder"
    "JSONEncoder"
    "Locale"
    "Measurement"
    "Notification"
    "NotificationCenter"
    "NumberFormatter"
    "OperationQueue"
    "ProcessInfo"
    "PropertyListDecoder"
    "PropertyListEncoder"
    "RunLoop"
    "Scanner"
    "Thread"
    "TimeInterval"
    "TimeZone"
    "Timer"
    "URL"
    "URLComponents"
    "URLRequest"
    "URLResponse"
    "URLSession"
    "URLSessionConfiguration"
    "UUID"
    "UserDefaults"
    "Alignment"
    "Angle"
    "AnyView"
    "Axis"
    "Binding"
    "Button"
    "Capsule"
    "Circle"
    "Color"
    "Divider"
    "EdgeInsets"
    "Edge"
    "EmptyView"
    "EnvironmentValues"
    "Font"
    "ForEach"
    "Form"
    "GeometryReader"
    "Group"
    "GroupBox"
    "HStack"
    "Image"
    "Label"
    "LazyHGrid"
    "LazyHStack"
    "LazyVGrid"
    "LazyVStack"
    "Link"
    "List"
    "Menu"
    "NavigationLink"
    "NavigationPath"
    "NavigationSplitView"
    "NavigationStack"
    "ObservableObject"
    "Path"
    "Picker"
    "ProgressView"
    "Rectangle"
    "RoundedRectangle"
    "Scene"
    "ScrollView"
    "Section"
    "SecureField"
    "Shape"
    "Slider"
    "Spacer"
    "Stepper"
    "TabView"
    "Text"
    "TextEditor"
    "TextField"
    "Toggle"
    "ToolbarItem"
    "VStack"
    "View"
    "ViewModifier"
    "WindowGroup"
    "ModelContainer"
    "ModelContext"
    "PersistentModel"
    "Logger"
    "OSLog"))

([
  (type_identifier) @type.builtin
  (call_expression
    (simple_identifier) @type.builtin)
]
  (#match? @type.builtin "^(NS|UI|CG|CA|CL|CN|CT|CV|AV|AR|EK|GK|HK|MK|MP|PK|SK|SC|VN|WK|OS)[A-Z][A-Za-z0-9]*$"))

; System functions from the standard library.
((call_expression
  (simple_identifier) @function.builtin)
  (#any-of? @function.builtin
    "print"
    "debugPrint"
    "dump"
    "readLine"
    "max"
    "min"
    "abs"
    "swap"
    "zip"
    "stride"
    "sequence"
    "repeatElement"
    "precondition"
    "preconditionFailure"
    "assert"
    "assertionFailure"
    "fatalError"
    "type"
    "unsafeBitCast"
    "withExtendedLifetime"
    "withUnsafePointer"
    "withUnsafeMutablePointer"
    "withUnsafeBytes"
    "withUnsafeMutableBytes"
    "withCheckedContinuation"
    "withCheckedThrowingContinuation"
    "withTaskGroup"
    "withThrowingTaskGroup"
    "withTaskCancellationHandler"
    "autoreleasepool"
    "isKnownUniquelyReferenced"
    "numericCast"
    "unsafeDowncast"))

; Compiler directives and diagnostics are preprocessor lines, not macros.
(directive) @preproc

(diagnostic) @preproc

; Marks: MARK, TODO, and FIXME comments, which Xcode shows in bold and lists in the jump bar.
((comment) @comment.mark
  (#match? @comment.mark "^//+ ?(MARK|TODO|FIXME):"))
