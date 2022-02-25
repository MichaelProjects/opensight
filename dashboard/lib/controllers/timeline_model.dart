import 'package:flutter/material.dart';

class TimelineModel with ChangeNotifier {
  int _startTime = 0;
  int _endTime = 0;

  int get startTime => _startTime;
  int get endTime => _endTime;

  getLast7Day() {
    var end = DateTime.now().toUtc().millisecondsSinceEpoch;

    notifyListeners();
  }
}
