public protocol MyProtocol {
    var pFoo: Int { get set }
    var pBar: String { get }

    mutating func pMemberFunction(pNewFoo: Int) -> Int

    init(foo: Int, bar: String)
}
