#[cfg(test)]
mod tests {
    use tree_sitter::{Parser, Query, QueryCursor};

    const RUNNABLES_QUERY: &str = include_str!("../languages/swift/runnables.scm");

    fn setup_parser() -> Parser {
        let mut parser = Parser::new();
        let language = unsafe {
            let ptr = tree_sitter_swift::LANGUAGE.into_raw()();
            std::mem::transmute(ptr)
        };
        parser.set_language(language).unwrap();
        parser
    }

    fn get_captures(source: &str, query_str: &str) -> Vec<(String, String, String)> {
        let mut parser = setup_parser();
        let tree = parser.parse(source, None).unwrap();
        let language = unsafe {
            let ptr = tree_sitter_swift::LANGUAGE.into_raw()();
            std::mem::transmute(ptr)
        };
        let query = Query::new(language, query_str).unwrap();
        let mut cursor = QueryCursor::new();

        let mut results = Vec::new();
        for match_ in cursor.matches(&query, tree.root_node(), source.as_bytes()) {
            let mut tag = String::new();
            let mut class_name = String::new();
            let mut func_name = String::new();

            for capture in match_.captures {
                let capture_name = &query.capture_names()[capture.index as usize];
                let text = capture.node.utf8_text(source.as_bytes()).unwrap();

                match capture_name.as_ref() {
                    "SWIFT_TEST_CLASS" => class_name = text.to_string(),
                    "SWIFT_TEST_FUNC" => func_name = text.to_string(),
                    _ => {}
                }
            }

            // Get the tag from pattern properties
            if let Some(props) = match_.pattern_index.checked_sub(0) {
                for prop in query.property_settings(props as usize) {
                    if prop.key.as_ref() == "tag" {
                        if let Some(value) = &prop.value {
                            tag = value.to_string();
                        }
                    }
                }
            }

            results.push((tag, class_name, func_name));
        }

        results
    }

