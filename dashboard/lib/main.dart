import 'package:dashboard/controllers/dashboard/analytics_controller.dart';
import 'package:dashboard/controllers/app_controller.dart';
import 'package:dashboard/screens/app_select/app_select.dart';
import 'package:dashboard/screens/overlay/overlay.dart' as topOverlay;
import 'package:dashboard/screens/dashboard/dashboard.dart';
import 'package:dashboard/utils/dark_theme.dart';
import 'package:dashboard/utils/light_theme.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import 'controllers/create_app_controller.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MultiProvider(
        providers: [
          ChangeNotifierProvider(
            create: (_) => ApplicationProvider(),
          ),
          ChangeNotifierProvider(create: (_) => AnalyticController()),
          ChangeNotifierProvider(create: (_) => CreateAppController()),
        ],
        child: MaterialApp(
            title: 'Dashboard | Opensight',
            debugShowCheckedModeBanner: false,
            darkTheme: buildDarkThemeData(context),
            theme: buildLightThemeData(context),
            home: Scaffold(body: AppSelect())));
    //topOverlay.Overlay(child: Dashboard())));
  }
}
