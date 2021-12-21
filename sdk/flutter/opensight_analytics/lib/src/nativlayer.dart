import 'dart:io';

import 'package:flutter/services.dart';
import 'dart:developer' as developer;

class NativeLayer {
  /// with [NativeLayer] opensight calls native code, or functions (written in swift, kotlin).
  static const platform = MethodChannel("io.opensight_analytics");

  static Future<String> determineOs() async {
    String os = "Unkown";
    if (Platform.isIOS) {
      os = "IOS";
    }
    if (Platform.isAndroid) {
      os = "Android";
    }
    return os;
  }

  static Future<String> determineOsVersion() async {
    final String result = await platform.invokeMethod("getPlatformVersion");
    return result;
  }

  static Future<String> determineDisplaysize() async {
    String displaySize = "None";
    try {
      final String result = await platform.invokeMethod("displaysize");
      displaySize = result;
    } catch (e) {
      developer.log(e.toString());
    }
    return displaySize;
  }

  static Future<String> determineLangCode() async {
    String langCode = "None";
    try {
      final String result = await platform.invokeMethod("getLangCode");
      langCode = result;
    } catch (e) {
      developer.log(e.toString());
    }
    return langCode;
  }

  static Future<String> determineDeviceType() async {
    String langCode = "None";
    try {
      final String result = await platform.invokeMethod("getDeviceType");
      langCode = result;
    } catch (e) {
      developer.log(e.toString());
    }
    return langCode;
  }

  static Future<String> determineAppVersion() async {
    String appVersion = "None";
    try {
      final String result = await platform.invokeMethod("getAppVersion");
      appVersion = result;
    } catch (e) {
      developer.log(e.toString());
    }
    return appVersion;
  }

  static Future<String> getConfig() async {
    String displaySize = "";
    try {
      final String result = await platform.invokeMethod("getOpensightConfig");
      displaySize = result;
    } catch (e) {
      developer.log(e.toString());
    }
    return displaySize;
  }
}
