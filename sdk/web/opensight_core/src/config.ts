/**
 * Copyright (c) Opensight
 *
 * This source code is licensed under the Apache license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @flow
 */
export interface Config{
    analyticsApi: string;
    appId: string;
    token: string;
    name: string;
    packageName: string;
}

export default function ConfigFromJson(data: string): Config | undefined{
    try{
        let config: Config = JSON.parse(data);
        return config
    }catch(e){
        console.log("Error parsing config file: " + e);
    }
}

export function configDev(){
    let config: Config = {analyticsApi: "localhost:28018", appId: "123", token: "123", name: "test", packageName: "io.opensight"};
    return config;
}