    #[test]
    fn test_swift_testing_suite() {
        let source = r#"
@Suite
struct MyTestSuite {
    @Test func testSomething() {}
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        // Should capture the @Suite struct
        assert!(
            captures
                .iter()
                .any(|(tag, class, _)| tag == "swift-testing-suite" && class == "MyTestSuite"),
            "Expected to find swift-testing-suite tag for MyTestSuite, got: {:?}",
            captures
        );
    }

    #[test]
    fn test_swift_testing_suite_class() {
        let source = r#"
@Suite
class MyTestClass {
    @Test func testSomething() {}
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        assert!(
            captures
                .iter()
                .any(|(tag, class, _)| tag == "swift-testing-suite" && class == "MyTestClass"),
            "Expected to find swift-testing-suite tag for MyTestClass"
        );
    }

    #[test]
    fn test_swift_testing_bare_func() {
        let source = r#"
@Test func testTopLevelFunction() {
    // test code
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        assert!(
            captures
                .iter()
                .any(|(tag, _, func)| tag == "swift-testing-bare-func"
                    && func == "testTopLevelFunction"),
            "Expected to find swift-testing-bare-func tag for testTopLevelFunction"
        );
    }

    #[test]
    fn test_swift_testing_member_func() {
        let source = r#"
struct TestSuite {
    @Test func testMemberFunction() {
        // test code
    }
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        assert!(
            captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-testing-member-func"
                    && class == "TestSuite"
                    && func == "testMemberFunction"),
            "Expected to find swift-testing-member-func tag"
        );
    }

    #[test]
    fn test_xctest_class() {
        let source = r#"
import XCTest

class MyTests: XCTestCase {
    func testExample() {
        XCTAssertTrue(true)
    }
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        assert!(
            captures
                .iter()
                .any(|(tag, class, _)| tag == "swift-xctest-class" && class == "MyTests"),
            "Expected to find swift-xctest-class tag for MyTests"
        );
    }

    #[test]
    fn test_xctest_func() {
        let source = r#"
class MyTests: XCTestCase {
    func testExample() {
        XCTAssertTrue(true)
    }

    func testAnotherThing() {
        XCTAssertEqual(1, 1)
    }
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        assert!(
            captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-xctest-func"
                    && class == "MyTests"
                    && func == "testExample"),
            "Expected to find swift-xctest-func tag for testExample"
        );

        assert!(
            captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-xctest-func"
                    && class == "MyTests"
                    && func == "testAnotherThing"),
            "Expected to find swift-xctest-func tag for testAnotherThing"
        );
    }

    #[test]
    fn test_xctest_func_requires_test_prefix() {
        let source = r#"
class MyTests: XCTestCase {
    func setUp() {
        // setup code
    }

    func helperMethod() {
        // not a test
    }
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        // setUp and helperMethod should NOT be captured as they don't start with "test"
        assert!(
            !captures
                .iter()
                .any(|(tag, _, func)| tag == "swift-xctest-func" && func == "setUp"),
            "setUp should not be captured as a test function"
        );

        assert!(
            !captures
                .iter()
                .any(|(tag, _, func)| tag == "swift-xctest-func" && func == "helperMethod"),
            "helperMethod should not be captured as a test function"
        );
    }

    #[test]
    fn test_quick_spec() {
        let source = r#"
import Quick
import Nimble

class MyQuickSpec: QuickSpec {
    override func spec() {
        describe("something") {
            it("does something") {
                expect(true).to(beTrue())
            }
        }
    }
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        assert!(
            captures
                .iter()
                .any(|(tag, _, _)| tag == "swift-test-quick-spec"),
            "Expected to find swift-test-quick-spec tag"
        );
    }

    #[test]
    fn test_async_spec() {
        let source = r#"
import Quick
import Nimble

class MyAsyncSpec: AsyncSpec {
    override func spec() {
        describe("async operations") {
            it("does async work") {
                await someAsyncFunction()
            }
        }
    }
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        assert!(
            captures
                .iter()
                .any(|(tag, _, _)| tag == "swift-test-async-spec"),
            "Expected to find swift-test-async-spec tag"
        );
    }

    #[test]
    fn test_multiple_test_types_in_same_file() {
        let source = r#"
@Test func topLevelTest() {}

@Suite
struct SwiftTestingSuite {
    @Test func swiftTestingTest() {}
}

class XCTestSuite: XCTestCase {
    func testXCTest() {}
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        // Should find all three types of tests
        assert!(
            captures
                .iter()
                .any(|(tag, _, _)| tag == "swift-testing-bare-func"),
            "Should find bare test function"
        );

        assert!(
            captures
                .iter()
                .any(|(tag, _, _)| tag == "swift-testing-suite"),
            "Should find Swift Testing suite"
        );

        assert!(
            captures
                .iter()
                .any(|(tag, _, _)| tag == "swift-xctest-class"),
            "Should find XCTest class"
        );
    }

    #[test]
    fn test_nested_classes_not_confused() {
        let source = r#"
class OuterTests: XCTestCase {
    func testOuter() {}

    class Inner {
        func testInner() {}
    }
}
"#;

        let captures = get_captures(source, RUNNABLES_QUERY);

        // Only testOuter should be captured as it's in the XCTestCase subclass
        assert!(
            captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-xctest-func"
                    && class == "OuterTests"
                    && func == "testOuter"),
            "Should find testOuter in XCTestCase"
        );

        // testInner should NOT be captured as it's in a nested class that doesn't inherit from XCTestCase
        let inner_test_count = captures
            .iter()
            .filter(|(tag, _, func)| tag == "swift-xctest-func" && func == "testInner")
            .count();

        assert_eq!(inner_test_count, 0, "testInner should not be captured");
    }

    #[test]
    fn test_query_is_valid() {
        // This test ensures the query itself is syntactically valid
        let language = unsafe {
            let ptr = tree_sitter_swift::LANGUAGE.into_raw()();
            std::mem::transmute(ptr)
        };
        let result = Query::new(language, RUNNABLES_QUERY);

        assert!(
            result.is_ok(),
            "Runnables query should be valid: {:?}",
            result.err()
        );
    }
}
