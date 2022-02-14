import 'package:dashboard/utils/api/client.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';

enum AnalyticsState { none, loading, loaded, error }

class AnaylticsStorage {
  static List<FlSpot> userHistoryData = [];
  static List<FlSpot> newUserData = [];
  static List<FlSpot> displaySizeData = [];
  static List<FlSpot> sessionLengthHistoryData = [];

  setUserHistoryData(List<FlSpot> data) {
    userHistoryData = data;
  }

  setNewUserData(List<FlSpot> data) {
    newUserData = data;
  }

  setDisplaySizeData(List<FlSpot> data) {
    displaySizeData = data;
  }

  setSessionLengthHistoryData(List<FlSpot> data) {
    sessionLengthHistoryData = data;
  }
}

class AnalyticModel with ChangeNotifier {
  Map _analyticData = {};

  AnalyticsState _analyticsState = AnalyticsState.none;
  AnalyticsState _userHistoryState = AnalyticsState.none;
  AnalyticsState _newUserState = AnalyticsState.none;
  AnalyticsState _displaySizeState = AnalyticsState.none;
  AnalyticsState _sessionLengthHistoryState = AnalyticsState.none;

  Map get analyticData => _analyticData;

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
    AnaylticsStorage().setUserHistoryData(data);
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
    AnaylticsStorage().setNewUserData(data);
    _newUserState = AnalyticsState.loaded;
    notifyListeners();
  }

  Future getDisplaySizeMetrics(String appId, int start, int end) async {
    _displaySizeState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getDisplaySize(appId, start, end);
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
    AnaylticsStorage().setSessionLengthHistoryData(data);
    _sessionLengthHistoryState = AnalyticsState.loaded;
    notifyListeners();
  }
}
