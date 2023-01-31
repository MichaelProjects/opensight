export class Config {
    analyticsApi: string;
    appId: string;
    token: string;
    name: string;
    packageName: string;

    constructor(analyticsApi: string, appId: string, token: string, name: string, packageName: string) {
        this.analyticsApi = analyticsApi;
        this.appId = appId;
        this.token = token;
        this.name = name;
        this.packageName = packageName;
    }
}

export class Transporter {
    baseUrl: string;
    token: string;
    constructor(baseUrl: string, token: string) {
        this.baseUrl = baseUrl;
        this.token = token;
    }
}

export class OpensightCore {
    config: Config;
    transporter: Transporter;
    private constructor(config: Config, transporter: Transporter) {
        this.config;
        this.transporter = transporter;
    }
    static initApp(config: Config) : OpensightCore {
        return new OpensightCore(config, new Transporter(config.analyticsApi, config.token))
    }
}

export interface Payload{
    
}