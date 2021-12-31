import 'package:dashboard/utils/api/http_client.dart';
import 'package:dashboard/utils/api/urls.dart';

class ApiClient {
  Future getApplications() async {
    Map response = await HttpClient().get(Urls.getAllApplications);
    print(response);
    if (response["error"] != true) {}
    return response["data"];
  }
}
