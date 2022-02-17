import 'package:flutter/material.dart';
import 'package:opensight_analytics/opensight_analytics.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  // OpensightAnalytics().initApp({
  //   "url": "http://example.host",
  //   "app_id": "application_id",
  //   "name": "name of your app",
  //   "token": "your_application_token",
  //   "package_name": "io.app"
  // });
  OpensightAnalytics().initApp({
    "url": "https://app-dev.fynancial.de",
    "app_id": "52e3fcd0-2595-4ef4-a733-9cdb5506bd59",
    "name": "Stackblog",
    "token": "12462544341020901870",
    "package_name": "io.stackblog"
  });
  runApp(const ExampleApp());
}

class ExampleApp extends StatelessWidget {
  const ExampleApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(home: Home());
  }
}

class Home extends StatefulWidget {
  Home({Key? key}) : super(key: key);

  @override
  State<Home> createState() => _HomeState();
}

class _HomeState extends State<Home> {
  int counter = 0;
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        floatingActionButton: FloatingActionButton(
          onPressed: () {
            setState(() {
              counter++;
            });
          },
        ),
        body: Column(children: [Center(child: CircularProgressIndicator()),Center(child: Text("Counter: $counter"))]));
  }
}
