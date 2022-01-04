import 'conf.dart';

class Opensight {
  /// [OpensightAnalytics] is the public api for this plugin, it is simple to implement just call [OpensightSDK.initApp] and the function takes your config.
  ///
  /// example:
  ///
  /// Map config_data = {add your config data here}
  /// Opensight_Analytics.initApp(config_data)
  static Config configObject = Config.def();

  static Future<void> initApp(Map config) async {
    configObject = Config.fromJson(config);
  }
}
