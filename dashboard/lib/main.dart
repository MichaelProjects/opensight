import 'package:dashboard/controllers/dashboard/analytics_model.dart';
import 'package:dashboard/controllers/app_controller.dart';
import 'package:dashboard/controllers/global/nav_route.dart';
import 'package:dashboard/utils/dark_theme.dart';
import 'package:dashboard/utils/light_theme.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:routemaster/routemaster.dart';

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
            create: (_) => ApplicationModel(),
          ),
          ChangeNotifierProvider(create: (_) => AnalyticModel()),
          ChangeNotifierProvider(create: (_) => CreateAppController()),
        ],
        child: MaterialApp.router(
          routerDelegate:
              RoutemasterDelegate(routesBuilder: (context) => routes),
          // ignore: prefer_const_constructors
          routeInformationParser: RoutemasterParser(),
          title: 'Dashboard | Opensight',
          debugShowCheckedModeBanner: false,
          darkTheme: buildDarkThemeData(context),
          theme: buildLightThemeData(context),
        ));
    //topOverlay.Overlay(child: Dashboard())));
  }
}
