import OpensightCore from "./app";

class Transporter {
    baseUrl: string;
    token: string;
    
    constructor(baseUrl: string, token: string) {
        this.baseUrl = baseUrl;
        this.token = token;
    }
    public async dispatchData(payload, path: string) {
    const xhr = new XMLHttpRequest();
        let auth = "Basic " + OpensightCore.getInstance().transporter.token
        xhr.setRequestHeader("Content-Type", "application/json");
        xhr.setRequestHeader("Authorization", auth);
        let uri = OpensightCore.getInstance().transporter.baseUrl + path;
        setTimeout(() => {
            xhr.open("POST", uri, true)
            let res = xhr.send(JSON.stringify(payload))
            return res;
        }, 5000);
    }
    
    public async updateData(payload, path: string) {
        const xhr = new XMLHttpRequest();
        let auth = "Basic " + OpensightCore.getInstance().transporter.token
        xhr.setRequestHeader("Content-Type", "application/json");
        xhr.setRequestHeader("Authorization", auth);
    
        let uri = OpensightCore.getInstance().transporter.baseUrl + path;
        setTimeout(() => {
            xhr.open("PATCH", uri, true)
            let res = xhr.send(JSON.stringify(payload))
            console.log(res);
        }, 5000);
    }
}

export default Transporter;