import 'package:opensight_core/opensight_core.dart';

import 'collection.dart';
import 'session.dart';

class OpensightAnalytics {
  /// [OpensightAnalytics] is the public api for this plugin, it is simple to implement just call [OpensightSDK.initApp] and the function takes your config.
  ///
  /// example:
  ///
  /// Map config_data = {add your config data here}
  /// Opensight_Analytics.initApp(config_data)
  Future initApp(Map configData) async {
    OpensightCore app = OpensightCore.initApp(configData);
    Collection data = await Collection.collect();
    Map response = await app.transport.dispatchData(
        data.prepareToSend(), "/analytic/v1/${app.appDetails.appId}/session");
    if (response != {}) {
      Session.id = response["session_id"];
    }
    tracking(app);
  }
}
