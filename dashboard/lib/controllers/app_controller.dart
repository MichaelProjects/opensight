import 'package:dashboard/model/application.dart';
import 'package:dashboard/utils/api/client.dart';
import 'package:flutter/material.dart';

enum AppStatus { none, loading, loaded, error }

class ApplicationProvider with ChangeNotifier {
  AppStatus _appStatus = AppStatus.none;
  List<Application> _apps = [];
  late Application _selectedApp;
  AppStatus get appStatus => _appStatus;
  List<Application> get apps => _apps;
  Application get selectedApp => _selectedApp;

  Future fetchApplications() async {
    _appStatus = AppStatus.loading;
    var response = await ApiClient().getApplications();
    _appStatus = AppStatus.loaded;
    List<Application> apps = [];
    for (var app in response["data"]) {
      apps.add(Application.fromJson(app));
    }
    _apps = apps;
  }
}
