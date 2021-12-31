import 'package:dashboard/model/application.dart';
import 'package:dashboard/utils/api/client.dart';
import 'package:flutter/material.dart';

enum AppStatus { none, loading, loaded, error }

class ApplicationProvider with ChangeNotifier {
  AppStatus _appStatus = AppStatus.none;
  AppStatus get appStatus => _appStatus;

  Future<Application> fetchApplications() async {
    _appStatus = AppStatus.loading;
    var response = await ApiClient().getApplications();
    _appStatus = AppStatus.loaded;
    var app = Application.fromJson(response["data"][0]);
    return app;
  }
}
