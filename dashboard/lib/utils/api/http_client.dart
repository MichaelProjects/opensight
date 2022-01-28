import 'dart:convert';
import 'dart:io';

import 'package:http/http.dart' as http;

Map buildResponse(bool error, String message, Map data) {
  //! Builds an internal response object, this is a structed way to pass data
  return {"error": error, "message": message, "data": data};
}

class HttpClient {
  Duration standartTimeOut = const Duration(seconds: 8);

  Future<Map> get(Uri uri) async {
    //! get request wrapper, returns a map with the response details
    http.Response response = await http.get(uri).timeout(standartTimeOut);
    if (response.statusCode == 200) {
      return buildResponse(false, "", jsonDecode(response.body));
    }
    return buildResponse(true, "", {});
  }

  Future<Map> post(Uri uri, Map body) async {
    //! post request wrapper, returns a map with the response details
    var client = HttpClient();
    HttpClientRequest request = await client.post(uri, body: body);
    request.headers.contentType =
        ContentType('application', 'json', charset: 'utf-8');

    var response = await http
        .post(
          uri,
          body: body,
        )
        .timeout(standartTimeOut);
    if (response.statusCode == 200) {
      return buildResponse(false, "", jsonDecode(response.body));
    }
    return buildResponse(true, "", {});
  }

  Future<Map> delete(Uri uri) async {
    //! delete request wrapper, returns a map with the response details
    var response = await http.delete(uri).timeout(standartTimeOut);
    if (response.statusCode == 200) {
      return buildResponse(false, "", jsonDecode(response.body));
    }
    return buildResponse(true, "", {});
  }
}
