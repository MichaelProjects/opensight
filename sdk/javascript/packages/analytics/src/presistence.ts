import { Session } from "./session";

class PresistenceLayer {
    storeKey = "io.opensight/analytics/"

    public isUserNew(): boolean {
        let isNew = false;
        let res = window.localStorage.getItem(this.storeKey + "isNewUser");
        if (res == null || res == undefined) {
            isNew = true;
        }
        return isNew;
    }
    public storeUserState(isNew: boolean) {
        window.localStorage.setItem(this.storeKey + "isNewUser", isNew.toString());
    }
    public storeSession(session) {
        window.localStorage.setItem(this.storeKey + "sessionData", JSON.stringify(session));
    }
    public storeLastLogin() {
        window.localStorage.setItem(this.storeKey + "lastLogin", new Date().getUTCMilliseconds().toString());
    }
    public getLastLoginDate(): number {
        let time = window.localStorage.getItem(this.storeKey + "lastLogin")
        if (time != null) {
            return parseInt(time);
        }
        return 0;
    }
    public loadSessions() : Session | null {
        let res = window.localStorage.getItem(this.storeKey + "sessionData");
        let session = Object.assign(new Session(), res)
        return session;
    }
    public removeSession() {
        window.localStorage.removeItem(this.storeKey + "sessionData");
    }
}

export default PresistenceLayer;