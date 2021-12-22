import 'dart:convert';
import 'persistence.dart';
import 'conf.dart';
import 'utils.dart';
import 'package:http/http.dart' as http;

class AnalyticsApiClient {
  Future dispatchData(Map<String, dynamic> payload, Config config) async {
    compressData(payload);
    try {
      Uri uri =
          Uri.parse("${config.analyticsApi}/analytic/${config.appId}/session");
      Map<String, String> headers = {
        "Content-Type": "application/json",
        "Authorization": "Bearer ${config.token}"
      };
      var response =
          await http.post(uri, body: jsonEncode(payload), headers: headers);
      if (response.statusCode == 202) {
        await PresistencesLayer().removeSession();
      }
      if (response.statusCode != 202) {
        throw Exception(
            "Could not connect to Analytics-Api, check your Config");
      }
    } catch (e) {
      throw Exception("Failed to connect to server");
    }
  }
}
