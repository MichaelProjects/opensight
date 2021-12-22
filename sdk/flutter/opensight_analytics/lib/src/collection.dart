import 'persistence.dart';
import 'nativlayer.dart';

class Collection {
  /// [Collection] contains all the needed data that will be sendet to the analtics_api.
  final DateTime collectedTime = DateTime.now().toUtc();
  String os;
  String deviceSize;
  bool newUser;
  String country;
  int lastSession;
  String deviceType;
  String version;

  Collection(
      {required this.os,
      required this.deviceSize,
      required this.newUser,
      required this.country,
      required this.lastSession,
      required this.deviceType,
      required this.version});

  /// [collect] calls all needed functions to get the needed data.
  static collect() async {
    bool newUser = await PresistencesLayer().isNewUser();
    if (newUser == true) {
      PresistencesLayer().storeUserState(false);
    }
    return Collection(
        os: await NativeLayer.determineOs(),
        deviceSize: await NativeLayer.determineDisplaysize(),
        newUser: newUser,
        country: await NativeLayer.determineLangCode(),
        lastSession: await loadSessionData(),
        deviceType: await NativeLayer.determineDeviceType(),
        version: await NativeLayer.determineAppVersion());
  }

  /// takes the time of the [Collection] element and parses
  /// into the needed time format
  ///
  /// exmaple:
  /// 2021-08-08T07:14:00
  getTimeInFormat(DateTime time) {
    var parts = time.toString().split(" ");
    return "${parts[0]}T${parts[1].split(".")[0]}";
  }

  /// parse the data into the excpeted structure for the analytics_api
  Map<String, dynamic> prepareToSend() {
    Map<String, dynamic> data = {
      "creation_time": getTimeInFormat(collectedTime),
      "os": os,
      "device_size": deviceSize,
      "new_user": newUser,
      "country": country,
      "last_session": lastSession,
      "device_type": deviceType,
      "version": version
    };
    return data;
  }
}

Future<int> loadSessionData() async {
  int? data = await PresistencesLayer().loadSessions();
  if (data != null) {
    return data;
  } else {
    return 0;
  }
}
