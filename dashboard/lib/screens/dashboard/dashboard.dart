import 'package:dashboard/controllers/dashboard/analytics_model.dart';
import 'package:dashboard/controllers/app_controller.dart';
import 'package:dashboard/controllers/timeline_controller.dart';
import 'package:dashboard/screens/dashboard/components/chart_wrapper.dart';
import 'package:dashboard/screens/dashboard/components/lineChart.dart';
import 'package:dashboard/screens/dashboard/components/pie_chart.dart';
import 'package:dashboard/utils/sizes.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:dashboard/screens/overlay/overlay.dart' as topOverlay;

class Dashboard extends StatefulWidget {
  const Dashboard({Key? key}) : super(key: key);

  @override
  _DashboardState createState() => _DashboardState();
}

TimeFrame before = TimeFrame.notSpecified;

class _DashboardState extends State<Dashboard> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance?.addPostFrameCallback((_) {
      AnalyticModel analyticsController =
          Provider.of<AnalyticModel>(context, listen: false);
      ApplicationModel appController =
          Provider.of<ApplicationModel>(context, listen: false);
      TimelineController timelineController =
          Provider.of<TimelineController>(context, listen: false);
      analyticsController.getUserHistory(appController.selectedApp.appID,
          timelineController.startTime, timelineController.endTime);
      analyticsController.getNewUserHistory(appController.selectedApp.appID,
          timelineController.startTime, timelineController.endTime);
      analyticsController.getSessionLengthHistory(
          appController.selectedApp.appID,
          timelineController.startTime,
          timelineController.endTime);
      analyticsController.getAppVersion(appController.selectedApp.appID,
          timelineController.startTime, timelineController.endTime);
    });
  }

  @override
  void didChangeDependencies() {
    WidgetsBinding.instance?.addPostFrameCallback((_) {
      TimelineController timelineController =
          Provider.of<TimelineController>(context, listen: false);
      if (timelineController.timeFrame != before) {
        AnalyticModel analyticsController =
            Provider.of<AnalyticModel>(context, listen: false);
        ApplicationModel appController =
            Provider.of<ApplicationModel>(context, listen: false);
        analyticsController.getUserHistory(appController.selectedApp.appID,
            timelineController.startTime, timelineController.endTime);
        analyticsController.getNewUserHistory(appController.selectedApp.appID,
            timelineController.startTime, timelineController.endTime);
        analyticsController.getSessionLengthHistory(
            appController.selectedApp.appID,
            timelineController.startTime,
            timelineController.endTime);
        analyticsController.getAppVersion(appController.selectedApp.appID,
            timelineController.startTime, timelineController.endTime);
        before = timelineController.timeFrame;
      }
    });
    super.didChangeDependencies();
  }

  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    AnalyticModel analyticsController = Provider.of<AnalyticModel>(context);
    TimelineController timeController =
        Provider.of<TimelineController>(context);
    return topOverlay.Overlay(
        child: SizedBox(
            height: size.height - topbarHeight,
            width: size.width - sidebarWidth,
            child: Builder(builder: (context) {
              return LayoutBuilder(builder: (context, constraints) {
                return GridView.count(
                  crossAxisCount: constraints.maxWidth > 1200
                      ? 2
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
