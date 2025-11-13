#[cfg(test)]
mod tests {
    use std::sync::OnceLock;
    use streaming_iterator::StreamingIterator;
    use tree_sitter::{Language, Parser, Query, QueryCursor};

    const RUNNABLES_QUERY: &str = include_str!("../languages/swift/runnables.scm");

    // PERFORMANCE NOTE:
    // With tree-sitter 0.23 and tree-sitter-swift 0.7, query compilation is significantly
    // faster than the previous versions (0.20/0.6). We use OnceLock to cache both the
    // language and compiled query across all tests for optimal performance.

    fn get_language() -> &'static Language {
        static LANGUAGE: OnceLock<Language> = OnceLock::new();
        LANGUAGE.get_or_init(|| tree_sitter_swift::LANGUAGE.into())
    }

    fn get_query() -> &'static Query {
        static QUERY: OnceLock<Query> = OnceLock::new();
        QUERY.get_or_init(|| Query::new(&get_language(), RUNNABLES_QUERY).unwrap())
    }

    fn setup_parser() -> Parser {
        let mut parser = Parser::new();
        parser.set_language(&get_language()).unwrap();
        parser
    }

    fn get_captures(source: &str, query: &Query) -> Vec<(String, String, String)> {
        let mut parser = setup_parser();
        let tree = parser.parse(source, None).unwrap();
        let mut cursor = QueryCursor::new();

        let mut results = Vec::new();
        let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());

        while let Some(match_) = matches.next() {
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

        let captures = get_captures(source, get_query());

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

        let captures = get_captures(source, get_query());

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

        let captures = get_captures(source, get_query());

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

        let captures = get_captures(source, get_query());

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

        let captures = get_captures(source, get_query());

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

        let captures = get_captures(source, get_query());

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

        let captures = get_captures(source, get_query());

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

        let captures = get_captures(source, get_query());

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

        let captures = get_captures(source, get_query());

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

        let captures = get_captures(source, get_query());

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

        let captures = get_captures(source, get_query());

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
    fn test_xctest_indirect_subclass() {
        // This test documents a known limitation: tree-sitter queries work on syntax, not semantics,
        // so they cannot follow inheritance chains. Indirect subclasses (MyTests <- MyTestsBase <- XCTestCase)
        // are NOT detected. Only direct subclasses of XCTestCase are captured.
        //
        // This is a tree-sitter limitation - queries can only match patterns in the syntax tree,
        // they cannot perform semantic analysis to resolve inheritance hierarchies.

        let source = r#"
import XCTest

class MyTestsBase: XCTestCase {
    // Base class with common functionality
}

class MyTests: MyTestsBase {
    func testSomething() {
        XCTAssertTrue(true)
    }

    func testAnotherThing() {
        XCTAssertEqual(1, 1)
    }
}
"#;

        let captures = get_captures(source, get_query());

        // Should capture MyTestsBase class (direct subclass of XCTestCase)
        assert!(
            captures
                .iter()
                .any(|(tag, class, _)| tag == "swift-xctest-class" && class == "MyTestsBase"),
            "Expected to find swift-xctest-class tag for MyTestsBase (direct subclass of XCTestCase)"
        );

        // MyTests will NOT be captured as an XCTest class because tree-sitter can't follow
        // the inheritance chain. It only sees that MyTests inherits from MyTestsBase, not that
        // MyTestsBase inherits from XCTestCase.
        assert!(
            !captures
                .iter()
                .any(|(tag, class, _)| tag == "swift-xctest-class" && class == "MyTests"),
            "MyTests should NOT be captured because tree-sitter cannot follow indirect inheritance"
        );

        // Test functions in MyTests will NOT be captured because the class itself wasn't
        // recognized as an XCTestCase subclass
        assert!(
            !captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-xctest-func"
                    && class == "MyTests"
                    && func == "testSomething"),
            "testSomething should NOT be captured - tree-sitter can't follow indirect inheritance"
        );

        assert!(
            !captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-xctest-func"
                    && class == "MyTests"
                    && func == "testAnotherThing"),
            "testAnotherThing should NOT be captured - tree-sitter can't follow indirect inheritance"
        );
    }

    #[test]
    fn test_comment_annotation_for_indirect_subclass() {
        // Test whether we can use comment annotations to mark indirect XCTest subclasses
        // This explores a potential workaround for the tree-sitter limitation

        let source = r#"
// @XCTestClass
class MyTests: MyTestsBase {
    func testSomething() {
        XCTAssertTrue(true)
    }

    func testAnotherThing() {
        XCTAssertEqual(1, 1)
    }
}
"#;

        // First, verify comments are in the syntax tree
        let mut parser = setup_parser();
        let tree = parser.parse(source, None).unwrap();

        println!("\n=== Testing Comment Annotation Workaround ===");
        println!("Syntax tree:\n{}", tree.root_node().to_sexp());

        // Test 1: Match comment + class pattern
        let class_query_str = r#"
(source_file
  (comment) @test_marker (#match? @test_marker ".*@XCTestClass.*")
  (class_declaration
    name: (type_identifier) @SWIFT_TEST_CLASS
  )
)
        "#;

        println!("\n--- Test 1: Matching class with @XCTestClass comment ---");
        match Query::new(&get_language(), class_query_str) {
            Ok(test_query) => {
                let mut cursor = QueryCursor::new();
                let mut matches = cursor.matches(&test_query, tree.root_node(), source.as_bytes());

                let mut found_match = false;
                while let Some(match_) = matches.next() {
                    found_match = true;
                    for capture in match_.captures {
                        let capture_name = &test_query.capture_names()[capture.index as usize];
                        let text = capture.node.utf8_text(source.as_bytes()).unwrap();
                        println!("Captured {}: '{}'", capture_name, text);
                    }
                }

                if found_match {
                    println!("✓ SUCCESS: Comment annotation for class works!");
                } else {
                    println!(
                        "✗ Pattern didn't match - comment + class pattern may need adjustment"
                    );
                }

                assert!(
                    found_match,
                    "Expected to match comment annotation pattern for class"
                );
            }
            Err(e) => {
                panic!(
                    "Failed to compile class query for comment annotations: {}",
                    e
                );
            }
        }

        // Test 2: Match test functions within annotated class
        let func_query_str = r#"
(source_file
  (comment) @test_marker (#match? @test_marker ".*@XCTestClass.*")
  (class_declaration
    name: (type_identifier) @SWIFT_TEST_CLASS
    body: (class_body
      (function_declaration
        name: (simple_identifier) @SWIFT_TEST_FUNC @run (#match? @run "^test")
      )
    )
  )
)
        "#;

        println!("\n--- Test 2: Matching test functions in annotated class ---");
        match Query::new(&get_language(), func_query_str) {
            Ok(test_query) => {
                let mut cursor = QueryCursor::new();
                let mut matches = cursor.matches(&test_query, tree.root_node(), source.as_bytes());

                let mut test_funcs = Vec::new();
                while let Some(match_) = matches.next() {
                    for capture in match_.captures {
                        let capture_name = &test_query.capture_names()[capture.index as usize];
                        let text = capture.node.utf8_text(source.as_bytes()).unwrap();
                        println!("Captured {}: '{}'", capture_name, text);
                        if *capture_name == "SWIFT_TEST_FUNC" {
                            test_funcs.push(text.to_string());
                        }
                    }
                }

                if test_funcs.len() > 0 {
                    println!(
                        "✓ SUCCESS: Found {} test functions in annotated class",
                        test_funcs.len()
                    );
                    println!("Test functions: {:?}", test_funcs);
                } else {
                    println!("✗ No test functions captured");
                }

                assert!(
                    test_funcs.contains(&"testSomething".to_string()),
                    "Expected to find testSomething"
                );
                assert!(
                    test_funcs.contains(&"testAnotherThing".to_string()),
                    "Expected to find testAnotherThing"
                );

                println!("\n=== CONCLUSION ===");
                println!("✓ Comment annotations WORK as a workaround!");
                println!("Users can add '// @XCTestClass' before indirect XCTest subclasses,");
                println!("and we can update runnables.scm to detect them.");
            }
            Err(e) => {
                panic!(
                    "Failed to compile function query for comment annotations: {}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_query_is_valid() {
        // This test ensures the query itself is syntactically valid
        // The query is compiled on first access via get_query()
        let query = get_query();

        // If we got here, the query compiled successfully
        assert!(
            query.capture_names().len() > 0,
            "Query should have capture names"
        );
    }
}
