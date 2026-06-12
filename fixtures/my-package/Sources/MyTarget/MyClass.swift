public class MyClass {
    public var cFoo: Int
    public let cBar: String

    public func cMemberFunction(cNewFoo: Int) -> Int {
        self.cFoo = cNewFoo
        return cNewFoo
    }

    public init(foo: Int, bar: String) {
        self.cFoo = foo
        self.cBar = bar
    }

    deinit {
        self.cFoo = 0
    }
}
