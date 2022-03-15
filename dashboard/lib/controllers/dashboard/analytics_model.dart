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
  List<PieChartSectionData> _appVersionData = [];
  List<FlSpot> _sessionCountData = [];

  AnalyticsState _analyticsState = AnalyticsState.none;
  AnalyticsState _userHistoryState = AnalyticsState.none;
  AnalyticsState _newUserState = AnalyticsState.none;
  AnalyticsState _displaySizeState = AnalyticsState.none;
  AnalyticsState _sessionLengthHistoryState = AnalyticsState.none;
  AnalyticsState _appVersionState = AnalyticsState.none;
  AnalyticsState _sessionCountState = AnalyticsState.none;

  Map get analyticData => _analyticData;
  List<FlSpot> get userHistoryData => _userHistoryData;
  List<FlSpot> get newUserData => _newUserData;
  String get displaySizeData => _displaySizeData;
  List<FlSpot> get sessionLengthHistoryData => _sessionLengthHistoryData;
  List<PieChartSectionData> get appVersionData => _appVersionData;
  List<FlSpot> get sessionCountData => _sessionCountData;

  AnalyticsState get analyticsState => _analyticsState;
  AnalyticsState get userHistoryState => _userHistoryState;
  AnalyticsState get newUserState => _newUserState;
  AnalyticsState get displaySizeState => _displaySizeState;
  AnalyticsState get sessionHistoryState => _sessionLengthHistoryState;
  AnalyticsState get appVersionState => _appVersionState;
  AnalyticsState get sessionCountState => _sessionCountState;

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
        data.add(parseFlspot(day));
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
        data.add(parseFlspot(day));
      }
    }
    _newUserData = data;
    _newUserState = AnalyticsState.loaded;
    notifyListeners();
  }

  Future getSessionCount(String appId, int start, int end) async {
    _sessionCountState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getDisplaySize(appId, start, end);
    List<FlSpot> data = [];
    if (response["error"] == false) {
      for (var day in response["data"]["data"]) {
        data.add(parseFlspot(day));
      }
    }
    _sessionCountData = data;

    _sessionCountState = AnalyticsState.loaded;
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
        data.add(parseFlspot(day));
      }
    }
    _sessionLengthHistoryData = data;
    _sessionLengthHistoryState = AnalyticsState.loaded;
    notifyListeners();
  }

  Future getAppVersion(String appId, int start, int end) async {
    _appVersionState = AnalyticsState.loading;
    notifyListeners();

    var response = await ApiClient().getAppVersion(appId, start, end);
    List<PieChartSectionData> data = [];
    if (response["error"] == false) {
      int counter = 0;
      for (Map day in response["data"]["data"]) {
        counter += day["counter"] as int;
      }
      for (Map day in response["data"]["data"]) {
        double precentage = calculatePercentage(counter, day["counter"]);
        data.add(PieChartSectionData(
          value: precentage,
          title: day["day"],
        ));
      }
      _appVersionData = data;
      _appVersionState = AnalyticsState.loaded;
      notifyListeners();
    }
  }
}

double calculatePercentage(int total, int value) {
  double percentage = (value / total) * 100;
  return percentage;
}

FlSpot parseFlspot(Map day) {
  double formatDay = double.parse(day["day"].toString().replaceAll("-", ""));
  var dataPoint = FlSpot(formatDay, day["counter"]);
  return dataPoint;
}
