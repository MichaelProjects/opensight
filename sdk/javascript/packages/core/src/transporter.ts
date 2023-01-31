import { Payload, Transporter } from "./models"



export async function dispatchData(transporter: Transporter, payload: Payload, path: string) {
    const xhr = new XMLHttpRequest();
    let auth = "Basic " + transporter.token
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.setRequestHeader("Authorization", auth);
    let uri = transporter.baseUrl + path;
    setTimeout(() => {
        xhr.open("POST", uri, true)
        let res = xhr.send(JSON.stringify(payload))
        return res;
    }, 5000);
}

export async function updateData(transporter: Transporter, payload: Payload, path: string) {
    const xhr = new XMLHttpRequest();
    let auth = "Basic " + transporter.token
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.setRequestHeader("Authorization", auth);

    let uri = transporter.baseUrl + path;
    setTimeout(() => {
        xhr.open("PATCH", uri, true)
        let res = xhr.send(JSON.stringify(payload))
        console.log(res);
    }, 5000);
}