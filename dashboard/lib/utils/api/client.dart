import 'package:dashboard/utils/api/http_client.dart';
import 'package:dashboard/utils/api/urls.dart';

class ApiClient {
  static Map<String, String> authHeader = {"Authorization": "Bearer 123123"};

  Future getApplications() async {
    Map response = await HttpClient().get(Urls.getAllApplications);
    if (response["error"] != true) {}
    return response["data"];
  }

  Future getApplication(String appId) async {
    Map response = await HttpClient().get(Urls.getApplication(appId));
    if (response["error"] != true) {}
    return response["data"];
  }

  Future createApplication(Map payload) async {
    Map response = await HttpClient().post(Urls.createApplication, payload);
    if (response["error"] != true) {}
    return response["data"];
  }

  Future getAnalticsEntrys(String appId, int limit, int start, int end) async {
    Map response =
        await HttpClient().get(Urls.getAllAnalytics(appId, limit, start, end));
    return response;
  }
  // analytics endpoints

  Future getUserHistory(String appId, int startFrame, int endFrame) async {
    Map response = await HttpClient()
        .get(Urls.getAnalyseUser(appId, startFrame, endFrame), authHeader);
    return response;
  }

  Future getNewUsers(String appId, int startFrame, int endFrame) async {
    Map response = await HttpClient()
        .get(Urls.getAnalyseNewUser(appId, startFrame, endFrame), authHeader);
    return response;
  }

  Future getDisplaySize(String appId, int startFrame, int endFrame) async {
    Map response = await HttpClient().get(
        Urls.getAnalyseDisplaySize(appId, startFrame, endFrame), authHeader);
    return response;
  }

  Future getSessionCount(String appId, int startFrame, int endFrame) async {
    Map response = await HttpClient()
        .get(Urls.getSessionCount(appId, startFrame, endFrame), authHeader);
    return response;
  }

  Future getSessionLengthHistory(
      String appId, int startFrame, int endFrame) async {
    Map response = await HttpClient().get(
        Urls.getSessionLengthHistory(appId, startFrame, endFrame), authHeader);
    return response;
  }

  Future getAppVersion(String appId, int startFrame, int endFrame) async {
    Map response = await HttpClient()
        .get(Urls.getAppVersion(appId, startFrame, endFrame), authHeader);
    return response;
  }
}
