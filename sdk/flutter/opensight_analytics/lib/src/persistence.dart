import 'session.dart';
import 'package:shared_preferences/shared_preferences.dart';

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

  Future<void> storeSession(Session session) async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    int encodedData = session.store();
    pref.setInt(storeKey + "sessionData", encodedData);
  }

  Future<int?> loadSessions() async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    int? data = pref.getInt(storeKey + "sessionData");
    return data;
  }

  Future<void> removeSession() async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    pref.remove(storeKey + "sessionData");
  }
}
