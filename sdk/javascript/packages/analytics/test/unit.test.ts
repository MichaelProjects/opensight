import Collection from "../src/collection"



test('test collector', () => {
    let version = "0.1.0";
    let x = Collection.collect(version);
    console.log(x);
    expect(x).toBeDefined();
})