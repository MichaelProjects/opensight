import 'package:dashboard/screens/overlay/components/app_details.dart';
import 'package:dashboard/screens/overlay/components/data_sidebar.dart';
import 'package:flutter/material.dart';

class Sidebar extends StatelessWidget {
  const Sidebar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        width: 225,
        padding: EdgeInsets.all(10),
        child: Column(
          children: [Appdetails(), DataSidebar()],
        ));
  }
}
