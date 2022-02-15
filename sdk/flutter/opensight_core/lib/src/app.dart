import 'package:opensight_core/src/transport.dart';

import 'conf.dart';

class OpensightCore {
  Config appDetails;
  TransportClient transport;
  OpensightCore({
    required this.appDetails,
    required this.transport,
  });

  /// [OpensightCore] is the public api for this plugin, it is simple to implement just call [OpensightCore.initApp] and the function takes your config.
  ///
  /// example:
  ///
  /// Map config_data = {add your config data here}
  /// Opensight_Analytics.initApp(config_data)

  factory OpensightCore.initApp(Map config) {
    Config conf = Config.fromJson(config);
    var app = OpensightCore(
        appDetails: conf,
        transport: TransportClient(conf.analyticsApi!, conf.token!));
    return app;
  }
}
