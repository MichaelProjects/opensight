import 'package:dashboard/controllers/dashboard/analytics_model.dart';
import 'package:dashboard/controllers/app_controller.dart';
import 'package:dashboard/screens/dashboard/components/graphs/line_chart.dart';
import 'package:dashboard/screens/dashboard/components/graphs/pi_chart.dart';
import 'package:dashboard/utils/sizes.dart';
import 'package:fl_chart/fl_chart.dart';
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
  void initState() {
    super.initState();
    WidgetsBinding.instance?.addPostFrameCallback((_) {
      AnalyticModel analyticsController =
          Provider.of<AnalyticModel>(context, listen: false);
      ApplicationModel appController =
          Provider.of<ApplicationModel>(context, listen: false);
      analyticsController.fetchEntrys(appController.selectedApp.appID);
    });
  }

  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    AnalyticModel analyticsController = Provider.of<AnalyticModel>(context);
    print(analyticsController.analyticsState);
    return topOverlay.Overlay(
        child: Container(
            height: size.height - topbarHeight,
            width: size.width - sidebarWidth,
            child: Builder(builder: (context) {
              switch (analyticsController.analyticsState) {
                case AnalyticsState.loading:
                  return Center(child: CircularProgressIndicator());
                case AnalyticsState.loaded:
                  return Builder(builder: (context) {
                    return Scrollbar(
                        child: ListView(children: [
                      Card(
                          child: Container(
                              padding: EdgeInsets.all(10),
                              height: 400,
                              width: 100,
                              child: SimpleLineChart.withSampleData())),
                      Card(child: DonutPieChart.withSampleData()),
                    ]));
                  });
                default:
                  return const Text("Error");
              }
            })));
  }
}
