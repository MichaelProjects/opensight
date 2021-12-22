# Opensight for Flutter
[![codecov](https://codecov.io/gh/MichaelProjects/opensightSDK/branch/master/graph/badge.svg?token=36EP9PZJQ7)](https://codecov.io/gh/MichaelProjects/opensightSDK) [![analytics_sdk_flutter](https://github.com/MichaelProjects/opensightSDK/actions/workflows/tests.yaml/badge.svg?branch=master)](https://github.com/MichaelProjects/opensightSDK/actions/workflows/tests.yaml)
## Quickstart

Move the generated config file from the Opensight Dashboard into the root folder of your app. Then add this code snipped to your app.

```dart
void main() {
  WidgetsFlutterBinding.ensureInitialized();
  OpensightAnalytics.initApp({
  "url": "http://example.host",
  "app_id": "application_id",
  "name": "name of your app",
  "token": "your_application_token",
  "package_name": "io.app"
  });
  runApp(ExampleApp());
}
...
```

and now your app is connected with the opensight service.

## Getting Started
Documentation comming soon ->

To get started with Opensight please, follow the instructions on http://opensight.io/docs

## issues or feedback
Please file specific issues, bugs, or feature requests in our [issue tracker](https://github.com/MichaelProjects/opensightSDK/issues/new)