import 'dart:convert';
import 'dart:io';

/// [compressData] compresses the data and returns the result
compressData(Map data) {
  var rawData = utf8.encode(data.toString());
  var result = GZipCodec(level: 9).encode(rawData);
  //print("before: ${data.toString().length}");
  //print("after: ${result.length}");
  return result;
}
