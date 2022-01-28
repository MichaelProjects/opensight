import 'package:dashboard/controllers/dashboard/analytics_controller.dart';
import 'package:dashboard/controllers/app_controller.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:dashboard/screens/overlay/overlay.dart' as topOverlay;

class Dashboard extends StatefulWidget {
  const Dashboard({Key? key}) : super(key: key);

  @override
  _DashboardState createState() => _DashboardState();
}

class _DashboardState extends State<Dashboard> {
  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    AnalyticController analyticsController =
        Provider.of<AnalyticController>(context);
    ApplicationProvider appController =
        Provider.of<ApplicationProvider>(context);
    Future _getData() async {
      await Future.delayed(const Duration(seconds: 2));
      return await analyticsController
          .fetchEntrys(appController.selectedApp.appID);
    }

    return Scaffold(body: topOverlay.Overlay(child: Container()));
  }
}
