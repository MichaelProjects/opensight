import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:opensight_analytics/src/collection.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() {
  testWidgets("Test collector", (widgetTester) async {
    SharedPreferences.setMockInitialValues({"io.opensight/isNewUser": true});
    MethodChannel channel = const MethodChannel("io.opensight_analytics");
    widgetTester.binding.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, (message) async {
      switch (message.method) {
        case "getPlatformVersion":
          return "30";
        case "displaysize":
          return "400x400";
        case "getOpensightConfig":
          return "";
        case "getLangCode":
          return "de";
        case "getAppVersion":
          return "1.0";
        case "getDeviceType":
          return "iPhone17";
        default:
          return null;
      }
    });

    Collection collection = await Collection.collect();
    assert(collection.country == "de");
    assert(collection.deviceType == "iPhone17");
    assert(collection.deviceSize == "400x400");
  });
  test("Test collector", () async {});
}
