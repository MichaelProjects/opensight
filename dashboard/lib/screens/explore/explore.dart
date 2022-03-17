import 'package:dashboard/controllers/timeline_controller.dart';
import 'package:dashboard/utils/sizes.dart';
import 'package:flutter/material.dart';
import 'package:dashboard/screens/overlay/overlay.dart' as topOverlay;
import 'package:provider/provider.dart';

import '../../controllers/app_controller.dart';
import '../../controllers/dashboard/analytics_model.dart';

class Explore extends StatefulWidget {
  Explore({Key? key}) : super(key: key);

  @override
  State<Explore> createState() => _ExploreState();
}

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

    return topOverlay.Overlay(
        child: Container(
            height: size.height - topbarHeight,
            width: size.width - sidebarWidth,
            color: Colors.red,
            child: Table(
              children: [],
            )));
  }
}
