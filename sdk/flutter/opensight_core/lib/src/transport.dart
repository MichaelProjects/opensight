import 'dart:convert';
import 'utils.dart';
import 'package:http/http.dart' as http;

class TransportClient {
  final String _baseUrl;
  final String token;
  TransportClient(this._baseUrl, this.token);
  Future<Map> dispatchData(Map<String, dynamic> payload, String path) async {
    compressData(payload);
    try {
      Uri uri = Uri.parse("$_baseUrl$path");
      Map<String, String> headers = {
        "Content-Type": "application/json",
        "Authorization": "Bearer $token"
      };
      var response =
          await http.post(uri, body: jsonEncode(payload), headers: headers);
      if (response.statusCode == 200) {
        return jsonDecode(response.body);
      }
      if (response.statusCode != 202) {
        throw Exception(
            "Could not connect to Analytics-Api, check your Config");
      }
    } catch (e) {
      throw Exception("Failed to connect to server");
    }
    return {};
  }

  Future updateData(Map<String, dynamic> payload, String path) async {
    compressData(payload);
    try {
      Uri uri = Uri.parse("$_baseUrl$path");
      Map<String, String> headers = {
        "Content-Type": "application/json",
        "Authorization": "Bearer $token"
      };
      var response =
          await http.patch(uri, body: jsonEncode(payload), headers: headers);
      if (response.statusCode == 200) {}
      if (response.statusCode != 202) {
        throw Exception(
            "Could not connect to Analytics-Api, check your Config");
      }
    } catch (e) {
      throw Exception("Failed to connect to server");
    }
  }
}
