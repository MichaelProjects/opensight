import 'package:dashboard/model/application.dart';
import 'package:dashboard/utils/api/client.dart';
import 'package:flutter/material.dart';

enum CreateAppState {
  none,
  enterName,
  enterPlattform,
  enterPackageName,
  loading,
  created,
  failed
}

enum Plattform { android, ios, web }

class CreateAppController with ChangeNotifier {
  CreateAppState _state = CreateAppState.none;
  String name = "";
  String plattform = "";
  String packageName = "";

  CreateAppState get state => _state;

  Future createApplication() async {
    _state = CreateAppState.loading;
    Map<String, String> payload = {
      "name": name,
      "package_name": packageName,
      "os": plattform,
    };
    print(payload);
    var response = await ApiClient().createApplication(payload);
    List<Application> apps = [];
    apps.add(Application.fromJson(response["data"]));
    _state = CreateAppState.created;
    notifyListeners();
  }

  /// setters for the controller variables
  void setName(String newName) {
    name = newName;
  }

  void setPlatform(Plattform newPlattform) {
    plattform = newPlattform.toString().split(".")[1];
  }

  void setPackageName(String newPackageName) {
    packageName = newPackageName;
  }

  /// updates the state if called.
  void enterName() {
    _state = CreateAppState.enterName;
    notifyListeners();
  }

  void enterPlattform() {
    _state = CreateAppState.enterPlattform;
    notifyListeners();
  }

  void enterPackageName() {
    _state = CreateAppState.enterPackageName;
    notifyListeners();
  }
}
