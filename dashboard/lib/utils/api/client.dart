import 'package:dashboard/utils/api/http_client.dart';
import 'package:dashboard/utils/api/urls.dart';

class ApiClient {
  Future getApplications() async {
    Map response = await HttpClient().get(Urls.getAllApplications);
    if (response["error"] != true) {}
    return response["data"];
  }

  Future getAnalticsEntrys(String appId) async {
    Map response = await HttpClient().get(Urls.getAllAnalytics(appId));
    return response;
  }
}
