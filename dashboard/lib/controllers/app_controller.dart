import 'package:dashboard/model/application.dart';
import 'package:dashboard/utils/api/client.dart';
import 'package:dashboard/utils/guard.dart';
import 'package:flutter/material.dart';

enum AppStatus { none, loading, loaded, error }

class ApplicationModel with ChangeNotifier {
  AppStatus _appStatus = AppStatus.none;
  List<Application> _apps = [];
  Application _selectedApp = Application.mock();
  AppStatus get appStatus => _appStatus;
  List<Application> get apps => _apps;
  Application get selectedApp => _selectedApp;

  Future fetchApplications() async {
    _appStatus = AppStatus.loading;
    var response = await ApiClient().getApplications();
    List<Application> apps = [];
    for (var app in response["data"]) {
      apps.add(Application.fromJson(app));
    }
    _apps = apps;
    _appStatus = AppStatus.loaded;
    notifyListeners();
  }

  Future<Application> fetchApplication(String appId) async {
    _appStatus = AppStatus.loading;
    var response = await ApiClient().getApplication(appId);
    Application app = Application.fromJson(response["data"]);
    _appStatus = AppStatus.loaded;
    return app;
  }

  void setCurrentApp(Application app) {
    _selectedApp = app;
    AuthGuard.isAppSelected = true;
    notifyListeners();
  }
}
