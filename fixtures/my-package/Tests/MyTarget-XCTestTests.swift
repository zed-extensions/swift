import MyTarget
import XCTest

class MyTarget_XCTestTests: XCTestCase {
    func testMyActor() async {
        let myActor = MyActor(foo: 1, bar: "initial")

        let newFoo = await myActor.aMemberFunction(aNewFoo: 2)
        let currentAFoo = await myActor.aFoo
        XCTAssertTrue(newFoo == currentAFoo)
    }

    func testMyClass() {
        let myClass = MyClass(foo: 1, bar: "initial")

        let newFoo = myClass.cMemberFunction(cNewFoo: 2)
        XCTAssertTrue(newFoo == myClass.cFoo)
    }

    func testMyStruct() {
        var myStruct = MyStruct(foo: 1, bar: "initial")

        let newFoo = myStruct.sMemberFunction(sNewFoo: 2)
        XCTAssertTrue(newFoo == myStruct.sFoo)
    }

    func testMyProtocol() {
        let myProtocolWitness = MyProtocolWitness(foo: 1, bar: "initial")

        let newFoo = myProtocolWitness.pMemberFunction(pNewFoo: 2)
        XCTAssertTrue(newFoo == myProtocolWitness.pFoo)
    }

    func testTopLevel() {
        let newFoo = topLevelFunction(tNewFoo: topLevelFoo + 1)

        XCTAssertTrue(newFoo == topLevelFoo)
    }

    func testMyEnum() {
        let myEnum = MyEnum.foo(1)

        let newFoo = myEnum.eMemberFunction(newFoo: 2)
        XCTAssertTrue(newFoo == MyEnum.foo(2))
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
