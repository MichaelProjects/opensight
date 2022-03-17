import 'package:dashboard/model/timeseries.dart';
import 'package:dashboard/utils/api/client.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

enum AnalyticsState { none, loading, loaded, error }

class AnalyticModel with ChangeNotifier {
  Map _analyticData = {};
  Timeseries _userHistoryData = Timeseries.newObject();
  Timeseries _newUserData = Timeseries.newObject();
  String _displaySizeData = "";
  Timeseries _sessionLengthHistoryData = Timeseries.newObject();
  List<PieChartSectionData> _appVersionData = [];
  Timeseries _sessionCountData = Timeseries.newObject();

  AnalyticsState _analyticsState = AnalyticsState.none;
  AnalyticsState _userHistoryState = AnalyticsState.none;
  AnalyticsState _newUserState = AnalyticsState.none;
  AnalyticsState _displaySizeState = AnalyticsState.none;
  AnalyticsState _sessionLengthHistoryState = AnalyticsState.none;
  AnalyticsState _appVersionState = AnalyticsState.none;
  AnalyticsState _sessionCountState = AnalyticsState.none;

  Map get analyticData => _analyticData;
  Timeseries get userHistoryData => _userHistoryData;
  Timeseries get newUserData => _newUserData;
  String get displaySizeData => _displaySizeData;
  Timeseries get sessionLengthHistoryData => _sessionLengthHistoryData;
  List<PieChartSectionData> get appVersionData => _appVersionData;
  Timeseries get sessionCountData => _sessionCountData;

  AnalyticsState get analyticsState => _analyticsState;
  AnalyticsState get userHistoryState => _userHistoryState;
  AnalyticsState get newUserState => _newUserState;
  AnalyticsState get displaySizeState => _displaySizeState;
  AnalyticsState get sessionHistoryState => _sessionLengthHistoryState;
  AnalyticsState get appVersionState => _appVersionState;
  AnalyticsState get sessionCountState => _sessionCountState;

  Future fetchEntrys(String appId) async {
    _analyticsState = AnalyticsState.loading;
    Map response = await ApiClient().getAnalticsEntrys(appId);
    if (response["error"] == false) {
      _analyticData = response;
      _analyticsState = AnalyticsState.loaded;
    } else {
      _analyticsState = AnalyticsState.error;
    }
    notifyListeners();
  }

  Future getUserHistory(String appId, int start, int end) async {
    _userHistoryState = AnalyticsState.loading;
    notifyListeners();
    Map response = await ApiClient().getUserHistory(appId, start, end);
    if (response["error"] == false) {
      _userHistoryData = Timeseries.fromJson(response);
      _userHistoryState = AnalyticsState.loaded;
    } else {
      _userHistoryState = AnalyticsState.error;
    }
    notifyListeners();
  }

  Future getNewUserHistory(String appId, int start, int end) async {
    _newUserState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getNewUsers(appId, start, end);
    if (response["error"] == false) {
      _newUserData = Timeseries.fromJson(response);
      _newUserState = AnalyticsState.loaded;
    } else {
      _newUserState = AnalyticsState.error;
    }
    notifyListeners();
  }

  Future getSessionCount(String appId, int start, int end) async {
    _sessionCountState = AnalyticsState.loading;
    notifyListeners();
    var response = await ApiClient().getDisplaySize(appId, start, end);
    List<FlSpot> data = [];
    double counter = 0;

    if (response["error"] == false) {
      _sessionCountData = Timeseries.fromJson(response);
      _sessionCountState = AnalyticsState.loaded;
    } else {
      _sessionCountState = AnalyticsState.error;
    }
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
    double counter = 0;

    if (response["error"] == false) {
      _sessionLengthHistoryData = Timeseries.fromJson(response);
      _sessionLengthHistoryState = AnalyticsState.loaded;
    } else {
      _sessionLengthHistoryState = AnalyticsState.error;
    }
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

FlSpot parseFlspot(double counter, Map day) {
  DateFormat format = new DateFormat("yyyy-MM-dd");
  String rawDate = day["day"];
  double dateTamp = format.parse(rawDate).millisecondsSinceEpoch.toDouble();
  var calc = day["day"].toString().split("-");
  var dataPoint = FlSpot(counter, day["counter"]);
  return dataPoint;
}
