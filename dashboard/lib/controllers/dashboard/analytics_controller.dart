import 'package:dashboard/utils/api/client.dart';
import 'package:flutter/material.dart';

enum AnalyticsState { none, loading, loaded, error }

class AnalyticController with ChangeNotifier {
  AnalyticsState _analyticsState = AnalyticsState.none;
  AnalyticsState get analyticsState => _analyticsState;

  Future<Map> fetchEntrys(String appId) async {
    _analyticsState = AnalyticsState.loading;
    var response = await ApiClient().getAnalticsEntrys(appId);
    _analyticsState = AnalyticsState.loaded;
    return response;
  }
}
