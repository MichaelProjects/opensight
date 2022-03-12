import 'package:flutter/material.dart';

enum TimeFrame { today, days7, days30, monthCurrent, custom, notSpecified }

class TimelineController with ChangeNotifier {
  static DateTime end = DateTime.now().toUtc();
  int _startTime =
      DateTime(end.year, end.month, 1).toUtc().millisecondsSinceEpoch;
  int _endTime = DateTime.now().toUtc().millisecondsSinceEpoch;

  TimeFrame _timeFrame = TimeFrame.notSpecified;

  int get startTime => _startTime;
  int get endTime => _endTime;

  getLast30Days() {
    _startTime = DateTime.now()
        .toUtc()
        .subtract(Duration(days: 30))
        .millisecondsSinceEpoch;
    _endTime = DateTime.now().millisecondsSinceEpoch;
    _timeFrame = TimeFrame.days30;
    notifyListeners();
  }

  getCurrentMonth() {
    var end = DateTime.now().toUtc();
    var start = DateTime(end.year, end.month, 1).toUtc();
    _startTime = start.millisecondsSinceEpoch;
    _endTime = end.millisecondsSinceEpoch;
    _timeFrame = TimeFrame.monthCurrent;
    notifyListeners();
  }

  getLast7Day() {
    var start = DateTime.now()
        .toUtc()
        .subtract(Duration(days: 7))
        .millisecondsSinceEpoch;
    _startTime = start;
    _endTime = DateTime.now().toUtc().millisecondsSinceEpoch;
    _timeFrame = TimeFrame.days7;
    notifyListeners();
  }

  today() {
    var end = DateTime.now().toUtc();
    var start = DateTime(end.year, end.month, end.day, 0, 0, 0, 0).toUtc();
    _startTime = start.millisecondsSinceEpoch;
    _endTime = end.millisecondsSinceEpoch;
    _timeFrame = TimeFrame.today;
    notifyListeners();
  }

  custom(int start, int end) {
    _startTime = start;
    _endTime = end;
    _timeFrame = TimeFrame.custom;
    notifyListeners();
  }
}
