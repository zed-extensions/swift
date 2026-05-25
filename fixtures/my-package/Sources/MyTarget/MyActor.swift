public actor MyActor {
    public var aFoo: Int
    public let aBar: String

    public func aMemberFunction(aNewFoo: Int) -> Int {
        self.aFoo = aNewFoo
        return aNewFoo
    }

    public init(foo: Int, bar: String) {
        self.aFoo = foo
        self.aBar = bar
    }

    deinit {
        self.aFoo = 0
    }
}
