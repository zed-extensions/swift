//! # Runnables Tests
//!
//! Unit tests for the Swift runnables tree-sitter query file (`languages/swift/runnables.scm`).
//!
//! ## Overview
//!
//! The runnables query is used by Zed to identify test functions and test classes in Swift code.
//! The tests validate that the query correctly captures:
//!
//! 1. **Swift Testing Framework** (modern Swift 6+ testing)
//!    - `@Suite` annotations on structs and classes
//!    - `@Test` annotations on top-level functions
//!    - `@Test` annotations on member functions within test suites
//!
//! 2. **XCTest Framework** (traditional Swift testing)
//!    - Classes that inherit from `XCTestCase`
//!    - Classes marked with `@XCTestCase` comment annotation (for indirect subclasses)
//!    - Test methods within XCTest classes (must start with `test` prefix)
//!
//! 3. **Quick/Nimble Framework** (BDD-style testing)
//!    - `QuickSpec` subclasses
//!    - `AsyncSpec` subclasses
//!
//! ## Running Tests
//!
//! ```bash
//! # Run all tests
//! cargo test --lib
//!
//! # Run only the runnables tests
//! cargo test --lib runnables_test
//! ```
//!
//! ## Test Structure
//!
//! Each test:
//! 1. Defines a Swift code snippet containing test code
//! 2. Parses the code using tree-sitter-swift
//! 3. Runs the runnables query against the parsed tree
//! 4. Validates that the correct captures are returned with the appropriate tags
//!
//! ## Tags
//!
//! The query assigns tags to different types of test definitions:
//!
//! - `swift-testing-suite` - A struct/class with `@Suite` annotation
//! - `swift-testing-bare-func` - A top-level function with `@Test` annotation
//! - `swift-testing-member-func` - A member function with `@Test` annotation within a suite
//! - `swift-xctest-class` - A class that inherits from `XCTestCase` or is marked with `@XCTestCase` comment
//! - `swift-xctest-func` - A test method within an XCTest class
//! - `swift-test-quick-spec` - A QuickSpec subclass
//! - `swift-test-async-spec` - An AsyncSpec subclass
//!
//! ## Adding New Tests
//!
//! When adding support for new test frameworks or patterns:
//!
//! 1. Add a new test function in this module
//! 2. Create a Swift code snippet that demonstrates the pattern
//! 3. Use `get_captures()` to run the query
//! 4. Assert that the expected tags and names are captured
//!
//! Example:
//!
//! ```rust
//! #[test]
//! fn test_new_framework() {
//!     let source = r#"
//! // Your Swift test code here
//! "#;
//!
//!     let captures = get_captures(source, get_query());
//!
//!     assert!(
//!         captures
//!             .iter()
//!             .any(|(tag, class, func)| tag == "expected-tag" && class == "ExpectedClass"),
//!         "Expected to find the test pattern"
//!     );
//! }
//! ```

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
        // are NOT detected WITHOUT annotation.
        //
        // However, users can use the // @XCTestClass comment annotation as a workaround.

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
    fn test_xctest_indirect_subclass_with_annotation() {
        // This test verifies that the // @XCTestClass comment annotation workaround
        // allows indirect XCTest subclasses to be detected by the runnables.scm query.

        let source = r#"
import XCTest

class MyTestsBase: XCTestCase {
    // Base class with common functionality
}

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

        let captures = get_captures(source, get_query());

        // Should capture MyTestsBase class (direct subclass of XCTestCase)
        assert!(
            captures
                .iter()
                .any(|(tag, class, _)| tag == "swift-xctest-class" && class == "MyTestsBase"),
            "Expected to find swift-xctest-class tag for MyTestsBase (direct subclass of XCTestCase)"
        );

        // MyTests SHOULD now be captured because of the @XCTestClass annotation
        assert!(
            captures
                .iter()
                .any(|(tag, class, _)| tag == "swift-xctest-class" && class == "MyTests"),
            "Expected to find swift-xctest-class tag for MyTests with @XCTestClass annotation"
        );

        // Test functions in MyTests SHOULD be captured with the annotation
        assert!(
            captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-xctest-func"
                    && class == "MyTests"
                    && func == "testSomething"),
            "Expected to find testSomething in annotated indirect subclass"
        );

        assert!(
            captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-xctest-func"
                    && class == "MyTests"
                    && func == "testAnotherThing"),
            "Expected to find testAnotherThing in annotated indirect subclass"
        );
    }

    #[test]
    fn test_comment_annotation_for_indirect_subclass() {
        // This test demonstrates the complete workaround for indirect XCTest subclasses.
        // It shows a realistic scenario with a base test class and an annotated subclass.

        let source = r#"
import XCTest

// Base test class with shared setup/teardown
class BaseTestCase: XCTestCase {
    var sharedResource: String!

    override func setUp() {
        super.setUp()
        sharedResource = "test"
    }
}

// @XCTestClass
class MyFeatureTests: BaseTestCase {
    func testFeatureA() {
        XCTAssertNotNil(sharedResource)
    }

    func testFeatureB() {
        XCTAssertEqual(sharedResource, "test")
    }

    func helperMethod() {
        // Not a test
    }
}

// Another annotated indirect subclass
// @XCTestClass
class MyOtherTests: BaseTestCase {
    func testAnotherFeature() {
        XCTAssertTrue(true)
    }
}
"#;

        let captures = get_captures(source, get_query());

        // Should capture the base class (direct XCTestCase subclass)
        assert!(
            captures
                .iter()
                .any(|(tag, class, _)| tag == "swift-xctest-class" && class == "BaseTestCase"),
            "Expected to find BaseTestCase"
        );

        // Should capture MyFeatureTests class (annotated)
        assert!(
            captures
                .iter()
                .any(|(tag, class, _)| tag == "swift-xctest-class" && class == "MyFeatureTests"),
            "Expected to find MyFeatureTests with @XCTestClass annotation"
        );

        // Should capture test functions in MyFeatureTests
        assert!(
            captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-xctest-func"
                    && class == "MyFeatureTests"
                    && func == "testFeatureA"),
            "Expected to find testFeatureA"
        );

        assert!(
            captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-xctest-func"
                    && class == "MyFeatureTests"
                    && func == "testFeatureB"),
            "Expected to find testFeatureB"
        );

        // helperMethod should NOT be captured (doesn't start with "test")
        assert!(
            !captures
                .iter()
                .any(|(tag, _, func)| tag == "swift-xctest-func" && func == "helperMethod"),
            "helperMethod should not be captured"
        );

        // Should capture MyOtherTests class (annotated)
        assert!(
            captures
                .iter()
                .any(|(tag, class, _)| tag == "swift-xctest-class" && class == "MyOtherTests"),
            "Expected to find MyOtherTests with @XCTestClass annotation"
        );

        // Should capture test in MyOtherTests
        assert!(
            captures
                .iter()
                .any(|(tag, class, func)| tag == "swift-xctest-func"
                    && class == "MyOtherTests"
                    && func == "testAnotherFeature"),
            "Expected to find testAnotherFeature"
        );
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
