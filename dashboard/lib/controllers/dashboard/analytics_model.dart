import 'package:dashboard/utils/api/client.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';

enum AnalyticsState { none, loading, loaded, error }

class AnalyticModel with ChangeNotifier {
  Map _analyticData = {};
  AnalyticsState _analyticsState = AnalyticsState.none;
  AnalyticsState _userHistoryState = AnalyticsState.none;
  AnalyticsState _newUserState = AnalyticsState.none;
  AnalyticsState _displaySizeState = AnalyticsState.none;

  Map get analyticData => _analyticData;
  AnalyticsState get analyticsState => _analyticsState;
  AnalyticsState get userHistoryState => _userHistoryState;
  AnalyticsState get newUserState => _newUserState;
  AnalyticsState get displaySizeState => _displaySizeState;

  Future fetchEntrys(String appId) async {
    _analyticsState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getAnalticsEntrys(appId);
    _analyticData = response;
    _analyticsState = AnalyticsState.loaded;
    notifyListeners();
  }

  Future getUserHistory(String appId, int start, int end) async {
    _userHistoryState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getUserHistory(appId, start, end);
    _analyticData = response;
    print(response);
    _userHistoryState = AnalyticsState.loaded;
    notifyListeners();
  }

  Future getNewUserHistory(String appId, int start, int end) async {
    _newUserState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getNewUsers(appId, start, end);
    _analyticData = response;
    _newUserState = AnalyticsState.loaded;
    notifyListeners();
  }

  Future getDisplaySizeMetrics(String appId, int start, int end) async {
    _displaySizeState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getDisplaySize(appId, start, end);
    _analyticData = response;
    _displaySizeState = AnalyticsState.loaded;
    notifyListeners();
  }
}
