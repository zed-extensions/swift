;; Tags are named according to which testing library the test uses:
;; swift-testing-* = Swift Testing library
;; swift-xctest-* = XCTest library
;;
;; While the tasks defined in this extension don't care which library is used,
;; other tasks built by users might.

;; @Suite struct/class
(
  (class_declaration
    (modifiers
      (attribute
        (user_type
          (type_identifier) @run (#eq? @run "Suite")
        )
      )
    )
    name: (type_identifier) @_name @SWIFT_TEST_CLASS
  ) @_swift-testing-suite
  (#set! tag swift-testing-suite)
)

;; @Test top-level func
(
    (source_file
        (function_declaration
        (modifiers
            (attribute
            (user_type
                (type_identifier) @run (#eq? @run "Test")
            )
            )
        )
        name: (simple_identifier) @_name @SWIFT_TEST_FUNC
        ) @_swift-testing-bare-func
    )
    (#set! tag swift-testing-bare-func)
)

;; @Test within struct/class
(
  (class_declaration
    name: (type_identifier) @_name @SWIFT_TEST_CLASS
    body: (class_body
      (function_declaration
        (modifiers
            (attribute
                (user_type
                (type_identifier) @run (#eq? @run "Test")
                )
            )
            )
            name: (simple_identifier) @_name @SWIFT_TEST_FUNC
      )
    )
  ) @_swift-testing-member-func
  (#set! tag swift-testing-member-func)
)

;; XCTestCase subclass
(
  (class_declaration
    name: (type_identifier) @SWIFT_TEST_CLASS
    (inheritance_specifier
      inherits_from: (user_type
        (type_identifier) @run (#eq? @run "XCTestCase")
      )
    )
  ) @_swift-xctest-class
  (#set! tag swift-xctest-class)
)

;; Test function within XCTestCase
(
  (class_declaration
    name: (type_identifier) @SWIFT_TEST_CLASS
    (inheritance_specifier
      inherits_from: (user_type
        (type_identifier) @_superclass_name (#eq? @_superclass_name "XCTestCase")
      )
    )
    body: (class_body
      (function_declaration
        name: (simple_identifier) @_name @SWIFT_TEST_FUNC @run (#match? @run "^test")
      )
    )
  ) @_swift-xctest-func
  (#set! tag swift-xctest-func)
)


;; QuickSpec subclass
(
  (class_declaration
    name: (type_identifier) @_name
    (inheritance_specifier
      inherits_from: (user_type
        (type_identifier) @run (#eq? @run "QuickSpec")
      )
    )
  ) @_swift-test-quick-spec
  (#set! tag swift-test-quick-spec)
)

;; AsyncSpec subclass
(
  (class_declaration
    name: (type_identifier) @_name
    (inheritance_specifier
      inherits_from: (user_type
        (type_identifier) @run (#eq? @run "AsyncSpec")
      )
    )
  ) @_swift-test-async-spec
  (#set! tag swift-test-async-spec)
)