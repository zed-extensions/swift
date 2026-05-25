public struct MyStruct {
    public var sFoo: Int
    public let sBar: String

    public mutating func sMemberFunction(sNewFoo: Int) -> Int {
        self.sFoo = sNewFoo
        return sNewFoo
    }

    public init(foo: Int, bar: String) {
        self.sFoo = foo
        self.sBar = bar
    }
}
