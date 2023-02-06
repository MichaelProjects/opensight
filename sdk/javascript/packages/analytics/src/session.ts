import OpensightCore from "../../core/src/app";
import PresistenceLayer from "./presistence";

const TRACKINTERVALL = 2;

enum SessionState { started, background, resumed }

export class Session {
    id: string;
    sessionState: SessionState = SessionState.started;
    resumedLength = 0;
    startTime: number = new Date().getUTCDate();
    isFirstToday: boolean = false;


    public sendUpdate(length: number, sessionId: string) {
        let payload = {
            "session_id": sessionId,
            "length": length
        }
        let instance = OpensightCore.getInstance();
        instance.transporter.dispatchData(payload, "/analytic/v1/" + instance.config.appId + "/session");
    }
}

void function tracking(sessionId: string){
    let session = new Session();
    let lastLogin = new PresistenceLayer().getLastLoginDate();

    if (lastLogin != null) {
        // todo implemneted that correctly
        session.isFirstToday = checkDate(lastLogin);
    }
    setTimeout(trackingLoop, TRACKINTERVALL * 1000);
}

function trackingLoop(){

}

function checkDate(toCheck: number) : boolean {
    let now = new Date().getUTCMilliseconds();
    
}