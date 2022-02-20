import 'package:dashboard/controllers/dashboard/analytics_model.dart';
import 'package:dashboard/controllers/app_controller.dart';
import 'package:dashboard/screens/dashboard/components/chart_wrapper.dart';
import 'package:dashboard/screens/dashboard/components/lineChart.dart';
import 'package:dashboard/screens/dashboard/components/pie_chart.dart';
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
  void didUpdateWidget(covariant Dashboard oldWidget) {
    print("update");
    super.didUpdateWidget(oldWidget);
  }

  @override
  void initState() {
    super.initState();
    int start = 1633680415;
    int end = DateTime.now().toUtc().millisecondsSinceEpoch;
    WidgetsBinding.instance?.addPostFrameCallback((_) {
      AnalyticModel analyticsController =
          Provider.of<AnalyticModel>(context, listen: false);
      ApplicationModel appController =
          Provider.of<ApplicationModel>(context, listen: false);
      analyticsController.getUserHistory(
          appController.selectedApp.appID, start, end);
      analyticsController.getNewUserHistory(
          appController.selectedApp.appID, start, end);
      analyticsController.getSessionLengthHistory(
          appController.selectedApp.appID, start, end);
      analyticsController.getAppVersion(
          appController.selectedApp.appID, start, end);
    });
  }

  List<FlSpot> lulData = [
    FlSpot(19, 24),
    FlSpot(20, 24),
  ];
  // todo fl_chart is stuck if there is only one element in the list
  Widget pieChart = PieChart(
    PieChartData(sections: [PieChartSectionData()]),
    swapAnimationDuration: const Duration(milliseconds: 150), // Optional
    swapAnimationCurve: Curves.linear, // Optional
  );

  Widget appVersions = BarChart(BarChartData(
    barGroups: [
      BarChartGroupData(x: 1, barRods: [
        BarChartRodData(y: 100),
      ])
    ],
    gridData: FlGridData(show: false),
    borderData: FlBorderData(show: false),
  ));
  Widget screenSizes = BarChart(BarChartData(
    barGroups: [
      BarChartGroupData(x: 1, barRods: [
        BarChartRodData(y: 100),
        BarChartRodData(y: 80),
      ])
    ],
    gridData: FlGridData(show: false),
    borderData: FlBorderData(show: false),
  ));
  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    AnalyticModel analyticsController = Provider.of<AnalyticModel>(context);
    return topOverlay.Overlay(
        child: SizedBox(
            height: size.height - topbarHeight,
            width: size.width - sidebarWidth,
            child: Builder(builder: (context) {
              return LayoutBuilder(builder: (context, constraints) {
                return GridView.count(
                  crossAxisCount: constraints.maxWidth > 1200
                      ? 3
                      : constraints.maxWidth < 800
                          ? 1
                          : 2,
                  childAspectRatio: 1.9,
                  padding: const EdgeInsets.all(16),
                  crossAxisSpacing: 16,
                  mainAxisSpacing: 16,
                  children: [
                    ChartWrapper(
                        title: "Active Users",
                        state: analyticsController.userHistoryState,
                        child: LineChartCard(
                          analyticsController.userHistoryData,
                        )),
                    ChartWrapper(
                        title: "New User",
                        state: analyticsController.newUserState,
                        child: LineChartCard(
                          analyticsController.newUserData,
                        )),
                    ChartWrapper(
                        title: "Session Length",
                        state: analyticsController.sessionHistoryState,
                        child: LineChartCard(
                          analyticsController.sessionLengthHistoryData,
                        )),
                    ChartWrapper(
                        title: "App Versions",
                        state: analyticsController.appVersionState,
                        child:
                            CustomPieChart(analyticsController.appVersionData)),
                  ],
                );
              });
            })));
  }
}

bool isLoading(AnalyticsState state) {
  if (state != AnalyticsState.loaded) {
    return true;
  }
  return false;
}
