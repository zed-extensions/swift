;; @Suite struct TestSuite
(
  (class_declaration
    (modifiers
      (attribute
        (user_type
          (type_identifier) @run (#eq? @run "Suite")
        )
      )
    )
    name: (type_identifier) @_name
  ) @_swift-test-suite
  (#set! tag swift-test-suite)
)

;; @Test test func
(
  (function_declaration
    (modifiers
      (attribute
        (user_type
          (type_identifier) @run (#eq? @run "Test")
        )
      )
    )
    name: (simple_identifier) @_name
  ) @_swift-test-test
  (#set! tag swift-test-test)
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

;; XCTestCase subclass
(
  (class_declaration
    name: (type_identifier) @_name
    (inheritance_specifier
      inherits_from: (user_type
        (type_identifier) @run (#eq? @run "XCTestCase")
      )
    )
  ) @_swift-test-test-case
  (#set! tag swift-test-test-case)
)

;; Test function within XCTestCase
(
  (class_declaration
    (inheritance_specifier
      inherits_from: (user_type
        (type_identifier) @test_class_name (#eq? @test_class_name "XCTestCase")
      )
    )
    body: (class_body
      (function_declaration
        name: (simple_identifier) @_name @run (#match? @run "^test")
      )
    )
  ) @_swift-test-func
  (#set! tag swift-test-func)
)
