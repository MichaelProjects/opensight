import 'package:dashboard/controllers/timeline_controller.dart';
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

List<String> headers = [
  "session_id",
  "application_id",
  "analytic",
  "event",
  "crash"
];

class _ExploreState extends State<Explore> {
  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    AnalyticModel analyticsController =
        Provider.of<AnalyticModel>(context, listen: false);
    ApplicationModel appController =
        Provider.of<ApplicationModel>(context, listen: false);
    TimelineController timelineController =
        Provider.of<TimelineController>(context, listen: false);

    analyticsController.fetchEntrys(appController.selectedApp.appID);

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
        firstCell: Container(child: gen()[0]), children: gen());
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

    return topOverlay.Overlay(
        child: Container(
            height: size.height - topbarHeight,
            width: size.width - sidebarWidth,
            child: Container(
                margin: EdgeInsets.all(15),
                child: ExpandableTable(
                    rows: rows,
                    header: header,
                    scrollShadowColor: Theme.of(context).focusColor))));
  }
}
