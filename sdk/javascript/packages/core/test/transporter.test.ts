import { Transporter } from "../src/models";
import { dispatchData } from "../src/transporter";


let tp = new Transporter("", "")

test("Test DispatchData", () => {
    dispatchData().then(data => {
        console.log(data)
    })})