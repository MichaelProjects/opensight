import 'package:dashboard/utils/api/client.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';

enum AnalyticsState { none, loading, loaded, error }

class AnalyticModel with ChangeNotifier {
  Map _analyticData = {};
  List<FlSpot> _userHistoryData = [];
  List<FlSpot> _newUserData = [];
  String _displaySizeData = "";
  List<FlSpot> _sessionLengthHistoryData = [];

  AnalyticsState _analyticsState = AnalyticsState.none;
  AnalyticsState _userHistoryState = AnalyticsState.none;
  AnalyticsState _newUserState = AnalyticsState.none;
  AnalyticsState _displaySizeState = AnalyticsState.none;
  AnalyticsState _sessionLengthHistoryState = AnalyticsState.none;

  Map get analyticData => _analyticData;
  List<FlSpot> get userHistoryData => _userHistoryData;
  List<FlSpot> get newUserData => _newUserData;
  String get displaySizeData => _displaySizeData;
  List<FlSpot> get sessionLengthHistoryData => _sessionLengthHistoryData;

  AnalyticsState get analyticsState => _analyticsState;
  AnalyticsState get userHistoryState => _userHistoryState;
  AnalyticsState get newUserState => _newUserState;
  AnalyticsState get displaySizeState => _displaySizeState;
  AnalyticsState get sessionHistoryState => _sessionLengthHistoryState;

  Future fetchEntrys(String appId) async {
    _analyticsState = AnalyticsState.loading;
    var response = await ApiClient().getAnalticsEntrys(appId);
    _analyticData = response;
    _analyticsState = AnalyticsState.loaded;
  }

  Future getUserHistory(String appId, int start, int end) async {
    _userHistoryState = AnalyticsState.loading;
    notifyListeners();
    Map response = await ApiClient().getUserHistory(appId, start, end);
    List<FlSpot> data = [];
    if (response["error"] == false) {
      for (var day in response["data"]["data"]) {
        double formatDay = double.parse(day["day"].toString().split("-").last);
        var dataPoint = FlSpot(formatDay, day["counter"]);
        data.add(dataPoint);
      }
    }
    _userHistoryData = data;
    _userHistoryState = AnalyticsState.loaded;
    notifyListeners();
  }

  Future getNewUserHistory(String appId, int start, int end) async {
    _newUserState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getNewUsers(appId, start, end);
    List<FlSpot> data = [];
    if (response["error"] == false) {
      for (var day in response["data"]["data"]) {
        double formatDay = double.parse(day["day"].toString().split("-").last);
        var dataPoint = FlSpot(formatDay, day["counter"]);
        data.add(dataPoint);
      }
    }
    print("new user data: $data");
    _newUserData = data;
    _newUserState = AnalyticsState.loaded;
    notifyListeners();
  }

  Future getDisplaySizeMetrics(String appId, int start, int end) async {
    _displaySizeState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getDisplaySize(appId, start, end);
    _displaySizeData = response["data"]["data"]["display_size"];
    _displaySizeState = AnalyticsState.loaded;
    notifyListeners();
  }

  Future getSessionLengthHistory(String appId, int start, int end) async {
    _sessionLengthHistoryState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getSessionLengthHistory(appId, start, end);
    List<FlSpot> data = [];
    if (response["error"] == false) {
      for (var day in response["data"]["data"]) {
        double formatDay = double.parse(day["day"].toString().split("-").last);
        var dataPoint = FlSpot(formatDay, day["counter"]);
        data.add(dataPoint);
      }
    }
    print("session length: $data");
    _sessionLengthHistoryData = data;
    _sessionLengthHistoryState = AnalyticsState.loaded;
    notifyListeners();
  }
}
