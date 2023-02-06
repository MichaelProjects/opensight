
import OpensightCore from "../src/app";
import { Config } from "../src/models";

const CORE_URL = "127.0.0.1:28019"
const ANALYTIC_API = ""
const AUTH_TOKEN = ""
const APP_ID = ""
const APP_NAME = ""
const PACKAGE_NAME = ""

let config = new Config(ANALYTIC_API, APP_ID, AUTH_TOKEN, APP_NAME, PACKAGE_NAME)

test('Create opensight core singelton', () => {
    let app = OpensightCore.getInstance(config);
    expect(app).toBeDefined()
})

test('Dispatch Sample Data to core', () => {
    
})