import 'dart:convert';

import 'session.dart';
import 'package:shared_preferences/shared_preferences.dart';

//todo verify user is logged in first time today.
//todo use app name in the storeKey, so if multiple apps are using opensight there is no data overwirte
//todo resume session, if there was a 5 min pause.
//todo clear way to store the session data.

class PresistencesLayer {
  final String storeKey = "io.opensight/";

  /// check if an exists for the given key below.
  /// if so it returns the needed value.
  Future<bool> isNewUser() async {
    bool isNew = true;
    SharedPreferences pref = await SharedPreferences.getInstance();
    var result = pref.getBool(storeKey + "isNewUser");
    if (result != null) {
      isNew = result;
    }
    return isNew;
  }

  Future<void> storeUserState(bool isNew) async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    pref.setBool(storeKey + "isNewUser", isNew);
  }

  /// example payload:
  /// {
  ///   "session_id": id,
  ///   "session_start": start,
  ///   "session_pause": pause,
  /// }
  Future<void> storeSession(Map<String, dynamic> session) async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    pref.setString(storeKey + "sessionData", jsonEncode(session));
  }

  Future<bool?> isFirstSessionToday() async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    bool? result = pref.getBool(storeKey + "isFirstSessionToday");
    return result;
  }

  Future<String?> loadSessions() async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    String? data = pref.getString(storeKey + "sessionData");
    return data;
  }

  Future<void> removeSession() async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    pref.remove(storeKey + "sessionData");
  }
}
