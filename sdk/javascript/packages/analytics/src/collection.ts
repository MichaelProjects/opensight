import PresistenceLayer from "./presistence";
import { SessionState } from './model';


class Collection{
    collectedTime: number = new Date().getUTCDate();
    os: string;
    deviceSize: string;
    newUser: boolean;
    country: string;
    deviceType: string;
    version: string;

    private constructor(os: string, deviceSize: string, newUser: boolean, country: string, deviceType: string, version: string) {
        this.os = os;
        this.deviceSize = deviceSize;
        this.newUser = newUser;
        this.country = country;
        this.deviceType = deviceType;
        this.version = version;
    }

    static collect(version: string) : Collection {
        let player = new PresistenceLayer();
        // check if user is new
        let newUser = player.isUserNew();

        // if user is new, store new state
        if (newUser) {
            player.storeUserState(false);
        }
        let plattform: string | null = navigator.platform;
        
        if (plattform == undefined){
            plattform = navigator.userAgentData.platform;
        }
        let size = window.screen.width + "x" + window.screen.height;
        let deviceType: string | null = navigator.userAgentData.brands[-1]
        if (deviceType == null) {
            deviceType = navigator.userAgent.split(" ")[-1]
        }
        return new Collection(
            plattform!,
            size,
            newUser,
            navigator.language,
            deviceType!,
            version
        )
    }
    static checkAppActivity() : SessionState{
        
        return SessionState.background
    }
}

export default Collection;