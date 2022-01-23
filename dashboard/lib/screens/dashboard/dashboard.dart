import 'package:dashboard/controllers/dashboard/analytics_controller.dart';
import 'package:dashboard/controllers/dashboard/sidebar/app_controller.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

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
      return await analyticsController.fetchEntrys(appController.appId);
    }

    return Expanded(
        child: SizedBox(
            height: size.height - 70,
            child: FutureBuilder(
                future: _getData(),
                builder: (context, snap) {
                  if (snap.hasData) {
                    return Column(
                      children: [SelectableText(snap.data.toString())],
                    );
                  } else {
                    return const SelectableText("Loading...");
                  }
                })));
  }
}
