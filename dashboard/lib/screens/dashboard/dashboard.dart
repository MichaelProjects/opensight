import 'package:dashboard/controllers/dashboard/analytics_model.dart';
import 'package:dashboard/controllers/app_controller.dart';
import 'package:dashboard/screens/dashboard/components/chart_wrapper.dart';
import 'package:dashboard/screens/dashboard/components/lineChart.dart';
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
    int start = 1633680415;
    int end = 1646301852;
    WidgetsBinding.instance?.addPostFrameCallback((_) {
      AnalyticModel analyticsController =
          Provider.of<AnalyticModel>(context, listen: false);
      ApplicationModel appController =
          Provider.of<ApplicationModel>(context, listen: false);
      analyticsController.fetchEntrys(appController.selectedApp.appID);
      analyticsController.getUserHistory(
          appController.selectedApp.appID, start, end);
      analyticsController.getNewUserHistory(
          appController.selectedApp.appID, start, end);
      analyticsController.getSessionLengthHistory(
          appController.selectedApp.appID, start, end);
    });
  }

  Widget sessionLength = LineChart(
    LineChartData(
      lineBarsData: [
        LineChartBarData(
          colors: const [Color(0xFFFF26B5), Color(0xFFFF5B5B)],
          dotData: FlDotData(show: false),
          belowBarData: BarAreaData(
            show: true,
            colors: const [Color(0x10FF26B5), Color(0x00FF26B5)],
            gradientFrom: const Offset(0.5, 0),
            gradientTo: const Offset(0.5, 1),
          ),
          spots: [
            const FlSpot(0, 24),
            const FlSpot(1, 24),
            const FlSpot(2, 40),
            const FlSpot(3, 84),
            const FlSpot(4, 100),
            const FlSpot(5, 80),
            const FlSpot(6, 64),
            const FlSpot(7, 86),
            const FlSpot(8, 108),
            const FlSpot(9, 105),
            const FlSpot(10, 105),
            const FlSpot(11, 124),
          ],
        )
      ],
      maxY: 140,
      gridData: FlGridData(show: false),
      borderData: FlBorderData(show: false),
    ),
  );

  Widget newUsers = LineChart(
    LineChartData(
      lineBarsData: [
        LineChartBarData(
          colors: const [Color(0xFFFF26B5), Color(0xFFFF5B5B)],
          dotData: FlDotData(show: false),
          belowBarData: BarAreaData(
            show: true,
            colors: const [Color(0x10FF26B5), Color(0x00FF26B5)],
            gradientFrom: const Offset(0.5, 0),
            gradientTo: const Offset(0.5, 1),
          ),
          spots: [
            const FlSpot(0, 24),
            const FlSpot(1, 24),
            const FlSpot(2, 40),
            const FlSpot(3, 84),
            const FlSpot(4, 100),
            const FlSpot(5, 80),
            const FlSpot(6, 64),
            const FlSpot(7, 86),
            const FlSpot(8, 108),
            const FlSpot(9, 105),
            const FlSpot(10, 105),
            const FlSpot(11, 124),
          ],
        )
      ],
      maxY: 140,
      gridData: FlGridData(show: false),
      borderData: FlBorderData(show: false),
    ),
  );

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
              switch (analyticsController.analyticsState) {
                case AnalyticsState.loading:
                  return const Center(child: CircularProgressIndicator());
                case AnalyticsState.loaded:
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
                            data: analyticsController.userHistoryData,
                            loading:
                                isLoading(analyticsController.userHistoryState),
                            child: LineChartCard(
                              analyticsController.userHistoryData,
                            )),
                        ChartWrapper(
                            title: "New User",
                            data: analyticsController.sessionLengthHistoryData,
                            loading: isLoading(
                                analyticsController.sessionHistoryState),
                            child: LineChartCard(
                              analyticsController.sessionLengthHistoryData,
                            )),
                        // ChartWrapper(
                        //     title: "Session Length",
                        //     loading: isLoading(
                        //         analyticsController.sessionHistoryState),
                        //     child: LineChartCard()),
                        // Card(
                        //     child: Padding(
                        //         padding: const EdgeInsets.only(
                        //             right: 18, top: 18, bottom: 18),
                        //         child: appVersions)),
                        // Card(
                        //     child: Padding(
                        //         padding: const EdgeInsets.only(
                        //             right: 18, top: 18, bottom: 18),
                        //         child: pieChart)),
                        // Card(
                        //     child: Padding(
                        //         padding: const EdgeInsets.only(
                        //             right: 18, top: 18, bottom: 18),
                        //         child: screenSizes)),
                      ],
                    );
                  });
                default:
                  return const Text("Error");
              }
            })));
  }
}

bool isLoading(AnalyticsState state) {
  if (state != AnalyticsState.loaded) {
    return true;
  }
  return false;
}
