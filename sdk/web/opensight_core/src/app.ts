/**
 * Copyright (c) Opensight
 *
 * This source code is licensed under the Apache license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @flow
 */
import ConfigFromJson, { Config } from "./config";
import Transport from "./transport";

class OpensightCore{
    config: Config;
    transporter: Transport;

    constructor(config: Config, transporter: Transport){
    this.config = config;
    this.transporter = transporter;
    }
    static initApp(config: JSON){
        let configString = JSON.stringify(config);
        let theConfig: Config | undefined = ConfigFromJson(configString);
        console.log(theConfig)
    }
}
export default OpensightCore;