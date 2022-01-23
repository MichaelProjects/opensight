/**
 * Copyright (c) Opensight
 *
 * This source code is licensed under the Apache license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @flow
 */
import axios from "axios";

export interface TransportInterface{
    baseUrl: string;
    token: string;
}

class Transport{
    instance: TransportInterface;

    constructor(props: TransportInterface){
        this.instance = props;
    }

    dispatchData(payload: JSON, path: string){
        let url = "${this.instance.baseUrl}/${path}";
        console.log(url);
    }
}
export default Transport;