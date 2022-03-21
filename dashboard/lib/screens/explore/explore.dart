import 'package:dashboard/controllers/timeline_controller.dart';
import 'package:dashboard/model/explore_entry.dart';
import 'package:dashboard/utils/sizes.dart';
import 'package:flutter/material.dart';
import 'package:dashboard/screens/overlay/overlay.dart' as topOverlay;
import 'package:flutter_expandable_table/flutter_expandable_table.dart';
import 'package:provider/provider.dart';

import '../../controllers/app_controller.dart';
import '../../controllers/dashboard/analytics_model.dart';

class Explore extends StatefulWidget {
  Explore({Key? key}) : super(key: key);

  @override
  State<Explore> createState() => _ExploreState();
}

//List<String> headers = ["application_id", "analytic", "event", "crash"];

List<String> headers = [
  "application_id",
  "creationTime",
  "os",
  "deviceSize",
  "newUser",
  "country",
  "deviceType",
  "version"
];

class _ExploreState extends State<Explore> {
  @override
  void initState() {
    AnalyticModel analyticsController =
        Provider.of<AnalyticModel>(context, listen: false);
    ApplicationModel appController =
        Provider.of<ApplicationModel>(context, listen: false);
    TimelineController timelineController =
        Provider.of<TimelineController>(context, listen: false);

    analyticsController.fetchEntrys(appController.selectedApp.appID, 200,
        timelineController.startTime, timelineController.endTime);
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    AnalyticModel analyticsController = Provider.of<AnalyticModel>(context);

    List<Widget> gen() {
      List<Widget> a = [];
      for (var x in headers) {
        var y = Container(
            color: Theme.of(context).primaryColor,
            child: Center(child: Text(x)));
        a.add(y);
      }
      return a;
    }

    ExpandableTableHeader header = ExpandableTableHeader(
        firstCell: Container(
            child: Container(
                color: Theme.of(context).primaryColor,
                child: Center(child: Text("session_id")))),
        children: gen());
//Creation rows
    List<ExpandableTableRow> rows = List.generate(
        headers.length,
        (rowIndex) => ExpandableTableRow(
              height: 50,
              firstCell: Container(
                  color: Theme.of(context).primaryColor,
                  margin: EdgeInsets.all(1),
                  child: Center(
                      child: Text(
                    'Row $rowIndex',
                  ))),
              children: List<Widget>.generate(
                  headers.length,
                  (columnIndex) => Container(
                      color: Theme.of(context).primaryColor,
                      margin: EdgeInsets.all(1),
                      child: Center(
                          child: Text(
                        'Cell $rowIndex:$columnIndex',
                      )))),
            ));

    List<ExpandableTableRow> parseRows(ExploreEntry data) {
      List<ExpandableTableRow> rows = [];
      for (var x in data.analytic) {
        var y = ExpandableTableRow(
            height: 50,
            firstCell: Container(
                color: Theme.of(context).primaryColor,
                margin: EdgeInsets.all(1),
                child: Center(
                    child: Text(
                  x.sessionId,
                ))),
            children: x
                .to_list()
                .map((e) => Container(
                    color: Theme.of(context).primaryColor,
                    margin: EdgeInsets.all(1),
                    child: Center(child: Text(e))))
                .toList());
        rows.add(y);
      }
      return rows;
    }

    return topOverlay.Overlay(
        child: SizedBox(
            height: size.height - topbarHeight,
            width: size.width - sidebarWidth,
            child: Builder(builder: (context) {
              switch (analyticsController.exploreState) {
                case AnalyticsState.loading:
                  return Center(child: CircularProgressIndicator());
                case AnalyticsState.loaded:
                  return Container(
                      margin: EdgeInsets.all(15),
                      child: ExpandableTable(
                          rows: parseRows(analyticsController.exploreData),
                          header: header,
                          scrollShadowColor: Theme.of(context).focusColor));
                default:
                  return Center(child: Text("An error occurred while loading"));
              }
            })));
  }
}
