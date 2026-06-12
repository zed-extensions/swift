import MyTarget
import Testing

@Suite
struct MyTarget_SwiftTestingTests {
    @Test
    func myActor() async {
        let myActor = MyActor(foo: 1, bar: "initial")

        let newFoo = await myActor.aMemberFunction(aNewFoo: 2)
        let currentAFoo = await myActor.aFoo
        #expect(newFoo == currentAFoo)
    }

    @Test
    func myClass() {
        let myClass = MyClass(foo: 1, bar: "initial")

        let newFoo = myClass.cMemberFunction(cNewFoo: 2)
        #expect(newFoo == myClass.cFoo)
    }

    @Test
    func myStruct() {
        var myStruct = MyStruct(foo: 1, bar: "initial")

        let newFoo = myStruct.sMemberFunction(sNewFoo: 2)
        #expect(newFoo == myStruct.sFoo)
    }

    @Test
    func myProtocol() {
        let myProtocolWitness = MyProtocolWitness(foo: 1, bar: "initial")

        let newFoo = myProtocolWitness.pMemberFunction(pNewFoo: 2)
        #expect(newFoo == myProtocolWitness.pFoo)
    }

    @Test
    func topLevel() {
        let newFoo = topLevelFunction(tNewFoo: topLevelFoo + 1)

        #expect(newFoo == topLevelFoo)
    }

    @Test
    func myEnum() {
        let myEnum = MyEnum.foo(1)

        let newFoo = myEnum.eMemberFunction(newFoo: 2)
        #expect(newFoo == MyEnum.foo(2))
    }
}

private class MyProtocolWitness: MyProtocol {
    var pFoo: Int
    let pBar: String

    func pMemberFunction(pNewFoo: Int) -> Int {
        self.pFoo = pNewFoo
        return pNewFoo
    }

    required init(foo: Int, bar: String) {
        self.pFoo = foo
        self.pBar = bar
    }
}
