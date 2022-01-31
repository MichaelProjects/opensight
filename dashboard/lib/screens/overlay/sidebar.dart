import 'package:dashboard/glob_components/app_details.dart';
import 'package:dashboard/screens/overlay/components/data_sidebar.dart';
import 'package:dashboard/utils/sizes.dart';
import 'package:flutter/material.dart';

class Sidebar extends StatelessWidget {
  const Sidebar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    return Container(
        width: sidebarWidth,
        height: size.height - topbarHeight,
        padding: const EdgeInsets.only(top: 5),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisAlignment: MainAxisAlignment.start,
          children: [
            Appdetails(),
            const DataSidebar(),
          ],
        ));
  }
}
