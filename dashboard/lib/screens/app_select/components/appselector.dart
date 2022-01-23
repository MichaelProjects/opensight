// ignore_for_file: prefer_const_constructors

import 'package:dashboard/controllers/dashboard/sidebar/app_controller.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class AppSelector extends StatefulWidget {
  @override
  _AppSelectorState createState() => _AppSelectorState();
}

class _AppSelectorState extends State<AppSelector> {
  @override
  Widget build(BuildContext context) {
    ApplicationProvider appController =
        Provider.of<ApplicationProvider>(context);
    var data = MediaQuery.of(context).size;
    return Column(
      children: [
        SizedBox(height: 20),
        SelectableText("Your projects",
            style: Theme.of(context).textTheme.headline6),
        SizedBox(height: 20),
        FutureBuilder(
            future: appController.fetchApplications(),
            builder: (context, snap) {
              if (!snap.hasData) {
                return Center(child: CircularProgressIndicator());
              }
              return Text(snap.data.toString());
            })
      ],
    );
  }
}
