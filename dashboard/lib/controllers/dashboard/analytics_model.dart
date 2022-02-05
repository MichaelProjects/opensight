import 'package:dashboard/utils/api/client.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';

enum AnalyticsState { none, loading, loaded, error }

class AnalyticModel with ChangeNotifier {
  Map _analyticData = {};
  AnalyticsState _analyticsState = AnalyticsState.none;
  Map get analyticData => _analyticData;
  AnalyticsState get analyticsState => _analyticsState;

  Future fetchEntrys(String appId) async {
    _analyticsState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getAnalticsEntrys(appId);
    print(response);
    _analyticData = response;
    _analyticsState = AnalyticsState.loaded;
    notifyListeners();
  }
}
