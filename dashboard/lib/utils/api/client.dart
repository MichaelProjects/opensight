import 'package:dashboard/utils/api/http_client.dart';
import 'package:dashboard/utils/api/urls.dart';

class ApiClient {
  Future getApplications() async {
    Map response = await HttpClient().get(Urls.getAllApplications);
    if (response["error"] != true) {}
    return response["data"];
  }

  Future createApplication(Map payload) async {
    Map response = await HttpClient().post(Urls.createApplication, payload);
    if (response["error"] != true) {}
    return response["data"];
  }

  Future getAnalticsEntrys(String appId) async {
    Map response = await HttpClient().get(Urls.getAllAnalytics(appId));
    return response;
  }
  // analytics endpoints

  Future getUserHistory(String appId, int startFrame, int endFrame) async {
    Map response = await HttpClient()
        .get(Urls.getAnalyseUser(appId, startFrame, endFrame));
    return response;
  }

  Future getNewUsers(String appId, int startFrame, int endFrame) async {
    Map response = await HttpClient()
        .get(Urls.getAnalyseNewUser(appId, startFrame, endFrame));
    return response;
  }

  Future getDisplaySize(String appId, int startFrame, int endFrame) async {
    Map response = await HttpClient()
        .get(Urls.getAnalyseDisplaySize(appId, startFrame, endFrame));
    return response;
  }
}
