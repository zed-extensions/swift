public enum MyEnum: Equatable {
    case foo(Int)
    case bar(String)

    public func eMemberFunction(newFoo: Int) -> Self? {
        switch self {
            case .foo:
            return .foo(newFoo)
            case .bar:
            return nil
        }
    }
}
