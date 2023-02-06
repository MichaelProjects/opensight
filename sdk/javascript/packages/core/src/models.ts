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

