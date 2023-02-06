import { Config } from "./models";
import Transporter from "./transporter";


class OpensightCore {
    private static instance: OpensightCore;
    
    config: Config;
    transporter: Transporter;
    private constructor(config: Config, transporter: Transporter) {
        this.config;
        this.transporter = transporter;
    }

    static getInstance(config?: Config) : OpensightCore {
        if (!config && !OpensightCore.instance){
            throw new Error("No config found, you need to configure your opensight app, with the first [initApp] call.");
            
        }
        if (!OpensightCore.instance){
            OpensightCore.instance = new OpensightCore(config!, new Transporter(config!.analyticsApi, config!.token));
        }
        return OpensightCore.instance;
    }
}
export default OpensightCore;