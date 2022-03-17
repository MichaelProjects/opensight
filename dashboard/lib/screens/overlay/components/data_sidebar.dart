import 'package:coolicons/coolicons.dart';
import 'package:dashboard/screens/overlay/components/sidebar_button.dart';
import 'package:dashboard/screens/overlay/components/sidebar_container.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:routemaster/routemaster.dart';

import '../../../controllers/app_controller.dart';

class DataSidebar extends StatelessWidget {
  const DataSidebar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    ApplicationModel appController = Provider.of<ApplicationModel>(context);
    return SidebarContainer(
        child: Column(
      children: [
        SidebarButton(
            deactivated: false,
            label: "Dashboard",
            icon: Coolicons.bar_chart,
            onPressed: () {
              Routemaster.of(context)
                  .push('/dashboard/${appController.selectedApp.appID}');
            }),
        SidebarButton(
            deactivated: true,
            label: "Events",
            icon: Coolicons.check_all_big,
            onPressed: () {}),
        SidebarButton(
            deactivated: true,
            label: "Crashes",
            icon: Coolicons.settings_future,
            onPressed: () {}),
        SidebarButton(
            deactivated: false,
            label: "Explore",
            icon: Coolicons.bar_chart_circle,
            onPressed: () {
              Routemaster.of(context)
                  .push('/explore/${appController.selectedApp.appID}');
            }),
      ],
    ));
  }
}
